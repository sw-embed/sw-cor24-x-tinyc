//! Variable store helpers.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::fp_store_word_r0;
use tc24r_emit_macros::emit;

/// Store r0 into a named variable.
pub fn gen_store_by_name(state: &mut CodegenState, name: &str) {
    if state.globals.contains(name) {
        emit!(state, "        la      r1,_{name}");
        emit!(state, "        sw      r0,0(r1)");
    } else {
        let offset = state.locals[name];
        fp_store_word_r0(state, offset);
    }
}
