use cc24_ast::Stmt;
use cc24_codegen_state::CodegenState;

use crate::gen_expr::gen_expr;

pub fn gen_stmt(stmt: &Stmt, state: &mut CodegenState) {
    match stmt {
        Stmt::Return(expr) => cc24_stmt_simple::gen_return(state, expr, gen_expr),
        Stmt::Expr(expr) => cc24_stmt_simple::gen_expr_stmt(state, expr, gen_expr),
        Stmt::LocalDecl { name, init, .. } => {
            cc24_stmt_simple::gen_local_decl(state, name, init.as_ref(), gen_expr)
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => cc24_stmt_control::gen_if(
            state,
            cond,
            then_body,
            else_body.as_ref(),
            gen_expr,
            gen_stmt,
        ),
        Stmt::While { cond, body } => {
            cc24_stmt_control::gen_while(state, cond, body, gen_expr, gen_stmt)
        }
        Stmt::DoWhile { body, cond } => {
            cc24_stmt_control::gen_do_while(state, body, cond, gen_expr, gen_stmt)
        }
        Stmt::For {
            init,
            cond,
            inc,
            body,
        } => cc24_stmt_control::gen_for(
            state,
            init.as_deref(),
            cond.as_ref(),
            inc.as_ref(),
            body,
            gen_expr,
            gen_stmt,
        ),
        Stmt::Break => cc24_stmt_simple::gen_break(state),
        Stmt::Continue => cc24_stmt_simple::gen_continue(state),
        Stmt::Asm(text) => cc24_stmt_simple::gen_asm(state, text),
    }
}
