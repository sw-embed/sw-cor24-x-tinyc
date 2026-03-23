//! Type-safe COR24 assembly DSL.
//!
//! Replaces raw string emission with compile-time-checked instruction
//! construction. Catches register name typos, invalid addressing modes,
//! and instruction format errors at compile time.
//!
//! # Usage
//!
//! ```rust
//! use tc24r_asm_dsl::*;
//!
//! struct S { pub out: String }
//! let mut s = S { out: String::new() };
//!
//! // Register-register operations:
//! asm!(s, add(R0, R1));
//! asm!(s, mov(FP, SP));
//!
//! // Immediate operations:
//! asm!(s, lc(R0, 42));
//! asm!(s, addi(SP, -6));
//!
//! // Memory operations with offset(base):
//! asm!(s, lw(R0, mem(FP, -3)));
//! asm!(s, sw(R0, mem(FP, -6)));
//! asm!(s, sb(R0, mem(R1, 0)));
//!
//! // Branch and jump:
//! asm!(s, bra("L0"));
//! asm!(s, la(R0, "_main"));
//! asm!(s, jal(R1, ind(R0)));
//! asm!(s, jmp(ind(R1)));
//!
//! // Labels and directives:
//! label!(s, "L0");
//! dir!(s, globl("_main"));
//! dir!(s, text);
//! dir!(s, data);
//! dir!(s, word(42));
//! dir!(s, byte_list(&[72, 101, 0]));
//! ```

mod fmt;
mod fmt2;
mod inst;
mod reg;

pub use reg::{FP, IR, IV, R0, R1, R2, Reg, SP, Z};

// Re-export internal formatting helpers used by asm!/label!/dir! macros.
#[doc(hidden)]
pub use fmt::{_fmt_mem, _fmt_ri, _fmt_rl, _fmt_rr};
#[doc(hidden)]
pub use fmt2::{_fmt_branch, _fmt_jal, _fmt_jmp, _fmt_movc, _fmt_r1};

/// Sentinel constant for `mov(R0, C)` -- reads the condition flag.
/// Used as: `asm!(s, movc(R0))` to emit `mov r0,c`.
pub const C: () = ();
