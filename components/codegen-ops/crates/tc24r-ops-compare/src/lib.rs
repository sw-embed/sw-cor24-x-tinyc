//! Comparison operator code generation (eq, ne, lt, gt, le, ge).

mod compare;

pub use compare::{gen_compare_eq, gen_compare_rel};
