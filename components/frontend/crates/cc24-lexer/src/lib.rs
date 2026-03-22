//! Lexer for the cc24 C compiler.

mod operators;
mod readers;

use cc24_error::CompileError;
use cc24_span::Span;
use cc24_token::{Token, TokenKind};

/// Tokenises a source string into a sequence of tokens.
pub struct Lexer<'a> {
    pub(crate) source: &'a [u8],
    pub(crate) pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            if self.pos >= self.source.len() {
                tokens.push(Token {
                    kind: TokenKind::Eof,
                    span: Span::new(self.pos, 0),
                });
                break;
            }
            tokens.push(self.next_token()?);
        }
        Ok(tokens)
    }

    pub(crate) fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    pub(crate) fn peek_char(&self) -> Option<u8> {
        self.source.get(self.pos + 1).copied()
    }
}
