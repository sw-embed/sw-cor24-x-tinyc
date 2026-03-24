//! Bitwise operators. Assumes r0=lhs, r1=rhs already evaluated.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Bitwise AND: r0 = r0 & r1.
pub fn gen_bitwise_and(state: &mut CodegenState) {
    emit!(state, "        and     r0,r1");
}

/// Bitwise OR: r0 = r0 | r1.
pub fn gen_bitwise_or(state: &mut CodegenState) {
    emit!(state, "        or      r0,r1");
}

/// Bitwise XOR: r0 = r0 ^ r1.
pub fn gen_bitwise_xor(state: &mut CodegenState) {
    emit!(state, "        xor     r0,r1");
}

/// Shift left: r0 = r0 << r1.
pub fn gen_shl(state: &mut CodegenState) {
    emit!(state, "        shl     r0,r1");
}

/// Shift right (arithmetic): r0 = r0 >> r1 (sign-extending).
///
/// C99 §6.5.7: right shift of signed values is implementation-defined.
/// We choose arithmetic shift (sign-extend) which is the conventional
/// behavior matching GCC/Clang. Use `gen_shr_logical` for unsigned types.
pub fn gen_shr(state: &mut CodegenState) {
    emit!(state, "        sra     r0,r1");
}

/// Shift right (logical): r0 = r0 >>> r1 (zero-fill).
/// Used for unsigned integer right shifts.
pub fn gen_shr_logical(state: &mut CodegenState) {
    emit!(state, "        srl     r0,r1");
}
