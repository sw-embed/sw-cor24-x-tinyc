//! Compound assignment desugaring helpers.

use tc24r_ast::{BinOp, Expr};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

/// Try to eat a compound-assignment token, returning the corresponding `BinOp`.
pub fn eat_compound_assign(ts: &mut TokenStream) -> Option<BinOp> {
    let pairs = [
        (TokenKind::PlusAssign, BinOp::Add),
        (TokenKind::MinusAssign, BinOp::Sub),
        (TokenKind::StarAssign, BinOp::Mul),
        (TokenKind::SlashAssign, BinOp::Div),
        (TokenKind::PercentAssign, BinOp::Mod),
        (TokenKind::AmpAssign, BinOp::BitAnd),
        (TokenKind::PipeAssign, BinOp::BitOr),
        (TokenKind::CaretAssign, BinOp::BitXor),
        (TokenKind::LShiftAssign, BinOp::Shl),
        (TokenKind::RShiftAssign, BinOp::Shr),
    ];
    for (tok, op) in pairs {
        if ts.eat(tok) {
            return Some(op);
        }
    }
    None
}

/// Build a plain assignment node from lhs and value.
pub fn make_assign(lhs: Expr, value: Expr) -> Result<Expr, CompileError> {
    match lhs {
        Expr::Ident(name) => Ok(Expr::Assign {
            name,
            value: Box::new(value),
        }),
        Expr::Deref(ptr) => Ok(Expr::DerefAssign {
            ptr,
            value: Box::new(value),
        }),
        Expr::MemberAccess { object, member } => Ok(Expr::MemberAssign {
            object,
            member,
            value: Box::new(value),
        }),
        _ => Err(CompileError::new(
            "left side of assignment must be a variable or dereference",
            None,
        )),
    }
}

/// Desugar `lhs op= rhs` into `lhs = lhs op rhs`.
pub fn desugar_compound(lhs: Expr, op: BinOp, rhs: Expr) -> Result<Expr, CompileError> {
    match &lhs {
        Expr::Ident(_) | Expr::Deref(_) | Expr::MemberAccess { .. } => {}
        _ => {
            return Err(CompileError::new(
                "left side of compound assignment must be a variable or dereference",
                None,
            ));
        }
    }
    let binop = Expr::BinOp {
        op,
        lhs: Box::new(clone_lvalue(&lhs)),
        rhs: Box::new(rhs),
    };
    make_assign(lhs, binop)
}

/// Clone an lvalue expression for the read side of compound assignment.
/// Also clones sub-expressions (like pointer arithmetic in a[i]).
fn clone_lvalue(expr: &Expr) -> Expr {
    // Expr derives Clone, so we can clone any sub-expression.
    expr.clone()
}
