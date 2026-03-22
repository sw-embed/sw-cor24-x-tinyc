//! Expression code generation.

use cc24_ast::Expr;

use crate::Codegen;

impl Codegen {
    /// Generate code for an expression. Result is left in r0.
    pub(crate) fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLit(val) => self.load_immediate(*val),
            Expr::StringLit(_) => {}
            Expr::Ident(name) => self.gen_load(name),
            Expr::Assign { name, value } => self.gen_assign(name, value),
            Expr::Call { name, args } => self.gen_call(name, args),
            Expr::UnaryOp { op, operand } => self.gen_unary(*op, operand),
            Expr::BinOp { op, lhs, rhs } => self.gen_binop(*op, lhs, rhs),
        }
    }

    fn gen_load(&mut self, name: &str) {
        if self.globals.contains(name) {
            self.emit(&format!("        la      r1,_{name}"));
            self.emit("        lw      r0,0(r1)");
        } else {
            let offset = self.locals[name];
            self.emit(&format!("        lw      r0,{offset}(fp)"));
        }
    }

    fn gen_assign(&mut self, name: &str, value: &Expr) {
        self.gen_expr(value);
        if self.globals.contains(name) {
            self.emit(&format!("        la      r1,_{name}"));
            self.emit("        sw      r0,0(r1)");
        } else {
            let offset = self.locals[name];
            self.emit(&format!("        sw      r0,{offset}(fp)"));
        }
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
