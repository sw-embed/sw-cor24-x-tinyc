//! Division and modulo via runtime helper calls.
//!
//! The COR24 ISA has no hardware divide, so div/mod are implemented as
//! runtime subroutines using repeated subtraction.

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Emit a call to a div/mod runtime helper.
/// Assumes r0=lhs (dividend), r1=rhs (divisor) already evaluated.
///
/// Pass `is_mod: false` for division, `true` for modulo.
pub fn gen_divmod_call(state: &mut CodegenState, is_mod: bool) {
    let label = if is_mod {
        state.needs_mod = true;
        "__tc24r_mod"
    } else {
        state.needs_div = true;
        "__tc24r_div"
    };
    emit!(state, "        push    r1");
    emit!(state, "        push    r0");
    emit!(state, "        la      r0,{label}");
    emit!(state, "        jal     r1,(r0)");
    emit!(state, "        add     sp,6");
}

/// Emit runtime subroutines if needed. Call at end of code generation.
pub fn emit_divmod_runtime(state: &mut CodegenState) {
    if !state.needs_div && !state.needs_mod {
        return;
    }
    state.out.push('\n');
    if state.needs_div {
        emit_div_routine(state);
    }
    if state.needs_mod {
        emit_mod_routine(state);
    }
}

/// Emit the __tc24r_div subroutine (repeated subtraction, returns quotient).
fn emit_div_routine(state: &mut CodegenState) {
    emit!(state, "__tc24r_div:");
    emit!(state, "        push    fp");
    emit!(state, "        push    r2");
    emit!(state, "        push    r1");
    emit!(state, "        mov     fp,sp");
    emit!(state, "        lw      r0,9(fp)");
    emit!(state, "        lw      r1,12(fp)");
    emit!(state, "        lc      r2,0");
    emit!(state, "__tc24r_div_lp:");
    emit!(state, "        cls     r0,r1");
    emit!(state, "        brt     __tc24r_div_dn");
    emit!(state, "        sub     r0,r1");
    emit!(state, "        add     r2,1");
    emit!(state, "        bra     __tc24r_div_lp");
    emit_runtime_epilogue(state, "__tc24r_div_dn", true);
}

/// Emit the __tc24r_mod subroutine (repeated subtraction, returns remainder).
fn emit_mod_routine(state: &mut CodegenState) {
    emit!(state, "__tc24r_mod:");
    emit!(state, "        push    fp");
    emit!(state, "        push    r2");
    emit!(state, "        push    r1");
    emit!(state, "        mov     fp,sp");
    emit!(state, "        lw      r0,9(fp)");
    emit!(state, "        lw      r1,12(fp)");
    emit!(state, "__tc24r_mod_lp:");
    emit!(state, "        cls     r0,r1");
    emit!(state, "        brt     __tc24r_mod_dn");
    emit!(state, "        sub     r0,r1");
    emit!(state, "        bra     __tc24r_mod_lp");
    emit_runtime_epilogue(state, "__tc24r_mod_dn", false);
}

/// Shared epilogue for div/mod: label, optional mov r0,r2, standard return.
fn emit_runtime_epilogue(state: &mut CodegenState, done_label: &str, is_div: bool) {
    emit!(state, "{done_label}:");
    if is_div {
        emit!(state, "        mov     r0,r2");
    }
    emit!(state, "        mov     sp,fp");
    emit!(state, "        pop     r1");
    emit!(state, "        pop     r2");
    emit!(state, "        pop     fp");
    emit!(state, "        jmp     (r1)");
}
