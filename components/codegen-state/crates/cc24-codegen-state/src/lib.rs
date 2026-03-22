//! Code generator state for the COR24 compiler.
//!
//! This crate owns the `CodegenState` struct that holds all mutable state
//! used during code generation. Handler crates depend on this crate (and
//! `cc24-traits`) so they can manipulate codegen state without depending
//! on the full `cc24-codegen` crate.

use std::collections::{HashMap, HashSet};

use cc24_ast::Type;

/// All mutable state carried through code generation.
#[derive(Clone, Debug, Default)]
pub struct CodegenState {
    /// Accumulated assembly output.
    pub out: String,
    /// Monotonic counter for generating unique labels.
    pub label_counter: usize,
    /// Map from local variable name to its stack-frame offset.
    pub locals: HashMap<String, i32>,
    /// Map from local variable name to its type.
    pub local_types: HashMap<String, Type>,
    /// Total bytes allocated for locals in the current frame.
    pub locals_size: i32,
    /// Set of declared global variable names.
    pub globals: HashSet<String>,
    /// Map from global variable name to its type.
    pub global_types: HashMap<String, Type>,
    /// Label to jump to for a `return` statement.
    pub return_label: String,
    /// String literals collected during generation (emitted in .data).
    pub string_literals: Vec<String>,
    /// Whether the program needs the division runtime helper.
    pub needs_div: bool,
    /// Whether the program needs the modulo runtime helper.
    pub needs_mod: bool,
    /// Stack of break-target labels for loops.
    pub break_labels: Vec<String>,
    /// Stack of continue-target labels for loops.
    pub continue_labels: Vec<String>,
}
