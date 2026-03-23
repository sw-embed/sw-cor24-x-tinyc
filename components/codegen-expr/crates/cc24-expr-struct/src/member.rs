//! Struct member access and assignment.

use cc24_ast::{Expr, Type};
use cc24_codegen_state::CodegenState;
use cc24_emit_core::load_immediate;
use cc24_emit_macros::emit;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Load struct member `object.member` into r0.
pub fn gen_member_access(
    state: &mut CodegenState,
    object: &Expr,
    member: &str,
    _gen_expr_fn: GenExprFn,
) {
    let (fp_offset, mem_offset, is_char) = resolve_member(state, object, member);
    let addr_offset = fp_offset + mem_offset;
    load_immediate(state, addr_offset);
    emit!(state, "        add     r0,fp");
    if is_char {
        emit!(state, "        lbu     r0,0(r0)");
    } else {
        emit!(state, "        lw      r0,0(r0)");
    }
}

/// Store value into struct member `object.member`.
pub fn gen_member_assign(
    state: &mut CodegenState,
    object: &Expr,
    member: &str,
    value: &Expr,
    gen_expr_fn: GenExprFn,
) {
    let (fp_offset, mem_offset, is_char) = resolve_member(state, object, member);
    gen_expr_fn(value, state);
    emit!(state, "        push    r0");
    let addr_offset = fp_offset + mem_offset;
    load_immediate(state, addr_offset);
    emit!(state, "        add     r0,fp");
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
    if is_char {
        emit!(state, "        sb      r0,0(r1)");
    } else {
        emit!(state, "        sw      r0,0(r1)");
    }
}

/// Resolve member info: (variable fp_offset, member_offset, is_char).
fn resolve_member(state: &CodegenState, object: &Expr, member: &str) -> (i32, i32, bool) {
    let Expr::Ident(name) = object else {
        panic!("struct member access on non-identifier");
    };
    let fp_offset = state.locals[name.as_str()];
    let ty = &state.local_types[name.as_str()];
    let m = ty.find_member(member).expect("unknown struct member");
    (fp_offset, m.offset, m.ty == Type::Char)
}
