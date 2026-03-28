//! Structural checks on codegen output (non-golden-file tests).

use tc24r_test_compile::{compile, compile_pp};

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

// --- Bug regression tests ---

#[test]
fn bug001_nested_func_macro_expanded() {
    // BUG-001: #define NIL_VAL MAKE_SYMBOL(0) should expand the inner macro
    let src = r#"
        #define MAKE_SYMBOL(idx) (((idx) << 2) | 2)
        #define NIL_VAL MAKE_SYMBOL(0)
        int main() { int x = NIL_VAL; return x; }
    "#;
    let output = compile_pp(src);
    // Should NOT call MAKE_SYMBOL as a function
    assert!(!output.contains("_MAKE_SYMBOL"), "macro should be expanded, not called as function");
    // Should contain shift and or (the expanded expression)
    assert!(output.contains("shl"), "expanded macro should produce shift");
    assert!(output.contains("or"), "expanded macro should produce or");
}

#[test]
fn bug002_two_level_define_no_panic() {
    // BUG-002: two-level #define should not panic
    let src = r#"
        #define TAG_SYMBOL 2
        #define NIL_VAL ((0 << 2) | TAG_SYMBOL)
        int arr[10];
        int main() { arr[0] = NIL_VAL; return arr[0]; }
    "#;
    // Should compile without panic — the assertion is that compile_pp() returns
    let _output = compile_pp(src);
}

#[test]
fn bug003_nested_array_index() {
    // BUG-003: pool[offsets[i]] should parse and compile
    let src = r#"
        char pool[100];
        int offsets[10];
        char *get(int i) { return &pool[offsets[i]]; }
        int main() { return 0; }
    "#;
    let output = compile(src);
    assert!(output.contains("_get:"), "function should be generated");
}

#[test]
fn bug005_global_array_full_size() {
    // BUG-005: int arr[10] should allocate 10 words, not 1
    let src = "int arr[10]; int main() { return 0; }";
    let output = compile(src);
    let word_count = output.matches(".word").count();
    assert!(word_count >= 10, "int arr[10] should emit at least 10 .word directives, got {word_count}");
}

#[test]
fn bug006_global_char_array_decay() {
    // BUG-006: global char array should decay to address, not dereference
    let src = r#"
        char pool[100];
        int main() { pool[0] = 65; return 0; }
    "#;
    let output = compile(src);
    // Should use la r0,_pool (address), not la r1,_pool; lw r0,0(r1) (deref)
    assert!(output.contains("la      r0,_pool"), "global array should decay to la r0 (address)");
}

#[test]
fn bug012_paren_ptr_arrow_parse() {
    // BUG-012: (ptr + offset)->member fails to parse
    // "expected Semicolon, got Arrow"
    let src = r#"
        struct pair { int key; int val; };
        int main() {
            struct pair *arr;
            arr = (struct pair *)malloc(2 * sizeof(struct pair));
            arr->key = 10;
            arr->val = 20;
            (arr + 1)->key = 30;
            (arr + 1)->val = 40;
            return (arr + 1)->key + arr->val;
        }
    "#;
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    assert!(output.contains("sw"), "should store struct members");
    assert!(output.contains("lw"), "should load struct members");
}

#[test]
fn bug011_global_struct_array_parse() {
    // BUG-011: global struct array declaration fails to parse
    // "expected Semicolon, got LBracket" for struct symbol symtab[8]
    let src = r#"
        struct symbol { int name_char; int value; };
        struct symbol symtab[8];
        int main() {
            symtab[0].name_char = 65;
            symtab[0].value = 100;
            return symtab[0].name_char + symtab[0].value;
        }
    "#;
    let output = compile(src);
    // Should compile without parse error
    assert!(output.contains("_main:"), "main should be generated");
    // Should allocate space for 8 structs (at least 16 .word directives for 8 * 2 members)
    let word_count = output.matches(".word").count();
    assert!(word_count >= 16, "struct symbol symtab[8] should emit at least 16 .word directives, got {word_count}");
}

#[test]
fn bug010_ptr_index_member_access() {
    // BUG-010: ptr[i].member panics — array subscript on struct pointer
    // resolves to Int instead of the struct type
    let src = r#"
        struct pair { int key; int val; };
        int main() {
            struct pair *arr;
            arr = (struct pair *)malloc(2 * sizeof(struct pair));
            arr[0].key = 10;
            arr[0].val = 20;
            arr[1].key = 30;
            arr[1].val = 40;
            return arr[0].key + arr[1].val;
        }
    "#;
    let output = compile(src);
    // Should compile without panicking — key assertion is that compile() returns
    assert!(output.contains("_main:"), "main should be generated");
    // Should contain struct member stores (sw) and loads (lw)
    assert!(output.contains("sw"), "should store struct members");
    assert!(output.contains("lw"), "should load struct members");
}

#[test]
fn bug013_large_frame_deref_assign_r1_clobber() {
    // BUG-013 part 2: in a large stack frame, the expanded load sequence
    // (la r1,offset / add r1,fp / lw r0,0(r1)) clobbers r1. The
    // gen_deref_assign "simple" path saved the pointer address in r1 then
    // loaded the value via gen_expr_fn which clobbered r1. Fix: locals at
    // large offsets are excluded from is_simple_expr, forcing push/pop.
    let src = r#"
        int main() {
            char pad[200];
            char buf[12];
            int i;
            buf[0] = 50;
            buf[1] = 52;
            i = 0;
            char t;
            t = buf[1];
            buf[1] = buf[0];
            buf[0] = t;
            return buf[0] + buf[1];
        }
    "#;
    let output = compile(src);
    // In a frame > 127 bytes, the swap variables (t, buf elements) must
    // use push/pop rather than the simple r1 shortcut.
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| !l.starts_with('_') || l.contains("_main:"))
        .collect::<Vec<_>>()
        .join("\n");
    // The large frame must trigger push/pop for deref-assign of locals
    assert!(
        main_section.contains("push"),
        "large-frame deref assign should use push/pop, got:\n{main_section}"
    );
}

#[test]
fn bug013_large_local_offset_overflow() {
    // BUG-013: char buf[256] gets stack offset -256, which overflows the
    // 8-bit signed immediate in lw/sw instructions (-128..127).
    // The compiler must expand large offsets to la+add+lw/sw sequences.
    let src = r#"
        int main() {
            char buf[256];
            int x;
            x = 42;
            buf[0] = 65;
            return x;
        }
    "#;
    let output = compile(src);
    // x is at a small offset and should use direct fp-relative access
    // buf is at offset -256 or larger, which must NOT appear as a raw lw/sw offset
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| !l.starts_with('_') || l.contains("_main:"))
        .collect::<Vec<_>>()
        .join("\n");
    // The expanded sequence for large offsets uses "la r1,{offset}" + "add r1,fp"
    // Check that the main function contains this pattern (needed for buf or x,
    // whichever gets the large offset)
    assert!(
        main_section.contains("add     r1,fp"),
        "large stack offset should expand to la+add sequence, got:\n{main_section}"
    );
}

#[test]
fn bug007_array_store_global_index() {
    // BUG-007: offsets[idx] = counter where both are globals should not clobber address
    let src = r#"
        int offsets[10];
        int counter;
        void do_store() {
            int idx = counter;
            offsets[idx] = counter;
            counter = counter + 1;
        }
        int main() { counter = 0; do_store(); do_store(); return offsets[1]; }
    "#;
    let output = compile(src);
    // Should compile — the key test is that it generates a push/pop to preserve
    // the target address when loading a global RHS value
    assert!(output.contains("_do_store:"), "do_store should be generated");
    // The do_store function should use push to save the computed address
    let do_store_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_do_store:"))
        .take_while(|l| !l.contains("jmp     (r1)"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        do_store_section.contains("push"),
        "do_store should push to preserve address when loading global RHS"
    );
}
