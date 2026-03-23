//! Short-circuit logical AND and OR.
//!
//! These operators need to conditionally evaluate the RHS, so they take
//! a `gen_expr_fn` callback from the caller at a higher DAG level.

use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::{emit_bra, emit_brf, emit_brt, new_label};
use tc24r_emit_macros::emit;

/// Callback type for expression code generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Short-circuit `&&`: if LHS is 0, result is 0 without evaluating RHS.
pub fn gen_log_and(state: &mut CodegenState, lhs: &Expr, rhs: &Expr, gen_expr_fn: GenExprFn) {
    let false_label = new_label(state);
    let done_label = new_label(state);
    gen_expr_fn(lhs, state);
    emit!(state, "        ceq     r0,z");
    emit_brt(state, &false_label);
    gen_expr_fn(rhs, state);
    emit!(state, "        ceq     r0,z");
    emit_brt(state, &false_label);
    emit!(state, "        lc      r0,1");
    emit_bra(state, &done_label);
    emit!(state, "{false_label}:");
    emit!(state, "        lc      r0,0");
    emit!(state, "{done_label}:");
}

/// Short-circuit `||`: if LHS is nonzero, result is 1 without evaluating RHS.
pub fn gen_log_or(state: &mut CodegenState, lhs: &Expr, rhs: &Expr, gen_expr_fn: GenExprFn) {
    let true_label = new_label(state);
    let done_label = new_label(state);
    gen_expr_fn(lhs, state);
    emit!(state, "        ceq     r0,z");
    emit_brf(state, &true_label);
    gen_expr_fn(rhs, state);
    emit!(state, "        ceq     r0,z");
    emit_brf(state, &true_label);
    emit!(state, "        lc      r0,0");
    emit_bra(state, &done_label);
    emit!(state, "{true_label}:");
    emit!(state, "        lc      r0,1");
    emit!(state, "{done_label}:");
}
