//! Tests for cc24-dispatch.

#[cfg(test)]
mod tests {
    use cc24_dispatch::{Chain, Handler};

    struct DoubleHandler;

    impl Handler<i32, Vec<i32>> for DoubleHandler {
        fn can_handle(&self, input: &i32) -> bool {
            *input > 0
        }

        fn handle(&self, input: &i32, state: &mut Vec<i32>) {
            state.push(*input * 2);
        }
    }

    struct NegateHandler;

    impl Handler<i32, Vec<i32>> for NegateHandler {
        fn can_handle(&self, input: &i32) -> bool {
            *input < 0
        }

        fn handle(&self, input: &i32, state: &mut Vec<i32>) {
            state.push(-*input);
        }
    }

    #[test]
    fn dispatch_to_first_matching_handler() {
        let mut chain: Chain<dyn Handler<i32, Vec<i32>>> = Chain::new();
        chain.register(DoubleHandler);
        chain.register(NegateHandler);

        let mut state = Vec::new();
        assert!(chain.dispatch(&5, &mut state));
        assert_eq!(state, vec![10]);
    }

    #[test]
    fn dispatch_negative_to_second_handler() {
        let mut chain: Chain<dyn Handler<i32, Vec<i32>>> = Chain::new();
        chain.register(DoubleHandler);
        chain.register(NegateHandler);

        let mut state = Vec::new();
        assert!(chain.dispatch(&-3, &mut state));
        assert_eq!(state, vec![3]);
    }

    #[test]
    fn dispatch_returns_false_when_no_handler() {
        let mut chain: Chain<dyn Handler<i32, Vec<i32>>> = Chain::new();
        chain.register(DoubleHandler);

        let mut state = Vec::new();
        assert!(!chain.dispatch(&0, &mut state));
        assert!(state.is_empty());
    }
}
