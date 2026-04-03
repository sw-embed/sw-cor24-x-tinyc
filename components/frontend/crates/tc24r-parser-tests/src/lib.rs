//! tc24r-parser.

#[cfg(test)]
pub(crate) fn parse_source(src: &str) -> tc24r_ast::Program {
    let tokens = tc24r_lexer::Lexer::new(src).tokenize().unwrap();
    tc24r_parser::parse(tokens).unwrap()
}

#[cfg(test)]
mod tests_basic;
#[cfg(test)]
mod tests_enum;
#[cfg(test)]
mod tests_fnptr;
#[cfg(test)]
mod tests_types;
#[cfg(test)]
mod tests_postfix;
