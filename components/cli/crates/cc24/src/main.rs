//! cc24 -- C compiler targeting the COR24 ISA.

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: cc24 <input.c> [-o output.s]");
        process::exit(1);
    }

    let source = read_source(&args[1]);
    let output = compile(&source);
    write_output(&args, &output);
}

fn read_source(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cc24: cannot read {path}: {e}");
            process::exit(1);
        }
    }
}

fn compile(source: &str) -> String {
    let tokens = match cc24_lexer::Lexer::new(source).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cc24: {e}");
            process::exit(1);
        }
    };

    let program = match cc24_parser::parse(tokens) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("cc24: {e}");
            process::exit(1);
        }
    };

    cc24_codegen::Codegen::new().generate(&program)
}

fn write_output(args: &[String], output: &str) {
    let output_path = args
        .windows(2)
        .find(|w| w[0] == "-o")
        .map(|w| w[1].as_str());

    match output_path {
        Some(path) => {
            if let Err(e) = std::fs::write(path, output) {
                eprintln!("cc24: cannot write {path}: {e}");
                process::exit(1);
            }
        }
        None => print!("{output}"),
    }
}
