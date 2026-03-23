//! Do-while loop statement code generation.

use tc24r_ast::{Block, Expr, Stmt};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::new_label;
use tc24r_emit_macros::emit;
use tc24r_stmt_simple::GenStmtFn;
use tc24r_type_infer::GenExprFn;

/// Generate code for `do { body } while (cond);`.
///
/// Body executes first, then condition is checked. Continue jumps to
/// the condition check, not the top of the loop.
pub fn gen_do_while(
    state: &mut CodegenState,
    body: &Block,
    cond: &Expr,
    gen_expr_fn: GenExprFn,
    gen_stmt_fn: GenStmtFn,
) {
    let loop_label = new_label(state);
    let cont_label = new_label(state);
    let done_label = new_label(state);
    state.break_labels.push(done_label.clone());
    state.continue_labels.push(cont_label.clone());

    emit!(state, "{loop_label}:");
    emit_block(state, &body.stmts, gen_stmt_fn);
    emit!(state, "{cont_label}:");
    gen_expr_fn(cond, state);
    emit!(state, "        ceq     r0,z");
    emit!(state, "        brf     {loop_label}");
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
