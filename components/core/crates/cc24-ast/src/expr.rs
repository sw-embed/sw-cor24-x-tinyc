//! Expression AST nodes.

/// An expression node.
#[derive(Debug)]
pub enum Expr {
    IntLit(i32),
    StringLit(String),
    Ident(String),
    Assign {
        name: String,
        value: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    BinOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
    },
}

/// Binary operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    BitNot,
    LogNot,
}
