//! Top-level and declaration parsing.

use cc24_ast::{Function, GlobalDecl, Param, Program, Type};
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

use crate::stmt::parse_block;

/// Parse a full program (sequence of functions and globals).
pub fn parse_program(ts: &mut TokenStream) -> Result<Program, CompileError> {
    let mut functions = Vec::new();
    let mut globals = Vec::new();
    while !ts.at_eof() {
        if is_global_decl(ts) {
            globals.push(parse_global_decl(ts)?);
        } else {
            functions.push(parse_function(ts)?);
        }
    }
    Ok(Program { functions, globals })
}

fn is_global_decl(ts: &TokenStream) -> bool {
    matches!(ts.lookahead(0), TokenKind::Int | TokenKind::Void)
        && matches!(ts.lookahead(1), TokenKind::Ident(_))
        && !matches!(ts.lookahead(2), TokenKind::LParen)
}

fn parse_global_decl(ts: &mut TokenStream) -> Result<GlobalDecl, CompileError> {
    let ty = parse_type(ts)?;
    let name = ts.expect_ident()?;
    let init = if ts.eat(TokenKind::Assign) {
        Some(crate::expr::parse_expr(ts)?)
    } else {
        None
    };
    ts.expect(TokenKind::Semicolon)?;
    Ok(GlobalDecl { name, ty, init })
}

fn parse_function(ts: &mut TokenStream) -> Result<Function, CompileError> {
    let span = ts.current_span();
    let return_ty = parse_type(ts)?;
    let name = ts.expect_ident()?;
    ts.expect(TokenKind::LParen)?;
    let params = parse_params(ts)?;
    ts.expect(TokenKind::RParen)?;
    let body = parse_block(ts)?;
    Ok(Function {
        name,
        return_ty,
        params,
        body,
        span,
    })
}

fn parse_params(ts: &mut TokenStream) -> Result<Vec<Param>, CompileError> {
    let mut params = Vec::new();
    if ts.check(&TokenKind::RParen) {
        return Ok(params);
    }
    loop {
        let ty = parse_type(ts)?;
        let name = ts.expect_ident()?;
        params.push(Param { name, ty });
        if !ts.eat(TokenKind::Comma) {
            break;
        }
    }
    Ok(params)
}

pub fn parse_type(ts: &mut TokenStream) -> Result<Type, CompileError> {
    match ts.peek().kind {
        TokenKind::Int => {
            ts.advance();
            Ok(Type::Int)
        }
        TokenKind::Void => {
            ts.advance();
            Ok(Type::Void)
        }
        _ => Err(CompileError::new(
            format!("expected type, got {:?}", ts.peek().kind),
            Some(ts.current_span()),
        )),
    }
}
