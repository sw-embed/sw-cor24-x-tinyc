use std::collections::{HashMap, HashSet};
use std::path::Path;

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
    };
    process_text(source, &mut ctx)
}

pub(crate) struct Context {
    pub defines: HashMap<String, String>,
    pub func_macros: HashMap<String, FuncMacro>,
    pub included: HashSet<std::path::PathBuf>,
    pub source_dir: Option<std::path::PathBuf>,
    pub system_paths: Vec<std::path::PathBuf>,
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
    if let Some(rest) = trimmed.strip_prefix("#define ") {
        handle_define(rest, ctx);
    } else if trimmed.starts_with("#include ") {
        include::handle_include(trimmed, ctx, output);
    } else if trimmed == "#pragma once" {
        // Handled at include time -- no output
    } else {
        let expanded = substitute::expand_line(line, &ctx.defines, &ctx.func_macros);
        output.push_str(&expanded);
        output.push('\n');
    }
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
