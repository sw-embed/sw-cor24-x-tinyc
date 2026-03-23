//! Golden file tests comparing compiler output to expected assembly.

use tc24r_test_golden::golden_test;

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

#[test]
fn golden_control_flow() {
    golden_test("while_loop");
    golden_test("call");
    golden_test("fib");
    golden_test("globals");
}
