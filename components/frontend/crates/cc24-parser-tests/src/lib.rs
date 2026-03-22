//! Tests for cc24-parser.

#[cfg(test)]
pub(crate) fn parse_source(src: &str) -> cc24_ast::Program {
    let tokens = cc24_lexer::Lexer::new(src).tokenize().unwrap();
    cc24_parser::parse(tokens).unwrap()
}

#[cfg(test)]
mod tests_basic;
#[cfg(test)]
mod tests_enum;
#[cfg(test)]
mod tests_types;
