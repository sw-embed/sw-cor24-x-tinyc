use cc24::codegen::Codegen;
use cc24::lexer::Lexer;
use cc24::parser::Parser;

fn compile(source: &str) -> String {
    let tokens = Lexer::new(source).tokenize().expect("lexer failed");
    let program = Parser::new(tokens).parse().expect("parser failed");
    Codegen::new().generate(&program)
}

fn golden_test(name: &str) {
    let c_source =
        std::fs::read_to_string(format!("tests/fixtures/{name}.c")).expect("missing .c fixture");
    let expected = std::fs::read_to_string(format!("tests/fixtures/{name}.expected.s"))
        .expect("missing .expected.s fixture");
    let actual = compile(&c_source);
    assert_eq!(actual, expected, "golden test failed for {name}");
}

// Phase 1: constants
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

// Phase 2: locals, operators, control flow
#[test]
fn golden_add_locals() {
    golden_test("add_locals");
}

#[test]
fn golden_if_else() {
    golden_test("if_else");
}

#[test]
fn golden_while_loop() {
    golden_test("while_loop");
}

// Phase 3: function calls and globals
#[test]
fn golden_call() {
    golden_test("call");
}

#[test]
fn golden_fib() {
    golden_test("fib");
}

#[test]
fn golden_globals() {
    golden_test("globals");
}
