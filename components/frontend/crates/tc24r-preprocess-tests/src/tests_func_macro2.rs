//! Tests for function-like macro expansion (part 2).

use tc24r_preprocess::preprocess;

#[test]
fn func_macro_no_params() {
    let input = "#define NOP() 0\nint x = NOP();\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int x = 0;\n");
}

#[test]
fn func_macro_no_substitution_in_strings() {
    let input = "#define F(x) \"hello\" x\nF(world);\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "\"hello\" world;\n");
}

#[test]
fn func_macro_combined_with_simple_define() {
    let input = "#define VAL 42\n#define F(x) x + VAL\nint y = F(1);\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int y = 1 + 42;\n");
}

#[test]
fn func_macro_not_invoked_without_parens() {
    let input = "#define F(x) (x)\nint F = 1;\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int F = 1;\n");
}
