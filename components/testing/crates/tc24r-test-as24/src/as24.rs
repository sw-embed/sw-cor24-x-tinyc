/// Send assembly to the as24 HTTP service and return (status, body).
pub fn assemble_via_http(asm: &str) -> Option<(u16, String)> {
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
pub fn assert_assembles_http(name: &str, source: &str) {
    let asm = tc24r_test_compile::compile(source);
    let (code, body) = assemble_via_http(&asm).expect("as24 service not reachable");
    assert_eq!(
        code, 200,
        "as24 rejected assembly for {name}:\n{body}\n\nGenerated assembly:\n{asm}"
    );
}
