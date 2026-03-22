//! Break, continue, and inline assembly statement code generation.

use cc24_codegen_state::CodegenState;
use cc24_emit_macros::emit;

/// Generate code for a `break` statement.
///
/// Emits an unconditional branch to the innermost loop's break label.
pub fn gen_break(state: &mut CodegenState) {
    if let Some(label) = state.break_labels.last() {
        let label = label.clone();
        emit!(state, "        bra     {label}");
    }
}

/// Generate code for a `continue` statement.
///
/// Emits an unconditional branch to the innermost loop's continue label.
pub fn gen_continue(state: &mut CodegenState) {
    if let Some(label) = state.continue_labels.last() {
        let label = label.clone();
        emit!(state, "        bra     {label}");
    }
}

/// Generate code for an inline assembly block.
///
/// Emits each line of the assembly text with 8-space indentation.
pub fn gen_asm(state: &mut CodegenState, text: &str) {
    for line in text.lines() {
        emit!(state, "        {line}");
    }
}
