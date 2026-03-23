use tc24r_span::Span;

use crate::TokenKind;

/// A single token with its kind and source span.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
