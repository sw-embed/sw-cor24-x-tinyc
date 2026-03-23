//! Operator and punctuation tokenisation.

use tc24r_error::CompileError;
use tc24r_span::Span;
use tc24r_token::{Token, TokenKind};

use crate::Lexer;

impl Lexer<'_> {
    /// Dispatch the next token starting at `self.pos`.
    pub(crate) fn next_token(&mut self) -> Result<Token, CompileError> {
        let start = self.pos;
        let ch = self.source[self.pos];

        if let Some(tok) = self.try_three_char(start) {
            return Ok(tok);
        }
        if let Some(tok) = self.try_two_char(start) {
            return Ok(tok);
        }
        if let Some(tok) = self.try_single_char(start) {
            return Ok(tok);
        }

        self.dispatch_literal(start, ch)
    }

    fn try_three_char(&mut self, start: usize) -> Option<Token> {
        let ch = self.source[start];
        let b = self.peek_char()?;
        let c = self.peek_char_at(2)?;
        let kind = match (ch, b, c) {
            (b'<', b'<', b'=') => TokenKind::LShiftAssign,
            (b'>', b'>', b'=') => TokenKind::RShiftAssign,
            _ => return None,
        };
        self.pos += 3;
        Some(Token {
            kind,
            span: Span::new(start, 3),
        })
    }

    fn try_two_char(&mut self, start: usize) -> Option<Token> {
        let ch = self.source[start];
        let next = self.peek_char()?;
        let kind = match (ch, next) {
            (b'+', b'+') => TokenKind::PlusPlus,
            (b'-', b'>') => TokenKind::Arrow,
            (b'-', b'-') => TokenKind::MinusMinus,
            (b'&', b'&') => TokenKind::AmpAmp,
            (b'|', b'|') => TokenKind::PipePipe,
            (b'<', b'<') => TokenKind::LShift,
            (b'>', b'>') => TokenKind::RShift,
            (b'=', b'=') => TokenKind::EqEq,
            (b'!', b'=') => TokenKind::BangEq,
            (b'<', b'=') => TokenKind::LtEq,
            (b'>', b'=') => TokenKind::GtEq,
            (b'+', b'=') => TokenKind::PlusAssign,
            (b'-', b'=') => TokenKind::MinusAssign,
            (b'*', b'=') => TokenKind::StarAssign,
            (b'/', b'=') => TokenKind::SlashAssign,
            (b'%', b'=') => TokenKind::PercentAssign,
            (b'&', b'=') => TokenKind::AmpAssign,
            (b'|', b'=') => TokenKind::PipeAssign,
            (b'^', b'=') => TokenKind::CaretAssign,
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
            b'[' => TokenKind::LBracket,
            b']' => TokenKind::RBracket,
            b';' => TokenKind::Semicolon,
            b',' => TokenKind::Comma,
            b'.' => TokenKind::Dot,
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
            b'?' => TokenKind::Question,
            b':' => TokenKind::Colon,
            _ => return None,
        };
        self.pos += 1;
        Some(Token {
            kind,
            span: Span::new(start, 1),
        })
    }
}
