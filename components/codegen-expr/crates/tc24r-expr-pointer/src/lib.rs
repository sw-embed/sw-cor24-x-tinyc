//! Pointer expression code generation (Deref, DerefAssign, Cast).

mod cast;
mod deref;

pub use cast::gen_cast;
pub use deref::{gen_deref, gen_deref_assign, gen_inc_dec_deref};
