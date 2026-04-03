//! Pre-pass to collect local variable declarations and allocate stack slots.

use tc24r_ast::{Expr, Stmt};
use tc24r_codegen_state::CodegenState;

/// Walk a block of statements, allocating stack slots for any local
/// variable declarations found (including nested scopes).
///
/// Uses a two-pass approach: first collects max sizes needed per variable
/// name across all scopes, then allocates slots.
pub fn collect_locals_block(state: &mut CodegenState, stmts: &[Stmt]) {
    for stmt in stmts {
        collect_locals_stmt(state, stmt);
    }
}

/// Allocate a stack slot for a local declaration, or recurse into
/// compound statements to find nested declarations.
pub fn collect_locals_stmt(state: &mut CodegenState, stmt: &Stmt) {
    match stmt {
        Stmt::LocalDecl { name, ty, init } => {
            let alloc = ty.size().max(3); // min 3 (word-aligned)
            if !state.locals.contains_key(name) || ty.size() > state.local_types[name].size() {
                // First declaration or redeclaration with a larger type.
                // Allocate (or re-allocate) a slot for the larger size.
                state.locals_size += alloc;
                let offset = -state.locals_size;
                state.locals.insert(name.clone(), offset);
                state.local_types.insert(name.clone(), ty.clone());
            }
            if let Some(expr) = init {
                scan_expr_locals(state, expr);
            }
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => {
            scan_expr_locals(state, cond);
            collect_locals_block(state, &then_body.stmts);
            if let Some(eb) = else_body {
                collect_locals_block(state, &eb.stmts);
            }
        }
        Stmt::While { cond, body } => {
            scan_expr_locals(state, cond);
            collect_locals_block(state, &body.stmts);
        }
        Stmt::DoWhile { body, cond } => {
            collect_locals_block(state, &body.stmts);
            scan_expr_locals(state, cond);
        }
        Stmt::For {
            init,
            cond,
            inc,
            body,
        } => {
            if let Some(init_stmt) = init {
                collect_locals_stmt(state, init_stmt);
            }
            if let Some(c) = cond {
                scan_expr_locals(state, c);
            }
            if let Some(i) = inc {
                scan_expr_locals(state, i);
            }
            collect_locals_block(state, &body.stmts);
        }
        Stmt::Switch {
            expr,
            cases,
            default,
        } => {
            scan_expr_locals(state, expr);
            for case in cases {
                for s in &case.stmts {
                    collect_locals_stmt(state, s);
                }
            }
            if let Some(stmts) = default {
                for s in stmts {
                    collect_locals_stmt(state, s);
                }
            }
        }
        Stmt::Block(block) => {
            collect_locals_block(state, &block.stmts);
        }
        Stmt::Expr(expr) | Stmt::Return(expr) => scan_expr_locals(state, expr),
        _ => {}
    }
}

/// Walk an expression tree looking for StmtExpr blocks that may
/// contain local declarations needing stack slots.
fn scan_expr_locals(state: &mut CodegenState, expr: &Expr) {
    match expr {
        Expr::StmtExpr(block) => collect_locals_block(state, &block.stmts),
        Expr::BinOp { lhs, rhs, .. } => {
            scan_expr_locals(state, lhs);
            scan_expr_locals(state, rhs);
        }
        Expr::UnaryOp { operand, .. } => scan_expr_locals(state, operand),
        Expr::Assign { value, .. } => scan_expr_locals(state, value),
        Expr::DerefAssign { ptr, value } => {
            scan_expr_locals(state, ptr);
            scan_expr_locals(state, value);
        }
        Expr::Deref(inner) | Expr::Cast { expr: inner, .. } => {
            scan_expr_locals(state, inner);
        }
        Expr::Call { args, .. } => {
            for a in args {
                scan_expr_locals(state, a);
            }
        }
        Expr::IndirectCall { callee, args } => {
            scan_expr_locals(state, callee);
            for a in args {
                scan_expr_locals(state, a);
            }
        }
        Expr::MemberAccess { object, .. } => scan_expr_locals(state, object),
        Expr::MemberAssign { object, value, .. } => {
            scan_expr_locals(state, object);
            scan_expr_locals(state, value);
        }
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => {
            scan_expr_locals(state, cond);
            scan_expr_locals(state, then_expr);
            scan_expr_locals(state, else_expr);
        }
        _ => {}
    }
}
