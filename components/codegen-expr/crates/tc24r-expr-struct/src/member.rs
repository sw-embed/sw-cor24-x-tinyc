//! Struct member access and assignment.

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::load_immediate;
use tc24r_emit_macros::emit;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Load struct member `object.member` into r0.
pub fn gen_member_access(
    state: &mut CodegenState,
    object: &Expr,
    member: &str,
    gen_expr_fn: GenExprFn,
) {
    let (mem_offset, is_char) = member_info(state, object, member);
    emit_member_addr(state, object, mem_offset, gen_expr_fn);
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
    let (mem_offset, is_char) = member_info(state, object, member);
    gen_expr_fn(value, state);
    emit!(state, "        push    r0");
    emit_member_addr(state, object, mem_offset, gen_expr_fn);
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
    if is_char {
        emit!(state, "        sb      r0,0(r1)");
    } else {
        emit!(state, "        sw      r0,0(r1)");
    }
}

/// Emit code to compute member address into r0.
fn emit_member_addr(
    state: &mut CodegenState,
    object: &Expr,
    mem_offset: i32,
    gen_expr_fn: GenExprFn,
) {
    match object {
        Expr::Ident(name) => {
            // Local struct: fp + var_offset + member_offset
            let fp_offset = state.locals[name.as_str()];
            load_immediate(state, fp_offset + mem_offset);
            emit!(state, "        add     r0,fp");
        }
        Expr::Deref(ptr) => {
            // Pointer to struct: evaluate ptr, add member_offset
            gen_expr_fn(ptr, state);
            if mem_offset != 0 {
                emit!(state, "        push    r0");
                load_immediate(state, mem_offset);
                emit!(state, "        pop     r1");
                emit!(state, "        add     r0,r1");
            }
        }
        _ => {
            // General case: evaluate object as address, add offset
            gen_expr_fn(object, state);
            if mem_offset != 0 {
                emit!(state, "        push    r0");
                load_immediate(state, mem_offset);
                emit!(state, "        pop     r1");
                emit!(state, "        add     r0,r1");
            }
        }
    }
}

/// Get member offset and whether it's a char type.
fn member_info(state: &CodegenState, object: &Expr, member: &str) -> (i32, bool) {
    let ty = object_type(state, object);
    let m = ty.find_member(member).expect("unknown struct member");
    (m.offset, m.ty == Type::Char)
}

/// Determine the struct type of the object expression.
fn object_type<'a>(state: &'a CodegenState, object: &Expr) -> &'a Type {
    match object {
        Expr::Ident(name) => &state.local_types[name.as_str()],
        Expr::Deref(ptr) => {
            // ptr is a pointer to struct; get the pointee type
            if let Expr::Ident(name) = ptr.as_ref() {
                if let Some(Type::Ptr(inner)) = state.local_types.get(name.as_str()) {
                    return inner;
                }
            }
            // Fallback: shouldn't happen with well-typed code
            panic!("cannot determine struct type for -> access");
        }
        _ => panic!("cannot determine struct type"),
    }
}
