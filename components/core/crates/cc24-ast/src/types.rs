//! C type representations.

/// A C type.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Void,
}
