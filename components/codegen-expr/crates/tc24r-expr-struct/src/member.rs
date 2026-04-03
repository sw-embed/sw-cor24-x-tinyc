//! Struct member access and assignment.

use tc24r_ast::{BinOp, Expr, Type};
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

/// Pre/post increment or decrement on a struct member: `obj.member++`, `--obj.member`.
pub fn gen_inc_dec_member(
    state: &mut CodegenState,
    object: &Expr,
    member: &str,
    delta: i32,
    post: bool,
    gen_expr_fn: GenExprFn,
) {
    let (mem_offset, is_char) = member_info(state, object, member);
    emit_member_addr(state, object, mem_offset, gen_expr_fn);
    emit!(state, "        mov     r1,r0");
    if is_char {
        emit!(state, "        lbu     r0,0(r1)");
    } else {
        emit!(state, "        lw      r0,0(r1)");
    }
    if post {
        emit!(state, "        push    r0");
    }
    emit!(state, "        add     r0,{delta}");
    if is_char {
        emit!(state, "        sb      r0,0(r1)");
    } else {
        emit!(state, "        sw      r0,0(r1)");
    }
    if post {
        emit!(state, "        pop     r0");
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
    let m = ty
        .find_member(member)
        .unwrap_or_else(|| panic!("unknown struct member '{member}' in type {ty:?}"));
    (m.offset, m.ty == Type::Char || m.ty == Type::UnsignedChar)
}

/// Determine the struct type of the object expression.
/// Walks the expression tree to resolve nested member accesses.
fn object_type(state: &CodegenState, object: &Expr) -> Type {
    let ty = match object {
        Expr::Ident(name) => {
            if let Some(ty) = state.local_types.get(name.as_str()) {
                ty.clone()
            } else if let Some(ty) = state.global_types.get(name.as_str()) {
                ty.clone()
            } else {
                Type::Int
            }
        }
        Expr::Deref(ptr) => {
            let ptr_ty = object_type(state, ptr);
            match ptr_ty {
                Type::Ptr(inner) => *inner,
                _ => ptr_ty,
            }
        }
        Expr::Call { name, .. } => {
            if let Some(ty) = state.function_types.get(name.as_str()) {
                ty.clone()
            } else {
                Type::Int
            }
        }
        Expr::MemberAccess {
            object: obj,
            member,
        } => {
            let struct_ty = object_type(state, obj);
            if let Some(m) = struct_ty.find_member(member) {
                m.ty.clone()
            } else {
                Type::Int
            }
        }
        Expr::BinOp {
            op: BinOp::Add,
            lhs,
            ..
        }
        | Expr::BinOp {
            op: BinOp::Sub,
            lhs,
            ..
        } => {
            // Pointer arithmetic preserves pointer type (e.g. arr + i)
            let lhs_ty = object_type(state, lhs);
            match lhs_ty {
                Type::Ptr(_) => lhs_ty,
                Type::Array(inner, _) => Type::Ptr(inner), // array decay
                _ => Type::Int,
            }
        }
        _ => Type::Int,
    };
    // Resolve incomplete struct placeholders via the struct registry
    resolve_struct(state, ty)
}

/// If a struct type is an empty placeholder (forward-declared),
/// look up the full definition from the struct registry.
fn resolve_struct(state: &CodegenState, ty: Type) -> Type {
    match &ty {
        Type::Struct {
            tag: Some(name),
            members,
            ..
        } if members.is_empty() => {
            if let Some(full) = state.struct_types.get(name) {
                full.clone()
            } else {
                ty
            }
        }
        Type::Ptr(inner) => Type::Ptr(Box::new(resolve_struct(state, *inner.clone()))),
        _ => ty,
    }
}
