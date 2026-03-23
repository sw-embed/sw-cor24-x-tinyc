//! Local variable collection pre-pass for COR24.

mod collect;

pub use collect::{collect_locals_block, collect_locals_stmt};
