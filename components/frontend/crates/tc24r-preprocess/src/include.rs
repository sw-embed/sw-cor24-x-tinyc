//! #include directive handling.

use std::path::PathBuf;

use crate::{process_text, Context};

/// Handle a `#include "file"` or `#include <file>` directive.
/// Macro-expands the argument first so `#include M` works when M is a macro.
pub(crate) fn handle_include(line: &str, ctx: &mut Context, output: &mut String) {
    let raw = line.strip_prefix("#include ").unwrap().trim();
    // Expand macros in the include argument (C99 §6.10.2)
    let rest_owned = crate::substitute::expand_line(raw, &ctx.defines, &ctx.func_macros);
    let rest = rest_owned.trim();
    let path = if rest.starts_with('"') {
        resolve_quote(rest, ctx)
    } else if rest.starts_with('<') {
        resolve_angle(rest, ctx)
    } else {
        None
    };

    let Some(path) = path else {
        eprintln!("tc24r: cannot resolve include: {rest}");
        return;
    };

    include_file(&path, ctx, output);
}

fn include_file(path: &PathBuf, ctx: &mut Context, output: &mut String) {
    // Check #pragma once
    let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.clone());
    if ctx.included.contains(&canonical) {
        return;
    }

    let content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("tc24r: cannot read {}: {e}", path.display());
            return;
        }
    };

    // Check for #pragma once in the file
    if has_pragma_once(&content) {
        ctx.included.insert(canonical);
    }

    let expanded = process_text(&content, ctx);
    output.push_str(&expanded);
}

fn has_pragma_once(source: &str) -> bool {
    source.lines().any(|l| l.trim() == "#pragma once")
}

fn resolve_quote(rest: &str, ctx: &Context) -> Option<PathBuf> {
    let name = rest.trim_matches('"');
    if let Some(dir) = &ctx.source_dir {
        let path = dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    // Fall back to system paths
    resolve_in_system(name, ctx)
}

fn resolve_angle(rest: &str, ctx: &Context) -> Option<PathBuf> {
    let name = rest.strip_prefix('<')?.strip_suffix('>')?.trim();
    resolve_in_system(name, ctx)
}

fn resolve_in_system(name: &str, ctx: &Context) -> Option<PathBuf> {
    for dir in &ctx.system_paths {
        let path = dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}
