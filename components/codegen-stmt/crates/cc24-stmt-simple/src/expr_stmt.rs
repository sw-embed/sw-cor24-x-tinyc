//! Expression statement code generation.

use cc24_ast::Expr;
use cc24_codegen_state::CodegenState;
use cc24_type_infer::GenExprFn;

/// Generate code for an expression used as a statement.
///
/// Evaluates the expression for its side effects; the result in r0
/// is discarded.
pub fn gen_expr_stmt(state: &mut CodegenState, expr: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(expr, state);
}
