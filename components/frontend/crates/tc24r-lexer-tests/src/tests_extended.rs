//! Tests for strings, comments, and hex literals.

use tc24r_lexer::Lexer;
use tc24r_token::TokenKind;

#[test]
fn tokenize_string_literal() {
    let mut lexer = Lexer::new(r#""hello\n""#);
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].kind, TokenKind::StringLit("hello\n".to_string()));
}

#[test]
fn tokenize_line_comment() {
    let mut lexer = Lexer::new("int x; // comment\nreturn x;");
    let tokens = lexer.tokenize().unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            &TokenKind::Int,
            &TokenKind::Ident("x".to_string()),
            &TokenKind::Semicolon,
            &TokenKind::Return,
            &TokenKind::Ident("x".to_string()),
            &TokenKind::Semicolon,
            &TokenKind::Eof,
        ]
    );
}

#[test]
fn tokenize_block_comment() {
    let mut lexer = Lexer::new("int /* skip this */ x;");
    let tokens = lexer.tokenize().unwrap();
    let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            &TokenKind::Int,
            &TokenKind::Ident("x".to_string()),
            &TokenKind::Semicolon,
            &TokenKind::Eof,
        ]
    );
}

#[test]
fn tokenize_hex_literal() {
    let mut lexer = Lexer::new("0xFF0000");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].kind, TokenKind::IntLit(0xFF0000));
}

#[test]
fn tokenize_hex_small() {
    let mut lexer = Lexer::new("0x2A");
    let tokens = lexer.tokenize().unwrap();
    assert_eq!(tokens[0].kind, TokenKind::IntLit(42));
}
