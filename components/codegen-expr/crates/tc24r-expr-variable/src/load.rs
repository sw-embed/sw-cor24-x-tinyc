//! Variable load with array decay.

use tc24r_ast::Type;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_load_store::{gen_addr_of, gen_load_by_name};
use tc24r_emit_macros::emit;

/// Load a variable into r0. Arrays decay to a pointer (address of first element).
pub fn gen_ident(state: &mut CodegenState, name: &str) {
    if let Some(Type::Array(..)) = state.local_types.get(name) {
        gen_addr_of(state, name);
        return;
    }
    if state.globals.contains(name) {
        let is_char = state.global_types.get(name) == Some(&Type::Char);
        emit!(state, "        la      r1,_{name}");
        if is_char {
            emit!(state, "        lbu     r0,0(r1)");
        } else {
            emit!(state, "        lw      r0,0(r1)");
        }
    } else {
        gen_load_by_name(state, name);
    }
}
