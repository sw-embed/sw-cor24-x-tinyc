//! Operator expression dispatch (delegates to L2 codegen-ops crates).

mod binop;
mod unary;

pub use binop::gen_binop_dispatch;
pub use unary::gen_unary_dispatch;
