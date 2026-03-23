//! Attribute parsing helpers.

use crate::TokenStream;
use tc24r_token::TokenKind;

/// Consume `__attribute__((interrupt))` if present.
pub fn try_parse_interrupt_attr(ts: &mut TokenStream) -> bool {
    if !matches!(
        ts.peek().kind,
        TokenKind::Ident(ref s) if s == "__attribute__"
    ) {
        return false;
    }
    ts.advance(); // __attribute__
    if !ts.eat(TokenKind::LParen) {
        return false;
    }
    if !ts.eat(TokenKind::LParen) {
        return false;
    }
    // expect "interrupt" identifier
    if matches!(
        ts.peek().kind,
        TokenKind::Ident(ref s) if s == "interrupt"
    ) {
        ts.advance();
    }
    let _ = ts.eat(TokenKind::RParen);
    let _ = ts.eat(TokenKind::RParen);
    true
}
