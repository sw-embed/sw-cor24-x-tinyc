//! Whole-program dead-code elimination for tc24r.
//!
//! tc24r is single-translation-unit: by the time the parser returns
//! a `Program`, every function the program could possibly call is
//! already in `Program.functions`. That includes everything pulled
//! in from `<stdio.h>`, `<stdlib.h>`, `<string.h>`, etc., whether
//! the user actually calls it or not.
//!
//! `dce(&mut program)` filters `Program.functions` to just the set
//! reachable (directly or transitively) from a small set of roots:
//!
//! - `main`
//! - any function with `is_interrupt: true`
//! - any function name appearing in any global initializer
//! - any function whose name appears in an inline `asm("...")`
//!   string anywhere in a reachable function (asm-form: leading
//!   underscore — `foo` in C is `_foo` in asm)
//!
//! Roots and their transitive callees are kept; everything else is
//! dropped before codegen.

use std::collections::HashSet;

use tc24r_ast::{Block, Expr, Program, Stmt};

/// Filter `program.functions` to only those reachable from program
/// roots (`main`, interrupt handlers, globals' init expressions,
/// and inline-asm symbol references).
pub fn dce(program: &mut Program) {
    let func_names: HashSet<String> = program.functions.iter().map(|f| f.name.clone()).collect();

    let mut reachable: HashSet<String> = HashSet::new();
    let mut worklist: Vec<String> = Vec::new();

    // Roots: main and any interrupt handler.
    if func_names.contains("main") {
        worklist.push("main".to_string());
    }
    for f in &program.functions {
        if f.is_interrupt {
            worklist.push(f.name.clone());
        }
    }

    // Roots: anything referenced from a global's initializer.
    for g in &program.globals {
        if let Some(init) = &g.init {
            collect_refs(init, &func_names, &mut worklist);
        }
    }

    // BFS over the call graph. A function name may appear more than
    // once in `program.functions` — typically a prototype (body:None)
    // followed by the definition. Walk every body that matches the
    // reachable name, not just the first.
    while let Some(name) = worklist.pop() {
        if !reachable.insert(name.clone()) {
            continue;
        }
        for f in program.functions.iter().filter(|f| f.name == name) {
            if let Some(body) = &f.body {
                collect_block_refs(body, &func_names, &mut worklist);
            }
        }
    }

    program.functions.retain(|f| reachable.contains(&f.name));
}

/// Walk a `Block`'s statements collecting function-name references.
fn collect_block_refs(block: &Block, funcs: &HashSet<String>, out: &mut Vec<String>) {
    for s in &block.stmts {
        collect_stmt_refs(s, funcs, out);
    }
}

fn collect_stmt_refs(stmt: &Stmt, funcs: &HashSet<String>, out: &mut Vec<String>) {
    match stmt {
        Stmt::Return(e) | Stmt::Expr(e) => collect_refs(e, funcs, out),
        Stmt::LocalDecl { init, .. } => {
            if let Some(e) = init {
                collect_refs(e, funcs, out);
            }
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            collect_refs(cond, funcs, out);
            collect_block_refs(then_body, funcs, out);
            if let Some(eb) = else_body {
                collect_block_refs(eb, funcs, out);
            }
        }
        Stmt::While { cond, body } | Stmt::DoWhile { body, cond } => {
            collect_refs(cond, funcs, out);
            collect_block_refs(body, funcs, out);
        }
        Stmt::For {
            init,
            cond,
            inc,
            body,
        } => {
            if let Some(s) = init {
                collect_stmt_refs(s, funcs, out);
            }
            if let Some(e) = cond {
                collect_refs(e, funcs, out);
            }
            if let Some(e) = inc {
                collect_refs(e, funcs, out);
            }
            collect_block_refs(body, funcs, out);
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            collect_refs(expr, funcs, out);
            for c in cases {
                collect_refs(&c.value, funcs, out);
                for s in &c.stmts {
                    collect_stmt_refs(s, funcs, out);
                }
            }
            if let Some(d) = default {
                for s in d {
                    collect_stmt_refs(s, funcs, out);
                }
            }
        }
        Stmt::Asm(s) => collect_asm_refs(s, funcs, out),
        Stmt::Block(b) => collect_block_refs(b, funcs, out),
        Stmt::Break | Stmt::Continue | Stmt::Goto(_) | Stmt::Label(_) => {}
    }
}

fn collect_refs(expr: &Expr, funcs: &HashSet<String>, out: &mut Vec<String>) {
    match expr {
        Expr::IntLit(_) | Expr::StringLit(_) => {}
        Expr::Ident(name) => {
            if funcs.contains(name) {
                out.push(name.clone());
            }
        }
        Expr::Assign { value, .. } => collect_refs(value, funcs, out),
        Expr::Call { name, args } => {
            // Mirror the codegen variadic rewrite for printf/sprintf
            // (see components/codegen-expr/.../call.rs:25-28). The
            // C source calls `printf(fmt, a, b, c)` but codegen emits
            // a call to `__tc24r_printfN` where N is the count of
            // extra args after `fmt`. The shim functions live in
            // <stdio.h> and are normal entries in program.functions,
            // so we have to mark the right N as reachable.
            if name == "printf" || name == "sprintf" {
                let extra = args.len().saturating_sub(1);
                let target = format!("__tc24r_printf{extra}");
                if funcs.contains(&target) {
                    out.push(target);
                }
            } else if funcs.contains(name) {
                out.push(name.clone());
            }
            for a in args {
                collect_refs(a, funcs, out);
            }
        }
        Expr::IndirectCall { callee, args } => {
            collect_refs(callee, funcs, out);
            for a in args {
                collect_refs(a, funcs, out);
            }
        }
        Expr::BinOp { lhs, rhs, .. } => {
            collect_refs(lhs, funcs, out);
            collect_refs(rhs, funcs, out);
        }
        Expr::UnaryOp { operand, .. } => collect_refs(operand, funcs, out),
        Expr::AddrOf(e) | Expr::Deref(e) => collect_refs(e, funcs, out),
        Expr::Cast { expr, .. } => collect_refs(expr, funcs, out),
        Expr::DerefAssign { ptr, value } => {
            collect_refs(ptr, funcs, out);
            collect_refs(value, funcs, out);
        }
        Expr::PreInc(e) | Expr::PreDec(e) | Expr::PostInc(e) | Expr::PostDec(e) => {
            collect_refs(e, funcs, out);
        }
        Expr::MemberAccess { object, .. } => collect_refs(object, funcs, out),
        Expr::MemberAssign { object, value, .. } => {
            collect_refs(object, funcs, out);
            collect_refs(value, funcs, out);
        }
        Expr::StmtExpr(b) => collect_block_refs(b, funcs, out),
        Expr::SizeofExpr(e) => collect_refs(e, funcs, out),
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => {
            collect_refs(cond, funcs, out);
            collect_refs(then_expr, funcs, out);
            collect_refs(else_expr, funcs, out);
        }
        Expr::InitList(items) => {
            for i in items {
                collect_refs(i, funcs, out);
            }
        }
    }
}

/// Scan an inline-asm string for asm-form symbol references that
/// match known C function names. The asm form of C function `foo`
/// is `_foo` (single leading underscore).
///
/// This is conservative: any token that *could* be an asm-name of
/// a known function is treated as a reference. False positives
/// keep an extra function (no correctness loss); the only failure
/// mode would be a false negative — missing a real reference and
/// dropping a function the asm actually uses. Since the scan
/// matches substrings preceded by a non-symbol character, that
/// case is unlikely.
fn collect_asm_refs(asm: &str, funcs: &HashSet<String>, out: &mut Vec<String>) {
    let bytes = asm.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // Look for an underscore that starts an asm symbol token.
        // A symbol starts at position i if bytes[i] == b'_' and
        // either i == 0 or the preceding byte is not symbol-y.
        if bytes[i] == b'_' && (i == 0 || !is_sym_byte(bytes[i - 1])) {
            let start = i + 1; // skip the leading underscore
            let mut end = start;
            while end < bytes.len() && is_sym_byte(bytes[end]) {
                end += 1;
            }
            if end > start {
                let candidate = &asm[start..end];
                if funcs.contains(candidate) {
                    out.push(candidate.to_string());
                }
            }
            i = end;
        } else {
            i += 1;
        }
    }
}

fn is_sym_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
