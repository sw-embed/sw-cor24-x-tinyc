//! Type parsing: consume tokens and produce a Type AST node.

use tc24r_ast::Type;
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

/// Consume and discard storage-class / type-qualifier keywords.
/// Returns `true` if at least one qualifier was consumed.
fn consume_qualifiers(ts: &mut TokenStream) -> bool {
    let mut had = false;
    while matches!(
        ts.peek().kind,
        TokenKind::Static | TokenKind::Extern | TokenKind::Const
    ) {
        ts.advance();
        had = true;
    }
    had
}

/// Parse a base type keyword (char/int/void/enum/struct/typedef alias).
fn parse_base_type(
    ts: &mut TokenStream,
    had_qualifier: bool,
) -> Result<Type, CompileError> {
    match ts.peek().kind {
        TokenKind::Char => { ts.advance(); Ok(Type::Char) }
        TokenKind::Int => { ts.advance(); Ok(Type::Int) }
        TokenKind::Void => { ts.advance(); Ok(Type::Void) }
        TokenKind::Enum => {
            ts.advance();
            if matches!(ts.peek().kind, TokenKind::Ident(_)) {
                ts.advance();
            }
            Ok(Type::Int)
        }
        TokenKind::Struct => {
            ts.advance();
            tc24r_parser_struct::parse_struct_type(ts, parse_type)
        }
        TokenKind::Ident(ref name) if ts.type_aliases.contains_key(name) => {
            let resolved = ts.type_aliases[name].clone();
            ts.advance();
            Ok(resolved)
        }
        _ if had_qualifier => Ok(Type::Int),
        _ => Err(CompileError::new(
            format!("expected type, got {:?}", ts.peek().kind),
            Some(ts.current_span()),
        )),
    }
}

/// Consume pointer stars and trailing const qualifiers after a base type.
fn consume_pointers(ts: &mut TokenStream, base: Type) -> Type {
    while ts.eat(TokenKind::Const) {}
    let mut ty = base;
    while ts.eat(TokenKind::Star) {
        while ts.eat(TokenKind::Const) {}
        ty = Type::Ptr(Box::new(ty));
    }
    ty
}

pub fn parse_type(ts: &mut TokenStream) -> Result<Type, CompileError> {
    let had_qualifier = consume_qualifiers(ts);
    let base = parse_base_type(ts, had_qualifier)?;
    Ok(consume_pointers(ts, base))
}
