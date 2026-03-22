//! Tests for emit macros.

use cc24_emit_macros::{emit, emit_comment, emit_label, emit_lines};

/// Minimal state struct for testing (mirrors CodegenState.out).
struct State {
    pub out: String,
}

#[test]
fn emit_literal() {
    let mut s = State { out: String::new() };
    emit!(s, "        add     r0,r1");
    assert_eq!(s.out, "        add     r0,r1\n");
}

#[test]
fn emit_with_format() {
    let mut s = State { out: String::new() };
    let offset = -3;
    emit!(s, "        lw      r0,{offset}(fp)");
    assert_eq!(s.out, "        lw      r0,-3(fp)\n");
}

#[test]
fn emit_lines_multiple() {
    let mut s = State { out: String::new() };
    emit_lines!(
        s,
        "        push    fp",
        "        push    r2",
        "        push    r1",
    );
    assert_eq!(
        s.out,
        "        push    fp\n        push    r2\n        push    r1\n"
    );
}

#[test]
fn emit_label_format() {
    let mut s = State { out: String::new() };
    emit_label!(s, "L0");
    assert_eq!(s.out, "L0:\n");
}

#[test]
fn emit_comment_format() {
    let mut s = State { out: String::new() };
    emit_comment!(s, "prologue");
    assert_eq!(s.out, "; prologue\n");
}
