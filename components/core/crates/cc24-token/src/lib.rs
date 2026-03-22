//! Token types produced by the lexer.

mod kind;

pub use kind::TokenKind;

use cc24_span::Span;

/// A single token with its kind and source span.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
