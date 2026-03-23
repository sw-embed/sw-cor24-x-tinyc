//! Startup code emission.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit_lines;

/// Emit startup code that calls `_main` and halts.
pub fn emit_start(state: &mut CodegenState) {
    emit_lines!(
        state,
        "        .globl  _start",
        "_start:",
        "        la      r0,_main",
        "        jal     r1,(r0)",
        "_halt:",
        "        bra     _halt",
    );
}
