//! Data section and startup code emission.

mod data;
mod start;

pub use data::emit_data_section;
pub use start::emit_start;
