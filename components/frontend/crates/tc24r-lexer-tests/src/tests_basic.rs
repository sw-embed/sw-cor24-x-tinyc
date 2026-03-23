//! Basic tokenisation tests.

use tc24r_lexer::Lexer;
use tc24r_token::TokenKind;

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
