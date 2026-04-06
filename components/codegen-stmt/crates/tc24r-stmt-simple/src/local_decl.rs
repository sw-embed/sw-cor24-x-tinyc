//! Local variable declaration code generation.

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::fp_store_word_r0;
use tc24r_type_infer::GenExprFn;

/// Generate code for a local variable declaration with optional initializer.
///
/// If the variable is missing from the locals map (e.g., inside a StmtExpr
/// whose scope was saved/restored), re-inserts it. The always-allocate
/// collect pass ensures there's enough frame space; we just need the
/// correct mapping.
pub fn gen_local_decl(
    state: &mut CodegenState,
    name: &str,
    ty: &Type,
    init: Option<&Expr>,
    gen_expr_fn: GenExprFn,
) {
    // Ensure the variable has a mapping. Inside StmtExpr scopes where
    // the outer map was saved/restored, the name may be missing.
    // The collect pass already reserved frame space via always-allocate,
    // so we allocate at the next available offset (within the frame).
    if !state.locals.contains_key(name) {
        let alloc = ty.size().max(3);
        state.locals_size += alloc;
        let offset = -state.locals_size;
        state.locals.insert(name.to_string(), offset);
        state.local_types.insert(name.to_string(), ty.clone());
    }

    if let Some(init_expr) = init {
        gen_expr_fn(init_expr, state);
        let offset = state.locals[name];
        fp_store_word_r0(state, offset);
    }
}
