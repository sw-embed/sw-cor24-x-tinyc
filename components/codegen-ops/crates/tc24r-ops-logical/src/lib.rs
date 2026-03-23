//! Short-circuit logical operator code generation (&& and ||).

mod logical;

pub use logical::{gen_log_and, gen_log_or};
