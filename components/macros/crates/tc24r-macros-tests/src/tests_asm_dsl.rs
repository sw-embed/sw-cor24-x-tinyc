//! Tests for the COR24 assembly DSL.

use tc24r_asm_dsl::*;

// Helper struct with pub out field, matching CodegenState pattern.
struct S {
    pub out: String,
}

#[test]
fn asm_register_register() {
    let mut s = S { out: String::new() };
    asm!(s, add(R0, R1));
    asm!(s, sub(R0, R1));
    asm!(s, mov(FP, SP));
    assert!(s.out.contains("        add     r0,r1\n"));
    assert!(s.out.contains("        sub     r0,r1\n"));
    assert!(s.out.contains("        mov     fp,sp\n"));
}

#[test]
fn asm_register_immediate() {
    let mut s = S { out: String::new() };
    asm!(s, lc(R0, 42));
    asm!(s, addi(SP, -6));
    assert!(s.out.contains("        lc      r0,42\n"));
    assert!(s.out.contains("        add     sp,-6\n"));
}

#[test]
fn asm_memory_operations() {
    let mut s = S { out: String::new() };
    asm!(s, lw(R0, mem(FP, -3)));
    asm!(s, sw(R0, mem(FP, -6)));
    asm!(s, lbu(R0, mem(R1, 0)));
    asm!(s, sb(R0, mem(R1, 0)));
    assert!(s.out.contains("        lw      r0,-3(fp)\n"));
    assert!(s.out.contains("        sw      r0,-6(fp)\n"));
    assert!(s.out.contains("        lbu     r0,0(r1)\n"));
    assert!(s.out.contains("        sb      r0,0(r1)\n"));
}

#[test]
fn asm_stack_operations() {
    let mut s = S { out: String::new() };
    asm!(s, push(FP));
    asm!(s, push(R0));
    asm!(s, pop(R1));
    assert!(s.out.contains("        push    fp\n"));
    assert!(s.out.contains("        push    r0\n"));
    assert!(s.out.contains("        pop     r1\n"));
}

#[test]
fn asm_branch_and_jump() {
    let mut s = S { out: String::new() };
    asm!(s, bra("L0"));
    asm!(s, brt("L1"));
    asm!(s, brf("L2"));
    asm!(s, la(R0, "_main"));
    asm!(s, jal(R1, ind(R0)));
    asm!(s, jmp(ind(R1)));
    assert!(s.out.contains("        bra     L0\n"));
    assert!(s.out.contains("        la      r0,_main\n"));
    assert!(s.out.contains("        jal     r1,(r0)\n"));
    assert!(s.out.contains("        jmp     (r1)\n"));
}

#[test]
fn asm_special_forms() {
    let mut s = S { out: String::new() };
    asm!(s, ceq(R0, Z));
    asm!(s, movc(R0));
    assert!(s.out.contains("        ceq     r0,z\n"));
    assert!(s.out.contains("        mov     r0,c\n"));
}

#[test]
fn label_and_directives() {
    let mut s = S { out: String::new() };
    label!(s, "L0");
    dir!(s, globl("_main"));
    dir!(s, text);
    dir!(s, data);
    dir!(s, word(42));
    dir!(s, byte_list(&[72, 101, 0]));
    assert!(s.out.contains("L0:\n"));
    assert!(s.out.contains("        .globl  _main\n"));
    assert!(s.out.contains("        .text\n"));
    assert!(s.out.contains("        .data\n"));
    assert!(s.out.contains("        .word   42\n"));
    assert!(s.out.contains("        .byte   72,101,0\n"));
}
