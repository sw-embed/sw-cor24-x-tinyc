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
    /// Address-of: &x
    AddrOf(String),
    /// Pointer dereference: *p
    Deref(Box<Expr>),
    /// Cast expression: (type)expr
    Cast {
        ty: crate::Type,
        expr: Box<Expr>,
    },
    /// Dereference assignment: *p = val
    DerefAssign {
        ptr: Box<Expr>,
        value: Box<Expr>,
    },
    /// Pre-increment: ++i (returns new value)
    PreInc(String),
    /// Pre-decrement: --i (returns new value)
    PreDec(String),
    /// Post-increment: i++ (returns old value)
    PostInc(String),
    /// Post-decrement: i-- (returns old value)
    PostDec(String),
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
    LogAnd,
    LogOr,
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    BitNot,
    LogNot,
}
