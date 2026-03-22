//! Basic emit and label helpers.

use cc24_codegen_state::CodegenState;

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
