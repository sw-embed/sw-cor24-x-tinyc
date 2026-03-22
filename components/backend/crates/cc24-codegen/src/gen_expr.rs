use cc24_ast::Expr;
use cc24_codegen_state::CodegenState;

pub fn gen_expr(expr: &Expr, state: &mut CodegenState) {
    match expr {
        Expr::IntLit(val) => cc24_expr_literal::gen_int_lit(state, *val),
        Expr::StringLit(s) => cc24_expr_literal::gen_string_lit(state, s),
        Expr::Ident(name) => cc24_expr_variable::gen_ident(state, name),
        Expr::Assign { name, value } => {
            cc24_expr_variable::gen_assign(state, name, value, gen_expr)
        }
        Expr::Call { name, args } => cc24_expr_call::gen_call(state, name, args, gen_expr),
        Expr::UnaryOp { op, operand } => {
            cc24_expr_ops::gen_unary_dispatch(state, *op, operand, gen_expr)
        }
        Expr::BinOp { op, lhs, rhs } => {
            cc24_expr_ops::gen_binop_dispatch(state, *op, lhs, rhs, gen_expr)
        }
        Expr::AddrOf(name) => cc24_expr_variable::gen_addr_of(state, name),
        Expr::Deref(ptr) => cc24_expr_pointer::gen_deref(state, ptr, gen_expr),
        Expr::Cast { expr: inner, .. } => gen_expr(inner, state),
        Expr::DerefAssign { ptr, value } => {
            cc24_expr_pointer::gen_deref_assign(state, ptr, value, gen_expr)
        }
        Expr::PreInc(name) | Expr::PreDec(name) | Expr::PostInc(name) | Expr::PostDec(name) => {
            let delta = match expr {
                Expr::PreInc(_) | Expr::PostInc(_) => 1,
                _ => -1,
            };
            let post = matches!(expr, Expr::PostInc(_) | Expr::PostDec(_));
            cc24_ops_incdec::gen_inc_dec(state, name, delta, post);
        }
    }
}
