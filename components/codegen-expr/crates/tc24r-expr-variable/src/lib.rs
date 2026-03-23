//! Variable expression code generation (Ident, Assign, AddrOf).

mod addr_of;
mod assign;
mod load;

pub use addr_of::gen_addr_of;
pub use assign::gen_assign;
pub use load::gen_ident;
