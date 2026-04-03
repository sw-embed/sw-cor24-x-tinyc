use tc24r_ast::Expr;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::{emit_bra, emit_brt, new_label};
use tc24r_emit_macros::emit;

use crate::gen_stmt::gen_stmt;

pub fn gen_expr(expr: &Expr, state: &mut CodegenState) {
    match expr {
        Expr::IntLit(val) => tc24r_expr_literal::gen_int_lit(state, *val),
        Expr::StringLit(s) => tc24r_expr_literal::gen_string_lit(state, s),
        Expr::Ident(name) => tc24r_expr_variable::gen_ident(state, name),
        Expr::Assign { name, value } => {
            tc24r_expr_variable::gen_assign(state, name, value, gen_expr)
        }
        Expr::Call { name, args } => tc24r_expr_call::gen_call(state, name, args, gen_expr),
        Expr::UnaryOp { op, operand } => {
            tc24r_expr_ops::gen_unary_dispatch(state, *op, operand, gen_expr)
        }
        Expr::BinOp { op, lhs, rhs } => {
            tc24r_expr_ops::gen_binop_dispatch(state, *op, lhs, rhs, gen_expr)
        }
        Expr::AddrOf(name) => tc24r_expr_variable::gen_addr_of(state, name),
        Expr::Deref(ptr) => tc24r_expr_pointer::gen_deref(state, ptr, gen_expr),
        Expr::Cast { ty, expr: inner } => tc24r_expr_pointer::gen_cast(state, ty, inner, gen_expr),
        Expr::DerefAssign { ptr, value } => {
            tc24r_expr_pointer::gen_deref_assign(state, ptr, value, gen_expr)
        }
        Expr::PreInc(operand)
        | Expr::PreDec(operand)
        | Expr::PostInc(operand)
        | Expr::PostDec(operand) => {
            let delta = match expr {
                Expr::PreInc(_) | Expr::PostInc(_) => 1,
                _ => -1,
            };
            let post = matches!(expr, Expr::PostInc(_) | Expr::PostDec(_));
            match operand.as_ref() {
                Expr::Ident(name) => {
                    tc24r_ops_incdec::gen_inc_dec(state, name, delta, post);
                }
                Expr::MemberAccess { object, member } => {
                    tc24r_expr_struct::gen_inc_dec_member(
                        state, object, member, delta, post, gen_expr,
                    );
                }
                Expr::Deref(ptr) => {
                    tc24r_expr_pointer::gen_inc_dec_deref(state, ptr, delta, post, gen_expr);
                }
                _ => {
                    gen_expr(operand, state);
                    if post {
                        emit!(state, "        push    r0");
                    }
                    emit!(state, "        add     r0,{delta}");
                    if post {
                        emit!(state, "        pop     r1");
                    }
                }
            }
        }
        Expr::StmtExpr(block) => {
            for s in &block.stmts {
                gen_stmt(s, state);
            }
        }
        Expr::MemberAccess { object, member } => {
            tc24r_expr_struct::gen_member_access(state, object, member, gen_expr)
        }
        Expr::MemberAssign {
            object,
            member,
            value,
        } => tc24r_expr_struct::gen_member_assign(state, object, member, value, gen_expr),
        Expr::IndirectCall { callee, args } => {
            tc24r_expr_call::gen_indirect_call(state, callee, args, gen_expr)
        }
        Expr::SizeofExpr(inner) => {
            // sizeof must not decay arrays: sizeof(arr) returns full array size
            let ty = match inner.as_ref() {
                Expr::Ident(name) => state
                    .local_types
                    .get(name)
                    .or_else(|| state.global_types.get(name))
                    .cloned(),
                _ => tc24r_type_infer::expr_type(state, inner),
            };
            let size = ty.map(|t| t.size()).unwrap_or(3);
            tc24r_expr_literal::gen_int_lit(state, size);
        }
        Expr::Ternary {
            cond,
            then_expr,
            else_expr,
        } => gen_ternary(state, cond, then_expr, else_expr),
    }
}

fn gen_ternary(state: &mut CodegenState, cond: &Expr, then_expr: &Expr, else_expr: &Expr) {
    let else_label = new_label(state);
    let done_label = new_label(state);

    gen_expr(cond, state);
    emit!(state, "        ceq     r0,z");
    emit_brt(state, &else_label);
    gen_expr(then_expr, state);
    emit_bra(state, &done_label);
    emit!(state, "{else_label}:");
    gen_expr(else_expr, state);
    emit!(state, "{done_label}:");
}
