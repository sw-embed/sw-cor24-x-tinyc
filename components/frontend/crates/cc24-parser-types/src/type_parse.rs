//! Type keyword detection and type parsing.

use cc24_ast::Type;
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::TokenKind;

/// Check whether a token kind starts a declaration (type or storage class).
pub fn is_type_keyword(kind: &TokenKind) -> bool {
    is_base_type(kind) || is_storage_class(kind)
}

pub fn is_base_type(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Char | TokenKind::Int | TokenKind::Void | TokenKind::Enum
    )
}

pub fn is_storage_class(kind: &TokenKind) -> bool {
    matches!(kind, TokenKind::Static | TokenKind::Extern)
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
        TokenKind::Enum => {
            ts.advance();
            // Skip optional enum tag name: `enum color`
            if matches!(ts.peek().kind, TokenKind::Ident(_)) {
                ts.advance();
            }
            Type::Int // enums are treated as int
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
