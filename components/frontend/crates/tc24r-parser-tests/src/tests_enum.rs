//! Enum parsing tests.

use tc24r_ast::{Expr, Stmt};

use crate::parse_source;

#[test]
fn enum_auto_values() {
    let p = parse_source("int main() { enum { A, B, C }; return B; }");
    let stmts = &p.functions[0].body.stmts;
    assert!(matches!(&stmts[1], Stmt::Return(Expr::IntLit(1))));
}

#[test]
fn enum_explicit_value() {
    let p = parse_source("int main() { enum { X=5, Y, Z }; return Y; }");
    let stmts = &p.functions[0].body.stmts;
    assert!(matches!(&stmts[1], Stmt::Return(Expr::IntLit(6))));
}

#[test]
fn enum_named() {
    let p = parse_source("int main() { enum color { R, G, B }; return G; }");
    let stmts = &p.functions[0].body.stmts;
    assert!(matches!(&stmts[1], Stmt::Return(Expr::IntLit(1))));
}

#[test]
fn enum_top_level() {
    let p = parse_source("enum { A, B, C }; int main() { return C; }");
    let stmts = &p.functions[0].body.stmts;
    assert!(matches!(&stmts[0], Stmt::Return(Expr::IntLit(2))));
}

#[test]
fn enum_var_decl() {
    let p = parse_source("int main() { enum color { R, G, B } c; return G; }");
    let stmts = &p.functions[0].body.stmts;
    assert!(matches!(&stmts[1], Stmt::Return(Expr::IntLit(1))));
}
