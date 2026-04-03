//! Parser tests for global declarations with initializers.

use tc24r_ast::{Expr, Type};

use crate::parse_source;

#[test]
fn parse_global_string_init() {
    let program =
        parse_source(r#"static const char msg[] = "hello"; int main(void) { return msg[0]; }"#);
    assert_eq!(program.globals.len(), 1);
    let g = &program.globals[0];
    assert_eq!(g.name, "msg");
    assert!(matches!(&g.ty, Type::Array(elem, 6) if **elem == Type::Char));
    assert!(matches!(&g.init, Some(Expr::StringLit(s)) if s == "hello"));
}

#[test]
fn parse_global_int_init_list() {
    let program = parse_source("int msg[] = {72, 105, 0}; int main(void) { return msg[0]; }");
    assert_eq!(program.globals.len(), 1);
    let g = &program.globals[0];
    assert_eq!(g.name, "msg");
    assert!(matches!(&g.ty, Type::Array(elem, 3) if **elem == Type::Int));
    assert!(matches!(&g.init, Some(Expr::InitList(v)) if v.len() == 3));
}

#[test]
fn parse_global_init_list_explicit_size() {
    let program = parse_source("int arr[5] = {10, 20, 30}; int main(void) { return arr[0]; }");
    assert_eq!(program.globals.len(), 1);
    let g = &program.globals[0];
    assert_eq!(g.name, "arr");
    assert!(matches!(&g.ty, Type::Array(elem, 5) if **elem == Type::Int));
    assert!(matches!(&g.init, Some(Expr::InitList(v)) if v.len() == 3));
}

#[test]
fn parse_global_char_init_list() {
    let program = parse_source("char msg[] = {72, 105, 0}; int main(void) { return msg[0]; }");
    assert_eq!(program.globals.len(), 1);
    let g = &program.globals[0];
    assert!(matches!(&g.ty, Type::Array(elem, 3) if **elem == Type::Char));
}

#[test]
fn parse_global_init_list_with_trailing_comma() {
    let program = parse_source("int arr[] = {1, 2, 3,}; int main(void) { return arr[0]; }");
    assert_eq!(program.globals.len(), 1);
    let g = &program.globals[0];
    assert!(matches!(&g.ty, Type::Array(elem, 3) if **elem == Type::Int));
}

#[test]
fn parse_global_scalar_init() {
    let program = parse_source("int x = 42; int main(void) { return x; }");
    assert_eq!(program.globals.len(), 1);
    let g = &program.globals[0];
    assert_eq!(g.name, "x");
    assert!(matches!(&g.init, Some(Expr::IntLit(42))));
}
