//! Expression entry point, assignment, unary, and primary parsing.

use cc24_ast::{Expr, UnaryOp};
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

use crate::bitwise::parse_or;

/// Parse an expression.
pub fn parse_expr(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    parse_assign(ts)
}

fn parse_assign(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let expr = parse_or(ts)?;
    if ts.eat(TokenKind::Assign) {
        let Expr::Ident(name) = expr else {
            return Err(CompileError::new(
                "left side of assignment must be a variable",
                None,
            ));
        };
        let value = parse_assign(ts)?;
        return Ok(Expr::Assign {
            name,
            value: Box::new(value),
        });
    }
    Ok(expr)
}

pub fn parse_unary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
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
    parse_primary(ts)
}

fn parse_primary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    if ts.eat(TokenKind::LParen) {
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
    Ok(Expr::Ident(name))
}
