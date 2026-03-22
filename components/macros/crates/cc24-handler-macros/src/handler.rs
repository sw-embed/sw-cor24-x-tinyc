//! Handler definition macro.

/// Define a dispatch handler struct with its `Handler` trait implementation.
///
/// Generates a unit struct and implements `cc24_dispatch::Handler<Input, State>`
/// with the provided `can_handle` match pattern and `handle` body.
///
/// # Parameters
///
/// - `$name` -- name of the handler struct to create
/// - `$input` -- the input type (e.g., `cc24_ast::Stmt`)
/// - `$state` -- the state type (e.g., `Codegen`)
/// - `$pattern` -- a pattern to match in `can_handle` (e.g., `Stmt::Return(_)`)
/// - `$input_var` -- binding name for the input in the `handle` body
/// - `$state_var` -- binding name for the state in the `handle` body
/// - `$body` -- the handler body block
///
/// # Usage
///
/// ```rust
/// # // This example demonstrates the macro expansion pattern.
/// # // In real usage, the types come from cc24-ast and cc24-codegen-state.
/// use cc24_dispatch::Handler;
/// use cc24_handler_macros::define_handler;
///
/// // Example types for demonstration:
/// enum Input { Add(i32, i32), Noop }
/// struct State { pub result: i32 }
///
/// // Define a handler that processes Add inputs:
/// //
/// // This expands to:
/// //   pub struct AddHandler;
/// //   impl Handler<Input, State> for AddHandler {
/// //       fn can_handle(&self, input: &Input) -> bool {
/// //           matches!(input, Input::Add(_, _))
/// //       }
/// //       fn handle(&self, input: &Input, state: &mut State) {
/// //           if let Input::Add(a, b) = input {
/// //               state.result = a + b;
/// //           }
/// //       }
/// //   }
/// define_handler!(
///     AddHandler,
///     Input,
///     State,
///     matches Input::Add(_, _),
///     |input, state| {
///         if let Input::Add(a, b) = input {
///             state.result = a + b;
///         }
///     }
/// );
///
/// let handler = AddHandler;
/// let input = Input::Add(3, 4);
/// let mut state = State { result: 0 };
///
/// assert!(handler.can_handle(&input));
/// handler.handle(&input, &mut state);
/// assert_eq!(state.result, 7);
///
/// // Does not handle Noop:
/// assert!(!handler.can_handle(&Input::Noop));
/// ```
#[macro_export]
macro_rules! define_handler {
    (
        $name:ident,
        $input:ty,
        $state:ty,
        matches $pattern:pat,
        |$input_var:ident, $state_var:ident| $body:block
    ) => {
        pub struct $name;

        impl cc24_dispatch::Handler<$input, $state> for $name {
            fn can_handle(&self, input: &$input) -> bool {
                matches!(input, $pattern)
            }

            fn handle(
                &self,
                $input_var: &$input,
                $state_var: &mut $state,
            ) $body
        }
    };
}
