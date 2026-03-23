//! Token kind enumeration.

/// All token variants recognised by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Char,
    Int,
    Void,
    Return,
    If,
    Else,
    Break,
    Continue,
    Do,
    While,
    For,
    Asm,
    Sizeof,
    Static,
    Extern,
    Const,
    Enum,
    Struct,
    Union,
    Typedef,

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
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Dot,

    // Operators
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Star,
    Slash,
    Percent,
    Amp,
    AmpAmp,
    Pipe,
    PipePipe,
    Caret,
    Tilde,
    Bang,
    LShift,
    RShift,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    AmpAssign,
    PipeAssign,
    CaretAssign,
    LShiftAssign,
    RShiftAssign,
    EqEq,
    BangEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Question,
    Colon,

    // End of file
    Eof,
}
