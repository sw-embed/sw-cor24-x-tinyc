//! Immediate value loading.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Load an integer constant into r0, choosing the shortest encoding.
pub fn load_immediate(state: &mut CodegenState, val: i32) {
    if (-128..=127).contains(&val) {
        emit!(state, "        lc      r0,{val}");
    } else {
        emit!(state, "        la      r0,{val}");
    }
}
