//! Handler trait definition.

/// A handler that can process an input given mutable state.
///
/// `Input` is what the handler inspects (e.g., a Stmt or Expr).
/// `State` is the mutable context (e.g., CodegenState).
pub trait Handler<Input, State> {
    /// Return true if this handler can process the given input.
    fn can_handle(&self, input: &Input) -> bool;

    /// Process the input, mutating state as needed.
    /// Only called if `can_handle` returned true.
    fn handle(&self, input: &Input, state: &mut State);
}
