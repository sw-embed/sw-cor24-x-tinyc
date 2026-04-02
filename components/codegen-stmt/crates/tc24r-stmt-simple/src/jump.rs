//! Break, continue, and inline assembly statement code generation.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::emit_bra;
use tc24r_emit_macros::{emit, emit_label};

/// Generate code for a `break` statement.
///
/// Emits an unconditional branch to the innermost loop's break label.
pub fn gen_break(state: &mut CodegenState) {
    if let Some(label) = state.break_labels.last() {
        let label = label.clone();
        emit_bra(state, &label);
    }
}

/// Generate code for a `continue` statement.
///
/// Emits an unconditional branch to the innermost loop's continue label.
pub fn gen_continue(state: &mut CodegenState) {
    if let Some(label) = state.continue_labels.last() {
        let label = label.clone();
        emit_bra(state, &label);
    }
}

/// Generate code for a `goto label;` statement.
///
/// Emits an unconditional branch to the named label.
pub fn gen_goto(state: &mut CodegenState, name: &str) {
    let label = format!("_goto_{name}");
    emit_bra(state, &label);
}

/// Generate code for a `label:` statement.
///
/// Emits the label as an assembly label.
pub fn gen_label(state: &mut CodegenState, name: &str) {
    emit_label!(state, format!("_goto_{name}"));
}

/// Generate code for an inline assembly block.
///
/// Emits each line of the assembly text with 8-space indentation.
pub fn gen_asm(state: &mut CodegenState, text: &str) {
    for line in text.lines() {
        emit!(state, "        {line}");
    }
}
