//! Arithmetic operator code generation (add, sub, mul) with pointer arithmetic.

mod arithmetic;

pub use arithmetic::{gen_add_sub, gen_mul};
