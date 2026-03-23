//! Basic parser tests: literals, locals, binary ops, control flow.

use tc24r_ast::{BinOp, Expr, Stmt, Type};

use crate::parse_source;

#[test]
fn parse_return_42() {
    let program = parse_source("int main() { return 42; }");
    assert_eq!(program.functions.len(), 1);
    let func = &program.functions[0];
    assert_eq!(func.name, "main");
    assert_eq!(func.return_ty, Type::Int);
    assert!(matches!(
        &func.body.stmts[0],
        Stmt::Return(Expr::IntLit(42))
    ));
}

#[test]
fn parse_local_decl_and_return() {
    let program = parse_source("int main() { int a = 5; return a; }");
    let stmts = &program.functions[0].body.stmts;
    assert_eq!(stmts.len(), 2);
    assert!(matches!(
        &stmts[0],
        Stmt::LocalDecl {
            name,
            ty: Type::Int,
            init: Some(Expr::IntLit(5)),
        } if name == "a"
    ));
    assert!(matches!(
        &stmts[1],
        Stmt::Return(Expr::Ident(n)) if n == "a"
    ));
}

#[test]
fn parse_binary_ops() {
    let program = parse_source("int main() { return 2 + 3 * 4; }");
    let Stmt::Return(expr) = &program.functions[0].body.stmts[0] else {
        panic!()
    };
    assert!(matches!(expr, Expr::BinOp { op: BinOp::Add, .. }));
}

#[test]
fn parse_if_else() {
    let program = parse_source("int main() { if (1) { return 3; } else { return 4; } }");
    assert!(matches!(
        &program.functions[0].body.stmts[0],
        Stmt::If {
            else_body: Some(_),
            ..
        }
    ));
}

#[test]
fn parse_while_loop() {
    let program = parse_source("int main() { int i = 0; while (i < 5) { i = i + 1; } }");
    assert!(matches!(
        &program.functions[0].body.stmts[1],
        Stmt::While { .. }
    ));
}
