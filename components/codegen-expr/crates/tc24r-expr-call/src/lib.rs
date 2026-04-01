//! Function call expression code generation.

mod call;

pub use call::{gen_call, gen_indirect_call};
