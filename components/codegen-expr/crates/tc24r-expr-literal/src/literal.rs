//! Integer and string literal code generation.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::load_immediate;
use tc24r_emit_macros::emit;

/// Load an integer literal into r0.
pub fn gen_int_lit(state: &mut CodegenState, val: i32) {
    load_immediate(state, val);
}

/// Register a string literal and load its address into r0.
pub fn gen_string_lit(state: &mut CodegenState, s: &str) {
    let idx = state.string_literals.len();
    state.string_literals.push(s.to_owned());
    emit!(state, "        la      r0,_S{idx}");
}
