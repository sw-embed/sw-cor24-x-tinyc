//! tc24r.
//!
//! Supports:
//! - `#define NAME value` constant substitution
//! - `#include "file.h"` (relative to source directory)
//! - `#include <file.h>` (search system include paths)
//! - `#pragma once` (skip file if already included)

mod func_args;
mod func_macro;
mod func_subst;
mod include;
mod preprocess;
mod substitute;

pub use func_macro::FuncMacro;
pub use preprocess::preprocess;
pub(crate) use preprocess::{Context, process_text};
