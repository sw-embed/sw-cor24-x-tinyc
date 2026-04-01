//! Typedef declaration parsing.

use tc24r_ast::{Expr, Stmt, Type};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

/// Parse a typedef after the `typedef` keyword has been consumed.
/// Stores the alias in `ts.type_aliases` and returns a no-op statement.
pub fn parse_typedef_decl(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    let base_ty = tc24r_parser_types::parse_type(ts)?;
    // Bare typedef (e.g. `typedef int;`) — accept and ignore
    if ts.check(&TokenKind::Semicolon) {
        ts.advance();
        return Ok(Stmt::Expr(Expr::IntLit(0)));
    }
    // Function pointer typedef: typedef int (*handler_t)(int);
    if ts.check(&TokenKind::LParen) && matches!(ts.lookahead(1), TokenKind::Star) {
        return parse_fn_ptr_typedef(ts, base_ty);
    }
    // Parse first declarator
    let ty = parse_pointer_and_array(ts, base_ty.clone())?;
    let alias = ts.expect_ident()?;
    let ty = parse_array_suffix(ts, ty)?;
    ts.type_aliases.insert(alias, ty);
    // Parse additional comma-separated declarators: typedef int A, B[4];
    while ts.eat(TokenKind::Comma) {
        let ty = parse_pointer_and_array(ts, base_ty.clone())?;
        let alias = ts.expect_ident()?;
        let ty = parse_array_suffix(ts, ty)?;
        ts.type_aliases.insert(alias, ty);
    }
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Expr(Expr::IntLit(0)))
}

/// Parse function pointer typedef: typedef int (*handler_t)(int);
fn parse_fn_ptr_typedef(ts: &mut TokenStream, return_ty: Type) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?; // (
    ts.expect(TokenKind::Star)?; // *
    let alias = ts.expect_ident()?;
    ts.expect(TokenKind::RParen)?; // )
    // Consume parameter list
    ts.expect(TokenKind::LParen)?;
    skip_fn_ptr_params(ts)?;
    ts.expect(TokenKind::RParen)?;
    ts.type_aliases
        .insert(alias, Type::Ptr(Box::new(return_ty)));
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Expr(Expr::IntLit(0)))
}

/// Skip a function pointer parameter list (balanced parens).
fn skip_fn_ptr_params(ts: &mut TokenStream) -> Result<(), CompileError> {
    if ts.check(&TokenKind::RParen) {
        return Ok(());
    }
    loop {
        while !ts.check(&TokenKind::RParen) && !ts.check(&TokenKind::Comma) {
            if ts.eat(TokenKind::LParen) {
                skip_fn_ptr_params(ts)?;
                ts.expect(TokenKind::RParen)?;
            } else {
                ts.advance();
            }
        }
        if !ts.eat(TokenKind::Comma) {
            break;
        }
    }
    Ok(())
}

/// Consume pointer stars after the base type.
fn parse_pointer_and_array(ts: &mut TokenStream, base: Type) -> Result<Type, CompileError> {
    let mut ty = base;
    while ts.eat(TokenKind::Star) {
        ty = Type::Ptr(Box::new(ty));
    }
    Ok(ty)
}

/// Consume optional `[N]` array suffix.
fn parse_array_suffix(ts: &mut TokenStream, ty: Type) -> Result<Type, CompileError> {
    if !ts.eat(TokenKind::LBracket) {
        return Ok(ty);
    }
    let TokenKind::IntLit(size) = ts.peek().kind else {
        return Err(CompileError::new(
            "expected array size in typedef",
            Some(ts.current_span()),
        ));
    };
    ts.advance();
    ts.expect(TokenKind::RBracket)?;
    Ok(Type::Array(Box::new(ty), size as usize))
}
