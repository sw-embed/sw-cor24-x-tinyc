//! Function call code generation.

use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Callback type for recursive expression generation.
pub type GenExprFn = fn(&Expr, &mut CodegenState);

/// Generate a function call: push args right-to-left, jal, cleanup stack.
///
/// For `printf` and `sprintf` calls, dispatches to `__tc24r_printfN` where
/// N is the number of extra args (0-6), enabling freestanding printf without
/// true varargs support.
pub fn gen_call(state: &mut CodegenState, name: &str, args: &[Expr], gen_expr_fn: GenExprFn) {
    let call_name = if name == "printf" || name == "sprintf" {
        let extra = if args.is_empty() { 0 } else { args.len() - 1 };
        state.needs_printf = true;
        format!("___tc24r_printf{extra}")
    } else {
        format!("_{name}")
    };

    for arg in args.iter().rev() {
        gen_expr_fn(arg, state);
        emit!(state, "        push    r0");
    }
    emit!(state, "        la      r0,{call_name}");
    emit!(state, "        jal     r1,(r0)");
    if !args.is_empty() {
        let cleanup = args.len() as i32 * 3;
        emit!(state, "        add     sp,{cleanup}");
    }
}
