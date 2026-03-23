//! Pointer dereference (read and write).

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;
use tc24r_type_infer::expr_type;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Dereference a pointer: evaluate `ptr`, load the value it points to into r0.
pub fn gen_deref(state: &mut CodegenState, ptr: &Expr, gen_expr_fn: GenExprFn) {
    let is_byte = is_char_ptr(state, ptr);
    gen_expr_fn(ptr, state);
    if is_byte {
        emit!(state, "        lbu     r0,0(r0)");
    } else {
        emit!(state, "        lw      r0,0(r0)");
    }
}

/// Assign through a pointer: evaluate `value`, evaluate `ptr`, store.
pub fn gen_deref_assign(
    state: &mut CodegenState,
    ptr: &Expr,
    value: &Expr,
    gen_expr_fn: GenExprFn,
) {
    let is_byte = is_char_ptr(state, ptr);
    gen_expr_fn(value, state);
    emit!(state, "        push    r0");
    gen_expr_fn(ptr, state);
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
    if is_byte {
        emit!(state, "        sb      r0,0(r1)");
    } else {
        emit!(state, "        sw      r0,0(r1)");
    }
}

/// Check whether `ptr` is a char pointer.
fn is_char_ptr(state: &CodegenState, ptr: &Expr) -> bool {
    matches!(
        expr_type(state, ptr),
        Some(Type::Ptr(inner)) if *inner == Type::Char
    )
}
