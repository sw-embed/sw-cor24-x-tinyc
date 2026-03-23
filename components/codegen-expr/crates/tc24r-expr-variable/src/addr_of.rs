//! Address-of expression.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_load_store::gen_addr_of as emit_addr_of;

/// Compute the address of a named variable into r0.
pub fn gen_addr_of(state: &mut CodegenState, name: &str) {
    emit_addr_of(state, name);
}
