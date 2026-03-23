//! Statement and block AST nodes.

use crate::expr::Expr;
use crate::types::Type;

/// A brace-delimited block of statements.
#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

/// A single case arm in a switch statement.
#[derive(Debug)]
pub struct SwitchCase {
    pub value: Expr,
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
    Switch {
        expr: Expr,
        cases: Vec<SwitchCase>,
        default: Option<Vec<Stmt>>,
    },
    Break,
    Continue,
    Asm(String),
    /// Inline block (for multi-declarations: int x, y;)
    Block(Block),
}
