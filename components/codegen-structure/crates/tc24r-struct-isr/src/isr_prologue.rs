//! ISR prologue: save all registers and condition flag.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit_lines;
use tc24r_struct_prologue::emit_locals_alloc;

/// Emit ISR prologue: push r0/r1/r2, save condition flag, set frame pointer,
/// allocate locals.
pub fn emit_isr_prologue(state: &mut CodegenState) {
    emit_lines!(
        state,
        "        push    r0",
        "        push    r1",
        "        push    r2",
        "        mov     r2,c",
        "        push    r2",
        "        push    fp",
        "        mov     fp,sp",
    );
    emit_locals_alloc(state);
}
