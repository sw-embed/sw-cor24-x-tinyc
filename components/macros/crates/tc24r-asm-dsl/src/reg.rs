//! COR24 register definitions.

/// A COR24 register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg {
    R0,
    R1,
    R2,
    Fp,
    Sp,
    Z,
    Iv,
    Ir,
}

impl Reg {
    /// Assembly text representation.
    pub fn name(self) -> &'static str {
        match self {
            Reg::R0 => "r0",
            Reg::R1 => "r1",
            Reg::R2 => "r2",
            Reg::Fp => "fp",
            Reg::Sp => "sp",
            Reg::Z => "z",
            Reg::Iv => "iv",
            Reg::Ir => "ir",
        }
    }
}

// Convenient constants for use in asm!() calls.
pub const R0: Reg = Reg::R0;
pub const R1: Reg = Reg::R1;
pub const R2: Reg = Reg::R2;
pub const FP: Reg = Reg::Fp;
pub const SP: Reg = Reg::Sp;
pub const Z: Reg = Reg::Z;
pub const IV: Reg = Reg::Iv;
pub const IR: Reg = Reg::Ir;
