//! Operator and punctuation tokenisation.

use cc24_error::CompileError;
use cc24_span::Span;
use cc24_token::{Token, TokenKind};

use crate::Lexer;

impl Lexer<'_> {
    /// Dispatch the next token starting at `self.pos`.
    pub(crate) fn next_token(&mut self) -> Result<Token, CompileError> {
        let start = self.pos;
        let ch = self.source[self.pos];

        if let Some(tok) = self.try_two_char(start) {
            return Ok(tok);
        }
        if let Some(tok) = self.try_single_char(start) {
            return Ok(tok);
        }

        self.dispatch_literal(start, ch)
    }

    fn try_two_char(&mut self, start: usize) -> Option<Token> {
        let ch = self.source[start];
        let next = self.peek_char()?;
        let kind = match (ch, next) {
            (b'<', b'<') => TokenKind::LShift,
            (b'>', b'>') => TokenKind::RShift,
            (b'=', b'=') => TokenKind::EqEq,
            (b'!', b'=') => TokenKind::BangEq,
            (b'<', b'=') => TokenKind::LtEq,
            (b'>', b'=') => TokenKind::GtEq,
            _ => return None,
        };
        self.pos += 2;
        Some(Token {
            kind,
            span: Span::new(start, 2),
        })
    }

    fn try_single_char(&mut self, start: usize) -> Option<Token> {
        let kind = match self.source[start] {
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'{' => TokenKind::LBrace,
            b'}' => TokenKind::RBrace,
            b';' => TokenKind::Semicolon,
            b',' => TokenKind::Comma,
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Minus,
            b'*' => TokenKind::Star,
            b'/' => TokenKind::Slash,
            b'%' => TokenKind::Percent,
            b'&' => TokenKind::Amp,
            b'|' => TokenKind::Pipe,
            b'^' => TokenKind::Caret,
            b'~' => TokenKind::Tilde,
            b'!' => TokenKind::Bang,
            b'=' => TokenKind::Assign,
            b'<' => TokenKind::Lt,
            b'>' => TokenKind::Gt,
            _ => return None,
        };
        self.pos += 1;
        Some(Token {
            kind,
            span: Span::new(start, 1),
        })
    }
}
