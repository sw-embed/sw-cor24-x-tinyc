//! Statement and block parsing.

use cc24_ast::{Block, Stmt};
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

use crate::control;
use crate::decl::parse_type;
use crate::expr::parse_expr;

/// Parse a brace-delimited block.
pub fn parse_block(ts: &mut TokenStream) -> Result<Block, CompileError> {
    ts.expect(TokenKind::LBrace)?;
    let mut stmts = Vec::new();
    while !ts.check(&TokenKind::RBrace) {
        stmts.push(parse_stmt(ts)?);
    }
    ts.expect(TokenKind::RBrace)?;
    Ok(Block { stmts })
}

/// Parse a single statement.
pub fn parse_stmt(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    if ts.eat(TokenKind::Return) {
        return parse_return(ts);
    }
    if ts.eat(TokenKind::If) {
        return control::parse_if(ts);
    }
    if ts.eat(TokenKind::While) {
        return control::parse_while(ts);
    }
    if ts.eat(TokenKind::For) {
        return control::parse_for(ts);
    }
    if ts.eat(TokenKind::Asm) {
        return control::parse_asm(ts);
    }
    if ts.check(&TokenKind::Int) || ts.check(&TokenKind::Void) {
        return parse_local_decl(ts);
    }
    let expr = parse_expr(ts)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Expr(expr))
}

fn parse_return(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    let expr = parse_expr(ts)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Return(expr))
}

pub fn parse_local_decl(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    let ty = parse_type(ts)?;
    let name = ts.expect_ident()?;
    let init = if ts.eat(TokenKind::Assign) {
        Some(parse_expr(ts)?)
    } else {
        None
    };
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::LocalDecl { name, ty, init })
}
