//! Type keyword detection and type parsing.

use cc24_ast::Type;
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

/// Check whether a token kind starts a declaration (type or storage class).
pub fn is_type_keyword(kind: &TokenKind) -> bool {
    is_base_type(kind) || is_storage_class(kind)
}

/// Check whether the current token starts a type (base type, storage
/// class, or typedef alias).
pub fn is_type_start(ts: &TokenStream) -> bool {
    is_type_keyword(&ts.peek().kind) || is_typedef_name(ts, &ts.peek().kind)
}

/// Check whether a token is a known typedef alias.
pub fn is_typedef_name(ts: &TokenStream, kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Ident(name) if ts.type_aliases.contains_key(name))
}

pub fn is_base_type(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Char | TokenKind::Int | TokenKind::Void | TokenKind::Enum | TokenKind::Struct
    )
}

pub fn is_storage_class(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Static | TokenKind::Extern | TokenKind::Const
    )
}

pub fn parse_type(ts: &mut TokenStream) -> Result<Type, CompileError> {
    // Consume and ignore storage-class/type-qualifier specifiers
    let mut had_qualifier = false;
    while matches!(
        ts.peek().kind,
        TokenKind::Static | TokenKind::Extern | TokenKind::Const
    ) {
        ts.advance();
        had_qualifier = true;
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
        TokenKind::Enum => {
            ts.advance();
            // Skip optional enum tag name: `enum color`
            if matches!(ts.peek().kind, TokenKind::Ident(_)) {
                ts.advance();
            }
            Type::Int // enums are treated as int
        }
        TokenKind::Struct => {
            ts.advance();
            cc24_parser_struct::parse_struct_type(ts, parse_type)?
        }
        TokenKind::Ident(ref name) if ts.type_aliases.contains_key(name) => {
            let resolved = ts.type_aliases[name].clone();
            ts.advance();
            resolved
        }
        _ if had_qualifier => {
            // Implicit int: "const x" means "const int x" (K&R style)
            Type::Int
        }
        _ => {
            return Err(CompileError::new(
                format!("expected type, got {:?}", ts.peek().kind),
                Some(ts.current_span()),
            ));
        }
    };
    // Consume trailing const: int const, char const
    while ts.eat(TokenKind::Const) {}
    // Consume pointer stars and const qualifiers: int *, int *const, char **
    let mut ty = base;
    while ts.eat(TokenKind::Star) {
        while ts.eat(TokenKind::Const) {}
        ty = Type::Ptr(Box::new(ty));
    }
    Ok(ty)
}
