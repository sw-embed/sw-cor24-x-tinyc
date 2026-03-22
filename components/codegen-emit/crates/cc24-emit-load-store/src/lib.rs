//! Load and store helpers for named variables.

mod load;
mod store;

pub use load::{gen_addr_of, gen_load_by_name};
pub use store::gen_store_by_name;
