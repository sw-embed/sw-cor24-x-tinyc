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
    // If `name` is a local or global variable (not a known function),
    // treat it as an indirect call through a function pointer variable.
    let is_variable = state.locals.contains_key(name) || state.globals.contains(name);
    let is_function = state.function_types.contains_key(name);
    if is_variable && !is_function {
        let callee = Expr::Ident(name.to_string());
        return gen_indirect_call(state, &callee, args, gen_expr_fn);
    }

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

/// Generate an indirect function call: push args right-to-left,
/// evaluate callee expression into r0, jal through r0.
pub fn gen_indirect_call(
    state: &mut CodegenState,
    callee: &Expr,
    args: &[Expr],
    gen_expr_fn: GenExprFn,
) {
    for arg in args.iter().rev() {
        gen_expr_fn(arg, state);
        emit!(state, "        push    r0");
    }
    gen_expr_fn(callee, state);
    emit!(state, "        jal     r1,(r0)");
    if !args.is_empty() {
        let cleanup = args.len() as i32 * 3;
        emit!(state, "        add     sp,{cleanup}");
    }
}
