/// Return the path to cor24-run if it exists (check PATH first, then known location).
pub fn cor24_run_path() -> Option<std::path::PathBuf> {
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
pub fn assemble_with_cor24_run(asm: &str) -> Result<String, String> {
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
pub fn assert_assembles_cor24(name: &str, source: &str) {
    if cor24_run_path().is_none() {
        eprintln!("SKIP {name}: cor24-run not found");
        return;
    }
    let asm = tc24r_test_compile::compile(source);
    match assemble_with_cor24_run(&asm) {
        Ok(_) => {}
        Err(e) => panic!("cor24-run assembly failed for {name}: {e}\n\nGenerated assembly:\n{asm}"),
    }
}
