//! Immediate value loading.

use cc24_codegen_state::CodegenState;

use crate::emit;

/// Load an integer constant into r0, choosing the shortest encoding.
pub fn load_immediate(state: &mut CodegenState, val: i32) {
    if (-128..=127).contains(&val) {
        emit(state, &format!("        lc      r0,{val}"));
    } else {
        emit(state, &format!("        la      r0,{val}"));
    }
}
