//! Code generator state for the COR24 compiler.
//!
//! This crate owns the `CodegenState` struct that holds all mutable state
//! used during code generation. Handler crates depend on this crate (and
//! tc24r-traits`) so they can manipulate codegen state without depending
//! tc24r-codegen` crate.

mod state;

pub use state::CodegenState;
