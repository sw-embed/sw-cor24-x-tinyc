//! Whole-program dead-code elimination tests.

use tc24r_dce::dce;

fn parse(src: &str) -> tc24r_ast::Program {
    let tokens = tc24r_lexer::Lexer::new(src).tokenize().unwrap();
    tc24r_parser::parse(tokens).unwrap()
}

fn fn_names(p: &tc24r_ast::Program) -> Vec<String> {
    p.functions
        .iter()
        .filter(|f| f.body.is_some())
        .map(|f| f.name.clone())
        .collect()
}

#[test]
fn drops_unreferenced_function() {
    let mut p = parse(
        r#"
        int used(void) { return 42; }
        int unused(void) { return 99; }
        int main(void) { return used(); }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(names.contains(&"main".to_string()));
    assert!(names.contains(&"used".to_string()));
    assert!(!names.contains(&"unused".to_string()), "got: {names:?}");
}

#[test]
fn keeps_transitive_callees() {
    let mut p = parse(
        r#"
        int leaf(void) { return 1; }
        int mid(void) { return leaf(); }
        int main(void) { return mid(); }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(names.contains(&"main".to_string()));
    assert!(names.contains(&"mid".to_string()));
    assert!(names.contains(&"leaf".to_string()));
}

#[test]
fn function_pointer_address_keeps_target() {
    // `int (*fp)(void) = callme;` takes the address of callme.
    // DCE must keep callme even though main never calls it
    // directly by name.
    let mut p = parse(
        r#"
        int callme(void) { return 7; }
        int (*fp)(void) = callme;
        int main(void) { return fp(); }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(names.contains(&"callme".to_string()), "got: {names:?}");
    assert!(names.contains(&"main".to_string()));
}

#[test]
fn forward_declaration_does_not_block_traversal() {
    // Regression for the "find returns first match" bug: when a
    // function has both a prototype (body: None) and a definition
    // (body: Some(...)), DCE must walk the definition's body, not
    // stop at the prototype.
    let mut p = parse(
        r#"
        int helper(void);
        int main(void) { return helper(); }
        int helper(void) { return inner(); }
        int inner(void) { return 5; }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(names.contains(&"helper".to_string()));
    assert!(
        names.contains(&"inner".to_string()),
        "transitive call through helper's body must survive: {names:?}"
    );
}

#[test]
fn printf_call_keeps_matching_variadic_shim() {
    // tc24r rewrites printf(fmt, a, b) to __tc24r_printf2 at codegen.
    // DCE must mirror this: a printf call with N extra args makes
    // __tc24r_printfN reachable.
    let mut p = parse(
        r#"
        int __tc24r_printf0(char *fmt) { return 0; }
        int __tc24r_printf1(char *fmt, int a) { return 0; }
        int __tc24r_printf2(char *fmt, int a, int b) { return 0; }
        int printf(char *fmt, ...);
        int main(void) {
            printf("%d %d", 1, 2);
            return 0;
        }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(
        names.contains(&"__tc24r_printf2".to_string()),
        "got: {names:?}"
    );
    assert!(!names.contains(&"__tc24r_printf0".to_string()));
    assert!(!names.contains(&"__tc24r_printf1".to_string()));
}

#[test]
fn printf_no_args_keeps_printf0() {
    let mut p = parse(
        r#"
        int __tc24r_printf0(char *fmt) { return 0; }
        int __tc24r_printf1(char *fmt, int a) { return 0; }
        int printf(char *fmt, ...);
        int main(void) {
            printf("hello");
            return 0;
        }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(
        names.contains(&"__tc24r_printf0".to_string()),
        "got: {names:?}"
    );
    assert!(!names.contains(&"__tc24r_printf1".to_string()));
}

#[test]
fn unused_function_with_body_dropped_even_when_prototype_exists() {
    let mut p = parse(
        r#"
        int wasted(void);
        int main(void) { return 0; }
        int wasted(void) { return 99; }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(!names.contains(&"wasted".to_string()), "got: {names:?}");
}

#[test]
fn function_referenced_only_through_global_init_kept() {
    let mut p = parse(
        r#"
        int callback(void) { return 1; }
        int (*hook)(void) = callback;
        int main(void) { return 0; }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(
        names.contains(&"callback".to_string()),
        "address taken in global init must be reachable: {names:?}"
    );
}

#[test]
fn function_referenced_only_in_inline_asm_kept() {
    // Inline-asm symbol scan should pick up `_target` and keep
    // the C function `target`.
    let mut p = parse(
        r#"
        int target(void) { return 9; }
        int main(void) {
            asm("call _target");
            return 0;
        }
        "#,
    );
    dce(&mut p);
    let names = fn_names(&p);
    assert!(
        names.contains(&"target".to_string()),
        "asm reference to _target must keep target: {names:?}"
    );
}

#[test]
fn no_main_no_crash() {
    // Pathological: a translation unit with only non-main functions.
    // DCE should drop everything (no roots) without panicking.
    let mut p = parse(
        r#"
        int orphan(void) { return 1; }
        int also_orphan(void) { return 2; }
        "#,
    );
    dce(&mut p);
    assert_eq!(fn_names(&p), Vec::<String>::new());
}
