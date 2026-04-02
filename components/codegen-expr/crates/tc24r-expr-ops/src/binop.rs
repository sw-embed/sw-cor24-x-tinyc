//! Binary operator dispatch -- evaluate lhs/rhs, delegate to L2 handlers.

use tc24r_ast::{BinOp, Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;
use tc24r_ops_arithmetic::{gen_add_sub, gen_mul};
use tc24r_ops_bitwise::{gen_bitwise_and, gen_bitwise_or, gen_bitwise_xor, gen_shl, gen_shr, gen_shr_logical};
use tc24r_ops_compare::gen_compare_eq;
use tc24r_ops_divmod::gen_divmod_call;
use tc24r_ops_logical::{gen_log_and, gen_log_or};
use tc24r_type_infer::{GenExprFn, expr_type, gen_simple_into_r1, is_simple_expr};

/// Dispatch a binary operation to the appropriate L2 handler.
pub fn gen_binop_dispatch(
    state: &mut CodegenState,
    op: BinOp,
    lhs: &Expr,
    rhs: &Expr,
    gen_expr_fn: GenExprFn,
) {
    match op {
        BinOp::Comma => {
            gen_expr_fn(lhs, state);
            gen_expr_fn(rhs, state);
            return;
        }
        BinOp::LogAnd => return gen_log_and(state, lhs, rhs, gen_expr_fn),
        BinOp::LogOr => return gen_log_or(state, lhs, rhs, gen_expr_fn),
        BinOp::Add | BinOp::Sub => return gen_add_sub(state, op, lhs, rhs, gen_expr_fn),
        _ => {}
    }
    let is_unsigned = matches!(expr_type(state, lhs), Some(Type::UnsignedInt));
    eval_lhs_rhs(state, lhs, rhs, gen_expr_fn);
    dispatch_simple(state, op, is_unsigned);
}

/// Evaluate lhs into r0, rhs into r1 (standard two-operand setup).
///
/// When rhs is a simple expression (literal, variable), loads it directly
/// into r1 without push/pop, saving 3 instructions.
fn eval_lhs_rhs(state: &mut CodegenState, lhs: &Expr, rhs: &Expr, gen_expr_fn: GenExprFn) {
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
}

/// Dispatch a simple binop (r0=lhs, r1=rhs already set).
fn dispatch_simple(state: &mut CodegenState, op: BinOp, is_unsigned: bool) {
    match op {
        BinOp::Mul => gen_mul(state),
        BinOp::BitAnd => gen_bitwise_and(state),
        BinOp::BitOr => gen_bitwise_or(state),
        BinOp::BitXor => gen_bitwise_xor(state),
        BinOp::Shl => gen_shl(state),
        BinOp::Shr => {
            if is_unsigned { gen_shr_logical(state) } else { gen_shr(state) }
        }
        BinOp::Eq => gen_compare_eq(state, false),
        BinOp::Ne => gen_compare_eq(state, true),
        BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
            emit_cmp_rel(state, op, is_unsigned)
        }
        BinOp::Div => gen_divmod_call(state, false),
        BinOp::Mod => gen_divmod_call(state, true),
        BinOp::Add | BinOp::Sub | BinOp::LogAnd | BinOp::LogOr | BinOp::Comma => unreachable!(),
    }
}

/// Emit relational comparison (r0=lhs, r1=rhs already set).
/// Uses `cls` (signed) or `clu` (unsigned) depending on operand type.
fn emit_cmp_rel(state: &mut CodegenState, op: BinOp, is_unsigned: bool) {
    let cmp = if is_unsigned { "clu" } else { "cls" };
    match op {
        BinOp::Lt => {
            emit!(state, "        {cmp}     r0,r1");
            emit!(state, "        mov     r0,c");
        }
        BinOp::Gt => {
            emit!(state, "        {cmp}     r1,r0");
            emit!(state, "        mov     r0,c");
        }
        BinOp::Le => {
            emit!(state, "        {cmp}     r1,r0");
            emit!(state, "        mov     r0,c");
            emit!(state, "        ceq     r0,z");
            emit!(state, "        mov     r0,c");
        }
        BinOp::Ge => {
            emit!(state, "        {cmp}     r0,r1");
            emit!(state, "        mov     r0,c");
            emit!(state, "        ceq     r0,z");
            emit!(state, "        mov     r0,c");
        }
        _ => unreachable!(),
    }
}
