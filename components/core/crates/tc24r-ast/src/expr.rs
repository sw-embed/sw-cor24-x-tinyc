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
    /// Indirect call through expression: expr(args)
    IndirectCall {
        callee: Box<Expr>,
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
    /// Pre-increment: ++lvalue (returns new value)
    PreInc(Box<Expr>),
    /// Pre-decrement: --lvalue (returns new value)
    PreDec(Box<Expr>),
    /// Post-increment: lvalue++ (returns old value)
    PostInc(Box<Expr>),
    /// Post-decrement: lvalue-- (returns old value)
    PostDec(Box<Expr>),
    /// Struct member access: expr.member
    MemberAccess {
        object: Box<Expr>,
        member: String,
    },
    /// Struct member assignment: expr.member = value
    MemberAssign {
        object: Box<Expr>,
        member: String,
        value: Box<Expr>,
    },
    /// GCC statement expression: ({ stmt1; stmt2; expr; })
    StmtExpr(crate::Block),
    /// sizeof applied to an expression: sizeof x, sizeof(expr)
    SizeofExpr(Box<Expr>),
    /// Ternary: cond ? then_expr : else_expr
    Ternary {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
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
    LogAnd,
    LogOr,
    Comma,
}

/// Unary operator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    BitNot,
    LogNot,
}
