//! Local variable declaration code generation.

use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;
use tc24r_type_infer::GenExprFn;

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
