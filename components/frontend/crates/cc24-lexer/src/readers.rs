//! Literal and identifier reading.

use cc24_error::CompileError;
use cc24_span::Span;
use cc24_token::{Token, TokenKind};

use crate::Lexer;

impl Lexer<'_> {
    /// Dispatch a string, number, identifier, or report an error.
    pub(crate) fn dispatch_literal(&mut self, start: usize, ch: u8) -> Result<Token, CompileError> {
        if ch == b'"' {
            return self.read_string(start);
        }
        if ch.is_ascii_digit() {
            return self.read_number(start);
        }
        if ch.is_ascii_alphabetic() || ch == b'_' {
            return Ok(self.read_ident(start));
        }
        Err(CompileError::new(
            format!("unexpected character: '{}'", ch as char),
            Some(Span::new(start, 1)),
        ))
    }

    fn read_number(&mut self, start: usize) -> Result<Token, CompileError> {
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.source[start..self.pos]).unwrap();
        let value: i32 = text.parse().map_err(|_| {
            CompileError::new(
                format!("invalid integer literal: {text}"),
                Some(Span::new(start, self.pos - start)),
            )
        })?;
        Ok(Token {
            kind: TokenKind::IntLit(value),
            span: Span::new(start, self.pos - start),
        })
    }

    fn read_string(&mut self, start: usize) -> Result<Token, CompileError> {
        self.pos += 1; // skip opening quote
        let mut value = String::new();
        while self.pos < self.source.len() && self.source[self.pos] != b'"' {
            if self.source[self.pos] == b'\\' {
                self.read_escape(&mut value, start)?;
            } else {
                value.push(self.source[self.pos] as char);
            }
            self.pos += 1;
        }
        if self.pos >= self.source.len() {
            return Err(CompileError::new(
                "unterminated string literal",
                Some(Span::new(start, self.pos - start)),
            ));
        }
        self.pos += 1; // skip closing quote
        Ok(Token {
            kind: TokenKind::StringLit(value),
            span: Span::new(start, self.pos - start),
        })
    }

    fn read_escape(&mut self, value: &mut String, start: usize) -> Result<(), CompileError> {
        self.pos += 1;
        if self.pos >= self.source.len() {
            return Err(CompileError::new(
                "unterminated string escape",
                Some(Span::new(start, self.pos - start)),
            ));
        }
        let escaped = match self.source[self.pos] {
            b'n' => '\n',
            b't' => '\t',
            b'\\' => '\\',
            b'"' => '"',
            b'0' => '\0',
            other => {
                return Err(CompileError::new(
                    format!("unknown escape: \\{}", other as char),
                    Some(Span::new(self.pos - 1, 2)),
                ));
            }
        };
        value.push(escaped);
        Ok(())
    }
}

/// Map identifier text to a keyword token kind, or `Ident`.
pub(crate) fn keyword_or_ident(text: &str) -> TokenKind {
    match text {
        "int" => TokenKind::Int,
        "void" => TokenKind::Void,
        "return" => TokenKind::Return,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "asm" => TokenKind::Asm,
        _ => TokenKind::Ident(text.to_string()),
    }
}

impl Lexer<'_> {
    fn read_ident(&mut self, start: usize) -> Token {
        while self.pos < self.source.len()
            && (self.source[self.pos].is_ascii_alphanumeric() || self.source[self.pos] == b'_')
        {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.source[start..self.pos]).unwrap();
        Token {
            kind: keyword_or_ident(text),
            span: Span::new(start, self.pos - start),
        }
    }
}
