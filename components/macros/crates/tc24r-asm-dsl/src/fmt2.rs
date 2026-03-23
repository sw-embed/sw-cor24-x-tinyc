//! More internal formatting helpers for the asm! macro.

use crate::Reg;

#[doc(hidden)]
pub fn _fmt_r1(out: &mut String, op: &str, a: Reg) {
    out.push_str(&format!("        {op}    {}\n", a.name()));
}

#[doc(hidden)]
pub fn _fmt_branch(out: &mut String, op: &str, label: &str) {
    out.push_str(&format!("        {op}     {label}\n"));
}

#[doc(hidden)]
pub fn _fmt_jal(out: &mut String, a: Reg, b: Reg) {
    out.push_str(&format!("        jal     {},({})\n", a.name(), b.name()));
}

#[doc(hidden)]
pub fn _fmt_jmp(out: &mut String, a: Reg) {
    out.push_str(&format!("        jmp     ({})\n", a.name()));
}

#[doc(hidden)]
pub fn _fmt_movc(out: &mut String, a: Reg) {
    out.push_str(&format!("        mov     {},c\n", a.name()));
}
