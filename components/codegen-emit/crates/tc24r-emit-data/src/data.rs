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
        match &g.ty {
            Type::Char | Type::UnsignedChar => {
                let val = match &g.init {
                    Some(Expr::IntLit(v)) => *v,
                    _ => 0,
                };
                emit!(state, "        .byte   {val}");
            }
            Type::Array(elem_ty, count) => {
                if **elem_ty == Type::Char || **elem_ty == Type::UnsignedChar {
                    match &g.init {
                        Some(Expr::StringLit(s)) => {
                            let bytes: Vec<String> = s
                                .bytes()
                                .chain(std::iter::once(0))
                                .map(|b| b.to_string())
                                .collect();
                            let byte_str = bytes.join(",");
                            emit!(state, "        .byte   {byte_str}");
                        }
                        Some(Expr::InitList(values)) => {
                            emit_char_array_init(state, count, values);
                        }
                        _ => {
                            for _ in 0..*count {
                                emit!(state, "        .byte   0");
                            }
                        }
                    }
                } else {
                    let words_per_elem = (elem_ty.size() + 2) / 3;
                    let total_words = *count * words_per_elem as usize;
                    match &g.init {
                        Some(Expr::InitList(values)) => {
                            emit_int_array_init(state, total_words, values);
                        }
                        Some(Expr::IntLit(v)) => {
                            for _ in 0..total_words {
                                emit!(state, "        .word   {v}");
                            }
                        }
                        _ => {
                            for _ in 0..total_words {
                                emit!(state, "        .word   0");
                            }
                        }
                    }
                }
            }
            _ => {
                let val = match &g.init {
                    Some(Expr::IntLit(v)) => *v,
                    _ => 0,
                };
                emit!(state, "        .word   {val}");
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

fn emit_char_array_init(state: &mut CodegenState, count: &usize, values: &[Expr]) {
    for i in 0..*count {
        if i < values.len() {
            match &values[i] {
                Expr::IntLit(v) => emit!(state, "        .byte   {v}"),
                _ => emit!(state, "        .byte   0"),
            }
        } else {
            emit!(state, "        .byte   0");
        }
    }
}

fn emit_int_array_init(state: &mut CodegenState, total_words: usize, values: &[Expr]) {
    for i in 0..total_words {
        if i < values.len() {
            match &values[i] {
                Expr::IntLit(v) => emit!(state, "        .word   {v}"),
                _ => emit!(state, "        .word   0"),
            }
        } else {
            emit!(state, "        .word   0");
        }
    }
}
