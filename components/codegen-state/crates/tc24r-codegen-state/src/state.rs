use std::collections::{HashMap, HashSet};

use tc24r_ast::Type;

/// All mutable state carried through code generation.
#[derive(Default)]
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
    /// Whether the program needs the printf runtime helpers.
    pub needs_printf: bool,
    /// Struct type registry: tag name → full struct type.
    pub struct_types: HashMap<String, Type>,
    /// Function return type registry: function name → return type.
    pub function_types: HashMap<String, Type>,
    /// Instruction counter within the current function (for branch range estimation).
    pub instruction_count: usize,
    /// Map from label name to the instruction count when it was emitted.
    pub label_positions: HashMap<String, usize>,
    /// Deferred short branches that need post-pass validation.
    /// Each entry: (byte offset in `out` where the line starts, instruction count
    /// at emission, target label, branch kind).
    pub deferred_branches: Vec<DeferredBranch>,
}

/// A short branch emitted optimistically, to be validated after all labels are known.
pub struct DeferredBranch {
    /// Byte offset in `state.out` where this branch line starts.
    pub out_offset: usize,
    /// The full line text (e.g., "        bra     L36").
    pub line: String,
    /// Instruction count when the branch was emitted.
    pub instruction_count: usize,
    /// Target label name.
    pub target: String,
    /// Branch kind (for generating the correct long-form replacement).
    pub kind: BranchKind,
}

#[derive(Clone, Copy)]
pub enum BranchKind {
    Bra,
    Brt,
    Brf,
}
