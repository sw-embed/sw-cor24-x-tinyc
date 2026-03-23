//! Type parsing utilities for the cc24 parser.

mod type_detect;
mod type_parse;

pub use type_detect::{
    is_base_type, is_storage_class, is_type_keyword, is_type_start, is_typedef_name,
};
pub use type_parse::parse_type;
