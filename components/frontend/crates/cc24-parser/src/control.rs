//! Control flow statement parsing (if, while, for, asm).

use cc24_ast::Stmt;
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

use crate::decl::is_type_keyword;
use crate::expr::parse_expr;
use crate::stmt::{parse_block, parse_local_decl};

pub fn parse_if(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let cond = parse_expr(ts)?;
    ts.expect(TokenKind::RParen)?;
    let then_body = parse_block(ts)?;
    let else_body = if ts.eat(TokenKind::Else) {
        Some(parse_block(ts)?)
    } else {
        None
    };
    Ok(Stmt::If {
        cond,
        then_body,
        else_body,
    })
}

pub fn parse_while(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let cond = parse_expr(ts)?;
    ts.expect(TokenKind::RParen)?;
    let body = parse_block(ts)?;
    Ok(Stmt::While { cond, body })
}

pub fn parse_for(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let init = parse_for_init(ts)?;
    let cond = if ts.check(&TokenKind::Semicolon) {
        None
    } else {
        Some(parse_expr(ts)?)
    };
    ts.expect(TokenKind::Semicolon)?;
    let inc = if ts.check(&TokenKind::RParen) {
        None
    } else {
        Some(parse_expr(ts)?)
    };
    ts.expect(TokenKind::RParen)?;
    let body = parse_block(ts)?;
    Ok(Stmt::For {
        init,
        cond,
        inc,
        body,
    })
}

fn parse_for_init(ts: &mut TokenStream) -> Result<Option<Box<Stmt>>, CompileError> {
    if ts.check(&TokenKind::Semicolon) {
        ts.expect(TokenKind::Semicolon)?;
        return Ok(None);
    }
    if is_type_keyword(&ts.peek().kind) {
        // local decl consumes its own semicolon
        return Ok(Some(Box::new(parse_local_decl(ts)?)));
    }
    let expr = parse_expr(ts)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Some(Box::new(Stmt::Expr(expr))))
}

pub fn parse_asm(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let TokenKind::StringLit(s) = &ts.peek().kind else {
        return Err(CompileError::new(
            "expected string literal after asm(",
            Some(ts.current_span()),
        ));
    };
    let s = s.clone();
    ts.advance();
    ts.expect(TokenKind::RParen)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Asm(s))
}

pub fn parse_do_while(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    let body = parse_block(ts)?;
    ts.expect(TokenKind::While)?;
    ts.expect(TokenKind::LParen)?;
    let cond = parse_expr(ts)?;
    ts.expect(TokenKind::RParen)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::DoWhile { body, cond })
}
