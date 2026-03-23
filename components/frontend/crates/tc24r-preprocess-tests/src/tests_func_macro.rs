//! Tests for function-like macro expansion (part 1).

use tc24r_preprocess::preprocess;

#[test]
fn func_macro_add() {
    let input = "#define ADD(a,b) ((a)+(b))\nint main() { return ADD(3, 4); }\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int main() { return ((3)+(4)); }\n");
}

#[test]
fn func_macro_assert() {
    let input = "#define ASSERT(x, y) assert(x, y, \"\")\nASSERT(42, 42);\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "assert(42, 42, \"\");\n");
}

#[test]
fn func_macro_nested_parens() {
    let input = "#define M(x) (x)\nM((3+2));\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "((3+2));\n");
}

#[test]
fn func_macro_nested_call_in_arg() {
    let input = "#define M(x) (x)\nM(func(a,b));\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "(func(a,b));\n");
}
