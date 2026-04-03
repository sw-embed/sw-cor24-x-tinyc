//! Parser tests for postfix operators: ++/-- on struct members and array elements,
//! address-of on struct members.

use tc24r_ast::{Expr, Stmt, Type};

use crate::parse_source;

#[test]
fn parse_postinc_struct_member() {
    let program =
        parse_source("struct s { int x; }; int main() { struct s a; a.x = 0; a.x++; return a.x; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[2], Stmt::Expr(Expr::PostInc(_))));
}

#[test]
fn parse_postdec_struct_member() {
    let program =
        parse_source("struct s { int x; }; int main() { struct s a; a.x = 5; a.x--; return a.x; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[2], Stmt::Expr(Expr::PostDec(_))));
}

#[test]
fn parse_preinc_struct_member() {
    let program =
        parse_source("struct s { int x; }; int main() { struct s a; a.x = 0; ++a.x; return a.x; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[2], Stmt::Expr(Expr::PreInc(_))));
}

#[test]
fn parse_predec_struct_member() {
    let program =
        parse_source("struct s { int x; }; int main() { struct s a; a.x = 5; --a.x; return a.x; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[2], Stmt::Expr(Expr::PreDec(_))));
}

#[test]
fn parse_postinc_array_element() {
    let program = parse_source("int main() { int a[10]; a[0] = 0; a[0]++; return a[0]; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[2], Stmt::Expr(Expr::PostInc(_))));
}

#[test]
fn parse_preinc_array_element() {
    let program = parse_source("int main() { int a[10]; a[0] = 0; ++a[0]; return a[0]; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[2], Stmt::Expr(Expr::PreInc(_))));
}

#[test]
fn parse_postinc_arrow_member() {
    let program = parse_source(
        "struct s { int x; }; int main() { struct s a; struct s *p = &a; p->x++; return p->x; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[2], Stmt::Expr(Expr::PostInc(_))));
}

#[test]
fn parse_postinc_plain_ident() {
    let program = parse_source("int main() { int i = 0; i++; return i; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[1], Stmt::Expr(Expr::PostInc(_))));
}

#[test]
fn parse_predec_plain_ident() {
    let program = parse_source("int main() { int i = 10; --i; return i; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(&stmts[1], Stmt::Expr(Expr::PreDec(_))));
}

#[test]
fn parse_addr_of_struct_member() {
    let program = parse_source(
        "struct s { int x; int y; }; int main() { struct s a; int *p = &a.y; return *p; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert_eq!(stmts.len(), 3);
    assert!(matches!(
        &stmts[1],
        Stmt::LocalDecl {
            ty: Type::Ptr(_),
            init: Some(Expr::AddrOf(_)),
            ..
        }
    ));
}

#[test]
fn parse_addr_of_struct_first_member() {
    let program = parse_source(
        "struct s { int x; int y; }; int main() { struct s a; int *p = &a.x; return *p; }",
    );
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[1],
        Stmt::LocalDecl {
            ty: Type::Ptr(_),
            init: Some(Expr::AddrOf(_)),
            ..
        }
    ));
}

#[test]
fn parse_addr_of_plain_ident() {
    let program = parse_source("int main() { int x = 42; int *p = &x; return *p; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert!(matches!(
        &stmts[1],
        Stmt::LocalDecl {
            ty: Type::Ptr(_),
            init: Some(Expr::AddrOf(_)),
            ..
        }
    ));
}

#[test]
fn parse_addr_of_deref_simplifies() {
    let program =
        parse_source("int main() { int x = 42; int *p = &x; int **pp = &p; return **pp; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    assert_eq!(stmts.len(), 4);
    // &p should NOT be AddrOf(Deref(Ident("p"))) — should simplify to just Ident("p")
    // But &p is AddrOf(Ident("p")) which is fine — the simplification only applies to &*expr
    assert!(matches!(
        &stmts[2],
        Stmt::LocalDecl {
            ty: Type::Ptr(_),
            init: Some(Expr::AddrOf(_)),
            ..
        }
    ));
}

#[test]
fn parse_addr_of_array_element() {
    let program = parse_source("int main() { int a[10]; int *p = &a[3]; return *p; }");
    let stmts = &program.functions[0].body.as_ref().unwrap().stmts;
    // &a[3] is &*(a+3) which simplifies to (a+3) — a BinOp::Add
    assert!(matches!(
        &stmts[1],
        Stmt::LocalDecl {
            ty: Type::Ptr(_),
            init: Some(Expr::BinOp { .. }),
            ..
        }
    ));
}
