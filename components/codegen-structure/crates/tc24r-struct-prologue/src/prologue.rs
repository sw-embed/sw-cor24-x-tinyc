//! Function prologue and locals allocation.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::{emit, emit_lines};

/// Emit the standard function prologue: save fp/r2/r1, set frame pointer,
/// allocate locals.
pub fn emit_prologue(state: &mut CodegenState) {
    emit_lines!(
        state,
        "        push    fp",
        "        push    r2",
        "        push    r1",
        "        mov     fp,sp",
    );
    emit_locals_alloc(state);
}

/// Emit stack adjustment for local variables.
///
/// Uses `add sp,-N` for small sizes (fits in signed 8-bit immediate),
/// `sub sp,N` otherwise.
pub fn emit_locals_alloc(state: &mut CodegenState) {
    if state.locals_size > 0 {
        if state.locals_size <= 127 {
            emit!(state, "        add     sp,-{}", state.locals_size);
        } else {
            emit!(state, "        sub     sp,{}", state.locals_size);
        }
    }
}
