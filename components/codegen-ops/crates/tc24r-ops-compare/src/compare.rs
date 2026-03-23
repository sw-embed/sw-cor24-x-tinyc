//! Comparison operators. Assumes r0=lhs, r1=rhs already evaluated.

use tc24r_codegen_state::CodegenState;
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
///
/// `kind` selects the comparison:
/// - `Lt`: r0 < r1
/// - `Gt`: r0 > r1
/// - `Le`: r0 <= r1
/// - `Ge`: r0 >= r1
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
