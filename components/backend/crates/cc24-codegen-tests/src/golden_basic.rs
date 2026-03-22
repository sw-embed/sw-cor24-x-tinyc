//! Golden file tests for basic expressions and locals.

use super::{compile, golden_test};

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
fn codegen_emits_start() {
    let output = compile("int main() { return 0; }");
    assert!(output.contains("_start:"));
    assert!(output.contains("la      r0,_main"));
    assert!(output.contains("jal     r1,(r0)"));
    assert!(output.contains("_halt:"));
    assert!(output.contains("bra     _halt"));
}
