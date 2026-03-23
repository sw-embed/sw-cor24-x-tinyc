//! Return statement code generation.

use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::emit_bra;
use tc24r_type_infer::GenExprFn;

/// Generate code for `return expr;`.
///
/// Evaluates the expression into r0, then branches to the function's
/// return label for epilogue cleanup.
pub fn gen_return(state: &mut CodegenState, expr: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(expr, state);
    let ret_label = state.return_label.clone();
    emit_bra(state, &ret_label);
}
