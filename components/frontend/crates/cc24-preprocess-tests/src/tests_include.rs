//! Tests for #include and #pragma once.

use cc24_preprocess::preprocess;

#[test]
fn include_quote_file() {
    // Create a temp dir with header and source
    let dir = tempfile::tempdir().unwrap();
    let hdr = dir.path().join("defs.h");
    std::fs::write(&hdr, "#define VAL 42\n").unwrap();

    let input = "#include \"defs.h\"\nreturn VAL;\n";
    let output = preprocess(input, Some(dir.path()), &[]);
    assert_eq!(output, "return 42;\n");
}

#[test]
fn include_angle_bracket() {
    let sys_dir = tempfile::tempdir().unwrap();
    let hdr = sys_dir.path().join("hw.h");
    std::fs::write(&hdr, "#define LED 0xFF0000\n").unwrap();

    let input = "#include <hw.h>\nreturn LED;\n";
    let output = preprocess(input, None, &[sys_dir.path()]);
    assert_eq!(output, "return 0xFF0000;\n");
}

#[test]
fn pragma_once() {
    let dir = tempfile::tempdir().unwrap();
    let hdr = dir.path().join("once.h");
    std::fs::write(&hdr, "#pragma once\n#define X 1\n").unwrap();

    // Include twice -- should only define X once (no error)
    let input = "#include \"once.h\"\n#include \"once.h\"\nreturn X;\n";
    let output = preprocess(input, Some(dir.path()), &[]);
    assert_eq!(output, "return 1;\n");
}
