//! Tests for #define substitution.

use tc24r_preprocess::preprocess;

#[test]
fn define_simple_constant() {
    let input = "#define FOO 42\nint x = FOO;\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int x = 42;\n");
}

#[test]
fn define_hex_constant() {
    let input = "#define LED_ADDR 0xFF0000\n*(char *)LED_ADDR = 0;\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "*(char *)0xFF0000 = 0;\n");
}

#[test]
fn multiple_defines() {
    let input = "#define A 1\n#define B 2\nreturn A + B;\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "return 1 + 2;\n");
}

#[test]
fn no_substitution_in_strings() {
    let input = "#define X 99\nchar *s = \"X is X\";\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "char *s = \"X is X\";\n");
}

#[test]
fn no_partial_match() {
    let input = "#define FOO 1\nint FOOBAR = 2;\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int FOOBAR = 2;\n");
}

#[test]
fn define_strips_line_comment() {
    let input = "#define FOO 10 // this is a comment\nint x = FOO;\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int x = 10;\n");
}

#[test]
fn define_preserves_slash_in_string() {
    let input = "#define URL \"http://example.com\"\nchar *s = URL;\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "char *s = \"http://example.com\";\n");
}

#[test]
fn func_macro_strips_line_comment() {
    let input = "#define ADD(a, b) ((a)+(b)) // sum\nint x = ADD(1, 2);\n";
    let output = preprocess(input, None, &[]);
    assert_eq!(output, "int x = ((1)+(2));\n");
}
