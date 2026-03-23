//! Logical, bitwise, and equality precedence levels.

use tc24r_ast::{BinOp, Expr};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

use crate::arithmetic::parse_relational;

pub fn parse_log_or(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_log_and(ts)?;
    while ts.eat(TokenKind::PipePipe) {
        let rhs = parse_log_and(ts)?;
        lhs = Expr::BinOp {
            op: BinOp::LogOr,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn parse_log_and(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_or(ts)?;
    while ts.eat(TokenKind::AmpAmp) {
        let rhs = parse_or(ts)?;
        lhs = Expr::BinOp {
            op: BinOp::LogAnd,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

pub fn parse_or(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_xor(ts)?;
    while ts.eat(TokenKind::Pipe) {
        let rhs = parse_xor(ts)?;
        lhs = Expr::BinOp {
            op: BinOp::BitOr,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn parse_xor(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_and(ts)?;
    while ts.eat(TokenKind::Caret) {
        let rhs = parse_and(ts)?;
        lhs = Expr::BinOp {
            op: BinOp::BitXor,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn parse_and(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_equality(ts)?;
    while ts.eat(TokenKind::Amp) {
        let rhs = parse_equality(ts)?;
        lhs = Expr::BinOp {
            op: BinOp::BitAnd,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn parse_equality(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_relational(ts)?;
    loop {
        let op = if ts.eat(TokenKind::EqEq) {
            BinOp::Eq
        } else if ts.eat(TokenKind::BangEq) {
            BinOp::Ne
        } else {
            break;
        };
        let rhs = parse_relational(ts)?;
        lhs = Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}
