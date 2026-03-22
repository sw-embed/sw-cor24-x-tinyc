//! Integration tests that validate cc24 output against the real as24 assembler HTTP service.
//!
//! These tests require the as24 HTTP service running on localhost:7412.
//! Run with: cargo test --test as24_validate -- --ignored
//!
//! For automated testing without the service, use the cor24_assemble tests instead.

use cc24::codegen::Codegen;
use cc24::lexer::Lexer;
use cc24::parser::Parser;

fn compile(source: &str) -> String {
    let tokens = Lexer::new(source).tokenize().expect("lexer failed");
    let program = Parser::new(tokens).parse().expect("parser failed");
    Codegen::new().generate(&program)
}

/// Send assembly to as24 HTTP service and return (status_code, body).
fn assemble_via_http(asm: &str) -> Option<(u16, String)> {
    let output = std::process::Command::new("curl")
        .args([
            "-s",
            "-o",
            "/dev/stdout",
            "-w",
            "\n%{http_code}",
            "-X",
            "POST",
            "http://localhost:7412/assemble",
            "--data-binary",
            "@-",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .ok()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(asm.as_bytes()).ok();
            }
            child.wait_with_output().ok()
        })?;

    if !output.status.success() {
        return None;
    }

    let full = String::from_utf8_lossy(&output.stdout).to_string();
    let lines: Vec<&str> = full.trim_end().rsplitn(2, '\n').collect();
    if lines.len() < 2 {
        return None;
    }
    let code: u16 = lines[0].trim().parse().ok()?;
    let body = lines[1].to_string();
    Some((code, body))
}

fn assert_assembles_http(name: &str, source: &str) {
    let asm = compile(source);
    let (code, body) = assemble_via_http(&asm).expect("as24 service not reachable");
    assert_eq!(
        code, 200,
        "as24 rejected assembly for {name}:\n{body}\n\nGenerated assembly:\n{asm}"
    );
}

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
fn as24_for_loop() {
    assert_assembles_http(
        "for_loop",
        "int main() { int s = 0; for (int i = 0; i < 10; i = i + 1) { s = s + i; } return s; }",
    );
}

// Phase 3: function calls and globals
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
        "int fib(int n) { if (n < 2) { return 1; } return fib(n - 1) + fib(n - 2); }",
    );
}

#[test]
#[ignore]
fn as24_globals() {
    assert_assembles_http("globals", "int x = 10; int main() { x = x + 5; return x; }");
}
