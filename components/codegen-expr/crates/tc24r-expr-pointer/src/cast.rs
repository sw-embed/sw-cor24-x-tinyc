//! Cast expression codegen with narrowing/widening.

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Generate a cast expression.
///
/// - `(char)expr`: sign-extend low byte (sxt)
/// - `(unsigned char)expr`: zero-extend low byte (zxt)
/// - Pointer and int casts are no-ops (all 24-bit on COR24)
pub fn gen_cast(state: &mut CodegenState, ty: &Type, expr: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(expr, state);
    match ty {
        Type::Char => {
            emit!(state, "        sxt     r0,r0");
        }
        Type::UnsignedChar => {
            emit!(state, "        zxt     r0,r0");
        }
        // Int, UnsignedInt, Ptr, Void — no-op on COR24 (all 24-bit)
        _ => {}
    }
}
