//! Cast expression (pass-through to gen_expr).

use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Generate a cast expression. COR24 casts are no-ops at the assembly level.
pub fn gen_cast(state: &mut CodegenState, expr: &Expr, gen_expr_fn: GenExprFn) {
    gen_expr_fn(expr, state);
}
