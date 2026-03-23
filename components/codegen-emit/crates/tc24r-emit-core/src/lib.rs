//! Core assembly emission primitives for the COR24 compiler.

mod emit;
mod immediate;

pub use emit::{emit, new_label};
pub use immediate::load_immediate;
