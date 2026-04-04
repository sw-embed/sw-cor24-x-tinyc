//! Parser tests for compound literal syntax: (type){initializer}.

use tc24r_ast::{Expr, Stmt};

use crate::parse_source;

#[test]
fn parse_compound_literal_scalar() {
    let program = parse_source("int main() { int x = (int){42}; return x; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            init: Some(Expr::StmtExpr(_)),
            ..
        }
    ));
}

#[test]
fn parse_compound_literal_addr_of() {
    let program = parse_source("int main() { int *p = &(int){42}; return *p; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            init: Some(Expr::AddrOf(inner)),
            ..
        } if matches!(inner.as_ref(), Expr::StmtExpr(_))
    ));
}

#[test]
fn parse_compound_literal_array() {
    let program = parse_source("int main() { int *p = (int[]){1, 2, 3}; return p[0]; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            init: Some(Expr::StmtExpr(_)),
            ..
        }
    ));
}

#[test]
fn parse_compound_literal_struct() {
    let program = parse_source(
        "struct S { int x; int y; }; int main() { struct S s = (struct S){10, 20}; return s.x; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            init: Some(Expr::StmtExpr(_)),
            ..
        }
    ));
}

#[test]
fn parse_compound_literal_struct_addr_of() {
    let program = parse_source(
        "struct S { int x; int y; }; int main() { struct S *p = &(struct S){3, 4}; return p->x; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            init: Some(Expr::AddrOf(inner)),
            ..
        } if matches!(inner.as_ref(), Expr::StmtExpr(_))
    ));
}
