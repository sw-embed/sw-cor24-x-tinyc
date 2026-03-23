//! Unary operator dispatch.

use tc24r_ast::{Expr, UnaryOp};
use tc24r_codegen_state::CodegenState;
use tc24r_ops_unary::gen_unary;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Evaluate the operand, then dispatch to the L2 unary handler.
pub fn gen_unary_dispatch(
    state: &mut CodegenState,
    op: UnaryOp,
    operand: &Expr,
    gen_expr_fn: GenExprFn,
) {
    gen_expr_fn(operand, state);
    gen_unary(state, op);
}
