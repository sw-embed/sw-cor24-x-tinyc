//! Handler traits for the COR24 code generator.
//!
//! Each handler crate implements one or both of these traits so it can be
//! registered with the dispatch chain. The traits deliberately take
//! `&CodegenState` / `&mut CodegenState` rather than the full `Codegen`
//! facade, keeping handler crates decoupled from the orchestration logic.

use cc24_ast::{Expr, Stmt};
use cc24_codegen_state::CodegenState;

/// A handler that can generate code for a subset of statements.
pub trait StmtHandler {
    /// Return `true` if this handler knows how to generate code for `stmt`.
    fn can_handle(&self, stmt: &Stmt) -> bool;
    /// Generate assembly for `stmt`, mutating `state`.
    fn handle(&self, stmt: &Stmt, state: &mut CodegenState);
}

/// A handler that can generate code for a subset of expressions.
pub trait ExprHandler {
    /// Return `true` if this handler knows how to generate code for `expr`.
    fn can_handle(&self, expr: &Expr) -> bool;
    /// Generate assembly for `expr`, mutating `state`.
    fn handle(&self, expr: &Expr, state: &mut CodegenState);
}
