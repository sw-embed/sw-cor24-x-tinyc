use std::process;

use cc24::codegen::Codegen;
use cc24::lexer::Lexer;
use cc24::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: cc24 <input.c> [-o output.s]");
        process::exit(1);
    }

    let input_path = &args[1];
    let source = match std::fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cc24: cannot read {input_path}: {e}");
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(&source).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cc24: {e}");
            process::exit(1);
        }
    };

    let program = match Parser::new(tokens).parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("cc24: {e}");
            process::exit(1);
        }
    };

    let output = Codegen::new().generate(&program);

    // Write to -o file or stdout
    let output_path = args
        .windows(2)
        .find(|w| w[0] == "-o")
        .map(|w| w[1].as_str());

    match output_path {
        Some(path) => {
            if let Err(e) = std::fs::write(path, &output) {
                eprintln!("cc24: cannot write {path}: {e}");
                process::exit(1);
            }
        }
        None => print!("{output}"),
    }
}
