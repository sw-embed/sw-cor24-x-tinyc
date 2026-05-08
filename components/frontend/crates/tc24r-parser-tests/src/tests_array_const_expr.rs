//! Constant-expression array sizes: `int buf[A * B]`, `[(N + 1) * 2]`, etc.

use crate::{parse_source, try_parse_source};
use tc24r_ast::Type;

fn first_global_array_len(src: &str) -> usize {
    let p = parse_source(src);
    let g = p.globals.first().expect("expected one global");
    match &g.ty {
        Type::Array(_, n) => *n,
        other => panic!("expected Type::Array, got {other:?}"),
    }
}

#[test]
fn array_size_literal_mul() {
    assert_eq!(first_global_array_len("char buf[8 * 16];"), 128);
}

#[test]
fn array_size_literal_add() {
    assert_eq!(first_global_array_len("char buf[2 + 3];"), 5);
}

#[test]
fn array_size_literal_sub() {
    assert_eq!(first_global_array_len("char buf[10 - 3];"), 7);
}

#[test]
fn array_size_literal_div() {
    assert_eq!(first_global_array_len("char buf[100 / 5];"), 20);
}

#[test]
fn array_size_paren_grouping() {
    assert_eq!(first_global_array_len("char buf[(2 + 3) * 4];"), 20);
}

#[test]
fn array_size_paren_right_associative_override() {
    assert_eq!(first_global_array_len("char buf[2 * (3 + 4)];"), 14);
}

#[test]
fn array_size_mul_higher_precedence_than_add() {
    // 2 + 3 * 4 must be 14 (not 20). Verifies * binds tighter than +.
    assert_eq!(first_global_array_len("char buf[2 + 3 * 4];"), 14);
}

#[test]
fn array_size_div_left_associative() {
    // 100 / 5 / 2 == (100 / 5) / 2 == 10. Right-associative would give 40.
    assert_eq!(first_global_array_len("char buf[100 / 5 / 2];"), 10);
}

#[test]
fn array_size_unary_minus_in_subexpr() {
    // 5 + -2 == 3. Unary minus inside primary is supported.
    assert_eq!(first_global_array_len("char buf[5 + -2];"), 3);
}

#[test]
fn array_size_nested_parens() {
    // ((2 + 1) * (4 - 1)) == 9
    assert_eq!(first_global_array_len("char buf[((2 + 1) * (4 - 1))];"), 9);
}

#[test]
fn array_size_unsubstituted_ident_errors_with_clear_message() {
    // Without preprocessor #define substitution, identifiers in array sizes
    // are not resolvable to compile-time integers. The parser must reject
    // them and the message must name the offending identifier.
    let err = try_parse_source("char buf[FOO * 2];").unwrap_err();
    assert!(
        err.message.contains("FOO"),
        "expected error to name 'FOO', got: {}",
        err.message
    );
    assert!(
        err.message.contains("compile-time integer constant"),
        "expected 'compile-time integer constant' phrasing, got: {}",
        err.message
    );
}

#[test]
fn array_size_broken_expression_errors() {
    // `5 *` with nothing after — the second operand is missing.
    assert!(try_parse_source("char buf[5 *];").is_err());
}

#[test]
fn array_size_trailing_garbage_errors() {
    // `5 5` — two literals back-to-back, no operator. Should fail to close
    // the bracket cleanly.
    assert!(try_parse_source("char buf[5 5];").is_err());
}

#[test]
fn array_size_existing_literal_still_works() {
    // Regression: bare integer literal must keep working.
    assert_eq!(first_global_array_len("char buf[42];"), 42);
}
