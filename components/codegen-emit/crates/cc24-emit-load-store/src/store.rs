//! Variable store helpers.

use cc24_codegen_state::CodegenState;
use cc24_emit_core::emit;

/// Store r0 into a named variable.
pub fn gen_store_by_name(state: &mut CodegenState, name: &str) {
    if state.globals.contains(name) {
        emit(state, &format!("        la      r1,_{name}"));
        emit(state, "        sw      r0,0(r1)");
    } else {
        let offset = state.locals[name];
        emit(state, &format!("        sw      r0,{offset}(fp)"));
    }
}
