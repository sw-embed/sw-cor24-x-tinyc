//! as24 HTTP validation tests for basic programs.

use super::assert_assembles_http;

#[test]
#[ignore]
fn as24_return_0() {
    assert_assembles_http("return_0", "int main() { return 0; }");
}

#[test]
#[ignore]
fn as24_return_42() {
    assert_assembles_http("return_42", "int main() { return 42; }");
}

#[test]
#[ignore]
fn as24_return_large() {
    assert_assembles_http("return_large", "int main() { return 1000; }");
}

#[test]
#[ignore]
fn as24_add_locals() {
    assert_assembles_http(
        "add_locals",
        "int main() { int a = 2; int b = 3; return a + b; }",
    );
}

#[test]
#[ignore]
fn as24_if_else() {
    assert_assembles_http(
        "if_else",
        "int main() { if (1) { return 3; } else { return 4; } }",
    );
}

#[test]
#[ignore]
fn as24_while_loop() {
    assert_assembles_http(
        "while_loop",
        "int main() { int i = 0; while (i < 5) { i = i + 1; } return i; }",
    );
}

#[test]
#[ignore]
fn as24_for_loop() {
    assert_assembles_http(
        "for_loop",
        "int main() { int s = 0; for (int i = 0; i < 10; i = i + 1) { s = s + i; } return s; }",
    );
}
