//! Expression entry point, assignment, unary, and primary parsing.

use cc24_ast::{BinOp, Expr, UnaryOp};
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

use crate::bitwise::parse_log_or;
use crate::decl::{is_type_keyword, parse_type};
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

/// Try to eat a compound-assignment token, returning the corresponding `BinOp`.
fn eat_compound_assign(ts: &mut TokenStream) -> Option<BinOp> {
    let pairs = [
        (TokenKind::PlusAssign, BinOp::Add),
        (TokenKind::MinusAssign, BinOp::Sub),
        (TokenKind::StarAssign, BinOp::Mul),
        (TokenKind::SlashAssign, BinOp::Div),
        (TokenKind::PercentAssign, BinOp::Mod),
        (TokenKind::AmpAssign, BinOp::BitAnd),
        (TokenKind::PipeAssign, BinOp::BitOr),
        (TokenKind::CaretAssign, BinOp::BitXor),
        (TokenKind::LShiftAssign, BinOp::Shl),
        (TokenKind::RShiftAssign, BinOp::Shr),
    ];
    for (tok, op) in pairs {
        if ts.eat(tok) {
            return Some(op);
        }
    }
    None
}

/// Build a plain assignment node from lhs and value.
fn make_assign(lhs: Expr, value: Expr) -> Result<Expr, CompileError> {
    match lhs {
        Expr::Ident(name) => Ok(Expr::Assign {
            name,
            value: Box::new(value),
        }),
        Expr::Deref(ptr) => Ok(Expr::DerefAssign {
            ptr,
            value: Box::new(value),
        }),
        _ => Err(CompileError::new(
            "left side of assignment must be a variable or dereference",
            None,
        )),
    }
}

/// Desugar `lhs op= rhs` into `lhs = lhs op rhs`.
fn desugar_compound(lhs: Expr, op: BinOp, rhs: Expr) -> Result<Expr, CompileError> {
    match &lhs {
        Expr::Ident(_) | Expr::Deref(_) => {}
        _ => {
            return Err(CompileError::new(
                "left side of compound assignment must be a variable or dereference",
                None,
            ));
        }
    }
    let binop = Expr::BinOp {
        op,
        lhs: Box::new(clone_lvalue(&lhs)),
        rhs: Box::new(rhs),
    };
    make_assign(lhs, binop)
}

/// Clone an lvalue expression for the read side of compound assignment.
fn clone_lvalue(expr: &Expr) -> Expr {
    match expr {
        Expr::Ident(name) => Expr::Ident(name.clone()),
        Expr::Deref(inner) => Expr::Deref(Box::new(clone_lvalue(inner))),
        _ => unreachable!("clone_lvalue called on non-lvalue"),
    }
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
        if is_type_keyword(&ts.peek().kind) {
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
            op: cc24_ast::BinOp::Add,
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
    Ok(Expr::Ident(name))
}

/// Parse `sizeof(type)` or `sizeof(expr)` after the `sizeof` token.
fn parse_sizeof(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    ts.expect(TokenKind::LParen)?;
    if is_type_keyword(&ts.peek().kind) {
        let ty = parse_type(ts)?;
        ts.expect(TokenKind::RParen)?;
        return Ok(Expr::IntLit(ty.size()));
    }
    // sizeof(expr) -- no type info available, assume int-sized
    let _expr = parse_unary(ts)?;
    ts.expect(TokenKind::RParen)?;
    Ok(Expr::IntLit(3))
}
