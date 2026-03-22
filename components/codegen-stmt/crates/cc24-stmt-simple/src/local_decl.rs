//! Local variable declaration code generation.

use cc24_ast::Expr;
use cc24_codegen_state::CodegenState;
use cc24_emit_macros::emit;
use cc24_type_infer::GenExprFn;

/// Generate code for a local variable declaration with optional initializer.
///
/// If an initializer is present, evaluates it into r0 and stores the
/// result at the variable's stack-frame offset.
pub fn gen_local_decl(
    state: &mut CodegenState,
    name: &str,
    init: Option<&Expr>,
    gen_expr_fn: GenExprFn,
) {
    if let Some(init_expr) = init {
        gen_expr_fn(init_expr, state);
        let offset = state.locals[name];
        emit!(state, "        sw      r0,{offset}(fp)");
    }
}
