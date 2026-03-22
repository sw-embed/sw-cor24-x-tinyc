//! Core token stream navigation.

use cc24_token::{Token, TokenKind};

/// Wrapper around a token vector providing navigation helpers.
pub struct TokenStream {
    pub(crate) tokens: Vec<Token>,
    pub(crate) pos: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    pub fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    pub fn eat(&mut self, kind: TokenKind) -> bool {
        if self.check(&kind) {
            self.advance();
            true
        } else {
            false
        }
    }
}
