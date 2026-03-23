//! Structural checks on codegen output (non-golden-file tests).

use tc24r_test_compile::compile;

#[test]
fn codegen_emits_start() {
    let output = compile("int main() { return 0; }");
    assert!(output.contains("_start:"));
    assert!(output.contains("la      r0,_main"));
    assert!(output.contains("jal     r1,(r0)"));
    assert!(output.contains("_halt:"));
    assert!(output.contains("bra     _halt"));
}

#[test]
fn codegen_struct_member_access() {
    let src = r#"
        int main() {
            struct { int x; int y; } p;
            p.x = 3;
            p.y = 4;
            return p.x + p.y;
        }
    "#;
    let output = compile(src);
    // Struct local should produce member stores and loads
    assert!(output.contains("sw"));
    assert!(output.contains("lw"));
}

#[test]
fn codegen_named_struct() {
    let src = r#"
        int main() {
            struct point { int x; int y; };
            struct point p;
            p.x = 10;
            p.y = 20;
            return p.x + p.y;
        }
    "#;
    let output = compile(src);
    assert!(output.contains("sw"));
    assert!(output.contains("lw"));
}

#[test]
fn codegen_isr_prologue_epilogue() {
    let output = compile("__attribute__((interrupt)) void isr() {} int main() { return 0; }");
    // ISR prologue saves all regs + condition flag
    assert!(output.contains("push    r0"));
    assert!(output.contains("mov     r2,c"));
    // ISR epilogue restores and uses jmp (ir)
    assert!(output.contains("clu     z,r2"));
    assert!(output.contains("jmp     (ir)"));
    // Normal main still uses jmp (r1)
    assert!(output.contains("jmp     (r1)"));
}
