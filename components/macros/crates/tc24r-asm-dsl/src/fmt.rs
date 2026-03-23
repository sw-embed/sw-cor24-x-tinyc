//! Internal formatting helpers for the asm! macro.
//!
//! These are doc-hidden implementation details. Users should use
//! the asm!, label!, and dir! macros instead.

use crate::Reg;

#[doc(hidden)]
pub fn _fmt_rr(out: &mut String, op: &str, a: Reg, b: Reg) {
    out.push_str(&format!("        {op}     {},{}\n", a.name(), b.name()));
}

#[doc(hidden)]
pub fn _fmt_ri(out: &mut String, op: &str, a: Reg, imm: i32) {
    out.push_str(&format!("        {op}     {},{imm}\n", a.name()));
}

#[doc(hidden)]
pub fn _fmt_rl(out: &mut String, op: &str, a: Reg, label: &str) {
    out.push_str(&format!("        {op}     {},{label}\n", a.name()));
}

#[doc(hidden)]
pub fn _fmt_mem(out: &mut String, op: &str, a: Reg, b: Reg, off: i32) {
    out.push_str(&format!(
        "        {op}     {},{off}({})\n",
        a.name(),
        b.name()
    ));
}
