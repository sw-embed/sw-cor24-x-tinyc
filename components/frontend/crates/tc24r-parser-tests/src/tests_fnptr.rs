//! Function pointer parser tests.

use tc24r_ast::{Expr, Stmt, Type};

use crate::parse_source;

#[test]
fn parse_fn_ptr_local_decl() {
    // int (*fp)(int, int);
    let program = parse_source("int main() { int (*fp)(int, int); return 0; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            name,
            ty: Type::Ptr(_),
            init: None,
        } if name == "fp"
    ));
}

#[test]
fn parse_fn_ptr_local_with_init() {
    // int (*fp)(int, int) = add;
    let program = parse_source(
        "int add(int a, int b) { return a + b; }
         int main() { int (*fp)(int, int) = add; return 0; }",
    );
    let stmts = &program.functions[1].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            name,
            ty: Type::Ptr(_),
            init: Some(Expr::Ident(_)),
        } if name == "fp"
    ));
}

#[test]
fn parse_fn_ptr_array_decl() {
    // int (*table[4])(int);
    let program = parse_source("int main() { int (*table[4])(int); return 0; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            name,
            ty: Type::Array(_, 4),
            init: None,
        } if name == "table"
    ));
    // Inner type should be Ptr
    if let Stmt::LocalDecl { ty: Type::Array(inner, _), .. } = &stmts[0] {
        assert!(matches!(inner.as_ref(), Type::Ptr(_)));
    } else {
        panic!("expected Array(Ptr(...))");
    }
}

#[test]
fn parse_fn_ptr_param() {
    // void apply(int (*f)(int), int x) { ... }
    let program = parse_source("void apply(int (*f)(int), int x) { return; } int main() { return 0; }");
    let apply = &program.functions[0];
    assert_eq!(apply.name, "apply");
    assert_eq!(apply.params.len(), 2);
    assert_eq!(apply.params[0].name, "f");
    assert!(matches!(apply.params[0].ty, Type::Ptr(_)));
    assert_eq!(apply.params[1].name, "x");
    assert_eq!(apply.params[1].ty, Type::Int);
}

#[test]
fn parse_fn_ptr_typedef() {
    // typedef int (*handler_t)(int);
    // handler_t h;
    let program = parse_source(
        "int main() { typedef int (*handler_t)(int); handler_t h; return 0; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    // First stmt is the typedef (no-op), then h decl
    // Find the LocalDecl for h
    let h_decl = stmts.iter().find(|s| matches!(s, Stmt::LocalDecl { name, .. } if name == "h"));
    assert!(h_decl.is_some(), "should find LocalDecl for h");
    if let Stmt::LocalDecl { ty, .. } = h_decl.unwrap() {
        assert!(matches!(ty, Type::Ptr(_)), "h should be Ptr type, got {:?}", ty);
    }
}

#[test]
fn parse_indirect_call_postfix() {
    // table[0](5) should produce IndirectCall
    let program = parse_source(
        "int main() { int (*table[4])(int); table[0](5); return 0; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    // stmt[1] should be Expr(IndirectCall { ... })
    assert!(matches!(
        &stmts[1],
        Stmt::Expr(Expr::IndirectCall { .. })
    ), "expected IndirectCall, got {:?}", &stmts[1]);
}

#[test]
fn parse_deref_indirect_call() {
    // (*fp)(3) should produce IndirectCall
    let program = parse_source(
        "int main() { int (*fp)(int); (*fp)(3); return 0; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[1],
        Stmt::Expr(Expr::IndirectCall { .. })
    ), "expected IndirectCall, got {:?}", &stmts[1]);
}
