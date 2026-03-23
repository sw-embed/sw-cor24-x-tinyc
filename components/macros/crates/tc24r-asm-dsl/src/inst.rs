//! Instruction emission macros.
//!
//! The `asm!` macro emits a single COR24 instruction. The `label!`
//! and `dir!` macros handle labels and assembler directives.
//! All operate on any struct with a `pub out: String` field.

/// Emit a single COR24 instruction.
///
/// Supports register-register, register-immediate, memory, branch,
/// jump, and stack forms. See module-level docs for complete examples.
///
/// # Examples
///
/// ```rust
/// # use tc24r_asm_dsl::*;
/// # struct S { pub out: String }
/// # let mut s = S { out: String::new() };
/// asm!(s, add(R0, R1));
/// asm!(s, lc(R0, 42));
/// asm!(s, lw(R0, mem(FP, -3)));
/// asm!(s, push(FP));
/// asm!(s, bra("L0"));
/// # assert!(s.out.contains("add     r0,r1"));
/// # assert!(s.out.contains("lc      r0,42"));
/// # assert!(s.out.contains("lw      r0,-3(fp)"));
/// # assert!(s.out.contains("push    fp"));
/// # assert!(s.out.contains("bra     L0"));
/// ```
#[macro_export]
macro_rules! asm {
    // Register-register: op ra,rb
    ($s:expr, add($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "add", $a, $b)
    };
    ($s:expr, sub($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "sub", $a, $b)
    };
    ($s:expr, mul($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "mul", $a, $b)
    };
    ($s:expr, and($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "and", $a, $b)
    };
    ($s:expr, or($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "or ", $a, $b)
    };
    ($s:expr, xor($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "xor", $a, $b)
    };
    ($s:expr, shl($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "shl", $a, $b)
    };
    ($s:expr, srl($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "srl", $a, $b)
    };
    ($s:expr, ceq($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "ceq", $a, $b)
    };
    ($s:expr, cls($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "cls", $a, $b)
    };
    ($s:expr, mov($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "mov", $a, $b)
    };
    ($s:expr, clu($a:expr, $b:expr)) => {
        $crate::_fmt_rr(&mut $s.out, "clu", $a, $b)
    };

    // Register-immediate: lc ra,imm  /  add ra,imm
    ($s:expr, lc($a:expr, $imm:expr)) => {
        $crate::_fmt_ri(&mut $s.out, "lc ", $a, $imm)
    };
    ($s:expr, addi($a:expr, $imm:expr)) => {
        $crate::_fmt_ri(&mut $s.out, "add", $a, $imm)
    };

    // Load address: la ra,label
    ($s:expr, la($a:expr, $label:expr)) => {
        $crate::_fmt_rl(&mut $s.out, "la ", $a, $label)
    };

    // Memory: lw/sw/lbu/sb ra,off(rb)
    ($s:expr, lw($a:expr, mem($b:expr, $off:expr))) => {
        $crate::_fmt_mem(&mut $s.out, "lw ", $a, $b, $off)
    };
    ($s:expr, lbu($a:expr, mem($b:expr, $off:expr))) => {
        $crate::_fmt_mem(&mut $s.out, "lbu", $a, $b, $off)
    };
    ($s:expr, sw($a:expr, mem($b:expr, $off:expr))) => {
        $crate::_fmt_mem(&mut $s.out, "sw ", $a, $b, $off)
    };
    ($s:expr, sb($a:expr, mem($b:expr, $off:expr))) => {
        $crate::_fmt_mem(&mut $s.out, "sb ", $a, $b, $off)
    };

    // Stack: push/pop ra
    ($s:expr, push($a:expr)) => {
        $crate::_fmt_r1(&mut $s.out, "push", $a)
    };
    ($s:expr, pop($a:expr)) => {
        $crate::_fmt_r1(&mut $s.out, "pop ", $a)
    };

    // Branch: bra/brt/brf label
    ($s:expr, bra($label:expr)) => {
        $crate::_fmt_branch(&mut $s.out, "bra", $label)
    };
    ($s:expr, brt($label:expr)) => {
        $crate::_fmt_branch(&mut $s.out, "brt", $label)
    };
    ($s:expr, brf($label:expr)) => {
        $crate::_fmt_branch(&mut $s.out, "brf", $label)
    };

    // Jump indirect: jal ra,(rb)  /  jmp (ra)
    ($s:expr, jal($a:expr, ind($b:expr))) => {
        $crate::_fmt_jal(&mut $s.out, $a, $b)
    };
    ($s:expr, jmp(ind($a:expr))) => {
        $crate::_fmt_jmp(&mut $s.out, $a)
    };

    // Special: mov ra,c (read condition flag -- c is not a Reg)
    ($s:expr, movc($a:expr)) => {
        $crate::_fmt_movc(&mut $s.out, $a)
    };
}

/// Emit a label definition (no indentation).
///
/// ```rust
/// # use tc24r_asm_dsl::*;
/// # struct S { pub out: String }
/// # let mut s = S { out: String::new() };
/// label!(s, "L0");
/// # assert_eq!(s.out, "L0:\n");
/// ```
#[macro_export]
macro_rules! label {
    ($s:expr, $name:expr) => {
        $s.out.push_str(&format!("{}:", $name));
        $s.out.push('\n');
    };
}

/// Emit an assembler directive.
///
/// ```rust
/// # use tc24r_asm_dsl::*;
/// # struct S { pub out: String }
/// # let mut s = S { out: String::new() };
/// dir!(s, globl("_main"));
/// dir!(s, text);
/// dir!(s, data);
/// dir!(s, word(42));
/// dir!(s, byte_list(&[72, 101, 0]));
/// # assert!(s.out.contains(".globl  _main"));
/// ```
#[macro_export]
macro_rules! dir {
    ($s:expr, globl($name:expr)) => {
        $s.out.push_str(&format!("        .globl  {}", $name));
        $s.out.push('\n');
    };
    ($s:expr, text) => {
        $s.out.push_str("        .text\n");
    };
    ($s:expr, data) => {
        $s.out.push_str("        .data\n");
    };
    ($s:expr, word($val:expr)) => {
        $s.out.push_str(&format!("        .word   {}", $val));
        $s.out.push('\n');
    };
    ($s:expr, byte($val:expr)) => {
        $s.out.push_str(&format!("        .byte   {}", $val));
        $s.out.push('\n');
    };
    ($s:expr, byte_list($bytes:expr)) => {{
        let list: Vec<String> = $bytes.iter().map(|b: &u8| b.to_string()).collect();
        $s.out
            .push_str(&format!("        .byte   {}", list.join(",")));
        $s.out.push('\n');
    }};
}

// Formatting helpers are in fmt.rs and fmt2.rs, re-exported from lib.rs.
