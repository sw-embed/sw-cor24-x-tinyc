//! Tests for handler definition macro.

use tc24r_dispatch::Handler;
use tc24r_handler_macros::define_handler;

/// Example input enum for testing.
#[derive(Debug)]
enum Op {
    Add(i32, i32),
    Negate(i32),
    Noop,
}

/// Example state for testing.
struct Acc {
    pub result: i32,
}

// -- Define handlers using the macro --

// Handles Op::Add by summing the two operands.
define_handler!(
    AddHandler,
    Op,
    Acc,
    matches Op::Add(_, _),
    |input, state| {
        if let Op::Add(a, b) = input {
            state.result = a + b;
        }
    }
);

// Handles Op::Negate by negating the operand.
define_handler!(
    NegateHandler,
    Op,
    Acc,
    matches Op::Negate(_),
    |input, state| {
        if let Op::Negate(v) = input {
            state.result = -v;
        }
    }
);

#[test]
fn handler_can_handle_matching() {
    let h = AddHandler;
    assert!(h.can_handle(&Op::Add(1, 2)));
    assert!(!h.can_handle(&Op::Negate(5)));
    assert!(!h.can_handle(&Op::Noop));
}

#[test]
fn handler_handle_executes() {
    let h = AddHandler;
    let mut acc = Acc { result: 0 };
    h.handle(&Op::Add(3, 4), &mut acc);
    assert_eq!(acc.result, 7);
}

#[test]
fn handler_in_chain() {
    use tc24r_dispatch::Chain;

    let mut chain: Chain<dyn Handler<Op, Acc>> = Chain::new();
    chain.register(AddHandler);
    chain.register(NegateHandler);

    let mut acc = Acc { result: 0 };

    // Dispatch Add
    assert!(chain.dispatch(&Op::Add(10, 20), &mut acc));
    assert_eq!(acc.result, 30);

    // Dispatch Negate
    assert!(chain.dispatch(&Op::Negate(7), &mut acc));
    assert_eq!(acc.result, -7);

    // Noop not handled
    assert!(!chain.dispatch(&Op::Noop, &mut acc));
}
