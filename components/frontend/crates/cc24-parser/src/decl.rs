//! Top-level and declaration parsing.

use cc24_ast::{Function, GlobalDecl, Param, Program, Type};
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

use crate::stmt::parse_block;
use cc24_parse_stream::try_parse_interrupt_attr;

/// Parse a full program (sequence of functions and globals).
pub fn parse_program(ts: &mut TokenStream) -> Result<Program, CompileError> {
    let mut functions = Vec::new();
    let mut globals = Vec::new();
    while !ts.at_eof() {
        let is_interrupt = try_parse_interrupt_attr(ts);
        if is_global_decl(ts) {
            globals.push(parse_global_decl(ts)?);
        } else {
            functions.push(parse_function(ts, is_interrupt)?);
        }
    }
    Ok(Program { functions, globals })
}

fn is_global_decl(ts: &TokenStream) -> bool {
    let mut i = 0;
    // Skip storage-class specifiers
    while is_storage_class(ts.lookahead(i)) {
        i += 1;
    }
    if !is_base_type(ts.lookahead(i)) {
        return false;
    }
    i += 1;
    // Skip pointer stars: int **ptr
    while matches!(ts.lookahead(i), TokenKind::Star) {
        i += 1;
    }
    matches!(ts.lookahead(i), TokenKind::Ident(_))
        && !matches!(ts.lookahead(i + 1), TokenKind::LParen)
}

/// Check whether a token kind starts a declaration (type or storage class).
pub fn is_type_keyword(kind: &TokenKind) -> bool {
    is_base_type(kind) || is_storage_class(kind)
}

fn is_base_type(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Char | TokenKind::Int | TokenKind::Void)
}

fn is_storage_class(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Static | TokenKind::Extern)
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

fn parse_function(ts: &mut TokenStream, is_interrupt: bool) -> Result<Function, CompileError> {
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
        is_interrupt,
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
    // Consume and ignore storage-class specifiers (static, extern)
    while matches!(ts.peek().kind, TokenKind::Static | TokenKind::Extern) {
        ts.advance();
    }
    let base = match ts.peek().kind {
        TokenKind::Char => {
            ts.advance();
            Type::Char
        }
        TokenKind::Int => {
            ts.advance();
            Type::Int
        }
        TokenKind::Void => {
            ts.advance();
            Type::Void
        }
        _ => {
            return Err(CompileError::new(
                format!("expected type, got {:?}", ts.peek().kind),
                Some(ts.current_span()),
            ));
        }
    };
    // Consume pointer stars: int *, char **, etc.
    let mut ty = base;
    while ts.eat(TokenKind::Star) {
        ty = Type::Ptr(Box::new(ty));
    }
    Ok(ty)
}
