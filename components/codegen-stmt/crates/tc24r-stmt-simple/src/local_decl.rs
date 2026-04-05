//! Local variable declaration code generation.

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::fp_store_word_r0;
use tc24r_type_infer::GenExprFn;

/// Generate code for a local variable declaration with optional initializer.
///
/// If an initializer is present, evaluates it into r0 and stores the
/// result at the variable's stack-frame offset.
pub fn gen_local_decl(
    state: &mut CodegenState,
    name: &str,
    _ty: &Type,
    init: Option<&Expr>,
    gen_expr_fn: GenExprFn,
) {
    if let Some(init_expr) = init {
        gen_expr_fn(init_expr, state);
        let offset = state.locals[name];
        fp_store_word_r0(state, offset);
    }
}
