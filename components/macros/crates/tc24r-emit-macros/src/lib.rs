//! tc24r code generation.
//!
//! These macros reduce boilerplate when emitting COR24 assembly lines.
//! They operate on any state that has a `pub out: String` field
//! (typically `CodegenState` or `Codegen`).
//!
//! # Macros
//!
//! - [`emit!`] -- emit a single assembly line
//! - [`emit_lines!`] -- emit multiple assembly lines
//! - [`emit_label!`] -- emit a label definition
//! - [`emit_comment!`] -- emit an assembly comment

mod emit;
