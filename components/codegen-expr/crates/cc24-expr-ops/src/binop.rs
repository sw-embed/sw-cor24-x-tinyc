//! Binary operator dispatch -- evaluate lhs/rhs, delegate to L2 handlers.

use cc24_ast::{BinOp, Expr};
use cc24_codegen_state::CodegenState;
use cc24_emit_macros::emit;
use cc24_ops_arithmetic::{gen_add_sub, gen_mul};
use cc24_ops_bitwise::{gen_bitwise_and, gen_bitwise_or, gen_bitwise_xor, gen_shl, gen_shr};
use cc24_ops_compare::gen_compare_eq;
use cc24_ops_divmod::gen_divmod_call;
use cc24_ops_logical::{gen_log_and, gen_log_or};
use cc24_type_infer::GenExprFn;

/// Dispatch a binary operation to the appropriate L2 handler.
pub fn gen_binop_dispatch(
    state: &mut CodegenState,
    op: BinOp,
    lhs: &Expr,
    rhs: &Expr,
    gen_expr_fn: GenExprFn,
) {
    match op {
        BinOp::LogAnd => return gen_log_and(state, lhs, rhs, gen_expr_fn),
        BinOp::LogOr => return gen_log_or(state, lhs, rhs, gen_expr_fn),
        BinOp::Add | BinOp::Sub => return gen_add_sub(state, op, lhs, rhs, gen_expr_fn),
        _ => {}
    }
    eval_lhs_rhs(state, lhs, rhs, gen_expr_fn);
    dispatch_simple(state, op);
}

/// Evaluate lhs into r0, rhs into r1 (standard two-operand setup).
fn eval_lhs_rhs(state: &mut CodegenState, lhs: &Expr, rhs: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(lhs, state);
    emit!(state, "        push    r0");
    gen_expr_fn(rhs, state);
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
}

/// Dispatch a simple binop (r0=lhs, r1=rhs already set).
fn dispatch_simple(state: &mut CodegenState, op: BinOp) {
    match op {
        BinOp::Mul => gen_mul(state),
        BinOp::BitAnd => gen_bitwise_and(state),
        BinOp::BitOr => gen_bitwise_or(state),
        BinOp::BitXor => gen_bitwise_xor(state),
        BinOp::Shl => gen_shl(state),
        BinOp::Shr => gen_shr(state),
        BinOp::Eq => gen_compare_eq(state, false),
        BinOp::Ne => gen_compare_eq(state, true),
        BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => emit_cmp_rel(state, op),
        BinOp::Div => gen_divmod_call(state, false),
        BinOp::Mod => gen_divmod_call(state, true),
        BinOp::Add | BinOp::Sub | BinOp::LogAnd | BinOp::LogOr => unreachable!(),
    }
}

/// Emit relational comparison (r0=lhs, r1=rhs already set).
fn emit_cmp_rel(state: &mut CodegenState, op: BinOp) {
    match op {
        BinOp::Lt => {
            emit!(state, "        cls     r0,r1");
            emit!(state, "        mov     r0,c");
        }
        BinOp::Gt => {
            emit!(state, "        cls     r1,r0");
            emit!(state, "        mov     r0,c");
        }
        BinOp::Le => {
            emit!(state, "        cls     r1,r0");
            emit!(state, "        mov     r0,c");
            emit!(state, "        ceq     r0,z");
            emit!(state, "        mov     r0,c");
        }
        BinOp::Ge => {
            emit!(state, "        cls     r0,r1");
            emit!(state, "        mov     r0,c");
            emit!(state, "        ceq     r0,z");
            emit!(state, "        mov     r0,c");
        }
        _ => unreachable!(),
    }
}
