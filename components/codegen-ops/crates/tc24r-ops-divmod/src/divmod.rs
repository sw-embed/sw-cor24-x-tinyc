//! Division and modulo via runtime helper calls.
//!
//! The COR24 ISA has no hardware divide, so div/mod are implemented as
//! runtime subroutines using repeated subtraction with sign handling.
//! C99 truncation-toward-zero semantics:
//!   -7 /  3 = -2,  -7 %  3 = -1
//!    7 / -3 = -2,   7 % -3 =  1

use tc24r_codegen_state::CodegenState;
use tc24r_emit_macros::emit;

/// Emit a call to a div/mod runtime helper.
/// Assumes r0=lhs (dividend), r1=rhs (divisor) already evaluated.
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

/// Emit __tc24r_div: signed division with truncation toward zero.
/// Sign rule: quotient is negative iff exactly one operand is negative.
fn emit_div_routine(state: &mut CodegenState) {
    emit!(state, "__tc24r_div:");
    emit!(state, "        push    fp");
    emit!(state, "        push    r2");
    emit!(state, "        push    r1");
    emit!(state, "        mov     fp,sp");
    emit!(state, "        lw      r0,9(fp)");
    emit!(state, "        lw      r1,12(fp)");
    // r2 = count of negative operands (0, 1, or 2)
    emit!(state, "        lc      r2,0");
    // If dividend < 0: negate, increment count
    emit!(state, "        cls     r0,z");
    emit!(state, "        brf     __td_dp");
    emit_negate_r0_save_r1(state);
    emit!(state, "        add     r2,1");
    emit!(state, "__td_dp:");
    // If divisor < 0: negate, increment count
    emit!(state, "        cls     r1,z");
    emit!(state, "        brf     __td_rp");
    emit_negate_r1_save_r0(state);
    emit!(state, "        add     r2,1");
    emit!(state, "__td_rp:");
    // Save sign count, use r2 as quotient
    emit!(state, "        push    r2");
    emit!(state, "        lc      r2,0");
    // Unsigned repeated subtraction: |dividend| / |divisor|
    emit!(state, "__td_lp:");
    emit!(state, "        cls     r0,r1");
    emit!(state, "        brt     __td_dn");
    emit!(state, "        sub     r0,r1");
    emit!(state, "        add     r2,1");
    emit!(state, "        bra     __td_lp");
    // Done: r2 = quotient (positive)
    emit!(state, "__td_dn:");
    emit!(state, "        mov     r0,r2");
    emit!(state, "        pop     r2");
    // Negate if sign count is odd (r2 & 1)
    emit!(state, "        lc      r1,1");
    emit!(state, "        and     r2,r1");
    emit!(state, "        ceq     r2,z");
    emit!(state, "        brt     __td_ret");
    // Negate quotient: r0 = 0 - r0
    emit!(state, "        push    r0");
    emit!(state, "        lc      r0,0");
    emit!(state, "        pop     r1");
    emit!(state, "        sub     r0,r1");
    emit!(state, "__td_ret:");
    emit_epilogue(state);
}

/// Emit __tc24r_mod: signed modulo (remainder same sign as dividend).
fn emit_mod_routine(state: &mut CodegenState) {
    emit!(state, "__tc24r_mod:");
    emit!(state, "        push    fp");
    emit!(state, "        push    r2");
    emit!(state, "        push    r1");
    emit!(state, "        mov     fp,sp");
    emit!(state, "        lw      r0,9(fp)");
    emit!(state, "        lw      r1,12(fp)");
    // r2 = 1 if dividend was negative
    emit!(state, "        lc      r2,0");
    emit!(state, "        cls     r0,z");
    emit!(state, "        brf     __tm_dp");
    emit_negate_r0_save_r1(state);
    emit!(state, "        lc      r2,1");
    emit!(state, "__tm_dp:");
    // Negate divisor if negative (doesn't affect remainder sign)
    emit!(state, "        cls     r1,z");
    emit!(state, "        brf     __tm_rp");
    emit_negate_r1_save_r0(state);
    emit!(state, "__tm_rp:");
    emit!(state, "        push    r2");
    // Unsigned repeated subtraction
    emit!(state, "__tm_lp:");
    emit!(state, "        cls     r0,r1");
    emit!(state, "        brt     __tm_dn");
    emit!(state, "        sub     r0,r1");
    emit!(state, "        bra     __tm_lp");
    // Done: r0 = remainder (positive)
    emit!(state, "__tm_dn:");
    emit!(state, "        pop     r2");
    emit!(state, "        ceq     r2,z");
    emit!(state, "        brt     __tm_ret");
    // Negate remainder
    emit!(state, "        push    r0");
    emit!(state, "        lc      r0,0");
    emit!(state, "        pop     r1");
    emit!(state, "        sub     r0,r1");
    emit!(state, "__tm_ret:");
    emit_epilogue(state);
}

/// Negate r0 without clobbering r1.
fn emit_negate_r0_save_r1(state: &mut CodegenState) {
    emit!(state, "        push    r1");
    emit!(state, "        push    r0");
    emit!(state, "        lc      r0,0");
    emit!(state, "        pop     r1");
    emit!(state, "        sub     r0,r1");
    emit!(state, "        pop     r1");
}

/// Negate r1 without clobbering r0.
fn emit_negate_r1_save_r0(state: &mut CodegenState) {
    emit!(state, "        push    r0");
    emit!(state, "        lc      r0,0");
    emit!(state, "        sub     r0,r1");
    emit!(state, "        mov     r1,r0");
    emit!(state, "        pop     r0");
}

/// Standard subroutine epilogue.
fn emit_epilogue(state: &mut CodegenState) {
    emit!(state, "        mov     sp,fp");
    emit!(state, "        pop     r1");
    emit!(state, "        pop     r2");
    emit!(state, "        pop     fp");
    emit!(state, "        jmp     (r1)");
}
