//! Integration tests that validate cc24 output using the cor24-run assembler.
//!
//! These tests use the locally-built cor24-run tool from ~/github/sw-embed/cor24-rs/.
//! If cor24-run is not found, tests are skipped (not failed).
//!
//! To build cor24-run:
//!   cd ~/github/sw-embed/cor24-rs/rust-to-cor24 && cargo build --release --bin cor24-run

use cc24::codegen::Codegen;
use cc24::lexer::Lexer;
use cc24::parser::Parser;

fn compile(source: &str) -> String {
    let tokens = Lexer::new(source).tokenize().expect("lexer failed");
    let program = Parser::new(tokens).parse().expect("parser failed");
    Codegen::new().generate(&program)
}

fn cor24_run_path() -> Option<std::path::PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let path = std::path::PathBuf::from(home)
        .join("github/sw-embed/cor24-rs/rust-to-cor24/target/release/cor24-run");
    if path.exists() { Some(path) } else { None }
}

/// Assemble a .s file using cor24-run. Returns Ok(listing) or Err(error message).
fn assemble_with_cor24_run(asm: &str) -> Result<String, String> {
    let cor24_run = cor24_run_path().ok_or("cor24-run not found")?;

    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let s_path = dir.path().join("test.s");
    let bin_path = dir.path().join("test.bin");
    let lst_path = dir.path().join("test.lst");

    std::fs::write(&s_path, asm).map_err(|e| e.to_string())?;

    let output = std::process::Command::new(cor24_run)
        .args([
            "--assemble",
            s_path.to_str().unwrap(),
            bin_path.to_str().unwrap(),
            lst_path.to_str().unwrap(),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("cor24-run failed:\n{stdout}\n{stderr}"));
    }

    std::fs::read_to_string(&lst_path).map_err(|e| e.to_string())
}

fn assert_assembles(name: &str, source: &str) {
    if cor24_run_path().is_none() {
        eprintln!(
            "SKIP {name}: cor24-run not found (build with: cd ~/github/sw-embed/cor24-rs/rust-to-cor24 && cargo build --release --bin cor24-run)"
        );
        return;
    }
    let asm = compile(source);
    match assemble_with_cor24_run(&asm) {
        Ok(_listing) => {} // assembled successfully
        Err(e) => panic!("cor24-run assembly failed for {name}: {e}\n\nGenerated assembly:\n{asm}"),
    }
}

// Phase 1: constants
#[test]
fn cor24_return_0() {
    assert_assembles("return_0", "int main() { return 0; }");
}

#[test]
fn cor24_return_42() {
    assert_assembles("return_42", "int main() { return 42; }");
}

#[test]
fn cor24_return_large() {
    assert_assembles("return_large", "int main() { return 1000; }");
}

// Phase 2: locals, operators, control flow
#[test]
fn cor24_add_locals() {
    assert_assembles(
        "add_locals",
        "int main() { int a = 2; int b = 3; return a + b; }",
    );
}

#[test]
fn cor24_if_else() {
    assert_assembles(
        "if_else",
        "int main() { if (1) { return 3; } else { return 4; } }",
    );
}

#[test]
fn cor24_while_loop() {
    assert_assembles(
        "while_loop",
        "int main() { int i = 0; while (i < 5) { i = i + 1; } return i; }",
    );
}

#[test]
fn cor24_binary_ops() {
    assert_assembles("sub", "int main() { return 7 - 2; }");
    assert_assembles("mul", "int main() { return 3 * 4; }");
    assert_assembles("bitand", "int main() { return 6 & 3; }");
    assert_assembles("bitor", "int main() { return 1 | 2; }");
    assert_assembles("bitxor", "int main() { return 5 ^ 3; }");
    assert_assembles("shl", "int main() { return 1 << 3; }");
    assert_assembles("shr", "int main() { return 8 >> 2; }");
}

#[test]
fn cor24_comparison_ops() {
    assert_assembles("eq", "int main() { return 1 == 1; }");
    assert_assembles("ne", "int main() { return 1 != 2; }");
    assert_assembles("lt", "int main() { return 1 < 2; }");
    assert_assembles("gt", "int main() { return 2 > 1; }");
    assert_assembles("le", "int main() { return 1 <= 2; }");
    assert_assembles("ge", "int main() { return 2 >= 1; }");
}

#[test]
fn cor24_unary_ops() {
    assert_assembles("neg", "int main() { return -42; }");
    assert_assembles("bitnot", "int main() { return ~0; }");
    assert_assembles("lognot", "int main() { return !0; }");
}

#[test]
fn cor24_for_loop() {
    assert_assembles(
        "for_loop",
        "int main() { int s = 0; for (int i = 0; i < 10; i = i + 1) { s = s + i; } return s; }",
    );
}

// Phase 3: function calls and globals
#[test]
fn cor24_function_call() {
    assert_assembles(
        "call",
        "int add(int a, int b) { return a + b; } int main() { return add(2, 5); }",
    );
}

#[test]
fn cor24_fib() {
    assert_assembles(
        "fib",
        "int fib(int n) { if (n < 2) { return 1; } return fib(n - 1) + fib(n - 2); }",
    );
}

#[test]
fn cor24_globals() {
    assert_assembles("globals", "int x = 10; int main() { x = x + 5; return x; }");
}
