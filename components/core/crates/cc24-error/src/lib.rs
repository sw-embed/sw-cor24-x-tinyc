//! Compiler error type with optional source span.

use cc24_span::Span;
use std::fmt;

/// A compilation error with a human-readable message and optional
/// source location.
#[derive(Debug)]
pub struct CompileError {
    pub message: String,
    pub span: Option<Span>,
}

impl CompileError {
    pub fn new(message: impl Into<String>, span: Option<Span>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = self.span {
            write!(f, "error at offset {}: {}", span.offset, self.message)
        } else {
            write!(f, "error: {}", self.message)
        }
    }
}

impl std::error::Error for CompileError {}
