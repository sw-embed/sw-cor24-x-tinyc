//! Conditional compilation: #if, #ifdef, #ifndef, #elif, #else, #endif.

use std::collections::HashMap;

use crate::func_macro::FuncMacro;

/// State for one level of #if nesting.
#[derive(Clone)]
struct IfState {
    /// Has any branch in this #if/#elif chain been taken?
    any_taken: bool,
    /// Is the current branch active (emitting output)?
    active: bool,
}

/// Tracks nested #if/#ifdef/#ifndef state.
pub struct CondStack {
    stack: Vec<IfState>,
}

impl CondStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Are we currently emitting output?
    pub fn is_active(&self) -> bool {
        self.stack.iter().all(|s| s.active)
    }

    /// Handle `#if <expr>` or `#ifdef`/`#ifndef`.
    pub fn push_if(&mut self, condition: bool) {
        self.stack.push(IfState {
            any_taken: condition,
            active: condition,
        });
    }

    /// Handle `#elif <expr>`.
    pub fn handle_elif(&mut self, condition: bool) {
        if let Some(top) = self.stack.last_mut() {
            if top.any_taken {
                top.active = false;
            } else if condition {
                top.active = true;
                top.any_taken = true;
            } else {
                top.active = false;
            }
        }
    }

    /// Handle `#else`.
    pub fn handle_else(&mut self) {
        if let Some(top) = self.stack.last_mut() {
            top.active = !top.any_taken;
            top.any_taken = true;
        }
    }

    /// Handle `#endif`.
    pub fn pop(&mut self) {
        self.stack.pop();
    }
}

/// Evaluate a simple preprocessor condition expression.
/// Supports: integer literals, defined(NAME), defined NAME,
/// macro names (1 if defined, 0 if not), basic +, -, *, ==, !=, <, >, <=, >=.
pub fn eval_condition(
    expr: &str,
    defines: &HashMap<String, String>,
    func_macros: &HashMap<String, FuncMacro>,
) -> bool {
    let expanded = expand_condition(expr.trim(), defines, func_macros);
    eval_expr(&expanded) != 0
}

/// Expand macros in condition, replacing `defined(X)` and `defined X`
/// with 1/0, then expanding remaining macros, then replacing unknown
/// identifiers with 0.
fn expand_condition(
    expr: &str,
    defines: &HashMap<String, String>,
    func_macros: &HashMap<String, FuncMacro>,
) -> String {
    let mut result = String::new();
    let bytes = expr.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' {
            let start = i;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &expr[start..i];

            if word == "defined" {
                // defined(NAME) or defined NAME
                let rest = &expr[i..].trim_start();
                if rest.starts_with('(') {
                    let close = rest.find(')').unwrap_or(rest.len());
                    let name = rest[1..close].trim();
                    let val = if defines.contains_key(name) || func_macros.contains_key(name) {
                        "1"
                    } else {
                        "0"
                    };
                    result.push_str(val);
                    // Advance past the closing paren in the original expr
                    i = expr.len() - rest.len() + close + 1;
                } else {
                    // defined NAME (no parens)
                    let name_start = i;
                    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                        i += 1;
                    }
                    let ns = i;
                    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_')
                    {
                        i += 1;
                    }
                    let name = &expr[ns..i];
                    let val = if !name.is_empty()
                        && (defines.contains_key(name) || func_macros.contains_key(name))
                    {
                        "1"
                    } else {
                        "0"
                    };
                    let _ = name_start; // suppress unused warning
                    result.push_str(val);
                }
            } else if let Some(val) = defines.get(word) {
                result.push_str(val);
            } else {
                // Unknown identifier in #if → 0 (C standard behavior)
                result.push('0');
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}

/// Evaluate a simple integer expression (no variables, just literals and ops).
fn eval_expr(expr: &str) -> i64 {
    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return 0;
    }

    // Handle comparison/equality operators (lowest precedence)
    if let Some(val) = try_binary_op(trimmed, "==", |a, b| if a == b { 1 } else { 0 }) {
        return val;
    }
    if let Some(val) = try_binary_op(trimmed, "!=", |a, b| if a != b { 1 } else { 0 }) {
        return val;
    }
    if let Some(val) = try_binary_op(trimmed, "<=", |a, b| if a <= b { 1 } else { 0 }) {
        return val;
    }
    if let Some(val) = try_binary_op(trimmed, ">=", |a, b| if a >= b { 1 } else { 0 }) {
        return val;
    }

    // Handle + and - (scan right-to-left for lowest precedence)
    if let Some(val) = try_binary_op_last(trimmed, '+', |a, b| a + b) {
        return val;
    }
    if let Some(val) = try_binary_op_last(trimmed, '-', |a, b| a - b) {
        return val;
    }

    // Handle *
    if let Some(val) = try_binary_op_last(trimmed, '*', |a, b| a * b) {
        return val;
    }

    // Parenthesized expression
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return eval_expr(&trimmed[1..trimmed.len() - 1]);
    }

    // Parse integer literal
    trimmed.parse::<i64>().unwrap_or(0)
}

/// Try splitting on a two-char operator (first occurrence).
fn try_binary_op(expr: &str, op: &str, f: fn(i64, i64) -> i64) -> Option<i64> {
    let pos = expr.find(op)?;
    let lhs = &expr[..pos];
    let rhs = &expr[pos + op.len()..];
    if lhs.is_empty() || rhs.is_empty() {
        return None;
    }
    Some(f(eval_expr(lhs), eval_expr(rhs)))
}

/// Try splitting on a single-char operator (last occurrence, for left-assoc).
fn try_binary_op_last(expr: &str, op: char, f: fn(i64, i64) -> i64) -> Option<i64> {
    // Scan right-to-left, skip inside parens
    let bytes = expr.as_bytes();
    let mut depth = 0;
    let mut pos = None;
    for i in (0..bytes.len()).rev() {
        match bytes[i] {
            b')' => depth += 1,
            b'(' => depth -= 1,
            c if c == op as u8 && depth == 0 && i > 0 => {
                // Don't split on unary minus (nothing to the left)
                let lhs = expr[..i].trim();
                if !lhs.is_empty()
                    && !lhs.ends_with('+')
                    && !lhs.ends_with('-')
                    && !lhs.ends_with('*')
                {
                    pos = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }
    let i = pos?;
    Some(f(eval_expr(&expr[..i]), eval_expr(&expr[i + 1..])))
}
