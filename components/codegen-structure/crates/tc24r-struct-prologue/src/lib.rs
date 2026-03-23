//! Function prologue and epilogue generation for COR24.

mod epilogue;
mod prologue;

pub use epilogue::emit_epilogue;
pub use prologue::{emit_locals_alloc, emit_prologue};
