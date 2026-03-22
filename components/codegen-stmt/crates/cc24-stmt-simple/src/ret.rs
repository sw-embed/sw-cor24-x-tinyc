//! Return statement code generation.

use cc24_ast::Expr;
use cc24_codegen_state::CodegenState;
use cc24_emit_macros::emit;
use cc24_type_infer::GenExprFn;

/// Generate code for `return expr;`.
///
/// Evaluates the expression into r0, then branches to the function's
/// return label for epilogue cleanup.
pub fn gen_return(state: &mut CodegenState, expr: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(expr, state);
    let ret_label = state.return_label.clone();
    emit!(state, "        bra     {ret_label}");
}
