//! Expression code generation.

use cc24_ast::{Expr, Type};

use crate::Codegen;
use crate::binop::expr_type;

impl Codegen {
    /// Generate code for an expression. Result is left in r0.
    pub(crate) fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLit(val) => self.load_immediate(*val),
            Expr::StringLit(s) => {
                let idx = self.string_literals.len();
                self.string_literals.push(s.clone());
                self.emit(&format!("        la      r0,_S{idx}"));
            }
            Expr::Ident(name) => self.gen_load(name),
            Expr::Assign { name, value } => self.gen_assign(name, value),
            Expr::Call { name, args } => self.gen_call(name, args),
            Expr::UnaryOp { op, operand } => self.gen_unary(*op, operand),
            Expr::BinOp { op, lhs, rhs } => self.gen_binop(*op, lhs, rhs),
            Expr::AddrOf(name) => self.gen_addr_of(name),
            Expr::Deref(ptr) => self.gen_deref(ptr),
            Expr::Cast { expr, .. } => self.gen_expr(expr),
            Expr::DerefAssign { ptr, value } => self.gen_deref_assign(ptr, value),
            Expr::PreInc(name) | Expr::PreDec(name) | Expr::PostInc(name) | Expr::PostDec(name) => {
                let delta = match expr {
                    Expr::PreInc(_) | Expr::PostInc(_) => 1,
                    _ => -1,
                };
                let post = matches!(expr, Expr::PostInc(_) | Expr::PostDec(_));
                crate::runtime::gen_inc_dec(self, name, delta, post);
            }
        }
    }

    fn gen_load(&mut self, name: &str) {
        // Array names decay to pointer to first element
        if let Some(Type::Array(..)) = self.local_types.get(name) {
            self.gen_addr_of(name);
            return;
        }
        if self.globals.contains(name) {
            let is_char = self.global_types.get(name) == Some(&Type::Char);
            self.emit(&format!("        la      r1,_{name}"));
            if is_char {
                self.emit("        lbu     r0,0(r1)");
            } else {
                self.emit("        lw      r0,0(r1)");
            }
        } else {
            let offset = self.locals[name];
            self.emit(&format!("        lw      r0,{offset}(fp)"));
        }
    }

    fn gen_assign(&mut self, name: &str, value: &Expr) {
        self.gen_expr(value);
        if self.globals.contains(name) {
            let is_char = self.global_types.get(name) == Some(&Type::Char);
            self.emit(&format!("        la      r1,_{name}"));
            if is_char {
                self.emit("        sb      r0,0(r1)");
            } else {
                self.emit("        sw      r0,0(r1)");
            }
        } else {
            let offset = self.locals[name];
            self.emit(&format!("        sw      r0,{offset}(fp)"));
        }
    }

    fn gen_addr_of(&mut self, name: &str) {
        if self.globals.contains(name) {
            self.emit(&format!("        la      r0,_{name}"));
        } else {
            let offset = self.locals[name];
            // r0 = fp + offset
            self.load_immediate(offset);
            self.emit("        add     r0,fp");
        }
    }

    fn gen_deref(&mut self, ptr: &Expr) {
        let is_byte = self.is_char_ptr(ptr);
        self.gen_expr(ptr);
        if is_byte {
            self.emit("        lbu     r0,0(r0)");
        } else {
            self.emit("        lw      r0,0(r0)");
        }
    }

    fn gen_deref_assign(&mut self, ptr: &Expr, value: &Expr) {
        let is_byte = self.is_char_ptr(ptr);
        self.gen_expr(value);
        self.emit("        push    r0");
        self.gen_expr(ptr);
        self.emit("        mov     r1,r0"); // r1 = address
        self.emit("        pop     r0"); // r0 = value
        if is_byte {
            self.emit("        sb      r0,0(r1)");
        } else {
            self.emit("        sw      r0,0(r1)");
        }
    }

    fn is_char_ptr(&self, ptr: &Expr) -> bool {
        matches!(
            expr_type(self, ptr),
            Some(Type::Ptr(inner)) if *inner == Type::Char
        )
    }

    fn gen_call(&mut self, name: &str, args: &[Expr]) {
        for arg in args.iter().rev() {
            self.gen_expr(arg);
            self.emit("        push    r0");
        }
        self.emit(&format!("        la      r0,_{name}"));
        self.emit("        jal     r1,(r0)");
        if !args.is_empty() {
            let cleanup = args.len() as i32 * 3;
            self.emit(&format!("        add     sp,{cleanup}"));
        }
    }
}
