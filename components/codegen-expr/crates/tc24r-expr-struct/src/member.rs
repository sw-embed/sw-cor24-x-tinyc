//! Struct member access and assignment.

use tc24r_ast::{BinOp, Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::load_immediate;
use tc24r_emit_macros::emit;

/// Full information about a struct member for codegen.
struct MemberInfo {
    offset: i32,
    ty: Type,
    bit_width: u8,
    bit_offset: u8,
}

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Load struct member `object.member` into r0.
/// If the member is an array type, returns the address (array-to-pointer decay).
/// For bitfields, extracts the field via shift+mask with sign extension.
pub fn gen_member_access(
    state: &mut CodegenState,
    object: &Expr,
    member: &str,
    gen_expr_fn: GenExprFn,
) {
    let minfo = full_member_info(state, object, member);
    emit_member_addr(state, object, minfo.offset, gen_expr_fn);
    if matches!(minfo.ty, Type::Array(_, _)) {
        return;
    }
    if minfo.ty == Type::Char || minfo.ty == Type::UnsignedChar {
        emit!(state, "        lbu     r0,0(r0)");
    } else {
        emit!(state, "        lw      r0,0(r0)");
    }
    // Bitfield extraction: shift right, then sign-extend
    if minfo.bit_width > 0 {
        if minfo.bit_offset > 0 {
            emit!(state, "        lc      r1,{}", minfo.bit_offset);
            emit!(state, "        sra     r0,r1");
        }
        // Sign-extend from bit_width: shift left (24 - width), then sra back
        let shift = 24 - minfo.bit_width;
        if shift > 0 {
            emit!(state, "        lc      r1,{shift}");
            emit!(state, "        shl     r0,r1");
            emit!(state, "        sra     r0,r1");
        }
    }
}

/// Store value into struct member `object.member`.
/// For bitfields, performs read-modify-write with mask.
pub fn gen_member_assign(
    state: &mut CodegenState,
    object: &Expr,
    member: &str,
    value: &Expr,
    gen_expr_fn: GenExprFn,
) {
    let minfo = full_member_info(state, object, member);
    if minfo.bit_width > 0 {
        gen_bitfield_assign(state, object, &minfo, value, gen_expr_fn);
        return;
    }
    let is_char = minfo.ty == Type::Char || minfo.ty == Type::UnsignedChar;
    gen_expr_fn(value, state);
    emit!(state, "        push    r0");
    emit_member_addr(state, object, minfo.offset, gen_expr_fn);
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
    if is_char {
        emit!(state, "        sb      r0,0(r1)");
    } else {
        emit!(state, "        sw      r0,0(r1)");
    }
}

/// Bitfield write: read-modify-write.
/// Approach: evaluate value into new_bits (masked+shifted), push it.
/// Then compute addr, load word, clear bits, OR new_bits, store.
fn gen_bitfield_assign(
    state: &mut CodegenState,
    object: &Expr,
    minfo: &MemberInfo,
    value: &Expr,
    gen_expr_fn: GenExprFn,
) {
    let mask = (1i32 << minfo.bit_width) - 1;
    let clear_mask = (!(mask << minfo.bit_offset)) & 0xFFFFFF;

    // Step 1: evaluate value → mask → shift → push new_bits
    gen_expr_fn(value, state);
    emit!(state, "        push    r0");
    load_immediate(state, mask);
    emit!(state, "        pop     r1");
    emit!(state, "        and     r0,r1"); // r0 = value & mask
    if minfo.bit_offset > 0 {
        emit!(state, "        lc      r1,{}", minfo.bit_offset);
        emit!(state, "        shl     r0,r1");
    }
    emit!(state, "        push    r0"); // stack: [new_bits]

    // Step 2: compute addr, load current word, clear bitfield bits
    emit_member_addr(state, object, minfo.offset, gen_expr_fn);
    emit!(state, "        lw      r0,0(r0)"); // r0 = current word
    emit!(state, "        push    r0");
    load_immediate(state, clear_mask);
    emit!(state, "        pop     r1");
    emit!(state, "        and     r0,r1"); // r0 = cleared word

    // Step 3: OR with new_bits
    emit!(state, "        pop     r1"); // r1 = new_bits
    emit!(state, "        or      r0,r1"); // r0 = updated word
    emit!(state, "        push    r0"); // save updated word

    // Step 4: recompute addr and store
    emit_member_addr(state, object, minfo.offset, gen_expr_fn);
    emit!(state, "        mov     r1,r0"); // r1 = addr
    emit!(state, "        pop     r0"); // r0 = updated word
    emit!(state, "        sw      r0,0(r1)");
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

/// Emit code to compute member address into r0 (resolves member name to offset).
pub fn gen_member_addr(
    state: &mut CodegenState,
    object: &Expr,
    member: &str,
    gen_expr_fn: GenExprFn,
) {
    let (mem_offset, _is_char) = member_info(state, object, member);
    emit_member_addr(state, object, mem_offset, gen_expr_fn);
}

/// Emit code to compute address of member at known offset into r0.
fn emit_member_addr(
    state: &mut CodegenState,
    object: &Expr,
    mem_offset: i32,
    gen_expr_fn: GenExprFn,
) {
    match object {
        Expr::Ident(name) => {
            if state.globals.contains(name.as_str()) {
                // Global struct: load base address, add member offset
                emit!(state, "        la      r0,_{name}");
                if mem_offset != 0 {
                    emit!(state, "        push    r0");
                    load_immediate(state, mem_offset);
                    emit!(state, "        pop     r1");
                    emit!(state, "        add     r0,r1");
                }
            } else {
                let fp_offset = state.locals[name.as_str()];
                load_immediate(state, fp_offset + mem_offset);
                emit!(state, "        add     r0,fp");
            }
        }
        Expr::Deref(ptr) => {
            gen_expr_fn(ptr, state);
            if mem_offset != 0 {
                emit!(state, "        push    r0");
                load_immediate(state, mem_offset);
                emit!(state, "        pop     r1");
                emit!(state, "        add     r0,r1");
            }
        }
        _ => {
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

/// Get full member info including bitfield data.
fn full_member_info(state: &CodegenState, object: &Expr, member: &str) -> MemberInfo {
    let ty = object_type(state, object);
    let m = ty
        .find_member(member)
        .unwrap_or_else(|| panic!("unknown struct member '{member}' in type {ty:?}"));
    MemberInfo {
        offset: m.offset,
        ty: m.ty.clone(),
        bit_width: m.bit_width,
        bit_offset: m.bit_offset,
    }
}

/// Get member offset, char flag, and full member type.
fn member_info_with_ty(state: &CodegenState, object: &Expr, member: &str) -> (i32, Type) {
    let ty = object_type(state, object);
    let m = ty
        .find_member(member)
        .unwrap_or_else(|| panic!("unknown struct member '{member}' in type {ty:?}"));
    let is_char = m.ty == Type::Char || m.ty == Type::UnsignedChar;
    (m.offset, if is_char { m.ty.clone() } else { m.ty.clone() })
}

/// Get member offset and whether it's a char type.
fn member_info(state: &CodegenState, object: &Expr, member: &str) -> (i32, bool) {
    let (offset, ty) = member_info_with_ty(state, object, member);
    (offset, ty == Type::Char || ty == Type::UnsignedChar)
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
            let lhs_ty = object_type(state, lhs);
            match lhs_ty {
                Type::Ptr(_) => lhs_ty,
                Type::Array(inner, _) => Type::Ptr(inner),
                _ => Type::Int,
            }
        }
        _ => Type::Int,
    };
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
