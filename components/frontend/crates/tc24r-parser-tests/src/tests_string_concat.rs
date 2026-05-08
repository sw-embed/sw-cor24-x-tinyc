//! Adjacent string-literal concatenation (C99 §5.1.1.2 phase 6):
//! `"abc" "def"` becomes a single `"abcdef"` token before the parser
//! returns it as `Expr::StringLit`.

use crate::{parse_source, try_parse_source};
use tc24r_ast::{Expr, Stmt, Type};

fn first_global_string(src: &str) -> String {
    let p = parse_source(src);
    let g = p.globals.first().expect("expected one global");
    match &g.init {
        Some(Expr::StringLit(s)) => s.clone(),
        other => panic!("expected StringLit init, got {other:?}"),
    }
}

fn first_global_array_len(src: &str) -> usize {
    let p = parse_source(src);
    let g = p.globals.first().expect("expected one global");
    match &g.ty {
        Type::Array(_, n) => *n,
        other => panic!("expected Type::Array, got {other:?}"),
    }
}

#[test]
fn pointer_init_two_adjacent_literals() {
    assert_eq!(
        first_global_string(r#"char *s = "abc" "def";"#),
        "abcdef"
    );
}

#[test]
fn pointer_init_n_adjacent_literals() {
    assert_eq!(
        first_global_string(r#"char *s = "a" "b" "c" "d" "e";"#),
        "abcde"
    );
}

#[test]
fn pointer_init_with_intervening_whitespace_and_newline() {
    // Comments are stripped by the lexer; here we just test that newlines
    // and extra whitespace between adjacent literals don't block concat.
    assert_eq!(
        first_global_string("char *s = \"abc\"\n   \"def\"\n   \"ghi\";"),
        "abcdefghi"
    );
}

#[test]
fn escapes_are_resolved_per_chunk_then_joined() {
    // The brief's subtle point: escape sequences resolve within each
    // chunk independently, then bytes are concatenated. So `"ab\n"` +
    // `"cd"` yields a 4-char string {a, b, '\n', c, d} (5 chars total
    // pre-null), NOT {a, b, '\\', 'n', c, d}.
    let s = first_global_string(r#"char *s = "ab\n" "cd";"#);
    assert_eq!(s.len(), 5, "got {:?}", s);
    assert_eq!(s.as_bytes(), b"ab\ncd");
}

#[test]
fn empty_chunks_are_concatenated() {
    assert_eq!(first_global_string(r#"char *s = "" "abc" "";"#), "abc");
}

#[test]
fn char_array_implicit_size_uses_concatenated_length() {
    // The bug this saga fixes: `char s[] = "abc" "def";` previously
    // peeked only the first StringLit and produced length 4. After
    // the fix, the inferred length is 7 (6 chars + null).
    let p = parse_source(r#"int main(void) { char s[] = "abc" "def"; return s[0]; }"#);
    let body = p.functions[0].body.as_ref().expect("expected function body");
    let local_ty = match &body.stmts[0] {
        Stmt::LocalDecl { ty, .. } => ty,
        other => panic!("expected LocalDecl, got {other:?}"),
    };
    match local_ty {
        Type::Array(_, n) => assert_eq!(*n, 7usize, "expected length 7, got {n}"),
        other => panic!("expected Type::Array, got {other:?}"),
    }
}

#[test]
fn global_char_array_implicit_size_uses_concatenated_length() {
    // Regression: the global path (decl.rs) already worked. Lock it in.
    assert_eq!(
        first_global_array_len(r#"char g[] = "abc" "def";"#),
        7
    );
}

#[test]
fn brace_init_element_concatenates() {
    // `char *strings[] = { "a" "b", "c" "d" };` — each element is a
    // pointer-to-char init, which goes through parse_assign and routes
    // through expr.rs's concat. The first element should be "ab".
    let p = parse_source(r#"char *xs[] = { "a" "b", "c" "d" };"#);
    let g = p.globals.first().expect("expected one global");
    match &g.init {
        Some(Expr::InitList(items)) => {
            assert_eq!(items.len(), 2);
            assert!(
                matches!(&items[0], Expr::StringLit(s) if s == "ab"),
                "got {:?}",
                items[0]
            );
            assert!(
                matches!(&items[1], Expr::StringLit(s) if s == "cd"),
                "got {:?}",
                items[1]
            );
        }
        other => panic!("expected InitList, got {other:?}"),
    }
}

#[test]
fn function_arg_concatenates() {
    // `puts("hello, " "world");` — the argument expression goes through
    // parse_assign → expr.rs primary → concat loop.
    let p = parse_source(
        r#"int puts(char *s); int main(void) { return puts("hello, " "world"); }"#,
    );
    let main = p
        .functions
        .iter()
        .find(|f| f.name == "main")
        .and_then(|f| f.body.as_ref())
        .expect("expected main with body");
    let ret = main.stmts.iter().find_map(|s| match s {
        Stmt::Return(e) => Some(e),
        _ => None,
    });
    let expr = ret.expect("expected a return expression");
    match expr {
        Expr::Call { args, .. } => {
            assert_eq!(args.len(), 1);
            assert!(
                matches!(&args[0], Expr::StringLit(s) if s == "hello, world"),
                "got {:?}",
                args[0]
            );
        }
        other => panic!("expected Call, got {other:?}"),
    }
}

#[test]
fn return_expression_concatenates() {
    let p = parse_source(r#"char *foo(void) { return "ab" "cd"; }"#);
    let ret = p.functions[0].body.as_ref().unwrap().stmts.iter().find_map(|s| match s {
        Stmt::Return(e) => Some(e),
        _ => None,
    });
    let expr = ret.expect("expected return expression");
    assert!(
        matches!(expr, Expr::StringLit(s) if s == "abcd"),
        "got {:?}",
        expr
    );
}

#[test]
fn single_literal_unchanged_regression() {
    // Single literals must keep working untouched.
    assert_eq!(first_global_string(r#"char *s = "hello";"#), "hello");
}

#[test]
fn string_followed_by_non_string_still_errors() {
    // The brief's negative case: `"abc" 42` is not a valid expression
    // and must fail (just confirms the concat loop doesn't swallow
    // unrelated tokens).
    assert!(try_parse_source(r#"char *s = "abc" 42;"#).is_err());
}
