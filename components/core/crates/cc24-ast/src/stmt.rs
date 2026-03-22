//! Statement and block AST nodes.

use crate::expr::Expr;
use crate::types::Type;

/// A brace-delimited block of statements.
#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

/// A single statement.
#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Expr(Expr),
    LocalDecl {
        name: String,
        ty: Type,
        init: Option<Expr>,
    },
    If {
        cond: Expr,
        then_body: Block,
        else_body: Option<Block>,
    },
    While {
        cond: Expr,
        body: Block,
    },
    DoWhile {
        body: Block,
        cond: Expr,
    },
    For {
        init: Option<Box<Stmt>>,
        cond: Option<Expr>,
        inc: Option<Expr>,
        body: Block,
    },
    Asm(String),
}
