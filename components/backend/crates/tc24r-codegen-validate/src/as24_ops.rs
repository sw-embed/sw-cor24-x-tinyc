//! as24 HTTP validation tests for operators and functions.

use tc24r_test_as24::assert_assembles_http;

#[test]
#[ignore]
fn as24_binary_ops() {
    assert_assembles_http("sub", "int main() { return 7 - 2; }");
    assert_assembles_http("mul", "int main() { return 3 * 4; }");
    assert_assembles_http("bitand", "int main() { return 6 & 3; }");
    assert_assembles_http("bitor", "int main() { return 1 | 2; }");
    assert_assembles_http("bitxor", "int main() { return 5 ^ 3; }");
    assert_assembles_http("shl", "int main() { return 1 << 3; }");
    assert_assembles_http("shr", "int main() { return 8 >> 2; }");
}

#[test]
#[ignore]
fn as24_comparison_ops() {
    assert_assembles_http("eq", "int main() { return 1 == 1; }");
    assert_assembles_http("ne", "int main() { return 1 != 2; }");
    assert_assembles_http("lt", "int main() { return 1 < 2; }");
    assert_assembles_http("gt", "int main() { return 2 > 1; }");
    assert_assembles_http("le", "int main() { return 1 <= 2; }");
    assert_assembles_http("ge", "int main() { return 2 >= 1; }");
}

#[test]
#[ignore]
fn as24_unary_ops() {
    assert_assembles_http("neg", "int main() { return -42; }");
    assert_assembles_http("bitnot", "int main() { return ~0; }");
    assert_assembles_http("lognot", "int main() { return !0; }");
}

#[test]
#[ignore]
fn as24_function_call() {
    assert_assembles_http(
        "call",
        "int add(int a, int b) { return a + b; } int main() { return add(2, 5); }",
    );
}

#[test]
#[ignore]
fn as24_fib() {
    assert_assembles_http(
        "fib",
        "int fib(int n) { if (n < 2) { return 1; } return fib(n - 1) + fib(n - 2); } int main() { return fib(6); }",
    );
}

#[test]
#[ignore]
fn as24_globals() {
    assert_assembles_http("globals", "int x = 10; int main() { x = x + 5; return x; }");
}
