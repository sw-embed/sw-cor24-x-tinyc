//! cor24-run validation tests for operators and functions.

use tc24r_test_cor24::assert_assembles_cor24;

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
        "int fib(int n) { if (n < 2) { return 1; } return fib(n - 1) + fib(n - 2); } int main() { return fib(6); }",
    );
}

#[test]
fn cor24_globals() {
    assert_assembles_cor24("globals", "int x = 10; int main() { x = x + 5; return x; }");
}

#[test]
fn cor24_global_string_init() {
    // static char[] from string literal — msg[0] = 'h' = 104
    assert_assembles_cor24(
        "global_string_init",
        "static const char msg[] = \"hello\"; int main(void) { return msg[0]; }",
    );
}

#[test]
fn cor24_global_int_init_list() {
    // int[] from initializer list — 72 + 105 + 0 = 177
    assert_assembles_cor24(
        "global_int_init_list",
        "int msg[] = {72, 105, 0}; int main(void) { return msg[0] + msg[1] + msg[2]; }",
    );
}

#[test]
fn cor24_global_char_init_list() {
    // char[] from initializer list — 'A' + 'B' = 65 + 66 = 131
    assert_assembles_cor24(
        "global_char_init_list",
        "char msg[] = {65, 66, 0}; int main(void) { return msg[0] + msg[1]; }",
    );
}

#[test]
fn cor24_global_init_list_zero_pad() {
    // int[5] with 2 values — arr[0] + arr[4] = 10 + 0 = 10
    assert_assembles_cor24(
        "global_init_list_zeropad",
        "int arr[5] = {10, 20}; int main(void) { return arr[0] + arr[2] + arr[4]; }",
    );
}

#[test]
fn cor24_postinc_plain() {
    // i = 5; j = i++; j should be 5 (old value), i should be 6
    assert_assembles_cor24(
        "postinc_plain",
        "int main() { int i = 5; int j = i++; return j + i; }",
    );
}

#[test]
fn cor24_preinc_plain() {
    // i = 5; j = ++i; j should be 6 (new value)
    assert_assembles_cor24(
        "preinc_plain",
        "int main() { int i = 5; int j = ++i; return j; }",
    );
}

#[test]
fn cor24_postdec_plain() {
    // i = 5; j = i--; j should be 5, i should be 4
    assert_assembles_cor24(
        "postdec_plain",
        "int main() { int i = 5; int j = i--; return j + i; }",
    );
}

#[test]
fn cor24_predec_plain() {
    assert_assembles_cor24(
        "predec_plain",
        "int main() { int i = 5; int j = --i; return j; }",
    );
}

#[test]
fn cor24_compound_literal_scalar() {
    assert_assembles_cor24(
        "compound_literal_scalar",
        "int main() { int x = (int){42}; return x; }",
    );
}

#[test]
fn cor24_compound_literal_addr_of() {
    assert_assembles_cor24(
        "compound_literal_addr_of",
        "int main() { int *p = &(int){42}; return *p; }",
    );
}

#[test]
fn cor24_compound_literal_write_through_ptr() {
    assert_assembles_cor24(
        "compound_literal_write",
        "int main() { int *p = &(int){42}; *p = 99; return *p; }",
    );
}

#[test]
fn cor24_compound_literal_array() {
    assert_assembles_cor24(
        "compound_literal_array",
        "int main() { int *p = (int[]){10, 20, 30}; return p[0] + p[1] + p[2]; }",
    );
}

#[test]
fn cor24_compound_literal_struct() {
    assert_assembles_cor24(
        "compound_literal_struct",
        "struct S { int x; int y; }; int main() { struct S *p = &(struct S){3, 4}; return p->x + p->y; }",
    );
}
