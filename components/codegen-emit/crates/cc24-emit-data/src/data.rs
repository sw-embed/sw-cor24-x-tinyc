//! Data section emission.

use cc24_ast::{Expr, Program, Type};
use cc24_codegen_state::CodegenState;
use cc24_emit_core::emit;

/// Emit the `.data` section for globals and string literals.
pub fn emit_data_section(state: &mut CodegenState, program: &Program) {
    let has_globals = !program.globals.is_empty();
    let has_strings = !state.string_literals.is_empty();
    if !has_globals && !has_strings {
        return;
    }
    emit(state, "");
    emit(state, "        .data");
    for g in &program.globals {
        emit(state, &format!("_{}:", g.name));
        let val = match &g.init {
            Some(Expr::IntLit(v)) => *v,
            _ => 0,
        };
        if g.ty == Type::Char {
            emit(state, &format!("        .byte   {val}"));
        } else {
            emit(state, &format!("        .word   {val}"));
        }
    }
    for (i, s) in state.string_literals.clone().iter().enumerate() {
        emit(state, &format!("_S{i}:"));
        let bytes: Vec<String> = s
            .bytes()
            .chain(std::iter::once(0))
            .map(|b| b.to_string())
            .collect();
        emit(state, &format!("        .byte   {}", bytes.join(",")));
    }
}
