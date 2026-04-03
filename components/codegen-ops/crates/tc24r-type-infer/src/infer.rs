//! Expression type inference.

use tc24r_ast::{BinOp, Expr, Type};
use tc24r_codegen_state::CodegenState;

/// Infer the type of an expression from codegen state.
///
/// Returns `None` for expressions whose type cannot be determined
/// (e.g., integer literals, function calls without return-type tracking).
pub fn expr_type(state: &CodegenState, expr: &Expr) -> Option<Type> {
    match expr {
        Expr::Ident(name) => {
            let ty = state
                .local_types
                .get(name)
                .or_else(|| state.global_types.get(name))
                .cloned()?;
            // Array decays to pointer to element
            match ty {
                Type::Array(inner, _) => Some(Type::Ptr(inner)),
                other => Some(other),
            }
        }
        Expr::Cast { ty, .. } => Some(ty.clone()),
        Expr::AddrOf(name) => {
            let inner = expr_type(state, &Expr::Ident(name.clone()))?;
            Some(Type::Ptr(Box::new(inner)))
        }
        Expr::Deref(inner) => match expr_type(state, inner)? {
            Type::Ptr(pointee) => {
                // Array result decays to pointer to element (e.g. x[0] on int x[2][3])
                match *pointee {
                    Type::Array(elem, _) => Some(Type::Ptr(elem)),
                    other => Some(other),
                }
            }
            _ => None,
        },
        Expr::StringLit(_) => Some(Type::Ptr(Box::new(Type::Char))),
        Expr::BinOp {
            op: BinOp::Add,
            lhs,
            ..
        }
        | Expr::BinOp {
            op: BinOp::Sub,
            lhs,
            ..
        } => {
            // Pointer arithmetic preserves pointer type
            let lhs_ty = expr_type(state, lhs)?;
            if matches!(lhs_ty, Type::Ptr(_)) {
                Some(lhs_ty)
            } else {
                None
            }
        }
        Expr::SizeofExpr(_) => Some(Type::Int),
        _ => None,
    }
}
