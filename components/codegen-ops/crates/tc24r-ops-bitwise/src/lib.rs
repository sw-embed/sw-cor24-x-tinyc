//! Bitwise operator code generation (and, or, xor, shl, shr).

mod bitwise;

pub use bitwise::{gen_bitwise_and, gen_bitwise_or, gen_bitwise_xor, gen_shl, gen_shr, gen_shr_logical};
