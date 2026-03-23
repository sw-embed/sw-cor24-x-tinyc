//! Division and modulo operator code generation via runtime calls.

mod divmod;

pub use divmod::{emit_divmod_runtime, gen_divmod_call};
