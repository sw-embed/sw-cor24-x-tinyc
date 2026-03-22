//! Variable load helpers.

use cc24_codegen_state::CodegenState;
use cc24_emit_core::{emit, load_immediate};

/// Load a named variable into r0.
pub fn gen_load_by_name(state: &mut CodegenState, name: &str) {
    if state.globals.contains(name) {
        emit(state, &format!("        la      r1,_{name}"));
        emit(state, "        lw      r0,0(r1)");
    } else {
        let offset = state.locals[name];
        emit(state, &format!("        lw      r0,{offset}(fp)"));
    }
}

/// Load the address of a named variable into r0.
pub fn gen_addr_of(state: &mut CodegenState, name: &str) {
    if state.globals.contains(name) {
        emit(state, &format!("        la      r0,_{name}"));
    } else {
        let offset = state.locals[name];
        // r0 = fp + offset
        load_immediate(state, offset);
        emit(state, "        add     r0,fp");
    }
}
