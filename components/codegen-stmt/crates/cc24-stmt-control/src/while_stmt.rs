//! While-loop statement code generation.

use cc24_ast::{Block, Expr, Stmt};
use cc24_codegen_state::CodegenState;
use cc24_emit_core::new_label;
use cc24_emit_macros::emit;
use cc24_stmt_simple::GenStmtFn;
use cc24_type_infer::GenExprFn;

/// Generate code for `while (cond) { body }`.
///
/// Condition is checked at the top of each iteration. Break and continue
/// labels are pushed for nested break/continue statements.
pub fn gen_while(
    state: &mut CodegenState,
    cond: &Expr,
    body: &Block,
    gen_expr_fn: GenExprFn,
    gen_stmt_fn: GenStmtFn,
) {
    let loop_label = new_label(state);
    let done_label = new_label(state);
    state.break_labels.push(done_label.clone());
    state.continue_labels.push(loop_label.clone());

    emit!(state, "{loop_label}:");
    gen_expr_fn(cond, state);
    emit!(state, "        ceq     r0,z");
    emit!(state, "        brt     {done_label}");

    emit_block(state, &body.stmts, gen_stmt_fn);
    emit!(state, "        bra     {loop_label}");
    emit!(state, "{done_label}:");
    state.break_labels.pop();
    state.continue_labels.pop();
}

/// Emit code for each statement in a block.
fn emit_block(state: &mut CodegenState, stmts: &[Stmt], gen_stmt_fn: GenStmtFn) {
    for s in stmts {
        gen_stmt_fn(s, state);
    }
}
