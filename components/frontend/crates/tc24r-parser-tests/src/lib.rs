//! tc24r-parser.

#[cfg(test)]
pub(crate) fn parse_source(src: &str) -> tc24r_ast::Program {
    let tokens = tc24r_lexer::Lexer::new(src).tokenize().unwrap();
    tc24r_parser::parse(tokens).unwrap()
}

#[cfg(test)]
pub(crate) fn try_parse_source(src: &str) -> Result<tc24r_ast::Program, tc24r_error::CompileError> {
    let tokens = tc24r_lexer::Lexer::new(src).tokenize().unwrap();
    tc24r_parser::parse(tokens)
}

#[cfg(test)]
mod tests_array_const_expr;
#[cfg(test)]
mod tests_basic;
#[cfg(test)]
mod tests_compound_literal;
#[cfg(test)]
mod tests_enum;
#[cfg(test)]
mod tests_fnptr;
#[cfg(test)]
mod tests_globals;
#[cfg(test)]
mod tests_postfix;
#[cfg(test)]
mod tests_types;
