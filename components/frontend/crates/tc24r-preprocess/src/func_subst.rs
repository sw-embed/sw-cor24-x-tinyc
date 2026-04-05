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
/// Handles stringification (`#param` → `"arg"`) and token pasting (`a ## b` → `ab`).
fn token_substitute(body: &str, map: &HashMap<&str, &str>) -> String {
    // First pass: substitute parameters and handle stringification
    let bytes = body.as_bytes();
    let mut result = String::with_capacity(body.len());
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'"' {
            result.push('"');
            i += 1;
            i = copy_string_into(&mut result, bytes, i);
        } else if bytes[i] == b'#' && !is_token_paste(bytes, i) {
            // Stringification: #param → "arg"
            i += 1;
            while i < bytes.len() && bytes[i] == b' ' {
                i += 1;
            }
            if i < bytes.len() && is_ident_start(bytes[i]) {
                let start = i;
                while i < bytes.len() && is_ident_char(bytes[i]) {
                    i += 1;
                }
                let word = &body[start..i];
                if let Some(arg) = map.get(word) {
                    stringify_into(&mut result, arg);
                } else {
                    result.push('#');
                    result.push_str(word);
                }
            } else {
                result.push('#');
            }
        } else if bytes[i] == b'#' && is_token_paste(bytes, i) {
            // Token pasting: trim trailing whitespace from result,
            // skip ## and leading whitespace, concatenate tokens
            while result.ends_with(' ') {
                result.pop();
            }
            i += 2; // skip ##
            while i < bytes.len() && bytes[i] == b' ' {
                i += 1;
            }
            // The next token will be appended directly (no space)
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

/// Check if `#` at position `i` is part of `##` (token paste), not stringification.
fn is_token_paste(bytes: &[u8], i: usize) -> bool {
    i + 1 < bytes.len() && bytes[i + 1] == b'#'
}

/// Convert an argument to a string literal: wrap in quotes, escape internal quotes and backslashes.
fn stringify_into(out: &mut String, arg: &str) {
    out.push('"');
    for ch in arg.chars() {
        if ch == '"' || ch == '\\' {
            out.push('\\');
        }
        out.push(ch);
    }
    out.push('"');
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

    #[test]
    fn stringify_basic() {
        let result = substitute_params(
            "assert(x, y, #y)",
            &["x".into(), "y".into()],
            &["42".into(), "5+20-4".into()],
        );
        assert_eq!(result, "assert(42, 5+20-4, \"5+20-4\")");
    }

    #[test]
    fn stringify_with_quotes() {
        let result = substitute_params("#x", &["x".into()], &["\"hello\"".into()]);
        assert_eq!(result, "\"\\\"hello\\\"\"");
    }

    #[test]
    fn stringify_non_param_passthrough() {
        let result = substitute_params("#unknown", &["x".into()], &["42".into()]);
        assert_eq!(result, "#unknown");
    }

    #[test]
    fn stringify_whitespace_after_hash() {
        let result = substitute_params("# x", &["x".into()], &["42".into()]);
        assert_eq!(result, "\"42\"");
    }

    #[test]
    fn stringify_chibicc_assert() {
        // #define ASSERT(x, y) assert(x, y, #y)
        // ASSERT(21, 5+20-4)
        let result = substitute_params(
            "assert(x, y, #y)",
            &["x".into(), "y".into()],
            &["21".into(), "5+20-4".into()],
        );
        assert_eq!(result, "assert(21, 5+20-4, \"5+20-4\")");
    }
}
