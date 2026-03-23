//! Argument parsing for function-like macro invocations.

/// Parse arguments from an invocation like `(42, 1+2)` starting at the `(`.
/// Returns the list of argument strings and the byte index past the closing `)`.
pub fn parse_invocation_args(text: &str) -> Option<(Vec<String>, usize)> {
    let bytes = text.as_bytes();
    if bytes.is_empty() || bytes[0] != b'(' {
        return None;
    }
    let mut args = Vec::new();
    let mut depth: u32 = 1;
    let mut current = String::new();
    let mut i = 1;

    while i < bytes.len() && depth > 0 {
        let ch = bytes[i];
        match ch {
            b'(' => {
                depth += 1;
                current.push('(');
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    args.push(current.trim().to_string());
                } else {
                    current.push(')');
                }
            }
            b',' if depth == 1 => {
                args.push(current.trim().to_string());
                current = String::new();
            }
            b'"' => {
                current.push('"');
                i += 1;
                i = copy_string_into(&mut current, bytes, i);
                continue;
            }
            _ => current.push(ch as char),
        }
        i += 1;
    }

    if depth != 0 {
        return None;
    }
    Some((args, i))
}

/// Copy a string literal (after opening `"`) into `out`, return index after closing `"`.
pub(crate) fn copy_string_into(out: &mut String, bytes: &[u8], start: usize) -> usize {
    let mut i = start;
    while i < bytes.len() {
        let ch = bytes[i];
        out.push(ch as char);
        i += 1;
        if ch == b'"' {
            return i;
        }
        if ch == b'\\' && i < bytes.len() {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invocation_simple() {
        let (args, end) = parse_invocation_args("(3, 4)").unwrap();
        assert_eq!(args, vec!["3", "4"]);
        assert_eq!(end, 6);
    }

    #[test]
    fn invocation_nested_parens() {
        let (args, _) = parse_invocation_args("(5, (3+2))").unwrap();
        assert_eq!(args, vec!["5", "(3+2)"]);
    }

    #[test]
    fn invocation_nested_func_call() {
        let (args, _) = parse_invocation_args("(5, func(a,b))").unwrap();
        assert_eq!(args, vec!["5", "func(a,b)"]);
    }

    #[test]
    fn invocation_string_with_comma() {
        let (args, _) = parse_invocation_args("(\"a,b\", 2)").unwrap();
        assert_eq!(args, vec!["\"a,b\"", "2"]);
    }
}
