//! Literal and identifier reading.

use tc24r_error::CompileError;
use tc24r_span::Span;
use tc24r_token::{Token, TokenKind};

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
        if ch == b'\'' {
            return self.read_char_lit(start);
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
        // Check for 0x hex prefix
        if self.pos + 1 < self.source.len()
            && self.source[self.pos] == b'0'
            && (self.source[self.pos + 1] == b'x' || self.source[self.pos + 1] == b'X')
        {
            return self.read_hex(start);
        }
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_digit() {
            self.pos += 1;
        }
        // Skip optional integer suffix (U, L, LL, UL, etc.)
        while self.pos < self.source.len()
            && matches!(self.source[self.pos], b'u' | b'U' | b'l' | b'L')
        {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.source[start..self.pos]).unwrap();
        let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
        // Parse as u64 then truncate to 24 bits
        let wide: u64 = digits.parse().unwrap_or(0);
        let value = (wide & 0xFFFFFF) as i32;
        Ok(Token {
            kind: TokenKind::IntLit(value),
            span: Span::new(start, self.pos - start),
        })
    }

    fn read_hex(&mut self, start: usize) -> Result<Token, CompileError> {
        self.pos += 2; // skip 0x
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_hexdigit() {
            self.pos += 1;
        }
        // Skip optional integer suffix (U, L, LL, UL, etc.)
        while self.pos < self.source.len()
            && matches!(self.source[self.pos], b'u' | b'U' | b'l' | b'L')
        {
            self.pos += 1;
        }
        let hex_text = std::str::from_utf8(&self.source[start + 2..self.pos]).unwrap();
        let hex_digits: String = hex_text.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        // Parse as u64 then truncate to 24 bits (COR24 is a 24-bit machine)
        let wide = u64::from_str_radix(&hex_digits, 16).unwrap_or(0);
        let value = (wide & 0xFFFFFF) as i32;
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
            b'a' => '\x07',
            b'b' => '\x08',
            b'r' => '\r',
            b'v' => '\x0B',
            b'f' => '\x0C',
            b'e' => '\x1B',
            b'\'' => '\'',
            b'x' => {
                // Hex escape: \xNN
                let mut val: u8 = 0;
                self.pos += 1;
                while self.pos < self.source.len() && self.source[self.pos].is_ascii_hexdigit() {
                    let d = self.source[self.pos];
                    let nibble = if d.is_ascii_digit() {
                        d - b'0'
                    } else {
                        (d | 0x20) - b'a' + 10
                    };
                    val = val.wrapping_mul(16).wrapping_add(nibble);
                    self.pos += 1;
                }
                // pos is now past the hex digits; caller will increment
                // so we need to back up one
                self.pos -= 1;
                value.push(val as char);
                return Ok(());
            }
            // Unknown escapes: emit the character literally (C allows
            // implementation-defined behavior for unknown escape sequences)
            other => other as char,
        };
        value.push(escaped);
        Ok(())
    }
}

impl Lexer<'_> {
    /// Read hex digits and return the accumulated byte value.
    /// Leaves `self.pos` on the last hex digit consumed (caller will advance).
    fn read_hex_escape_value(&mut self) -> u8 {
        self.pos += 1; // skip 'x'
        let mut val: u8 = 0;
        while self.pos < self.source.len() && self.source[self.pos].is_ascii_hexdigit() {
            let d = self.source[self.pos];
            let nibble = if d.is_ascii_digit() {
                d - b'0'
            } else {
                (d | 0x20) - b'a' + 10
            };
            val = val.wrapping_mul(16).wrapping_add(nibble);
            self.pos += 1;
        }
        self.pos -= 1; // back up so caller can advance
        val
    }

    fn read_char_lit(&mut self, start: usize) -> Result<Token, CompileError> {
        self.pos += 1; // skip opening '
        if self.pos >= self.source.len() {
            return Err(CompileError::new(
                "unterminated char literal",
                Some(Span::new(start, 1)),
            ));
        }
        let val = if self.source[self.pos] == b'\\' {
            self.pos += 1;
            match self.source.get(self.pos) {
                Some(b'n') => b'\n',
                Some(b't') => b'\t',
                Some(b'0') => 0,
                Some(b'\\') => b'\\',
                Some(b'\'') => b'\'',
                Some(b'a') => 7,
                Some(b'b') => 8,
                Some(b'r') => b'\r',
                Some(b'x') => self.read_hex_escape_value(),
                _ => {
                    return Err(CompileError::new(
                        "unknown char escape",
                        Some(Span::new(start, self.pos - start)),
                    ));
                }
            }
        } else {
            self.source[self.pos]
        };
        self.pos += 1; // skip char
        if self.source.get(self.pos) != Some(&b'\'') {
            return Err(CompileError::new(
                "expected closing '",
                Some(Span::new(start, self.pos - start)),
            ));
        }
        self.pos += 1; // skip closing '
        Ok(Token {
            kind: TokenKind::IntLit(val as i32),
            span: Span::new(start, self.pos - start),
        })
    }
}

/// Map identifier text to a keyword token kind, or `Ident`.
pub(crate) fn keyword_or_ident(text: &str) -> TokenKind {
    match text {
        "int" => TokenKind::Int,
        "void" => TokenKind::Void,
        "return" => TokenKind::Return,
        "break" => TokenKind::Break,
        "char" => TokenKind::Char,
        "const" => TokenKind::Const,
        "continue" => TokenKind::Continue,
        "do" => TokenKind::Do,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "for" => TokenKind::For,
        "asm" => TokenKind::Asm,
        "sizeof" => TokenKind::Sizeof,
        "offsetof" => TokenKind::Offsetof,
        "static" => TokenKind::Static,
        "enum" => TokenKind::Enum,
        "extern" => TokenKind::Extern,
        "struct" => TokenKind::Struct,
        "union" => TokenKind::Union,
        "typedef" => TokenKind::Typedef,
        "switch" => TokenKind::Switch,
        "case" => TokenKind::Case,
        "default" => TokenKind::Default,
        "long" => TokenKind::Long,
        "short" => TokenKind::Short,
        "signed" => TokenKind::Signed,
        "unsigned" => TokenKind::Unsigned,
        "inline" => TokenKind::Inline,
        "goto" => TokenKind::Goto,
        "restrict" | "__restrict" | "__restrict__" => TokenKind::Restrict,
        "_Noreturn" | "__noreturn__" => TokenKind::Noreturn,
        "volatile" => TokenKind::Volatile,
        "auto" => TokenKind::Auto,
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
