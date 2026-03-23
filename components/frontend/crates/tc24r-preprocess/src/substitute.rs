//! Token-aware substitution that skips string literals.

use std::collections::HashMap;

use crate::func_macro::{self, FuncMacro};

/// Expand defines in a single line, respecting string boundaries.
pub fn expand_line(
    line: &str,
    defines: &HashMap<String, String>,
    func_macros: &HashMap<String, FuncMacro>,
) -> String {
    if defines.is_empty() && func_macros.is_empty() {
        return line.to_string();
    }

    let mut result = String::with_capacity(line.len());
    let bytes = line.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'"' {
            result.push('"');
            i += 1;
            i = copy_string_literal(bytes, i, &mut result);
        } else if is_ident_start(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_ident_char(bytes[i]) {
                i += 1;
            }
            let word = &line[start..i];
            i = expand_ident(word, &line[i..], defines, func_macros, &mut result, i);
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

/// Expand an identifier: try function-like macro first, then simple define.
/// Returns the new byte index into the original line.
fn expand_ident(
    word: &str,
    rest: &str,
    defines: &HashMap<String, String>,
    func_macros: &HashMap<String, FuncMacro>,
    result: &mut String,
    pos: usize,
) -> usize {
    if let Some((fm, args, consumed)) = func_macros.get(word).and_then(|fm| {
        func_macro::parse_invocation_args(rest).map(|(args, consumed)| (fm, args, consumed))
    }) {
        let expanded = func_macro::substitute_params(&fm.body, &fm.params, &args);
        let re_expanded = expand_line(&expanded, defines, func_macros);
        result.push_str(&re_expanded);
        return pos + consumed;
    }
    if let Some(replacement) = defines.get(word) {
        result.push_str(replacement);
    } else {
        result.push_str(word);
    }
    pos
}

fn copy_string_literal(bytes: &[u8], start: usize, out: &mut String) -> usize {
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

fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
