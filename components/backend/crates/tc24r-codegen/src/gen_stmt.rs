use tc24r_ast::Stmt;
use tc24r_codegen_state::CodegenState;

use crate::gen_expr::gen_expr;

pub fn gen_stmt(stmt: &Stmt, state: &mut CodegenState) {
    match stmt {
        Stmt::Return(expr) => tc24r_stmt_simple::gen_return(state, expr, gen_expr),
        Stmt::Expr(expr) => tc24r_stmt_simple::gen_expr_stmt(state, expr, gen_expr),
        Stmt::LocalDecl { name, init, .. } => {
            tc24r_stmt_simple::gen_local_decl(state, name, init.as_ref(), gen_expr)
        }
        Stmt::If {
            cond,
            then_body,
            else_body,
        } => tc24r_stmt_control::gen_if(
            state,
            cond,
            then_body,
            else_body.as_ref(),
            gen_expr,
            gen_stmt,
        ),
        Stmt::While { cond, body } => {
            tc24r_stmt_control::gen_while(state, cond, body, gen_expr, gen_stmt)
        }
        Stmt::DoWhile { body, cond } => {
            tc24r_stmt_control::gen_do_while(state, body, cond, gen_expr, gen_stmt)
        }
        Stmt::For {
            init,
            cond,
            inc,
            body,
        } => tc24r_stmt_control::gen_for(
            state,
            init.as_deref(),
            cond.as_ref(),
            inc.as_ref(),
            body,
            gen_expr,
            gen_stmt,
        ),
        Stmt::Switch {
            expr,
            cases,
            default,
        } => tc24r_stmt_control::gen_switch(
            state,
            expr,
            cases,
            default.as_deref(),
            gen_expr,
            gen_stmt,
        ),
        Stmt::Break => tc24r_stmt_simple::gen_break(state),
        Stmt::Continue => tc24r_stmt_simple::gen_continue(state),
        Stmt::Asm(text) => tc24r_stmt_simple::gen_asm(state, text),
        Stmt::Block(block) => {
            for s in &block.stmts {
                gen_stmt(s, state);
            }
        }
    }
}
