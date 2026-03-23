//! Parameter substitution for function-like macro expansion.

use std::collections::HashMap;

use crate::func_args::copy_string_into;

/// Substitute parameter names in the macro body with argument values.
pub fn substitute_params(body: &str, params: &[String], args: &[String]) -> String {
    let map: HashMap<&str, &str> = params
        .iter()
        .zip(args.iter())
        .map(|(p, a)| (p.as_str(), a.as_str()))
        .collect();
    token_substitute(body, &map)
}

/// Token-aware substitution: replace identifiers matching keys in `map`.
fn token_substitute(body: &str, map: &HashMap<&str, &str>) -> String {
    let bytes = body.as_bytes();
    let mut result = String::with_capacity(body.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'"' {
            result.push('"');
            i += 1;
            i = copy_string_into(&mut result, bytes, i);
        } else if is_ident_start(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_ident_char(bytes[i]) {
                i += 1;
            }
            let word = &body[start..i];
            if let Some(replacement) = map.get(word) {
                result.push_str(replacement);
            } else {
                result.push_str(word);
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute_simple() {
        let result = substitute_params(
            "((a)+(b))",
            &["a".into(), "b".into()],
            &["3".into(), "4".into()],
        );
        assert_eq!(result, "((3)+(4))");
    }

    #[test]
    fn substitute_skips_strings() {
        let result = substitute_params(
            "assert(x,y,\"x\")",
            &["x".into(), "y".into()],
            &["42".into(), "42".into()],
        );
        assert_eq!(result, "assert(42,42,\"x\")");
    }
}
