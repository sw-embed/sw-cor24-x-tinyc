use crate::span::Span;

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub globals: Vec<GlobalDecl>,
}

#[derive(Debug)]
pub struct GlobalDecl {
    pub name: String,
    pub ty: Type,
    pub init: Option<Expr>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_ty: Type,
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

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
    For {
        init: Option<Box<Stmt>>,
        cond: Option<Expr>,
        inc: Option<Expr>,
        body: Block,
    },
    Asm(String),
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
    BitNot,
    LogNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Void,
}
