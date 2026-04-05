//! Token-aware substitution that skips string literals.

use std::collections::{HashMap, HashSet};

use crate::func_macro::{self, FuncMacro};

/// Maximum macro expansion depth to prevent stack overflow.
const MAX_EXPAND_DEPTH: usize = 128;

/// Expand defines in a single line, respecting string boundaries.
pub fn expand_line(
    line: &str,
    defines: &HashMap<String, String>,
    func_macros: &HashMap<String, FuncMacro>,
) -> String {
    expand_line_depth(line, defines, func_macros, &mut HashSet::new(), 0)
}

/// Depth-limited expansion with a set of currently-expanding macro names
/// to prevent infinite self-referencing (C99 §6.10.3.4).
fn expand_line_depth(
    line: &str,
    defines: &HashMap<String, String>,
    func_macros: &HashMap<String, FuncMacro>,
    expanding: &mut HashSet<String>,
    depth: usize,
) -> String {
    if depth >= MAX_EXPAND_DEPTH || (defines.is_empty() && func_macros.is_empty()) {
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
        } else if bytes[i] == b'\'' {
            // Skip char literals to avoid expanding L in L'x' etc.
            result.push('\'');
            i += 1;
            while i < bytes.len() && bytes[i] != b'\'' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() {
                    result.push(bytes[i] as char);
                    i += 1;
                }
                result.push(bytes[i] as char);
                i += 1;
            }
            if i < bytes.len() {
                result.push('\'');
                i += 1;
            }
        } else if is_ident_start(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_ident_char(bytes[i]) {
                i += 1;
            }
            let word = &line[start..i];
            i = expand_ident(
                word,
                &line[i..],
                defines,
                func_macros,
                &mut result,
                i,
                expanding,
                depth,
            );
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

/// Expand an identifier: try function-like macro first, then simple define.
/// Skips expansion if the macro name is in the `expanding` set (self-reference guard).
fn expand_ident(
    word: &str,
    rest: &str,
    defines: &HashMap<String, String>,
    func_macros: &HashMap<String, FuncMacro>,
    result: &mut String,
    pos: usize,
    expanding: &mut HashSet<String>,
    depth: usize,
) -> usize {
    // C99: a macro cannot recursively expand itself
    if expanding.contains(word) {
        result.push_str(word);
        return pos;
    }
    if let Some((fm, args, consumed)) = func_macros.get(word).and_then(|fm| {
        func_macro::parse_invocation_args(rest).map(|(args, consumed)| (fm, args, consumed))
    }) {
        let expanded = func_macro::substitute_params(&fm.body, &fm.params, &args);
        expanding.insert(word.to_string());
        let re_expanded = expand_line_depth(&expanded, defines, func_macros, expanding, depth + 1);
        expanding.remove(word);
        result.push_str(&re_expanded);
        return pos + consumed;
    }
    if let Some(replacement) = defines.get(word) {
        expanding.insert(word.to_string());
        let re_expanded =
            expand_line_depth(replacement, defines, func_macros, expanding, depth + 1);
        expanding.remove(word);
        result.push_str(&re_expanded);
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
