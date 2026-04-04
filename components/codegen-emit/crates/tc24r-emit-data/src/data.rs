//! Data section emission.

use tc24r_ast::{Expr, Program, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Emit the `.data` section for globals and string literals.
pub fn emit_data_section(state: &mut CodegenState, program: &Program) {
    let has_globals = !program.globals.is_empty();
    let has_strings = !state.string_literals.is_empty();
    if !has_globals && !has_strings {
        return;
    }
    emit!(state, "");
    emit!(state, "        .data");
    for g in &program.globals {
        let name = &g.name;
        emit!(state, "_{name}:");
        match &g.init {
            Some(Expr::InitList(values)) => {
                let mut iter = values.iter();
                emit_typed_data(state, &g.ty, &mut iter);
            }
            Some(Expr::StringLit(s)) => {
                let bytes: Vec<String> = s
                    .bytes()
                    .chain(std::iter::once(0))
                    .map(|b| b.to_string())
                    .collect();
                let byte_str = bytes.join(",");
                emit!(state, "        .byte   {byte_str}");
                // Pad if declared array is larger
                if let Type::Array(_, count) = &g.ty {
                    let pad = count.saturating_sub(s.len() + 1);
                    for _ in 0..pad {
                        emit!(state, "        .byte   0");
                    }
                }
            }
            Some(Expr::IntLit(v)) => {
                emit_scalar_or_fill(state, &g.ty, *v);
            }
            _ => {
                emit_zero_fill(state, &g.ty);
            }
        }
    }
    for (i, s) in state.string_literals.clone().iter().enumerate() {
        emit!(state, "_S{i}:");
        let bytes: Vec<String> = s
            .bytes()
            .chain(std::iter::once(0))
            .map(|b| b.to_string())
            .collect();
        let byte_str = bytes.join(",");
        emit!(state, "        .byte   {byte_str}");
    }
}

/// Emit data for a type, consuming values from the iterator in leaf order.
fn emit_typed_data<'a>(
    state: &mut CodegenState,
    ty: &Type,
    values: &mut impl Iterator<Item = &'a Expr>,
) {
    match ty {
        Type::Char | Type::UnsignedChar => {
            let val = values
                .next()
                .map(|e| match e {
                    Expr::IntLit(v) => *v,
                    _ => 0,
                })
                .unwrap_or(0);
            emit!(state, "        .byte   {val}");
        }
        Type::Array(elem, count) => {
            for _ in 0..*count {
                emit_typed_data(state, elem, values);
            }
        }
        Type::Struct { members, .. } => {
            let mut prev_end = 0i32;
            for m in members {
                // Emit zero padding between members
                let gap = m.offset - prev_end;
                for _ in 0..gap {
                    emit!(state, "        .byte   0");
                }
                emit_typed_data(state, &m.ty, values);
                prev_end = m.offset + m.ty.size();
            }
        }
        _ => {
            // Int, Ptr, UnsignedInt, Void — all 3-byte words
            let val = values.next();
            match val {
                Some(Expr::IntLit(v)) => emit!(state, "        .word   {v}"),
                _ => emit!(state, "        .word   0"),
            }
        }
    }
}

/// Emit a scalar value or fill for the given type.
fn emit_scalar_or_fill(state: &mut CodegenState, ty: &Type, val: i32) {
    match ty {
        Type::Char | Type::UnsignedChar => {
            emit!(state, "        .byte   {val}");
        }
        _ => {
            emit!(state, "        .word   {val}");
        }
    }
}

/// Emit zero-fill for the entire size of a type.
fn emit_zero_fill(state: &mut CodegenState, ty: &Type) {
    let total = ty.size();
    let mut off = 0;
    while off + 3 <= total {
        emit!(state, "        .word   0");
        off += 3;
    }
    while off < total {
        emit!(state, "        .byte   0");
        off += 1;
    }
}
