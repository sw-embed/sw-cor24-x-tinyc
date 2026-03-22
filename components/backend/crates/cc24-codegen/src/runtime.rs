//! Runtime helpers: ISR, divmod, and short-circuit logical operators.

use cc24_ast::Expr;

use crate::Codegen;

pub(crate) fn emit_isr_prologue(cg: &mut Codegen) {
    cg.emit("        push    r0");
    cg.emit("        push    r1");
    cg.emit("        push    r2");
    cg.emit("        mov     r2,c"); // save condition flag
    cg.emit("        push    r2");
    cg.emit("        push    fp");
    cg.emit("        mov     fp,sp");
    cg.emit_locals_alloc();
}

pub(crate) fn emit_isr_epilogue(cg: &mut Codegen) {
    let ret_label = cg.return_label.clone();
    cg.emit(&format!("{ret_label}:"));
    cg.emit("        mov     sp,fp");
    cg.emit("        pop     fp");
    cg.emit("        pop     r2");
    cg.emit("        clu     z,r2"); // restore condition flag
    cg.emit("        pop     r2");
    cg.emit("        pop     r1");
    cg.emit("        pop     r0");
    cg.emit("        jmp     (ir)"); // interrupt return
}

/// Emit runtime helper routines (div, mod) if needed.
pub(crate) fn emit_runtime(cg: &mut Codegen) {
    if cg.needs_div || cg.needs_mod {
        cg.emit("");
        emit_divmod(cg);
    }
}

/// Short-circuit &&: if LHS is 0, result is 0 without evaluating RHS.
pub(crate) fn gen_log_and(cg: &mut Codegen, lhs: &Expr, rhs: &Expr) {
    let false_label = cg.new_label();
    let done_label = cg.new_label();
    cg.gen_expr(lhs);
    cg.emit("        ceq     r0,z");
    cg.emit(&format!("        brt     {false_label}"));
    cg.gen_expr(rhs);
    cg.emit("        ceq     r0,z");
    cg.emit(&format!("        brt     {false_label}"));
    cg.emit("        lc      r0,1");
    cg.emit(&format!("        bra     {done_label}"));
    cg.emit(&format!("{false_label}:"));
    cg.emit("        lc      r0,0");
    cg.emit(&format!("{done_label}:"));
}

/// Short-circuit ||: if LHS is nonzero, result is 1 without evaluating RHS.
pub(crate) fn gen_log_or(cg: &mut Codegen, lhs: &Expr, rhs: &Expr) {
    let true_label = cg.new_label();
    let done_label = cg.new_label();
    cg.gen_expr(lhs);
    cg.emit("        ceq     r0,z");
    cg.emit(&format!("        brf     {true_label}"));
    cg.gen_expr(rhs);
    cg.emit("        ceq     r0,z");
    cg.emit(&format!("        brf     {true_label}"));
    cg.emit("        lc      r0,0");
    cg.emit(&format!("        bra     {done_label}"));
    cg.emit(&format!("{true_label}:"));
    cg.emit("        lc      r0,1");
    cg.emit(&format!("{done_label}:"));
}

/// Generate do { body } while (cond).
pub(crate) fn gen_do_while(cg: &mut Codegen, body: &cc24_ast::Block, cond: &Expr) {
    let loop_label = cg.new_label();
    cg.emit(&format!("{loop_label}:"));
    for s in &body.stmts {
        cg.gen_stmt(s);
    }
    cg.gen_expr(cond);
    cg.emit("        ceq     r0,z");
    cg.emit(&format!("        brf     {loop_label}"));
}

fn emit_divmod(cg: &mut Codegen) {
    // Shared loop: r0=dividend, r1=divisor -> r0=remainder, r2=quotient
    // Called with args on stack: 9(fp)=dividend, 12(fp)=divisor

    if cg.needs_div {
        cg.emit("__cc24_div:");
        cg.emit("        push    fp");
        cg.emit("        push    r2");
        cg.emit("        push    r1");
        cg.emit("        mov     fp,sp");
        cg.emit("        lw      r0,9(fp)");
        cg.emit("        lw      r1,12(fp)");
        cg.emit("        lc      r2,0");
        cg.emit("__cc24_div_lp:");
        cg.emit("        cls     r0,r1");
        cg.emit("        brt     __cc24_div_dn");
        cg.emit("        sub     r0,r1");
        cg.emit("        add     r2,1");
        cg.emit("        bra     __cc24_div_lp");
        cg.emit("__cc24_div_dn:");
        cg.emit("        mov     r0,r2"); // return quotient
        cg.emit("        mov     sp,fp");
        cg.emit("        pop     r1");
        cg.emit("        pop     r2");
        cg.emit("        pop     fp");
        cg.emit("        jmp     (r1)");
    }

    if cg.needs_mod {
        cg.emit("__cc24_mod:");
        cg.emit("        push    fp");
        cg.emit("        push    r2");
        cg.emit("        push    r1");
        cg.emit("        mov     fp,sp");
        cg.emit("        lw      r0,9(fp)");
        cg.emit("        lw      r1,12(fp)");
        cg.emit("__cc24_mod_lp:");
        cg.emit("        cls     r0,r1");
        cg.emit("        brt     __cc24_mod_dn");
        cg.emit("        sub     r0,r1");
        cg.emit("        bra     __cc24_mod_lp");
        cg.emit("__cc24_mod_dn:");
        // r0 already holds remainder
        cg.emit("        mov     sp,fp");
        cg.emit("        pop     r1");
        cg.emit("        pop     r2");
        cg.emit("        pop     fp");
        cg.emit("        jmp     (r1)");
    }
}
