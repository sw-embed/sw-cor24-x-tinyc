//! Expect, query, and lookahead helpers for `TokenStream`.

use cc24_error::CompileError;
use cc24_span::Span;
use cc24_token::TokenKind;

use crate::stream::TokenStream;

impl TokenStream {
    pub fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    pub fn current_span(&self) -> Span {
        self.peek().span
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<cc24_token::Token, CompileError> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(CompileError::new(
                format!("expected {kind:?}, got {:?}", self.peek().kind),
                Some(self.current_span()),
            ))
        }
    }

    pub fn expect_ident(&mut self) -> Result<String, CompileError> {
        if let TokenKind::Ident(_) = &self.peek().kind {
            let TokenKind::Ident(name) = self.advance().kind else {
                unreachable!()
            };
            Ok(name)
        } else {
            Err(CompileError::new(
                format!("expected identifier, got {:?}", self.peek().kind),
                Some(self.current_span()),
            ))
        }
    }

    /// Peek ahead by `n` tokens (0 = current).
    pub fn lookahead(&self, n: usize) -> &TokenKind {
        &self.tokens[self.pos + n].kind
    }
}
