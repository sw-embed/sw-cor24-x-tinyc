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
            self.skip_whitespace_and_comments();
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

    pub(crate) fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();
            if !self.skip_comment() {
                break;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn skip_comment(&mut self) -> bool {
        if self.pos + 1 >= self.source.len() {
            return false;
        }
        if self.source[self.pos] == b'/' && self.source[self.pos + 1] == b'/' {
            self.pos += 2;
            while self.pos < self.source.len() && self.source[self.pos] != b'\n' {
                self.pos += 1;
            }
            return true;
        }
        if self.source[self.pos] == b'/' && self.source[self.pos + 1] == b'*' {
            self.pos += 2;
            while self.pos + 1 < self.source.len() {
                if self.source[self.pos] == b'*' && self.source[self.pos + 1] == b'/' {
                    self.pos += 2;
                    return true;
                }
                self.pos += 1;
            }
            self.pos = self.source.len();
            return true;
        }
        false
    }

    pub(crate) fn peek_char(&self) -> Option<u8> {
        self.source.get(self.pos + 1).copied()
    }
}
