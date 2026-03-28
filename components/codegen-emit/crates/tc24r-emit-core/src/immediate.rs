//! Immediate value loading.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Load an integer constant into r0, choosing the shortest encoding.
pub fn load_immediate(state: &mut CodegenState, val: i32) {
    if (-128..=127).contains(&val) {
        emit!(state, "        lc      r0,{val}");
    } else {
        emit!(state, "        la      r0,{val}");
    }
}

/// Whether an offset fits in the 8-bit signed immediate of lw/sw/lb/sb/lbu.
fn offset_fits_imm8(offset: i32) -> bool {
    (-128..=127).contains(&offset)
}

/// Load word from `fp+offset` into r0. Uses r1 as temp for large offsets.
pub fn fp_load_word_r0(state: &mut CodegenState, offset: i32) {
    if offset_fits_imm8(offset) {
        emit!(state, "        lw      r0,{offset}(fp)");
    } else {
        emit!(state, "        la      r1,{offset}");
        emit!(state, "        add     r1,fp");
        emit!(state, "        lw      r0,0(r1)");
    }
}

/// Store r0 to `fp+offset`. Uses r1 as temp for large offsets.
pub fn fp_store_word_r0(state: &mut CodegenState, offset: i32) {
    if offset_fits_imm8(offset) {
        emit!(state, "        sw      r0,{offset}(fp)");
    } else {
        emit!(state, "        la      r1,{offset}");
        emit!(state, "        add     r1,fp");
        emit!(state, "        sw      r0,0(r1)");
    }
}

/// Load word from `fp+offset` into r1. Safe for large offsets.
pub fn fp_load_word_r1(state: &mut CodegenState, offset: i32) {
    if offset_fits_imm8(offset) {
        emit!(state, "        lw      r1,{offset}(fp)");
    } else {
        emit!(state, "        la      r1,{offset}");
        emit!(state, "        add     r1,fp");
        emit!(state, "        lw      r1,0(r1)");
    }
}
