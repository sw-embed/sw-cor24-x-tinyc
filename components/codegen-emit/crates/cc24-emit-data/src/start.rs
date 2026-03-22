//! Startup code emission.

use cc24_codegen_state::CodegenState;
use cc24_emit_core::emit;

/// Emit startup code that calls `_main` and halts.
pub fn emit_start(state: &mut CodegenState) {
    emit(state, "        .globl  _start");
    emit(state, "_start:");
    emit(state, "        la      r0,_main");
    emit(state, "        jal     r1,(r0)");
    emit(state, "_halt:");
    emit(state, "        bra     _halt");
}
