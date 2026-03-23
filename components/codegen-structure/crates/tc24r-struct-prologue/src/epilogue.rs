//! Function epilogue generation.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::{emit_label, emit_lines};

/// Emit the standard function epilogue: return label, restore sp/r1/r2/fp,
/// jump through return address.
pub fn emit_epilogue(state: &mut CodegenState) {
    let ret_label = state.return_label.clone();
    emit_label!(state, ret_label);
    emit_lines!(
        state,
        "        mov     sp,fp",
        "        pop     r1",
        "        pop     r2",
        "        pop     fp",
        "        jmp     (r1)",
    );
}
