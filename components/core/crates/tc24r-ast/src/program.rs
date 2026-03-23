use tc24r_span::Span;

use crate::{Block, Expr, Type};

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
    pub is_interrupt: bool,
}

/// A function parameter.
#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}
