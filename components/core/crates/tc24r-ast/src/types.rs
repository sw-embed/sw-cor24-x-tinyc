//! C type representations.

/// A C type.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Char,
    UnsignedChar,
    Int,
    UnsignedInt,
    Void,
    Ptr(Box<Type>),
    /// Fixed-size array: element type and count.
    Array(Box<Type>, usize),
    /// Struct type with optional tag name and members.
    Struct {
        tag: Option<String>,
        members: Vec<StructMember>,
        total_size: i32,
    },
}

/// A struct member with name, type, and byte offset from struct base.
/// For bitfields, `bit_width > 0` and `bit_offset` gives the position
/// within the word at `offset`.
#[derive(Debug, Clone, PartialEq)]
pub struct StructMember {
    pub name: String,
    pub ty: Type,
    pub offset: i32,
    /// 0 = regular member, >0 = bitfield width in bits.
    pub bit_width: u8,
    /// Bit position within the word at `offset` (0 = LSB).
    pub bit_offset: u8,
}

impl Type {
    /// Size in bytes for this type.
    pub fn size(&self) -> i32 {
        match self {
            Type::Char | Type::UnsignedChar => 1,
            Type::Int | Type::UnsignedInt | Type::Void | Type::Ptr(_) => 3,
            Type::Array(elem, count) => elem.size() * (*count as i32),
            Type::Struct { total_size, .. } => *total_size,
        }
    }

    /// Element type for arrays and pointers.
    pub fn element_type(&self) -> Option<&Type> {
        match self {
            Type::Ptr(inner) | Type::Array(inner, _) => Some(inner),
            _ => None,
        }
    }

    /// Look up a struct member by name.
    pub fn find_member(&self, name: &str) -> Option<&StructMember> {
        match self {
            Type::Struct { members, .. } => members.iter().find(|m| m.name == name),
            _ => None,
        }
    }
}
