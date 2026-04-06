//! Comparison operators. Assumes r0=lhs, r1=rhs already evaluated.
//!
//! Two modes:
//! - **Materialize**: produce a 0/1 boolean in r0 (for use as a value)
//! - **Branch fusion**: emit compare + branch directly (for if/while conditions)

use tc24r_ast::BinOp;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::{emit_brf, emit_brt};
use tc24r_emit_macros::emit;

/// Equality comparison: r0 = (r0 == r1) or r0 = (r0 != r1).
///
/// Pass `negate: false` for `==`, `true` for `!=`.
pub fn gen_compare_eq(state: &mut CodegenState, negate: bool) {
    emit!(state, "        ceq     r0,r1");
    emit!(state, "        mov     r0,c");
    if negate {
        emit!(state, "        ceq     r0,z");
        emit!(state, "        mov     r0,c");
    }
}

/// Relational comparison. Assumes r0=lhs, r1=rhs already evaluated.
pub fn gen_compare_rel(state: &mut CodegenState, kind: RelKind) {
    match kind {
        RelKind::Lt => {
            emit!(state, "        cls     r0,r1");
            emit!(state, "        mov     r0,c");
        }
        RelKind::Gt => {
            emit!(state, "        cls     r1,r0");
            emit!(state, "        mov     r0,c");
        }
        RelKind::Le => {
            emit!(state, "        cls     r1,r0");
            emit!(state, "        mov     r0,c");
            emit!(state, "        ceq     r0,z");
            emit!(state, "        mov     r0,c");
        }
        RelKind::Ge => {
            emit!(state, "        cls     r0,r1");
            emit!(state, "        mov     r0,c");
            emit!(state, "        ceq     r0,z");
            emit!(state, "        mov     r0,c");
        }
    }
}

/// Relational comparison kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelKind {
    Lt,
    Gt,
    Le,
    Ge,
}

/// Returns true if `op` is a comparison operator (==, !=, <, >, <=, >=).
pub fn is_comparison_op(op: BinOp) -> bool {
    matches!(
        op,
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge
    )
}

/// Select the compare-less instruction: `cls` (signed) or `clu` (unsigned).
fn cmp_less(is_unsigned: bool) -> &'static str {
    if is_unsigned {
        "clu"
    } else {
        "cls"
    }
}

/// Emit a comparison and branch to `label` when the condition is FALSE.
pub fn gen_compare_branch(
    state: &mut CodegenState,
    op: BinOp,
    skip_label: &str,
    is_unsigned: bool,
) {
    let cmp = cmp_less(is_unsigned);
    match op {
        BinOp::Eq => {
            emit!(state, "        ceq     r0,r1");
            emit_brf(state, skip_label);
        }
        BinOp::Ne => {
            emit!(state, "        ceq     r0,r1");
            emit_brt(state, skip_label);
        }
        BinOp::Lt => {
            emit!(state, "        {cmp}     r0,r1");
            emit_brf(state, skip_label);
        }
        BinOp::Ge => {
            emit!(state, "        {cmp}     r0,r1");
            emit_brt(state, skip_label);
        }
        BinOp::Gt => {
            emit!(state, "        {cmp}     r1,r0");
            emit_brf(state, skip_label);
        }
        BinOp::Le => {
            emit!(state, "        {cmp}     r1,r0");
            emit_brt(state, skip_label);
        }
        _ => panic!("gen_compare_branch called with non-comparison op: {op:?}"),
    }
}

/// Like `gen_compare_branch` but branches when the condition is TRUE.
pub fn gen_compare_branch_true(
    state: &mut CodegenState,
    op: BinOp,
    loop_label: &str,
    is_unsigned: bool,
) {
    let cmp = cmp_less(is_unsigned);
    match op {
        BinOp::Eq => {
            emit!(state, "        ceq     r0,r1");
            emit_brt(state, loop_label);
        }
        BinOp::Ne => {
            emit!(state, "        ceq     r0,r1");
            emit_brf(state, loop_label);
        }
        BinOp::Lt => {
            emit!(state, "        {cmp}     r0,r1");
            emit_brt(state, loop_label);
        }
        BinOp::Ge => {
            emit!(state, "        {cmp}     r0,r1");
            emit_brf(state, loop_label);
        }
        BinOp::Gt => {
            emit!(state, "        {cmp}     r1,r0");
            emit_brt(state, loop_label);
        }
        BinOp::Le => {
            emit!(state, "        {cmp}     r1,r0");
            emit_brf(state, loop_label);
        }
        _ => panic!("gen_compare_branch_true called with non-comparison op: {op:?}"),
    }
}

/// Compare r0 against zero (using z register) and branch when FALSE.
/// Handles Eq, Ne, Lt, Ge efficiently. Gt and Le fall back to r1 load.
pub fn gen_compare_branch_zero(
    state: &mut CodegenState,
    op: BinOp,
    skip_label: &str,
    is_unsigned: bool,
) {
    let cmp = cmp_less(is_unsigned);
    match op {
        BinOp::Eq => {
            emit!(state, "        ceq     r0,z");
            emit_brf(state, skip_label);
        }
        BinOp::Ne => {
            emit!(state, "        ceq     r0,z");
            emit_brt(state, skip_label);
        }
        BinOp::Lt => {
            emit!(state, "        {cmp}     r0,z");
            emit_brf(state, skip_label);
        }
        BinOp::Ge => {
            emit!(state, "        {cmp}     r0,z");
            emit_brt(state, skip_label);
        }
        _ => {
            // Gt, Le: fall back to loading 0 into r1
            emit!(state, "        lc      r1,0");
            gen_compare_branch(state, op, skip_label, is_unsigned);
        }
    }
}
