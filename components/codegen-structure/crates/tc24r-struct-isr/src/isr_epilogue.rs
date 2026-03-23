//! ISR epilogue: restore all registers, condition flag, and return via ir.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::{emit_label, emit_lines};

/// Emit ISR epilogue: return label, restore sp, pop fp, restore condition
/// flag, pop r2/r1/r0, jump through interrupt return register.
pub fn emit_isr_epilogue(state: &mut CodegenState) {
    let ret_label = state.return_label.clone();
    emit_label!(state, ret_label);
    emit_lines!(
        state,
        "        mov     sp,fp",
        "        pop     fp",
        "        pop     r2",
        "        clu     z,r2",
        "        pop     r2",
        "        pop     r1",
        "        pop     r0",
        "        jmp     (ir)",
    );
}
