//! Unary operators. Assumes r0=operand already evaluated.

use tc24r_ast::UnaryOp;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::{emit, emit_lines};

/// Generate a unary operation. Assumes r0 holds the operand value.
pub fn gen_unary(state: &mut CodegenState, op: UnaryOp) {
    match op {
        UnaryOp::Neg => {
            emit_lines!(
                state,
                "        push    r0",
                "        lc      r0,0",
                "        pop     r1",
                "        sub     r0,r1",
            );
        }
        UnaryOp::BitNot => {
            emit!(state, "        lc      r1,-1");
            emit!(state, "        xor     r0,r1");
        }
        UnaryOp::LogNot => {
            emit!(state, "        ceq     r0,z");
            emit!(state, "        mov     r0,c");
        }
    }
}
