//! Variable assignment (store).

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_load_store::gen_store_by_name;
use tc24r_emit_macros::emit;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Evaluate `value` and store the result into variable `name`.
pub fn gen_assign(state: &mut CodegenState, name: &str, value: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(value, state);
    if state.globals.contains(name) {
        let is_char = matches!(state.global_types.get(name), Some(Type::Char | Type::UnsignedChar));
        emit!(state, "        la      r1,_{name}");
        if is_char {
            emit!(state, "        sb      r0,0(r1)");
        } else {
            emit!(state, "        sw      r0,0(r1)");
        }
    } else {
        gen_store_by_name(state, name);
    }
}
