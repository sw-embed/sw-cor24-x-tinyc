/// Compile C source to COR24 assembly.
pub fn compile(source: &str) -> String {
    let tokens = tc24r_lexer::Lexer::new(source)
        .tokenize()
        .expect("lexer failed");
    let program = tc24r_parser::parse(tokens).expect("parser failed");
    tc24r_codegen::Codegen::new().generate(&program)
}
