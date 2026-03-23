//! Expression statement code generation.

use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;
use tc24r_type_infer::GenExprFn;

/// Generate code for an expression used as a statement.
///
/// Evaluates the expression for its side effects; the result in r0
/// is discarded.
pub fn gen_expr_stmt(state: &mut CodegenState, expr: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(expr, state);
}
