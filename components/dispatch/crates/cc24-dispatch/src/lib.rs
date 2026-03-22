//! Generic chain-of-responsibility dispatch.
//!
//! Provides a `Handler` trait and `Chain` collection that routes
//! inputs to the first handler that accepts them. Type-parameterised
//! so the same infrastructure works for statements, expressions,
//! operators, or any other dispatch need.

mod chain;
mod handler;

pub use chain::Chain;
pub use handler::Handler;
