//! Relational, shift, additive, and multiplicative precedence levels.

use tc24r_ast::{BinOp, Expr};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

use crate::expr::parse_unary;

pub fn parse_relational(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_shift(ts)?;
    loop {
        let op = if ts.eat(TokenKind::Lt) {
            BinOp::Lt
        } else if ts.eat(TokenKind::Gt) {
            BinOp::Gt
        } else if ts.eat(TokenKind::LtEq) {
            BinOp::Le
        } else if ts.eat(TokenKind::GtEq) {
            BinOp::Ge
        } else {
            break;
        };
        let rhs = parse_shift(ts)?;
        lhs = Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn parse_shift(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_additive(ts)?;
    loop {
        let op = if ts.eat(TokenKind::LShift) {
            BinOp::Shl
        } else if ts.eat(TokenKind::RShift) {
            BinOp::Shr
        } else {
            break;
        };
        let rhs = parse_additive(ts)?;
        lhs = Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn parse_additive(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_multiplicative(ts)?;
    loop {
        let op = if ts.eat(TokenKind::Plus) {
            BinOp::Add
        } else if ts.eat(TokenKind::Minus) {
            BinOp::Sub
        } else {
            break;
        };
        let rhs = parse_multiplicative(ts)?;
        lhs = Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn parse_multiplicative(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_unary(ts)?;
    loop {
        let op = if ts.eat(TokenKind::Star) {
            BinOp::Mul
        } else if ts.eat(TokenKind::Slash) {
            BinOp::Div
        } else if ts.eat(TokenKind::Percent) {
            BinOp::Mod
        } else {
            break;
        };
        let rhs = parse_unary(ts)?;
        lhs = Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}
