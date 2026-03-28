//! Core assembly emission primitives for the COR24 compiler.

mod emit;
mod immediate;

pub use emit::{emit, emit_bra, emit_brf, emit_brt, new_label, resolve_branches};
pub use immediate::{fp_load_word_r0, fp_load_word_r1, fp_store_word_r0, load_immediate};
