//! tc24r -- Tiny C compiler targeting the COR24 ISA.

use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: tc24r <input.c> [-o output.s] [-I dir]");
        process::exit(1);
    }

    let input_path = &args[1];
    let source = read_source(input_path);
    let source_dir = Path::new(input_path).parent();
    let include_dirs = collect_include_dirs(&args);
    let output = compile(&source, source_dir, &include_dirs);
    write_output(&args, &output);
}

fn read_source(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("tc24r: cannot read {path}: {e}");
            process::exit(1);
        }
    }
}

fn collect_include_dirs(args: &[String]) -> Vec<String> {
    args.windows(2)
        .filter(|w| w[0] == "-I")
        .map(|w| w[1].clone())
        .collect()
}

fn compile(source: &str, source_dir: Option<&Path>, include_dirs: &[String]) -> String {
    let sys_paths: Vec<&Path> = include_dirs.iter().map(|s| Path::new(s.as_str())).collect();
    let preprocessed = tc24r_preprocess::preprocess(source, source_dir, &sys_paths);
    let tokens = match tc24r_lexer::Lexer::new(&preprocessed).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("tc24r: {e}");
            process::exit(1);
        }
    };

    let program = match tc24r_parser::parse(tokens) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tc24r: {e}");
            process::exit(1);
        }
    };

    tc24r_codegen::Codegen::new().generate(&program)
}

fn write_output(args: &[String], output: &str) {
    let output_path = args
        .windows(2)
        .find(|w| w[0] == "-o")
        .map(|w| w[1].as_str());

    match output_path {
        Some(path) => {
            if let Err(e) = std::fs::write(path, output) {
                eprintln!("tc24r: cannot write {path}: {e}");
                process::exit(1);
            }
        }
        None => print!("{output}"),
    }
}
