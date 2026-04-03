//! cor24-run validation tests for pointer and char programs.

use tc24r_test_cor24::assert_assembles_cor24;

#[test]
fn cor24_pointer_deref() {
    assert_assembles_cor24(
        "pointer_deref",
        "int main() { int x = 42; int *p = &x; return *p; }",
    );
}

#[test]
fn cor24_pointer_write() {
    assert_assembles_cor24(
        "pointer_write",
        "int main() { int x = 0; int *p = &x; *p = 99; return x; }",
    );
}

#[test]
fn cor24_char_local() {
    assert_assembles_cor24("char_local", "int main() { char c = 65; return c; }");
}

#[test]
fn cor24_string_constant() {
    // String literal should produce UART output
    assert_assembles_cor24(
        "string_const",
        "void putc(int c) { *(char *)0xFF0100 = c; } int main() { char *s = \"AB\"; putc(*s); return 0; }",
    );
}

#[test]
fn cor24_div_mod() {
    // 17 / 5 = 3, 17 % 5 = 2, combined = 3 + 2 = 5
    assert_assembles_cor24("div_mod", "int main() { return 17 / 5 + 17 % 5; }");
}

#[test]
fn cor24_fn_ptr_basic() {
    assert_assembles_cor24(
        "fn_ptr_basic",
        "int add(int a, int b) { return a + b; } \
         int main() { int (*fp)(int, int) = add; return fp(3, 4); }",
    );
}

#[test]
fn cor24_fn_ptr_array() {
    assert_assembles_cor24(
        "fn_ptr_array",
        "int dbl(int x) { return x + x; } \
         int main() { int (*t[4])(int); t[0] = dbl; return t[0](5); }",
    );
}

#[test]
fn cor24_fn_ptr_param() {
    assert_assembles_cor24(
        "fn_ptr_param",
        "int dbl(int x) { return x + x; } \
         int apply(int (*f)(int), int x) { return f(x); } \
         int main() { return apply(dbl, 5); }",
    );
}

#[test]
fn cor24_char_ptr_arithmetic() {
    // char *p points to a char, p + 1 should advance by 1 byte
    assert_assembles_cor24(
        "char_ptr_arith",
        "int main() { char *p = (char *)0xFF0100; char *q = p + 1; return *q; }",
    );
}

#[test]
fn cor24_postinc_struct_member() {
    assert_assembles_cor24(
        "postinc_struct_member",
        "struct s { int x; }; int main() { struct s a; a.x = 5; a.x++; return a.x; }",
    );
}

#[test]
fn cor24_postinc_arrow_member() {
    assert_assembles_cor24(
        "postinc_arrow_member",
        "struct s { int x; }; int main() { struct s a; struct s *p = &a; p->x = 5; p->x++; return p->x; }",
    );
}

#[test]
fn cor24_postinc_array_element() {
    assert_assembles_cor24(
        "postinc_array_element",
        "int main() { int a[4]; a[2] = 10; a[2]++; return a[2]; }",
    );
}

#[test]
fn cor24_predec_array_element() {
    assert_assembles_cor24(
        "predec_array_element",
        "int main() { int a[4]; a[0] = 10; --a[0]; return a[0]; }",
    );
}

#[test]
fn cor24_addr_of_struct_member() {
    assert_assembles_cor24(
        "addr_of_struct_member",
        "struct s { int x; int y; }; int main() { struct s a; a.y = 42; int *p = &a.y; return *p; }",
    );
}

#[test]
fn cor24_addr_of_nested_member() {
    assert_assembles_cor24(
        "addr_of_nested_member",
        "struct inner { int x; }; struct outer { struct inner b; int mode; }; \
         int main() { struct outer a; a.b.x = 7; int *p = &a.b.x; return *p; }",
    );
}

#[test]
fn cor24_addr_of_passed_to_func() {
    assert_assembles_cor24(
        "addr_of_passed_to_func",
        "struct s { int x; }; int get(struct s *p) { return p->x; } \
         int main() { struct s a; a.x = 33; return get(&a); }",
    );
}
