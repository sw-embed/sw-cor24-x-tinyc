//! Token kind enumeration.

/// All token variants recognised by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Int,
    Void,
    Return,
    If,
    Else,
    While,
    For,
    Asm,

    // Literals
    IntLit(i32),
    StringLit(String),

    // Identifiers
    Ident(String),

    // Punctuation
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Bang,
    LShift,
    RShift,
    Assign,
    EqEq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // End of file
    Eof,
}
