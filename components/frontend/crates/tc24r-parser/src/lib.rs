//! tc24r C compiler.

mod arithmetic;
mod bitwise;
mod control;
mod decl;
mod expr;
mod stmt;

use tc24r_ast::Program;
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::Token;

/// Parse a token stream into an AST program.
pub fn parse(tokens: Vec<Token>) -> Result<Program, CompileError> {
    let mut ts = TokenStream::new(tokens);
    decl::parse_program(&mut ts)
}
