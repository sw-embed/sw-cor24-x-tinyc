//! Unary and binary operator code generation.

use cc24_ast::{BinOp, UnaryOp};

use crate::Codegen;

impl Codegen {
    pub(crate) fn gen_unary(&mut self, op: UnaryOp, operand: &cc24_ast::Expr) {
        self.gen_expr(operand);
        match op {
            UnaryOp::Neg => {
                self.emit("        push    r0");
                self.emit("        lc      r0,0");
                self.emit("        pop     r1");
                self.emit("        sub     r0,r1");
            }
            UnaryOp::BitNot => {
                self.emit("        lc      r1,-1");
                self.emit("        xor     r0,r1");
            }
            UnaryOp::LogNot => {
                self.emit("        ceq     r0,z");
                self.emit("        mov     r0,c");
            }
        }
    }

    pub(crate) fn gen_binop(&mut self, op: BinOp, lhs: &cc24_ast::Expr, rhs: &cc24_ast::Expr) {
        self.gen_expr(lhs);
        self.emit("        push    r0");
        self.gen_expr(rhs);
        self.emit("        mov     r1,r0"); // r1 = rhs
        self.emit("        pop     r0"); // r0 = lhs
        self.emit_binop(op);
    }

    fn emit_binop(&mut self, op: BinOp) {
        match op {
            BinOp::Add => self.emit("        add     r0,r1"),
            BinOp::Sub => self.emit("        sub     r0,r1"),
            BinOp::Mul => self.emit("        mul     r0,r1"),
            BinOp::BitAnd => self.emit("        and     r0,r1"),
            BinOp::BitOr => self.emit("        or      r0,r1"),
            BinOp::BitXor => self.emit("        xor     r0,r1"),
            BinOp::Shl => self.emit("        shl     r0,r1"),
            BinOp::Shr => self.emit("        srl     r0,r1"),
            BinOp::Eq | BinOp::Ne => self.emit_cmp_eq(op),
            BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => self.emit_cmp_rel(op),
            BinOp::Div => self.emit("        ; TODO: software divide r0/r1"),
            BinOp::Mod => self.emit("        ; TODO: software modulo r0%r1"),
        }
    }

    fn emit_cmp_eq(&mut self, op: BinOp) {
        self.emit("        ceq     r0,r1");
        self.emit("        mov     r0,c");
        if op == BinOp::Ne {
            self.emit("        ceq     r0,z");
            self.emit("        mov     r0,c");
        }
    }

    fn emit_cmp_rel(&mut self, op: BinOp) {
        match op {
            BinOp::Lt => {
                self.emit("        cls     r0,r1");
                self.emit("        mov     r0,c");
            }
            BinOp::Gt => {
                self.emit("        cls     r1,r0");
                self.emit("        mov     r0,c");
            }
            BinOp::Le => {
                self.emit("        cls     r1,r0");
                self.emit("        mov     r0,c");
                self.emit("        ceq     r0,z");
                self.emit("        mov     r0,c");
            }
            BinOp::Ge => {
                self.emit("        cls     r0,r1");
                self.emit("        mov     r0,c");
                self.emit("        ceq     r0,z");
                self.emit("        mov     r0,c");
            }
            _ => unreachable!(),
        }
    }
}
