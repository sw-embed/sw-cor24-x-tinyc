//! Function-like macro support: storage and definition parsing.

pub use crate::func_args::parse_invocation_args;
pub use crate::func_subst::substitute_params;

/// A function-like macro: `#define NAME(params) body`.
#[derive(Clone, Debug)]
pub struct FuncMacro {
    pub params: Vec<String>,
    pub body: String,
}

/// Parse parameter names from `"a, b)"` (after the opening paren).
/// Returns param list and the remaining body text after `) `.
pub fn parse_params_and_body(after_paren: &str) -> Option<(Vec<String>, String)> {
    let close = find_close_paren(after_paren)?;
    let param_str = &after_paren[..close];
    let params = parse_param_names(param_str);
    let body = crate::preprocess::strip_line_comment(after_paren[close + 1..].trim()).to_string();
    Some((params, body))
}

fn find_close_paren(s: &str) -> Option<usize> {
    s.find(')')
}

fn parse_param_names(s: &str) -> Vec<String> {
    if s.trim().is_empty() {
        return Vec::new();
    }
    s.split(',').map(|p| p.trim().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_params() {
        let (params, body) = parse_params_and_body("a, b) ((a)+(b))").unwrap();
        assert_eq!(params, vec!["a", "b"]);
        assert_eq!(body, "((a)+(b))");
    }

    #[test]
    fn parse_no_params() {
        let (params, body) = parse_params_and_body(") 1").unwrap();
        assert!(params.is_empty());
        assert_eq!(body, "1");
    }
}
