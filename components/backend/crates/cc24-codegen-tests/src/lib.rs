//! Integration tests for cc24-codegen.

#[cfg(test)]
mod as24_basic;
#[cfg(test)]
mod as24_ops;
#[cfg(test)]
mod cor24_basic;
#[cfg(test)]
mod cor24_ops;
#[cfg(test)]
mod golden_basic;
#[cfg(test)]
mod golden_flow;

/// Compile C source to COR24 assembly.
#[cfg(test)]
pub(crate) fn compile(source: &str) -> String {
    let tokens = cc24_lexer::Lexer::new(source)
        .tokenize()
        .expect("lexer failed");
    let program = cc24_parser::parse(tokens).expect("parser failed");
    cc24_codegen::Codegen::new().generate(&program)
}

/// Run a golden file test comparing compiler output to expected assembly.
#[cfg(test)]
pub(crate) fn golden_test(name: &str) {
    let c_source =
        std::fs::read_to_string(format!("fixtures/{name}.c")).expect("missing .c fixture");
    let expected = std::fs::read_to_string(format!("fixtures/{name}.expected.s"))
        .expect("missing .expected.s fixture");
    let actual = compile(&c_source);
    assert_eq!(actual, expected, "golden test failed for {name}");
}

/// Send assembly to the as24 HTTP service and return (status, body).
#[cfg(test)]
pub(crate) fn assemble_via_http(asm: &str) -> Option<(u16, String)> {
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

/// Assert that compiled C source assembles successfully via the as24 HTTP service.
#[cfg(test)]
pub(crate) fn assert_assembles_http(name: &str, source: &str) {
    let asm = compile(source);
    let (code, body) = assemble_via_http(&asm).expect("as24 service not reachable");
    assert_eq!(
        code, 200,
        "as24 rejected assembly for {name}:\n{body}\n\nGenerated assembly:\n{asm}"
    );
}

/// Return the path to cor24-run if it exists (check PATH first, then known location).
#[cfg(test)]
pub(crate) fn cor24_run_path() -> Option<std::path::PathBuf> {
    if let Some(output) = std::process::Command::new("which")
        .arg("cor24-run")
        .output()
        .ok()
        .filter(|o| o.status.success())
    {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        return Some(std::path::PathBuf::from(path));
    }
    let home = std::env::var("HOME").ok()?;
    let path = std::path::PathBuf::from(home)
        .join("github/sw-embed/cor24-rs/rust-to-cor24/target/release/cor24-run");
    if path.exists() { Some(path) } else { None }
}

/// Assemble source using the local cor24-run binary.
#[cfg(test)]
pub(crate) fn assemble_with_cor24_run(asm: &str) -> Result<String, String> {
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

/// Assert that compiled C source assembles successfully via cor24-run.
#[cfg(test)]
pub(crate) fn assert_assembles_cor24(name: &str, source: &str) {
    if cor24_run_path().is_none() {
        eprintln!("SKIP {name}: cor24-run not found");
        return;
    }
    let asm = compile(source);
    match assemble_with_cor24_run(&asm) {
        Ok(_) => {}
        Err(e) => panic!("cor24-run assembly failed for {name}: {e}\n\nGenerated assembly:\n{asm}"),
    }
}
