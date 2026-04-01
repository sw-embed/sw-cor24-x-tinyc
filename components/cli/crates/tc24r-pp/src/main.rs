//! tc24r-pp -- Standalone C preprocessor for the tc24r compiler.
//!
//! Reads a C source file, runs preprocessing (macro expansion, includes,
//! conditionals, stringification), and outputs the result.

use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: tc24r-pp <input.c> [-o output] [-I dir]");
        process::exit(1);
    }

    let input_path = &args[1];
    let source = match std::fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("tc24r-pp: cannot read {input_path}: {e}");
            process::exit(1);
        }
    };

    let source_dir = Path::new(input_path).parent();
    let include_dirs: Vec<String> = args
        .windows(2)
        .filter(|w| w[0] == "-I")
        .map(|w| w[1].clone())
        .collect();
    let sys_paths: Vec<&Path> = include_dirs.iter().map(|s| Path::new(s.as_str())).collect();

    let output = tc24r_preprocess::preprocess(&source, source_dir, &sys_paths);

    let output_path = args
        .windows(2)
        .find(|w| w[0] == "-o")
        .map(|w| w[1].as_str());

    match output_path {
        Some(path) => {
            if let Err(e) = std::fs::write(path, &output) {
                eprintln!("tc24r-pp: cannot write {path}: {e}");
                process::exit(1);
            }
        }
        None => print!("{output}"),
    }
}
