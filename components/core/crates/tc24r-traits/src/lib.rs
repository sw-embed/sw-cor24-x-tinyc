//! Handler traits for the COR24 code generator.
//!
//! Each handler crate implements one or both of these traits so it can be
//! registered with the dispatch chain. The traits deliberately take
//! `&CodegenState` / `&mut CodegenState` rather than the full `Codegen`
//! facade, keeping handler crates decoupled from the orchestration logic.

mod traits;

pub use traits::{ExprHandler, StmtHandler};
