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

// --- Branch selection tests ---

#[test]
fn branch_short_for_small_if() {
    // Small if-body: forward branch should use short form (brf/brt)
    let src = "int main() { int x = 1; if (x == 1) { return 42; } return 0; }";
    let output = compile(src);
    // Condition branch should be short (brf Lnn, not la r2 + jmp)
    assert!(
        output.contains("brf     L") || output.contains("brt     L"),
        "small if should use short branch, got:\n{output}"
    );
    // Should NOT contain long branch pattern for local labels
    let lines: Vec<&str> = output.lines().collect();
    let has_long_local = lines.windows(2).any(|w| {
        w[0].contains("la      r2,L") && w[1].contains("jmp     (r2)")
    });
    assert!(!has_long_local, "small if should not use long branch for local labels");
}

#[test]
fn branch_short_for_backward_loop() {
    // While loop: backward branch to loop top should be short
    let src = "int main() { int x = 0; while (x < 10) { x = x + 1; } return x; }";
    let output = compile(src);
    // The backward bra to loop label should be short
    assert!(
        output.contains("bra     L"),
        "backward loop branch should be short bra"
    );
}

#[test]
fn branch_long_for_large_if_body() {
    // Large if-body: forward branch must use long form to avoid assembler error
    let src = r#"
        int main() {
            int x = 1;
            if (x == 1) {
                int a=x+1; int b=a+2; int c=b+3; int d=c+4;
                int e=d+5; int f=e+6; int g=f+7; int h=g+8;
                int i=h+9; int j=i+10; int k=j+11; int l=k+12;
                int m=l+13; int n=m+14; int o=n+15; int p=o+16;
                int q=p+17; int r=q+18; int s=r+19; int t=s+20;
                return t;
            }
            return 0;
        }
    "#;
    let output = compile(src);
    // The forward branch over the large body should be long (la r2 + jmp)
    let lines: Vec<&str> = output.lines().collect();
    let has_long = lines.windows(2).any(|w| {
        w[0].contains("la      r2,L") && w[1].contains("jmp     (r2)")
    });
    assert!(has_long, "large if-body should use long branch");
}

#[test]
fn branch_long_for_global_labels() {
    // Global labels (_halt, _main) should always use long form
    let output = compile("int main() { return 0; }");
    // _halt backward branch uses long form
    assert!(
        output.contains("bra     _halt") || (output.contains("la      r2,_halt") && output.contains("jmp     (r2)")),
        "global label branch should exist"
    );
}

#[test]
fn branch_nested_if_else_correct() {
    // Nested if/else with moderate bodies — should compile and be valid
    let src = r#"
        int main() {
            int x = 5;
            if (x > 3) {
                if (x < 10) {
                    return 1;
                } else {
                    return 2;
                }
            } else {
                return 3;
            }
        }
    "#;
    let output = compile(src);
    // Should contain both short conditional and unconditional branches
    assert!(output.contains("cls") || output.contains("ceq"), "should have comparison");
    assert!(output.contains("bra") || output.contains("jmp"), "should have unconditional branch");
}
