//! Struct member access and assignment code generation.

mod member;

pub use member::{gen_inc_dec_member, gen_member_access, gen_member_addr, gen_member_assign};
