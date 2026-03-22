//! Golden file tests for basic expressions and locals.

use super::golden_test;

#[test]
fn golden_return_0() {
    golden_test("return_0");
}

#[test]
fn golden_return_42() {
    golden_test("return_42");
}

#[test]
fn golden_return_large() {
    golden_test("return_large");
}

#[test]
fn golden_add_locals() {
    golden_test("add_locals");
}

#[test]
fn golden_if_else() {
    golden_test("if_else");
}
