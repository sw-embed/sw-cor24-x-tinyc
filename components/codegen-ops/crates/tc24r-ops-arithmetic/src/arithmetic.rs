//! Add, subtract, and multiply with pointer arithmetic scaling.

use tc24r_ast::{BinOp, Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::load_immediate;
use tc24r_emit_macros::emit;
use tc24r_type_infer::{GenExprFn, expr_type};

/// Generate add or subtract, dispatching to the appropriate variant.
pub fn gen_add_sub(
    state: &mut CodegenState,
    op: BinOp,
    lhs: &Expr,
    rhs: &Expr,
    gen_expr_fn: GenExprFn,
) {
    let lhs_ty = expr_type(state, lhs);
    let rhs_ty = expr_type(state, rhs);
    let lhs_is_ptr = matches!(&lhs_ty, Some(Type::Ptr(_)));
    let rhs_is_ptr = matches!(&rhs_ty, Some(Type::Ptr(_)));

    if op == BinOp::Sub && lhs_is_ptr && rhs_is_ptr {
        gen_ptr_diff(state, lhs, rhs, &lhs_ty, gen_expr_fn);
    } else if lhs_is_ptr {
        gen_ptr_offset(state, op, lhs, rhs, &lhs_ty, gen_expr_fn);
    } else {
        gen_plain_add_sub(state, op, lhs, rhs, gen_expr_fn);
    }
}

/// ptr - ptr: subtract addresses, divide by element size.
fn gen_ptr_diff(
    state: &mut CodegenState,
    lhs: &Expr,
    rhs: &Expr,
    lhs_ty: &Option<Type>,
    gen_expr_fn: GenExprFn,
) {
    let elem_size = match lhs_ty {
        Some(Type::Ptr(inner)) => inner.size(),
        _ => 1,
    };
    gen_expr_fn(lhs, state);
    emit!(state, "        push    r0");
    gen_expr_fn(rhs, state);
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
    emit!(state, "        sub     r0,r1");
    if elem_size > 1 {
        emit_div_by_size(state, elem_size);
    }
}

/// Emit a division by a constant element size for ptr-ptr diff.
fn emit_div_by_size(state: &mut CodegenState, elem_size: i32) {
    state.needs_div = true;
    emit!(state, "        push    r0");
    load_immediate(state, elem_size);
    emit!(state, "        push    r0");
    emit!(state, "        pop     r1");
    emit!(state, "        pop     r0");
    emit!(state, "        push    r1");
    emit!(state, "        push    r0");
    emit!(state, "        la      r0,__tc24r_div");
    emit!(state, "        jal     r1,(r0)");
    emit!(state, "        add     sp,6");
}

/// ptr +/- int: scale the integer by the pointee element size.
fn gen_ptr_offset(
    state: &mut CodegenState,
    op: BinOp,
    lhs: &Expr,
    rhs: &Expr,
    lhs_ty: &Option<Type>,
    gen_expr_fn: GenExprFn,
) {
    let scale = match lhs_ty {
        Some(Type::Ptr(inner)) => inner.size(),
        _ => 1,
    };
    gen_expr_fn(lhs, state);
    emit!(state, "        push    r0");
    gen_expr_fn(rhs, state);
    if scale > 1 {
        emit!(state, "        lc      r1,{scale}");
        emit!(state, "        mul     r0,r1");
    }
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
    emit_add_or_sub(state, op);
}

/// Plain int +/- int with no scaling.
fn gen_plain_add_sub(
    state: &mut CodegenState,
    op: BinOp,
    lhs: &Expr,
    rhs: &Expr,
    gen_expr_fn: GenExprFn,
) {
    gen_expr_fn(lhs, state);
    emit!(state, "        push    r0");
    gen_expr_fn(rhs, state);
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
    emit_add_or_sub(state, op);
}

/// Emit the final add or sub instruction.
fn emit_add_or_sub(state: &mut CodegenState, op: BinOp) {
    match op {
        BinOp::Add => {
            emit!(state, "        add     r0,r1");
        }
        BinOp::Sub => {
            emit!(state, "        sub     r0,r1");
        }
        _ => unreachable!(),
    }
}

/// Generate multiply. Assumes r0=lhs, r1=rhs already evaluated.
pub fn gen_mul(state: &mut CodegenState) {
    emit!(state, "        mul     r0,r1");
}
