//! Switch statement code generation.

use tc24r_ast::{Expr, Stmt, SwitchCase};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::{emit_bra, emit_brt, new_label};
use tc24r_emit_macros::emit;
use tc24r_stmt_simple::GenStmtFn;
use tc24r_type_infer::GenExprFn;

/// Generate code for `switch (expr) { case V: ... default: ... }`.
///
/// Emits a compare-and-branch chain. Each case compares the switch
/// expression against the case value and branches to the case body.
/// Fall-through between cases is supported (no implicit break).
pub fn gen_switch(
    state: &mut CodegenState,
    expr: &Expr,
    cases: &[SwitchCase],
    default: Option<&[Stmt]>,
    gen_expr_fn: GenExprFn,
    gen_stmt_fn: GenStmtFn,
) {
    let done_label = new_label(state);
    state.break_labels.push(done_label.clone());

    // Evaluate switch expression and save on stack
    gen_expr_fn(expr, state);
    emit!(state, "        push    r0");

    // Generate case labels
    let case_labels: Vec<String> = cases.iter().map(|_| new_label(state)).collect();
    let default_label = new_label(state);

    // Emit compare-and-branch chain
    for (i, case) in cases.iter().enumerate() {
        emit!(state, "        pop     r0");
        emit!(state, "        push    r0");
        gen_case_compare(state, &case.value, &case_labels[i], gen_expr_fn);
    }

    // After all comparisons: jump to default or done
    emit!(state, "        pop     r0");
    if default.is_some() {
        emit_bra(state, &default_label);
    } else {
        emit_bra(state, &done_label);
    }

    // Emit case bodies (fall-through between them)
    for (i, case) in cases.iter().enumerate() {
        emit!(state, "{}:", case_labels[i]);
        for s in &case.stmts {
            gen_stmt_fn(s, state);
        }
    }

    // Emit default body
    if let Some(stmts) = default {
        emit!(state, "{default_label}:");
        for s in stmts {
            gen_stmt_fn(s, state);
        }
    }

    emit!(state, "{done_label}:");
    state.break_labels.pop();
}

/// Compare saved switch value (on stack) against case value and branch.
fn gen_case_compare(
    state: &mut CodegenState,
    value: &Expr,
    label: &str,
    gen_expr_fn: GenExprFn,
) {
    // r0 has the switch value (from pop+push above)
    // Save switch value in r1, evaluate case value into r0
    emit!(state, "        mov     r1,r0");
    gen_expr_fn(value, state);
    emit!(state, "        ceq     r0,r1");
    emit_brt(state, label);
}
