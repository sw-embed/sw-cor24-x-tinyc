//! Function call code generation.

use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Generate a function call: push args right-to-left, jal, cleanup stack.
pub fn gen_call(state: &mut CodegenState, name: &str, args: &[Expr], gen_expr_fn: GenExprFn) {
    for arg in args.iter().rev() {
        gen_expr_fn(arg, state);
        emit!(state, "        push    r0");
    }
    emit!(state, "        la      r0,_{name}");
    emit!(state, "        jal     r1,(r0)");
    if !args.is_empty() {
        let cleanup = args.len() as i32 * 3;
        emit!(state, "        add     sp,{cleanup}");
    }
}
