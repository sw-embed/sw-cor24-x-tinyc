//! Fused condition evaluation for branch targets.
//!
//! When a condition is a simple comparison (==, !=, <, etc.), emits the
//! compare instruction and branches directly to `skip_label` when the
//! condition is false — avoiding boolean materialization.

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::{emit_brf, emit_brt};
use tc24r_emit_macros::emit;
use tc24r_ops_compare::{gen_compare_branch, gen_compare_branch_true, is_comparison_op};
use tc24r_type_infer::{GenExprFn, expr_type, gen_simple_into_r1, is_simple_expr};

/// Evaluate `cond` and branch to `skip_label` when the condition is false.
///
/// If `cond` is a comparison binop, uses fused compare+branch (no boolean
/// materialization). Otherwise falls back to: gen_expr → ceq r0,z → brt.
pub fn gen_condition_skip(
    state: &mut CodegenState,
    cond: &Expr,
    skip_label: &str,
    gen_expr_fn: GenExprFn,
) {
    if let Expr::BinOp { op, lhs, rhs } = cond {
        if is_comparison_op(*op) {
            // Fused path: load operands into r0/r1, then compare+branch
            if is_simple_expr(rhs, state) {
                gen_expr_fn(lhs, state);
                gen_simple_into_r1(rhs, state);
            } else {
                gen_expr_fn(lhs, state);
                emit!(state, "        push    r0");
                gen_expr_fn(rhs, state);
                emit!(state, "        mov     r1,r0");
                emit!(state, "        pop     r0");
            }
            let is_unsigned = matches!(expr_type(state, lhs), Some(Type::UnsignedInt));
            gen_compare_branch(state, *op, skip_label, is_unsigned);
            return;
        }
    }

    // Fallback: evaluate condition as a boolean value, test against zero
    gen_expr_fn(cond, state);
    emit!(state, "        ceq     r0,z");
    emit_brt(state, skip_label);
}

/// Evaluate `cond` and branch to `loop_label` when the condition is TRUE.
/// Used for do-while loops (loop back when condition holds).
pub fn gen_condition_loop(
    state: &mut CodegenState,
    cond: &Expr,
    loop_label: &str,
    gen_expr_fn: GenExprFn,
) {
    if let Expr::BinOp { op, lhs, rhs } = cond {
        if is_comparison_op(*op) {
            if is_simple_expr(rhs, state) {
                gen_expr_fn(lhs, state);
                gen_simple_into_r1(rhs, state);
            } else {
                gen_expr_fn(lhs, state);
                emit!(state, "        push    r0");
                gen_expr_fn(rhs, state);
                emit!(state, "        mov     r1,r0");
                emit!(state, "        pop     r0");
            }
            let is_unsigned = matches!(expr_type(state, lhs), Some(Type::UnsignedInt));
            gen_compare_branch_true(state, *op, loop_label, is_unsigned);
            return;
        }
    }

    // Fallback: evaluate condition, branch back if non-zero
    gen_expr_fn(cond, state);
    emit!(state, "        ceq     r0,z");
    emit_brf(state, loop_label);
}
