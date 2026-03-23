//! Enum declaration parsing.

use tc24r_ast::Stmt;
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

/// Parse an enum declaration after the `enum` keyword has been consumed.
/// Populates `ts.enum_constants` and returns an empty expression statement.
pub fn parse_enum_decl(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    // Skip optional tag name: `enum color { ... }`
    if matches!(ts.peek().kind, TokenKind::Ident(_)) && matches!(ts.lookahead(1), TokenKind::LBrace)
    {
        ts.advance();
    }
    ts.expect(TokenKind::LBrace)?;
    parse_enum_body(ts)?;
    ts.expect(TokenKind::RBrace)?;
    consume_optional_var_decl(ts);
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Expr(tc24r_ast::Expr::IntLit(0)))
}

/// Parse comma-separated enumerator list, populating the constant map.
fn parse_enum_body(ts: &mut TokenStream) -> Result<(), CompileError> {
    let mut next_val: i32 = 0;
    while !ts.check(&TokenKind::RBrace) {
        let name = ts.expect_ident()?;
        if ts.eat(TokenKind::Assign) {
            next_val = parse_enum_value(ts)?;
        }
        ts.enum_constants.insert(name, next_val);
        next_val += 1;
        if !ts.eat(TokenKind::Comma) {
            break;
        }
    }
    Ok(())
}

/// Parse the integer value in `X = 5`. Handles optional negation.
fn parse_enum_value(ts: &mut TokenStream) -> Result<i32, CompileError> {
    let neg = ts.eat(TokenKind::Minus);
    let TokenKind::IntLit(val) = ts.peek().kind else {
        return Err(CompileError::new(
            "expected integer constant in enum",
            Some(ts.current_span()),
        ));
    };
    ts.advance();
    Ok(if neg { -val } else { val })
}

/// Skip optional variable declaration after enum body: `enum { ... } x;`
fn consume_optional_var_decl(ts: &mut TokenStream) {
    if matches!(ts.peek().kind, TokenKind::Ident(_)) {
        ts.advance();
    }
}
