//! Compiler configuration -- data only, no behavior.
//!
//! Constructed by the builder, consumed by pipeline stages.

use std::path::PathBuf;

/// Complete compiler configuration.
#[derive(Debug, Clone)]
pub struct CompilerConfig {
    /// Path to the input .c source file.
    pub source_path: PathBuf,
    /// Directory containing the source file (for #include "...").
    pub source_dir: PathBuf,
    /// Output .s assembly file path (None = stdout).
    pub output_path: Option<PathBuf>,
    /// System include directories (for #include <...>).
    pub include_dirs: Vec<PathBuf>,
}
