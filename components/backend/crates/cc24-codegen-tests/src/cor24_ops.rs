//! cor24-run validation tests for operators and functions.

use super::assert_assembles_cor24;

#[test]
fn cor24_binary_ops() {
    assert_assembles_cor24("sub", "int main() { return 7 - 2; }");
    assert_assembles_cor24("mul", "int main() { return 3 * 4; }");
    assert_assembles_cor24("bitand", "int main() { return 6 & 3; }");
    assert_assembles_cor24("bitor", "int main() { return 1 | 2; }");
    assert_assembles_cor24("bitxor", "int main() { return 5 ^ 3; }");
    assert_assembles_cor24("shl", "int main() { return 1 << 3; }");
    assert_assembles_cor24("shr", "int main() { return 8 >> 2; }");
}

#[test]
fn cor24_comparison_ops() {
    assert_assembles_cor24("eq", "int main() { return 1 == 1; }");
    assert_assembles_cor24("ne", "int main() { return 1 != 2; }");
    assert_assembles_cor24("lt", "int main() { return 1 < 2; }");
    assert_assembles_cor24("gt", "int main() { return 2 > 1; }");
    assert_assembles_cor24("le", "int main() { return 1 <= 2; }");
    assert_assembles_cor24("ge", "int main() { return 2 >= 1; }");
}

#[test]
fn cor24_unary_ops() {
    assert_assembles_cor24("neg", "int main() { return -42; }");
    assert_assembles_cor24("bitnot", "int main() { return ~0; }");
    assert_assembles_cor24("lognot", "int main() { return !0; }");
}

#[test]
fn cor24_function_call() {
    assert_assembles_cor24(
        "call",
        "int add(int a, int b) { return a + b; } int main() { return add(2, 5); }",
    );
}

#[test]
fn cor24_fib() {
    assert_assembles_cor24(
        "fib",
        "int fib(int n) { if (n < 2) { return 1; } return fib(n - 1) + fib(n - 2); }",
    );
}

#[test]
fn cor24_globals() {
    assert_assembles_cor24("globals", "int x = 10; int main() { x = x + 5; return x; }");
}
