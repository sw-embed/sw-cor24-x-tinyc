//! Core assembly emission primitives for the COR24 compiler.

mod emit;
mod immediate;

pub use emit::{emit, emit_bra, emit_brf, emit_brt, new_label};
pub use immediate::load_immediate;
