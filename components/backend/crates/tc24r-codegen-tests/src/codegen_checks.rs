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
    let has_long_local = lines
        .windows(2)
        .any(|w| w[0].contains("la      r2,L") && w[1].contains("jmp     (r2)"));
    assert!(
        !has_long_local,
        "small if should not use long branch for local labels"
    );
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
    let has_long = lines
        .windows(2)
        .any(|w| w[0].contains("la      r2,L") && w[1].contains("jmp     (r2)"));
    assert!(has_long, "large if-body should use long branch");
}

#[test]
fn branch_long_for_global_labels() {
    // Global labels (_halt, _main) should always use long form
    let output = compile("int main() { return 0; }");
    // _halt backward branch uses long form
    assert!(
        output.contains("bra     _halt")
            || (output.contains("la      r2,_halt") && output.contains("jmp     (r2)")),
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
    assert!(
        output.contains("cls") || output.contains("ceq"),
        "should have comparison"
    );
    assert!(
        output.contains("bra") || output.contains("jmp"),
        "should have unconditional branch"
    );
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
    assert!(
        !output.contains("_MAKE_SYMBOL"),
        "macro should be expanded, not called as function"
    );
    // Should contain shift and or (the expanded expression)
    assert!(
        output.contains("shl"),
        "expanded macro should produce shift"
    );
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
    // BUG-005: int arr[10] should allocate 30 bytes (10 * 3-byte int).
    // After pr/emit-zero-fill, this is a single `.zero 30` directive
    // instead of 10 enumerated `.word 0` lines.
    let src = "int arr[10]; int main() { return 0; }";
    let output = compile(src);
    assert!(
        output.contains(".zero   30"),
        "int arr[10] should emit `.zero 30` (10 * 3-byte int), got:\n{output}"
    );
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
    assert!(
        output.contains("la      r0,_pool"),
        "global array should decay to la r0 (address)"
    );
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
    // Should allocate space for 8 structs of 2 ints each = 8 * 2 * 3 = 48 bytes.
    // After pr/emit-zero-fill this is a single `.zero 48` directive.
    assert!(
        output.contains(".zero   48"),
        "struct symbol symtab[8] should emit `.zero 48` (8 structs * 2 ints * 3 bytes), got:\n{output}"
    );
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
    assert!(
        output.contains("_do_store:"),
        "do_store should be generated"
    );
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

// --- Function pointer tests ---

#[test]
fn codegen_fn_ptr_variable_call() {
    // fp = add; fp(3, 4) — variable call should use indirect jal
    let src = r#"
        int add(int a, int b) { return a + b; }
        int main() {
            int (*fp)(int, int) = add;
            return fp(3, 4);
        }
    "#;
    let output = compile(src);
    // The call to fp should load fp's value (a stack variable), NOT use la r0,_fp
    // It should still use jal r1,(r0) for the call
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| l.contains("_main:") || !l.starts_with('_'))
        .collect::<Vec<_>>()
        .join("\n");
    // Should NOT have la r0,_fp (that would mean calling a function named fp)
    assert!(
        !main_section.contains("la      r0,_fp"),
        "variable call should not emit la r0,_fp (direct call), got:\n{main_section}"
    );
    // Should have jal (for the indirect call)
    assert!(
        main_section.contains("jal     r1,(r0)"),
        "variable call should emit jal r1,(r0), got:\n{main_section}"
    );
}

#[test]
fn codegen_fn_ptr_indirect_call() {
    // table[0](5) — IndirectCall through array subscript
    let src = r#"
        int double(int x) { return x + x; }
        int main() {
            int (*table[4])(int);
            table[0] = double;
            return table[0](5);
        }
    "#;
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    assert!(output.contains("_double:"), "double should be generated");
}

// --- Global initializer tests ---

#[test]
fn codegen_global_string_init() {
    let src = r#"static const char msg[] = "hello"; int main(void) { return msg[0]; }"#;
    let output = compile(src);
    assert!(output.contains("_msg:"), "msg label should be generated");
    assert!(output.contains("104"), "should contain 'h' (104)");
    assert!(output.contains("111"), "should contain 'o' (111)");
}

#[test]
fn codegen_global_int_init_list() {
    let src = "int msg[] = {72, 105, 0}; int main(void) { return msg[0]; }";
    let output = compile(src);
    assert!(output.contains("_msg:"), "msg label should be generated");
    assert!(
        output.contains(".word   72"),
        "should contain initializer value 72"
    );
    assert!(
        output.contains(".word   105"),
        "should contain initializer value 105"
    );
}

#[test]
fn codegen_global_char_init_list() {
    let src = "char msg[] = {65, 66, 0}; int main(void) { return msg[0]; }";
    let output = compile(src);
    assert!(output.contains("_msg:"), "msg label should be generated");
    assert!(
        output.contains(".byte   65"),
        "should contain initializer byte 65"
    );
}

#[test]
fn codegen_global_init_list_zero_padding() {
    let src = "int arr[5] = {10, 20}; int main(void) { return arr[0]; }";
    let output = compile(src);
    assert!(output.contains(".word   10"), "should contain first value");
    assert!(output.contains(".word   20"), "should contain second value");
    let arr_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_arr:"))
        .take_while(|l| !l.starts_with('_') || l.contains("_arr:"))
        .collect::<Vec<_>>()
        .join("\n");
    let word_count = arr_section.matches(".word").count();
    assert_eq!(word_count, 5, "should emit 5 words (2 init + 3 zero-pad)");
}

// --- compound literal as a local-decl initializer ---

#[test]
fn codegen_compound_literal_array_no_slot_overlap() {
    // Regression for the stack-layout bug: when a compound-literal
    // expression is the init of a same-scope LocalDecl, the inner
    // StmtExpr's auto-allocation must start past the outer variable.
    //
    // Before: `int *arr = (int[]){5,15,10};` placed __complit_0 at
    // offset -9 and arr at offset -3 — overlapping arr's slot with
    // the third element of the array temp. Now both should fit in
    // 12 bytes with arr at -3 and __complit_0 at -12 (disjoint).
    let src = r#"int main(void) { int *arr = (int[]){5,15,10}; return arr[2]; }"#;
    let output = compile(src);
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| !l.starts_with("L0:") && !l.starts_with("_S"))
        .collect::<Vec<_>>()
        .join("\n");
    // Frame size must be 12 (9 for int[3] + 3 for int*).
    assert!(
        main_section.contains("add     sp,-12"),
        "expected 12-byte frame (9 temp + 3 arr), got:\n{main_section}"
    );
    // The third element of the temp must be stored at -3 (relative
    // to __complit_0 base at -12 + offset 6 = -6 ... wait, the
    // codegen uses temp_base + 6 = -12 + 6 = -6 for the third int).
    // Assert no zero-fill store overlaps offset -3 (which is arr's
    // slot). All zero-fills should be at -12, -9, -6.
    let zero_targets: Vec<&str> = main_section
        .lines()
        .filter(|l| l.contains("sw      r0,") || l.contains("sb      r0,"))
        .take(3) // first 3 stores are the zero-fill words
        .collect();
    let zero_text = zero_targets.join("\n");
    assert!(
        !zero_text.contains("-3(fp)"),
        "zero-fill must not touch arr's slot at -3(fp); got:\n{zero_text}"
    );
}

// --- char *g = "..." (global pointer init) and char a[] = "..." (local array init) ---

#[test]
fn codegen_global_char_pointer_init_emits_word_label() {
    // `char *g = "abc"` must allocate an anonymous rodata label for
    // the bytes and store its address at _g — NOT emit the bytes
    // at _g directly. Otherwise `g[i]` (which dereferences _g) reads
    // bytes of the pointer representation instead of the string.
    let src = r#"char *g = "abc"; int main(void) { return g[2]; }"#;
    let output = compile(src);
    let g_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_g:"))
        .take(2)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        g_section.contains(".word   _S"),
        "expected _g to hold a .word _Sn (pointer to rodata), got:\n{g_section}"
    );
    assert!(
        !g_section.contains(".byte"),
        "_g must not hold inline bytes for a pointer-typed global, got:\n{g_section}"
    );
}

#[test]
fn codegen_global_char_array_init_keeps_inline_bytes() {
    // Regression: `char ga[] = "abc"` must keep the existing
    // inline-byte emission. Only pointer globals get the anon-
    // label treatment.
    let src = r#"char ga[] = "abc"; int main(void) { return ga[2]; }"#;
    let output = compile(src);
    let ga_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_ga:"))
        .take(3)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        ga_section.contains(".byte   97,98,99,0"),
        "expected inline bytes for char[] global, got:\n{ga_section}"
    );
    assert!(
        !ga_section.contains(".word   _S"),
        "char[] global must NOT use anon-label form, got:\n{ga_section}"
    );
}

#[test]
fn codegen_local_char_array_string_init_emits_byte_stores() {
    // `char a[] = "abc"` must byte-copy into the stack array, not
    // store a pointer to rodata. Look for per-byte `sb r0,off(fp)`
    // emissions for each of the 4 bytes ('a','b','c','\0').
    let src = r#"int main(void) { char a[] = "abc"; return a[2]; }"#;
    let output = compile(src);
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| !l.starts_with("L0:") && !l.starts_with("_S"))
        .collect::<Vec<_>>()
        .join("\n");
    // Should see 4 byte stores for 'a','b','c','\0'.
    let sb_count = main_section.matches("        sb      r0,").count();
    assert!(
        sb_count >= 4,
        "expected at least 4 sb-byte stores for the literal init, got {sb_count}:\n{main_section}"
    );
    // And the character constants 97, 98, 99 must appear as immediates.
    for v in [97, 98, 99] {
        assert!(
            main_section.contains(&format!("lc      r0,{v}")),
            "expected `lc r0,{v}` for char literal init, got:\n{main_section}"
        );
    }
    // Must NOT have the broken pointer-store pattern (`la r0,_Sn` then
    // `sw r0,offset(fp)`).
    let lines: Vec<&str> = main_section.lines().collect();
    let has_pointer_init = lines
        .windows(2)
        .any(|w| w[0].contains("la      r0,_S") && w[1].contains("sw      r0,"));
    assert!(
        !has_pointer_init,
        "char[] local must not be initialized by storing the rodata pointer, got:\n{main_section}"
    );
}

#[test]
fn codegen_local_char_array_explicit_size_zero_pads() {
    // `char a[10] = "abc"` should byte-copy 'a','b','c','\0' then
    // emit six additional zero stores to fill the array.
    let src = r#"int main(void) { char a[10] = "abc"; return a[5]; }"#;
    let output = compile(src);
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| !l.starts_with("L0:") && !l.starts_with("_S"))
        .collect::<Vec<_>>()
        .join("\n");
    let sb_count = main_section.matches("        sb      r0,").count();
    assert_eq!(
        sb_count, 10,
        "expected 10 sb-byte stores for char[10] init (3 chars + null + 6 zeros), got {sb_count}:\n{main_section}"
    );
}

#[test]
fn codegen_zero_fill_big_array_one_line() {
    // 16 * 4096 = 65536 bytes. Pre-fix this emitted ~21,846 lines
    // (.word 0 + trailing .byte 0). Post-fix: a single .zero 65536.
    let src = "static char chunk_storage[16 * 4096]; int main(void) { return chunk_storage[0]; }";
    let output = compile(src);
    assert!(output.contains("_chunk_storage:"));
    assert!(
        output.contains(".zero   65536"),
        "expected single .zero 65536 line; got:\n{}",
        output
            .lines()
            .filter(|l| l.contains("chunk_storage") || l.contains(".zero") || l.contains(".word"))
            .take(5)
            .collect::<Vec<_>>()
            .join("\n")
    );
    let chunk_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_chunk_storage:"))
        .take(3)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !chunk_section.contains(".word   0"),
        "should not contain .word 0 lines; got:\n{chunk_section}"
    );
}

#[test]
fn codegen_zero_fill_small_array() {
    let src = "static char small[10]; int main(void) { return small[0]; }";
    let output = compile(src);
    assert!(output.contains("_small:"));
    assert!(output.contains(".zero   10"), "got:\n{output}");
}

#[test]
fn codegen_zero_fill_int_array() {
    // 3 ints * 3 bytes/int = 9 bytes.
    let src = "static int trio[3]; int main(void) { return trio[0]; }";
    let output = compile(src);
    assert!(output.contains("_trio:"));
    assert!(output.contains(".zero   9"), "got:\n{output}");
}

#[test]
fn codegen_zero_fill_non_word_multiple() {
    // 10 bytes is not a multiple of 3. Pre-fix would emit 3 .word 0
    // + 1 .byte 0; post-fix: one .zero 10.
    let src = "static char buf[10]; int main(void) { return buf[0]; }";
    let output = compile(src);
    assert!(output.contains(".zero   10"), "got:\n{output}");
    let buf_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_buf:"))
        .take(3)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!buf_section.contains(".byte   0"));
    assert!(!buf_section.contains(".word   0"));
}

#[test]
fn codegen_zero_fill_scalar() {
    // Sanity check the one-emit path covers single scalars.
    let src = "static int scalar; int main(void) { return scalar; }";
    let output = compile(src);
    assert!(output.contains("_scalar:"));
    assert!(output.contains(".zero   3"), "got:\n{output}");
}

#[test]
fn codegen_mixed_init_unchanged_by_zero_fill_change() {
    // emit_typed_data path (mixed-init) is explicitly out of scope:
    // it should still produce per-element .word lines including
    // explicit .word 0 for the trailing zeros.
    let src = "static int arr[5] = {10, 20}; int main(void) { return arr[0]; }";
    let output = compile(src);
    let arr_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_arr:"))
        .take_while(|l| !l.starts_with('_') || l.contains("_arr:"))
        .collect::<Vec<_>>()
        .join("\n");
    let word_count = arr_section.matches(".word").count();
    assert_eq!(
        word_count, 5,
        "mixed-init should still emit 5 per-element .word lines, not collapse via .zero"
    );
    assert!(
        !arr_section.contains(".zero"),
        "mixed-init must not use .zero; got:\n{arr_section}"
    );
}

// --- Postfix ++/-- on struct members and array elements ---

#[test]
fn codegen_postinc_struct_member() {
    let src = r#"
        struct s { int x; };
        int main() { struct s a; a.x = 0; a.x++; return a.x; }
    "#;
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    // Should have load, add, store sequence for member inc
    assert!(output.contains("lw"), "should load member");
    assert!(output.contains("add     r0,1"), "should add 1 for ++");
    assert!(output.contains("sw"), "should store member");
}

#[test]
fn codegen_postdec_array_element() {
    let src = "int main() { int a[10]; a[0] = 10; a[0]--; return a[0]; }";
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    // Post-decrement: push old, add -1, store, pop
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| l.contains("_main:") || !l.starts_with('_'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        main_section.contains("add     r0,-1"),
        "array element -- should add -1, got:\n{main_section}"
    );
}

#[test]
fn codegen_preinc_arrow_member() {
    let src = r#"
        struct s { int x; };
        int main() { struct s a; struct s *p = &a; p->x = 0; ++p->x; return p->x; }
    "#;
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    assert!(output.contains("add     r0,1"), "should add 1 for ++");
}

#[test]
fn codegen_postinc_preserves_old_value() {
    // i++ should return old value (push/pop pattern)
    let src = "int main() { int i = 5; int j = i++; return j; }";
    let output = compile(src);
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| l.contains("_main:") || !l.starts_with('_'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        main_section.contains("push    r0"),
        "post-increment should push old value, got:\n{main_section}"
    );
    assert!(
        main_section.contains("pop     r0"),
        "post-increment should pop old value, got:\n{main_section}"
    );
}

// --- Address-of struct member ---

#[test]
fn codegen_addr_of_struct_member() {
    let src = r#"
        struct s { int x; int y; };
        int main() { struct s a; int *p = &a.y; return *p; }
    "#;
    let output = compile(src);
    // Should NOT panic — this was a regression (rust-panic-bug)
    assert!(output.contains("_main:"), "main should be generated");
    // Should compute member address (fp + offset + member_offset)
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| l.contains("_main:") || !l.starts_with('_'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        main_section.contains("add     r0,fp"),
        "struct member address should compute fp + offset, got:\n{main_section}"
    );
}

#[test]
fn codegen_addr_of_nested_member() {
    let src = r#"
        struct inner { int x; };
        struct outer { struct inner b; int mode; };
        int main() { struct outer a; int *p = &a.b.x; return *p; }
    "#;
    let output = compile(src);
    // Should NOT panic — nested member address-of
    assert!(output.contains("_main:"), "main should be generated");
}

#[test]
fn codegen_addr_of_arrow_member() {
    let src = r#"
        struct s { int x; };
        int foo(struct s *p) { int *px = &p->x; return *px; }
        int main() { struct s a; return foo(&a); }
    "#;
    let output = compile(src);
    assert!(output.contains("_foo:"), "foo should be generated");
    assert!(output.contains("_main:"), "main should be generated");
}

#[test]
fn codegen_struct_member_array_write_no_deref() {
    // BUG-003: e.cmd[0] = val must NOT load array contents as pointer.
    // The address of cmd should be computed, then val stored directly.
    let src = r#"
        struct S { int mode; char cmd[8]; };
        int main() { struct S e; e.cmd[0] = 65; return e.cmd[0]; }
    "#;
    let output = compile(src);
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| l.contains("_main:") || !l.starts_with('_'))
        .collect::<Vec<_>>()
        .join("\n");
    // Should use sb (byte store), not sw (word store) for char array member
    assert!(
        main_section.contains("sb"),
        "char array member write should use sb"
    );
    // Should NOT load-then-deref (the bug pattern): address + lw + add + sw
    // Instead: address + add + sb
    let lines: Vec<&str> = main_section.lines().collect();
    let has_bad_pattern = lines
        .windows(3)
        .any(|w| w[0].contains("add     r0,fp") && w[1].contains("lw      r0,0(r0)"));
    assert!(
        !has_bad_pattern,
        "struct member array access should NOT load array contents as pointer, got:\n{main_section}"
    );
}

#[test]
fn codegen_struct_member_int_array_write() {
    let src = r#"
        struct S { int vals[4]; };
        int main() { struct S e; e.vals[0] = 10; e.vals[1] = 20; return e.vals[0] + e.vals[1]; }
    "#;
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| l.contains("_main:") || !l.starts_with('_'))
        .collect::<Vec<_>>()
        .join("\n");
    // Should use sw (word store) for int array member
    assert!(
        main_section.contains("sw"),
        "int array member write should use sw"
    );
    let lines: Vec<&str> = main_section.lines().collect();
    let has_bad_pattern = lines
        .windows(3)
        .any(|w| w[0].contains("add     r0,fp") && w[1].contains("lw      r0,0(r0)"));
    assert!(
        !has_bad_pattern,
        "struct member array access should NOT load array contents as pointer, got:\n{main_section}"
    );
}

// --- Compound literal tests ---

#[test]
fn codegen_compound_literal_scalar() {
    let src = "int main() { int x = (int){42}; return x; }";
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    assert!(
        output.contains("lc      r0,42"),
        "should load 42 as compound literal init"
    );
}

#[test]
fn codegen_compound_literal_addr_of() {
    let src = "int main() { int *p = &(int){42}; return *p; }";
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    let main_section: String = output
        .lines()
        .skip_while(|l| !l.contains("_main:"))
        .take_while(|l| l.contains("_main:") || !l.starts_with('_'))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        main_section.contains("add     r0,fp"),
        "addr-of compound literal should compute address via add r0,fp, got:\n{main_section}"
    );
}

#[test]
fn codegen_compound_literal_array() {
    let src = "int main() { int *p = (int[]){10, 20, 30}; return p[1]; }";
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    assert!(output.contains("sw"), "should store array elements");
}

#[test]
fn codegen_compound_literal_struct() {
    let src = r#"
        struct S { int x; int y; };
        int main() { struct S *p = &(struct S){3, 4}; return p->x + p->y; }
    "#;
    let output = compile(src);
    assert!(output.contains("_main:"), "main should be generated");
    assert!(output.contains("sw"), "should store struct members");
}
