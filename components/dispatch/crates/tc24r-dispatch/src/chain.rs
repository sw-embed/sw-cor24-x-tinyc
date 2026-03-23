//! Chain of handlers with ordered dispatch.

use crate::Handler;

/// An ordered chain of handlers. Dispatches to the first handler
/// that accepts the input.
pub struct Chain<H: ?Sized> {
    handlers: Vec<Box<H>>,
}

impl<Input, State> Chain<dyn Handler<Input, State>> {
    /// Create an empty chain.
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Register a handler at the end of the chain.
    pub fn register<H>(&mut self, handler: H)
    where
        H: Handler<Input, State> + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Dispatch an input to the first handler that accepts it.
    /// Returns true if a handler was found.
    pub fn dispatch(&self, input: &Input, state: &mut State) -> bool {
        for handler in &self.handlers {
            if handler.can_handle(input) {
                handler.handle(input, state);
                return true;
            }
        }
        false
    }
}

impl<Input, State> Default for Chain<dyn Handler<Input, State>> {
    fn default() -> Self {
        Self::new()
    }
}
