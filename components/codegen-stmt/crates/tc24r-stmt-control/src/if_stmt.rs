//! If/else statement code generation.

use tc24r_ast::{Block, Expr, Stmt};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::new_label;
use tc24r_emit_macros::emit;
use tc24r_stmt_simple::GenStmtFn;
use tc24r_type_infer::GenExprFn;

/// Generate code for `if (cond) { then } else { else }`.
///
/// Evaluates condition, branches around then-body (and optional else-body).
pub fn gen_if(
    state: &mut CodegenState,
    cond: &Expr,
    then_body: &Block,
    else_body: Option<&Block>,
    gen_expr_fn: GenExprFn,
    gen_stmt_fn: GenStmtFn,
) {
    let else_label = new_label(state);
    let done_label = new_label(state);

    gen_expr_fn(cond, state);
    emit!(state, "        ceq     r0,z");
    if else_body.is_some() {
        emit!(state, "        brt     {else_label}");
    } else {
        emit!(state, "        brt     {done_label}");
    }

    emit_block(state, &then_body.stmts, gen_stmt_fn);

    if let Some(eb) = else_body {
        emit!(state, "        bra     {done_label}");
        emit!(state, "{else_label}:");
        emit_block(state, &eb.stmts, gen_stmt_fn);
    }

    emit!(state, "{done_label}:");
}

/// Emit code for each statement in a block.
fn emit_block(state: &mut CodegenState, stmts: &[Stmt], gen_stmt_fn: GenStmtFn) {
    for s in stmts {
        gen_stmt_fn(s, state);
    }
}
