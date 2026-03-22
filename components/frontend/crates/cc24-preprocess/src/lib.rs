//! Simple C preprocessor for cc24.
//!
//! Supports:
//! - `#define NAME value` constant substitution
//! - `#include "file.h"` (relative to source directory)
//! - `#include <file.h>` (search system include paths)
//! - `#pragma once` (skip file if already included)

mod include;
mod substitute;

use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Preprocess source text with optional include support.
///
/// - `source_dir`: directory for resolving `#include "..."` (None disables)
/// - `system_paths`: directories for resolving `#include <...>`
pub fn preprocess(source: &str, source_dir: Option<&Path>, system_paths: &[&Path]) -> String {
    let mut ctx = Context {
        defines: HashMap::new(),
        included: HashSet::new(),
        source_dir: source_dir.map(Path::to_path_buf),
        system_paths: system_paths.iter().map(|p| p.to_path_buf()).collect(),
    };
    process_text(source, &mut ctx)
}

pub(crate) struct Context {
    pub defines: HashMap<String, String>,
    pub included: HashSet<std::path::PathBuf>,
    pub source_dir: Option<std::path::PathBuf>,
    pub system_paths: Vec<std::path::PathBuf>,
}

fn process_text(source: &str, ctx: &mut Context) -> String {
    let mut output = String::new();
    for line in source.lines() {
        process_line(line, ctx, &mut output);
    }
    output
}

fn process_line(line: &str, ctx: &mut Context, output: &mut String) {
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed.strip_prefix("#define ") {
        if let Some((name, value)) = parse_define(rest) {
            ctx.defines.insert(name, value);
        }
    } else if trimmed.starts_with("#include ") {
        include::handle_include(trimmed, ctx, output);
    } else if trimmed == "#pragma once" {
        // Handled at include time -- no output
    } else {
        let expanded = substitute::expand_line(line, &ctx.defines);
        output.push_str(&expanded);
        output.push('\n');
    }
}

fn parse_define(rest: &str) -> Option<(String, String)> {
    let mut parts = rest.splitn(2, |c: char| c.is_ascii_whitespace());
    let name = parts.next()?.to_string();
    let value = parts.next().unwrap_or("").trim().to_string();
    Some((name, value))
}
