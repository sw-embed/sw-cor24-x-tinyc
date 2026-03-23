//! ISR prologue and epilogue generation for COR24.

mod isr_epilogue;
mod isr_prologue;

pub use isr_epilogue::emit_isr_epilogue;
pub use isr_prologue::emit_isr_prologue;
