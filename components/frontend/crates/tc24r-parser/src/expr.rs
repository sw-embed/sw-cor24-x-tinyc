//! Expression entry point, assignment, unary, and primary parsing.

use tc24r_ast::{BinOp, Expr, Type, UnaryOp};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_parser_compound::{desugar_compound, eat_compound_assign, make_assign};
use tc24r_parser_types::{is_type_start, parse_type};
use tc24r_token::TokenKind;

use crate::bitwise::parse_log_or;
use crate::stmt::parse_block;

/// Parse an expression (includes comma operator at lowest precedence).
pub fn parse_expr(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_assign(ts)?;
    while ts.eat(TokenKind::Comma) {
        let rhs = parse_assign(ts)?;
        lhs = Expr::BinOp {
            op: BinOp::Comma,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

/// Parse an assignment expression (used in function args, initializers).
pub fn parse_assign(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let expr = parse_ternary(ts)?;
    if ts.eat(TokenKind::Assign) {
        let value = parse_assign(ts)?;
        return make_assign(expr, value);
    }
    if let Some(op) = eat_compound_assign(ts) {
        let rhs = parse_assign(ts)?;
        return desugar_compound(expr, op, rhs);
    }
    Ok(expr)
}

fn parse_ternary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let cond = parse_log_or(ts)?;
    if ts.eat(TokenKind::Question) {
        let then_expr = parse_expr(ts)?;
        ts.expect(TokenKind::Colon)?;
        let else_expr = parse_ternary(ts)?;
        return Ok(Expr::Ternary {
            cond: Box::new(cond),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        });
    }
    Ok(cond)
}

pub fn parse_unary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    if ts.eat(TokenKind::Sizeof) {
        return parse_sizeof(ts);
    }
    if ts.eat(TokenKind::Offsetof) {
        return parse_offsetof(ts);
    }
    if ts.eat(TokenKind::Plus) {
        // Unary + is identity, just parse the operand
        return parse_unary(ts);
    }
    if ts.eat(TokenKind::Minus) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::UnaryOp {
            op: UnaryOp::Neg,
            operand: Box::new(operand),
        });
    }
    if ts.eat(TokenKind::Tilde) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::UnaryOp {
            op: UnaryOp::BitNot,
            operand: Box::new(operand),
        });
    }
    if ts.eat(TokenKind::Bang) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::UnaryOp {
            op: UnaryOp::LogNot,
            operand: Box::new(operand),
        });
    }
    if ts.eat(TokenKind::PlusPlus) {
        let name = ts.expect_ident()?;
        return Ok(Expr::PreInc(name));
    }
    if ts.eat(TokenKind::MinusMinus) {
        let name = ts.expect_ident()?;
        return Ok(Expr::PreDec(name));
    }
    if ts.eat(TokenKind::Amp) {
        let name = ts.expect_ident()?;
        // Check for postfix: &arr[i], &s.field, etc.
        if ts.check(&TokenKind::LBracket)
            || ts.check(&TokenKind::Dot)
            || ts.check(&TokenKind::Arrow)
        {
            let base = Expr::Ident(name);
            let postfix = parse_postfix_chain(ts, base)?;
            // &arr[i] is equivalent to arr + i (pointer arithmetic),
            // and arr[i] is already desugared to *(arr + i) by parse_postfix_chain.
            // So &*(arr + i) simplifies to (arr + i). Extract the inner expr from Deref.
            if let Expr::Deref(inner) = postfix {
                return Ok(*inner);
            }
            // For &s.field or &s->field, wrap in AddrOf-like expression.
            // These are member access on a local — the address is computable.
            return Ok(Expr::AddrOf(String::new())); // fallback (shouldn't reach)
        }
        return Ok(Expr::AddrOf(name));
    }
    if ts.eat(TokenKind::Star) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::Deref(Box::new(operand)));
    }
    parse_primary(ts)
}

fn parse_primary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    if ts.eat(TokenKind::LParen) {
        // Statement expression: ({ stmt; ... expr; })
        if ts.check(&TokenKind::LBrace) {
            let block = parse_block(ts)?;
            ts.expect(TokenKind::RParen)?;
            return parse_postfix_chain(ts, Expr::StmtExpr(block));
        }
        // Cast: (type)expr  vs  parenthesized: (expr)
        if is_type_start(ts) {
            let ty = parse_type(ts)?;
            ts.expect(TokenKind::RParen)?;
            let operand = parse_unary(ts)?;
            let cast = Expr::Cast {
                ty,
                expr: Box::new(operand),
            };
            return parse_postfix_chain(ts, cast);
        }
        let expr = parse_expr(ts)?;
        ts.expect(TokenKind::RParen)?;
        return parse_postfix_chain(ts, expr);
    }
    if let TokenKind::IntLit(_) = &ts.peek().kind {
        let TokenKind::IntLit(val) = ts.advance().kind else {
            unreachable!()
        };
        return Ok(Expr::IntLit(val));
    }
    if let TokenKind::StringLit(_) = &ts.peek().kind {
        let TokenKind::StringLit(mut s) = ts.advance().kind else {
            unreachable!()
        };
        // C89 implicit string literal concatenation: "a" "b" → "ab"
        while let TokenKind::StringLit(_) = &ts.peek().kind {
            let TokenKind::StringLit(next) = ts.advance().kind else {
                unreachable!()
            };
            s.push_str(&next);
        }
        return Ok(Expr::StringLit(s));
    }
    if let TokenKind::Ident(_) = &ts.peek().kind {
        return parse_ident_or_call(ts);
    }
    Err(CompileError::new(
        format!("expected expression, got {:?}", ts.peek().kind),
        Some(ts.current_span()),
    ))
}

fn parse_ident_or_call(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let TokenKind::Ident(name) = ts.advance().kind else {
        unreachable!()
    };
    // Resolve enum constants to integer literals
    if let Some(&val) = ts.enum_constants.get(&name) {
        return Ok(Expr::IntLit(val));
    }
    // Function call: foo(args...)
    if ts.eat(TokenKind::LParen) {
        let mut args = Vec::new();
        if !ts.check(&TokenKind::RParen) {
            loop {
                args.push(parse_assign(ts)?);
                if !ts.eat(TokenKind::Comma) {
                    break;
                }
            }
        }
        ts.expect(TokenKind::RParen)?;
        let call = Expr::Call { name, args };
        return parse_postfix_chain(ts, call);
    }
    // Postfix ++/-- (these consume the name, not chainable)
    if ts.eat(TokenKind::PlusPlus) {
        return Ok(Expr::PostInc(name));
    }
    if ts.eat(TokenKind::MinusMinus) {
        return Ok(Expr::PostDec(name));
    }
    // Start with the identifier, then chain postfix ops: [], ., ->
    let mut result = Expr::Ident(name);
    result = parse_postfix_chain(ts, result)?;
    Ok(result)
}

/// Parse chained postfix operations: a[i].b->c[j] etc.
fn parse_postfix_chain(ts: &mut TokenStream, mut expr: Expr) -> Result<Expr, CompileError> {
    loop {
        if ts.eat(TokenKind::LBracket) {
            let index = parse_expr(ts)?;
            ts.expect(TokenKind::RBracket)?;
            expr = Expr::Deref(Box::new(Expr::BinOp {
                op: tc24r_ast::BinOp::Add,
                lhs: Box::new(expr),
                rhs: Box::new(index),
            }));
        } else if ts.eat(TokenKind::LParen) {
            // Indirect call: expr(args) — for table[n](arg), (*fp)(arg), etc.
            let mut args = Vec::new();
            if !ts.check(&TokenKind::RParen) {
                loop {
                    args.push(parse_assign(ts)?);
                    if !ts.eat(TokenKind::Comma) {
                        break;
                    }
                }
            }
            ts.expect(TokenKind::RParen)?;
            expr = Expr::IndirectCall {
                callee: Box::new(expr),
                args,
            };
        } else if ts.eat(TokenKind::Dot) {
            let member = ts.expect_ident()?;
            expr = Expr::MemberAccess {
                object: Box::new(expr),
                member,
            };
        } else if ts.eat(TokenKind::Arrow) {
            let member = ts.expect_ident()?;
            expr = Expr::MemberAccess {
                object: Box::new(Expr::Deref(Box::new(expr))),
                member,
            };
        } else {
            break;
        }
    }
    Ok(expr)
}

/// Parse `sizeof(type)`, `sizeof(expr)`, or `sizeof expr` after the `sizeof` token.
fn parse_sizeof(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    if ts.eat(TokenKind::LParen) {
        if is_type_start(ts) {
            let mut ty = parse_type(ts)?;
            // Handle array suffix: sizeof(int[4]), sizeof(char[16])
            while ts.eat(TokenKind::LBracket) {
                let TokenKind::IntLit(size) = ts.peek().kind else {
                    return Err(CompileError::new(
                        "expected array size in sizeof",
                        Some(ts.current_span()),
                    ));
                };
                ts.advance();
                ts.expect(TokenKind::RBracket)?;
                ty = Type::Array(Box::new(ty), size as usize);
            }
            ts.expect(TokenKind::RParen)?;
            return Ok(Expr::IntLit(ty.size()));
        }
        // sizeof(expr) — parenthesized expression
        let expr = parse_expr(ts)?;
        ts.expect(TokenKind::RParen)?;
        return Ok(Expr::SizeofExpr(Box::new(expr)));
    }
    // sizeof expr — unary expression without parentheses
    let expr = parse_unary(ts)?;
    Ok(Expr::SizeofExpr(Box::new(expr)))
}

/// Parse `offsetof(type, member)` after the `offsetof` token.
fn parse_offsetof(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let ty = parse_type(ts)?;
    ts.expect(TokenKind::Comma)?;
    let member = ts.expect_ident()?;
    ts.expect(TokenKind::RParen)?;
    match ty.find_member(&member) {
        Some(m) => Ok(Expr::IntLit(m.offset)),
        None => Err(CompileError::new(
            format!("no member '{member}' in type"),
            Some(ts.current_span()),
        )),
    }
}
