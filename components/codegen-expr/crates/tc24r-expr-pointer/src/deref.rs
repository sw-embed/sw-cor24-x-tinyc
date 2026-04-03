//! Pointer dereference (read and write).

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;
use tc24r_type_infer::{expr_type, is_simple_expr};

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
///
/// When value is a simple expression, evaluates ptr first then loads
/// value directly into r0, skipping push/pop.
pub fn gen_deref_assign(
    state: &mut CodegenState,
    ptr: &Expr,
    value: &Expr,
    gen_expr_fn: GenExprFn,
) {
    let is_byte = is_char_ptr(state, ptr);
    if is_simple_expr(value, state) && !is_global_var(value, state) {
        // Safe to skip push/pop: value loads into r0 without clobbering r1.
        // Globals are excluded because loading them uses r1 as scratch
        // (la r1,_name; lw r0,0(r1)), which would clobber the ptr in r1.
        gen_expr_fn(ptr, state);
        emit!(state, "        mov     r1,r0");
        gen_expr_fn(value, state);
    } else {
        gen_expr_fn(value, state);
        emit!(state, "        push    r0");
        gen_expr_fn(ptr, state);
        emit!(state, "        mov     r1,r0");
        emit!(state, "        pop     r0");
    }
    if is_byte {
        emit!(state, "        sb      r0,0(r1)");
    } else {
        emit!(state, "        sw      r0,0(r1)");
    }
}

/// Pre/post increment or decrement through a pointer: `*p++`, `--*p`, `arr[i]++`.
pub fn gen_inc_dec_deref(
    state: &mut CodegenState,
    ptr: &Expr,
    delta: i32,
    post: bool,
    gen_expr_fn: GenExprFn,
) {
    let is_byte = is_char_ptr(state, ptr);
    gen_expr_fn(ptr, state);
    emit!(state, "        mov     r1,r0");
    if is_byte {
        emit!(state, "        lbu     r0,0(r1)");
    } else {
        emit!(state, "        lw      r0,0(r1)");
    }
    if post {
        emit!(state, "        push    r0");
    }
    emit!(state, "        add     r0,{delta}");
    if is_byte {
        emit!(state, "        sb      r0,0(r1)");
    } else {
        emit!(state, "        sw      r0,0(r1)");
    }
    if post {
        emit!(state, "        pop     r0");
    }
}

/// Check whether `expr` is a global variable reference.
fn is_global_var(expr: &Expr, state: &CodegenState) -> bool {
    if let Expr::Ident(name) = expr {
        state.globals.contains(name)
    } else {
        false
    }
}

/// Check whether `ptr` is a char pointer.
fn is_char_ptr(state: &CodegenState, ptr: &Expr) -> bool {
    matches!(
        expr_type(state, ptr),
        Some(Type::Ptr(inner)) if *inner == Type::Char || *inner == Type::UnsignedChar
    )
}
