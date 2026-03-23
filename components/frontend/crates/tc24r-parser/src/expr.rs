//! Expression entry point, assignment, unary, and primary parsing.

use tc24r_ast::{Expr, UnaryOp};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_parser_compound::{desugar_compound, eat_compound_assign, make_assign};
use tc24r_parser_types::{is_type_start, parse_type};
use tc24r_token::TokenKind;

use crate::bitwise::parse_log_or;
use crate::stmt::parse_block;

/// Parse an expression.
pub fn parse_expr(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    parse_assign(ts)
}

fn parse_assign(ts: &mut TokenStream) -> Result<Expr, CompileError> {
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
            return Ok(Expr::StmtExpr(block));
        }
        // Cast: (type)expr  vs  parenthesized: (expr)
        if is_type_start(ts) {
            let ty = parse_type(ts)?;
            ts.expect(TokenKind::RParen)?;
            let operand = parse_unary(ts)?;
            return Ok(Expr::Cast {
                ty,
                expr: Box::new(operand),
            });
        }
        let expr = parse_expr(ts)?;
        ts.expect(TokenKind::RParen)?;
        return Ok(expr);
    }
    if let TokenKind::IntLit(_) = &ts.peek().kind {
        let TokenKind::IntLit(val) = ts.advance().kind else {
            unreachable!()
        };
        return Ok(Expr::IntLit(val));
    }
    if let TokenKind::StringLit(_) = &ts.peek().kind {
        let TokenKind::StringLit(s) = ts.advance().kind else {
            unreachable!()
        };
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
    if ts.eat(TokenKind::LParen) {
        let mut args = Vec::new();
        if !ts.check(&TokenKind::RParen) {
            loop {
                args.push(parse_expr(ts)?);
                if !ts.eat(TokenKind::Comma) {
                    break;
                }
            }
        }
        ts.expect(TokenKind::RParen)?;
        return Ok(Expr::Call { name, args });
    }
    // Array indexing: a[i] -> *(a + i)
    if ts.eat(TokenKind::LBracket) {
        let index = parse_expr(ts)?;
        ts.expect(TokenKind::RBracket)?;
        let base = Expr::Ident(name);
        return Ok(Expr::Deref(Box::new(Expr::BinOp {
            op: tc24r_ast::BinOp::Add,
            lhs: Box::new(base),
            rhs: Box::new(index),
        })));
    }
    // Postfix ++/--
    if ts.eat(TokenKind::PlusPlus) {
        return Ok(Expr::PostInc(name));
    }
    if ts.eat(TokenKind::MinusMinus) {
        return Ok(Expr::PostDec(name));
    }
    // Member access: p.x
    if ts.eat(TokenKind::Dot) {
        let member = ts.expect_ident()?;
        return Ok(Expr::MemberAccess {
            object: Box::new(Expr::Ident(name)),
            member,
        });
    }
    Ok(Expr::Ident(name))
}

/// Parse `sizeof(type)` or `sizeof(expr)` after the `sizeof` token.
fn parse_sizeof(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    ts.expect(TokenKind::LParen)?;
    if is_type_start(ts) {
        let ty = parse_type(ts)?;
        ts.expect(TokenKind::RParen)?;
        return Ok(Expr::IntLit(ty.size()));
    }
    // sizeof(expr) -- no type info available, assume int-sized
    let _expr = parse_unary(ts)?;
    ts.expect(TokenKind::RParen)?;
    Ok(Expr::IntLit(3))
}
