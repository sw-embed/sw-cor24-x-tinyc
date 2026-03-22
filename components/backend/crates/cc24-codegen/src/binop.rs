//! Unary and binary operator code generation.

use cc24_ast::{BinOp, Expr, Type, UnaryOp};

use crate::Codegen;

/// Infer the type of an expression from codegen state.
pub(crate) fn expr_type(cg: &Codegen, expr: &Expr) -> Option<Type> {
    match expr {
        Expr::Ident(name) => {
            let ty = cg
                .local_types
                .get(name)
                .or_else(|| cg.global_types.get(name))
                .cloned()?;
            // Array decays to pointer to element
            match ty {
                Type::Array(inner, _) => Some(Type::Ptr(inner)),
                other => Some(other),
            }
        }
        Expr::Cast { ty, .. } => Some(ty.clone()),
        Expr::AddrOf(name) => {
            let inner = expr_type(cg, &Expr::Ident(name.clone()))?;
            Some(Type::Ptr(Box::new(inner)))
        }
        Expr::Deref(inner) => match expr_type(cg, inner)? {
            Type::Ptr(pointee) => Some(*pointee),
            _ => None,
        },
        Expr::StringLit(_) => Some(Type::Ptr(Box::new(Type::Char))),
        Expr::BinOp {
            op: BinOp::Add,
            lhs,
            ..
        }
        | Expr::BinOp {
            op: BinOp::Sub,
            lhs,
            ..
        } => {
            // Pointer arithmetic preserves pointer type
            let lhs_ty = expr_type(cg, lhs)?;
            if matches!(lhs_ty, Type::Ptr(_)) {
                Some(lhs_ty)
            } else {
                None
            }
        }
        _ => None,
    }
}

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

    pub(crate) fn gen_binop(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) {
        let lhs_ty = expr_type(self, lhs);
        let rhs_ty = expr_type(self, rhs);
        let lhs_is_ptr = matches!(&lhs_ty, Some(Type::Ptr(_)));
        let rhs_is_ptr = matches!(&rhs_ty, Some(Type::Ptr(_)));

        // ptr - ptr: subtract then divide by element size
        if op == BinOp::Sub && lhs_is_ptr && rhs_is_ptr {
            let elem_size = match &lhs_ty {
                Some(Type::Ptr(inner)) => inner.size(),
                _ => 1,
            };
            self.gen_expr(lhs);
            self.emit("        push    r0");
            self.gen_expr(rhs);
            self.emit("        mov     r1,r0");
            self.emit("        pop     r0");
            self.emit("        sub     r0,r1");
            if elem_size > 1 {
                self.needs_div = true;
                self.emit("        push    r0");
                self.load_immediate(elem_size);
                self.emit("        push    r0");
                // stack: dividend, divisor
                // swap order: dividend first
                self.emit("        pop     r1"); // divisor
                self.emit("        pop     r0"); // dividend
                self.emit("        push    r1"); // arg2
                self.emit("        push    r0"); // arg1
                self.emit("        la      r0,__cc24_div");
                self.emit("        jal     r1,(r0)");
                self.emit("        add     sp,6");
            }
            return;
        }

        // ptr +/- int: scale integer operand by pointee size
        let scale = if matches!(op, BinOp::Add | BinOp::Sub) && lhs_is_ptr {
            match &lhs_ty {
                Some(Type::Ptr(inner)) => inner.size(),
                _ => 1,
            }
        } else {
            1
        };

        self.gen_expr(lhs);
        self.emit("        push    r0");
        self.gen_expr(rhs);
        if scale > 1 {
            self.emit(&format!("        lc      r1,{scale}"));
            self.emit("        mul     r0,r1");
        }
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
            BinOp::Div => {
                self.needs_div = true;
                self.emit_divmod_call("__cc24_div");
            }
            BinOp::Mod => {
                self.needs_mod = true;
                self.emit_divmod_call("__cc24_mod");
            }
        }
    }

    fn emit_divmod_call(&mut self, label: &str) {
        // r0=lhs(dividend), r1=rhs(divisor) already set
        self.emit("        push    r1"); // arg2: divisor
        self.emit("        push    r0"); // arg1: dividend
        self.emit(&format!("        la      r0,{label}"));
        self.emit("        jal     r1,(r0)");
        self.emit("        add     sp,6"); // clean 2 args
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
