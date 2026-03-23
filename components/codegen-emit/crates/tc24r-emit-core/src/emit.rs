//! Basic emit and label helpers.

use tc24r_codegen_state::CodegenState;

/// Append one line of assembly to the output buffer.
pub fn emit(state: &mut CodegenState, line: &str) {
    state.out.push_str(line);
    state.out.push('\n');
}

/// Allocate a fresh unique label and return it.
pub fn new_label(state: &mut CodegenState) -> String {
    let label = format!("L{}", state.label_counter);
    state.label_counter += 1;
    label
}

/// Emit a long branch-always: uses r2 for target to preserve r0.
pub fn emit_bra(state: &mut CodegenState, target: &str) {
    emit(state, &format!("        la      r2,{target}"));
    emit(state, "        jmp     (r2)");
}

/// Emit a long branch-if-true: brf over la+jmp (no range limit).
/// Uses r2 for target address to preserve r0.
pub fn emit_brt(state: &mut CodegenState, target: &str) {
    let skip = new_label(state);
    emit(state, &format!("        brf     {skip}"));
    emit(state, &format!("        la      r2,{target}"));
    emit(state, "        jmp     (r2)");
    emit(state, &format!("{skip}:"));
}

/// Emit a long branch-if-false: brt over la+jmp (no range limit).
/// Uses r2 for target address to preserve r0.
pub fn emit_brf(state: &mut CodegenState, target: &str) {
    let skip = new_label(state);
    emit(state, &format!("        brt     {skip}"));
    emit(state, &format!("        la      r2,{target}"));
    emit(state, "        jmp     (r2)");
    emit(state, &format!("{skip}:"));
}
