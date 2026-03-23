//! Type keyword detection: is this token the start of a type specifier?

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
