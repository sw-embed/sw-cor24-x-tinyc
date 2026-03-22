//! Parser for the cc24 C compiler.

mod arithmetic;
mod bitwise;
mod control;
mod decl;
mod expr;
mod stmt;

use cc24_ast::Program;
use cc24_error::CompileError;
use cc24_parse_stream::TokenStream;
use cc24_token::Token;

/// Parse a token stream into an AST program.
pub fn parse(tokens: Vec<Token>) -> Result<Program, CompileError> {
    let mut ts = TokenStream::new(tokens);
    decl::parse_program(&mut ts)
}
