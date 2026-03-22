//! Golden file tests for control flow and function calls.

use super::golden_test;

#[test]
fn golden_while_loop() {
    golden_test("while_loop");
}

#[test]
fn golden_call() {
    golden_test("call");
}

#[test]
fn golden_fib() {
    golden_test("fib");
}

#[test]
fn golden_globals() {
    golden_test("globals");
}
