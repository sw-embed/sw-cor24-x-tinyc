use crate::error::CompileError;
use crate::span::Span;
use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    source: &'a [u8],
    pos: usize,
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

    fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn peek_char(&self) -> Option<u8> {
        if self.pos + 1 < self.source.len() {
            Some(self.source[self.pos + 1])
        } else {
            None
        }
    }

    fn next_token(&mut self) -> Result<Token, CompileError> {
        let start = self.pos;
        let ch = self.source[self.pos];

        // Two-character operators
        if let Some(next) = self.peek_char() {
            let two_char = match (ch, next) {
                (b'<', b'<') => Some(TokenKind::LShift),
                (b'>', b'>') => Some(TokenKind::RShift),
                (b'=', b'=') => Some(TokenKind::EqEq),
                (b'!', b'=') => Some(TokenKind::BangEq),
                (b'<', b'=') => Some(TokenKind::LtEq),
                (b'>', b'=') => Some(TokenKind::GtEq),
                _ => None,
            };
            if let Some(kind) = two_char {
                self.pos += 2;
                return Ok(Token {
                    kind,
                    span: Span::new(start, 2),
                });
            }
        }

        // Single-character tokens
        let single = match ch {
            b'(' => Some(TokenKind::LParen),
            b')' => Some(TokenKind::RParen),
            b'{' => Some(TokenKind::LBrace),
            b'}' => Some(TokenKind::RBrace),
            b';' => Some(TokenKind::Semicolon),
            b',' => Some(TokenKind::Comma),
            b'+' => Some(TokenKind::Plus),
            b'-' => Some(TokenKind::Minus),
            b'*' => Some(TokenKind::Star),
            b'/' => Some(TokenKind::Slash),
            b'%' => Some(TokenKind::Percent),
            b'&' => Some(TokenKind::Amp),
            b'|' => Some(TokenKind::Pipe),
            b'^' => Some(TokenKind::Caret),
            b'~' => Some(TokenKind::Tilde),
            b'!' => Some(TokenKind::Bang),
            b'=' => Some(TokenKind::Assign),
            b'<' => Some(TokenKind::Lt),
            b'>' => Some(TokenKind::Gt),
            _ => None,
        };
        if let Some(kind) = single {
            self.pos += 1;
            return Ok(Token {
                kind,
                span: Span::new(start, 1),
            });
        }

        // String literals
        if ch == b'"' {
            return self.read_string(start);
        }

        // Integer literals
        if ch.is_ascii_digit() {
            return self.read_number(start);
        }

        // Identifiers and keywords
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

    fn read_ident(&mut self, start: usize) -> Token {
        while self.pos < self.source.len()
            && (self.source[self.pos].is_ascii_alphanumeric() || self.source[self.pos] == b'_')
        {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.source[start..self.pos]).unwrap();
        let kind = match text {
            "int" => TokenKind::Int,
            "void" => TokenKind::Void,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "asm" => TokenKind::Asm,
            _ => TokenKind::Ident(text.to_string()),
        };
        Token {
            kind,
            span: Span::new(start, self.pos - start),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_return_42() {
        let mut lexer = Lexer::new("int main() { return 42; }");
        let tokens = lexer.tokenize().unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                &TokenKind::Int,
                &TokenKind::Ident("main".to_string()),
                &TokenKind::LParen,
                &TokenKind::RParen,
                &TokenKind::LBrace,
                &TokenKind::Return,
                &TokenKind::IntLit(42),
                &TokenKind::Semicolon,
                &TokenKind::RBrace,
                &TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn tokenize_operators() {
        let mut lexer = Lexer::new("a + b << 2 == c");
        let tokens = lexer.tokenize().unwrap();
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                &TokenKind::Ident("a".to_string()),
                &TokenKind::Plus,
                &TokenKind::Ident("b".to_string()),
                &TokenKind::LShift,
                &TokenKind::IntLit(2),
                &TokenKind::EqEq,
                &TokenKind::Ident("c".to_string()),
                &TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn tokenize_if_while() {
        let mut lexer = Lexer::new("if (x) { while (y) { } }");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::If);
        assert_eq!(tokens[5].kind, TokenKind::While);
    }
}
