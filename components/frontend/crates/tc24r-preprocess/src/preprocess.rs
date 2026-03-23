use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::conditional::{self, CondStack};
use crate::func_macro::{self, FuncMacro};
use crate::include;
use crate::substitute;

/// Preprocess source text with optional include support.
///
/// - `source_dir`: directory for resolving `#include "..."` (None disables)
/// - `system_paths`: directories for resolving `#include <...>`
pub fn preprocess(source: &str, source_dir: Option<&Path>, system_paths: &[&Path]) -> String {
    let mut ctx = Context {
        defines: HashMap::new(),
        func_macros: HashMap::new(),
        included: HashSet::new(),
        source_dir: source_dir.map(Path::to_path_buf),
        system_paths: system_paths.iter().map(|p| p.to_path_buf()).collect(),
        cond_stack: CondStack::new(),
    };
    process_text(source, &mut ctx)
}

pub(crate) struct Context {
    pub defines: HashMap<String, String>,
    pub func_macros: HashMap<String, FuncMacro>,
    pub included: HashSet<std::path::PathBuf>,
    pub source_dir: Option<std::path::PathBuf>,
    pub system_paths: Vec<std::path::PathBuf>,
    pub cond_stack: CondStack,
}

pub(crate) fn process_text(source: &str, ctx: &mut Context) -> String {
    let mut output = String::new();
    for line in source.lines() {
        process_line(line, ctx, &mut output);
    }
    output
}

fn process_line(line: &str, ctx: &mut Context, output: &mut String) {
    let trimmed = line.trim_start();

    // Conditional directives are always processed, even in skipped blocks
    if let Some(rest) = strip_directive(trimmed, "#if ") {
        let cond = conditional::eval_condition(rest, &ctx.defines, &ctx.func_macros);
        ctx.cond_stack.push_if(cond);
        return;
    }
    if let Some(rest) = strip_directive(trimmed, "#ifdef ") {
        let name = rest.trim();
        let defined = ctx.defines.contains_key(name) || ctx.func_macros.contains_key(name);
        ctx.cond_stack.push_if(defined);
        return;
    }
    if let Some(rest) = strip_directive(trimmed, "#ifndef ") {
        let name = rest.trim();
        let defined = ctx.defines.contains_key(name) || ctx.func_macros.contains_key(name);
        ctx.cond_stack.push_if(!defined);
        return;
    }
    if let Some(rest) = strip_directive(trimmed, "#elif ") {
        let cond = conditional::eval_condition(rest, &ctx.defines, &ctx.func_macros);
        ctx.cond_stack.handle_elif(cond);
        return;
    }
    if trimmed_matches(trimmed, "#else") {
        ctx.cond_stack.handle_else();
        return;
    }
    if trimmed_matches(trimmed, "#endif") {
        ctx.cond_stack.pop();
        return;
    }

    // Skip all content in inactive conditional blocks
    if !ctx.cond_stack.is_active() {
        return;
    }

    // Regular directives (only processed when active)
    if let Some(rest) = trimmed.strip_prefix("#define ") {
        handle_define(rest, ctx);
    } else if let Some(rest) = strip_directive(trimmed, "#undef ") {
        let name = rest.trim();
        ctx.defines.remove(name);
        ctx.func_macros.remove(name);
    } else if trimmed.starts_with("#include ") {
        include::handle_include(trimmed, ctx, output);
    } else if trimmed == "#pragma once" {
        // Handled at include time -- no output
    } else if trimmed.starts_with('#') && !trimmed.starts_with("#define") {
        // Unknown directive (e.g. #line, # nnn "file") — skip silently
    } else {
        let expanded = substitute::expand_line(line, &ctx.defines, &ctx.func_macros);
        output.push_str(&expanded);
        output.push('\n');
    }
}

/// Strip a directive prefix, handling optional whitespace after `#`.
/// e.g. `#  if 1` matches prefix `#if ` and returns `1`.
fn strip_directive<'a>(trimmed: &'a str, prefix: &str) -> Option<&'a str> {
    // Try exact match first
    if let Some(rest) = trimmed.strip_prefix(prefix) {
        return Some(rest);
    }
    // Try with whitespace after #: "# if" or "#  if"
    if !trimmed.starts_with('#') {
        return None;
    }
    let after_hash = trimmed[1..].trim_start();
    let keyword = &prefix[1..]; // "if ", "ifdef ", etc.
    after_hash.strip_prefix(keyword)
}

/// Check if trimmed line matches a standalone directive (e.g. #else, #endif).
fn trimmed_matches(trimmed: &str, directive: &str) -> bool {
    if trimmed == directive {
        return true;
    }
    // Handle "# else", "#  endif", etc.
    if !trimmed.starts_with('#') {
        return false;
    }
    let after_hash = trimmed[1..].trim_start();
    let keyword = &directive[1..]; // "else", "endif"
    after_hash == keyword || after_hash.starts_with(&format!("{keyword} "))
}

fn handle_define(rest: &str, ctx: &mut Context) {
    if let Some((name, func)) = try_parse_func_define(rest) {
        ctx.func_macros.insert(name, func);
    } else if let Some((name, value)) = parse_simple_define(rest) {
        ctx.defines.insert(name, value);
    }
}

/// Try to parse `NAME(params) body` -- returns None if not function-like.
fn try_parse_func_define(rest: &str) -> Option<(String, FuncMacro)> {
    let paren_pos = rest.find('(')?;
    let name = &rest[..paren_pos];
    // Must be a valid identifier with no whitespace before `(`
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    let after_paren = &rest[paren_pos + 1..];
    let (params, body) = func_macro::parse_params_and_body(after_paren)?;
    Some((name.to_string(), FuncMacro { params, body }))
}

fn parse_simple_define(rest: &str) -> Option<(String, String)> {
    let mut parts = rest.splitn(2, |c: char| c.is_ascii_whitespace());
    let name = parts.next()?.to_string();
    let value = parts.next().unwrap_or("").trim().to_string();
    Some((name, value))
}
