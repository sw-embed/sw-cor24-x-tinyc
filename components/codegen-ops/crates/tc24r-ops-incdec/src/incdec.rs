//! Pre/post increment and decrement.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_load_store::{gen_load_by_name, gen_store_by_name};
use tc24r_emit_macros::emit;

/// Generate pre/post increment or decrement for a named variable.
///
/// - `delta`: 1 for `++`, -1 for `--`.
/// - `post`: if true, returns the old value (post-increment/decrement).
pub fn gen_inc_dec(state: &mut CodegenState, name: &str, delta: i32, post: bool) {
    gen_load_by_name(state, name);
    if post {
        emit!(state, "        push    r0"); // save old value
    }
    emit!(state, "        add     r0,{delta}");
    gen_store_by_name(state, name);
    if post {
        emit!(state, "        pop     r0"); // restore old value
    }
}
