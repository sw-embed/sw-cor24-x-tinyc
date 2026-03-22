//! Abstract syntax tree nodes for the cc24 C compiler.

mod expr;
mod stmt;
mod types;

pub use expr::{BinOp, Expr, UnaryOp};
pub use stmt::{Block, Stmt};
pub use types::Type;

use cc24_span::Span;

/// A complete translation unit.
#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub globals: Vec<GlobalDecl>,
}

/// A top-level global variable declaration.
#[derive(Debug)]
pub struct GlobalDecl {
    pub name: String,
    pub ty: Type,
    pub init: Option<Expr>,
}

/// A function definition.
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_ty: Type,
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Span,
}

/// A function parameter.
#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}
