//! Statement code generation.

use cc24_ast::Stmt;

use crate::Codegen;

impl Codegen {
    pub(crate) fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Return(expr) => self.gen_return(expr),
            Stmt::Expr(expr) => self.gen_expr(expr),
            Stmt::LocalDecl { name, init, .. } => self.gen_local_decl(name, init.as_ref()),
            Stmt::If {
                cond,
                then_body,
                else_body,
            } => self.gen_if(cond, then_body, else_body.as_ref()),
            Stmt::While { cond, body } => self.gen_while(cond, body),
            Stmt::DoWhile { body, cond } => crate::runtime::gen_do_while(self, body, cond),
            Stmt::For {
                init,
                cond,
                inc,
                body,
            } => self.gen_for(init.as_deref(), cond.as_ref(), inc.as_ref(), body),
            Stmt::Break => {
                if let Some(label) = self.break_labels.last() {
                    self.emit(&format!("        bra     {label}"));
                }
            }
            Stmt::Continue => {
                if let Some(label) = self.continue_labels.last() {
                    self.emit(&format!("        bra     {label}"));
                }
            }
            Stmt::Asm(text) => self.gen_asm(text),
        }
    }

    fn gen_return(&mut self, expr: &cc24_ast::Expr) {
        self.gen_expr(expr);
        let ret_label = self.return_label.clone();
        self.emit(&format!("        bra     {ret_label}"));
    }

    fn gen_local_decl(&mut self, name: &str, init: Option<&cc24_ast::Expr>) {
        if let Some(init_expr) = init {
            self.gen_expr(init_expr);
            let offset = self.locals[name];
            self.emit(&format!("        sw      r0,{offset}(fp)"));
        }
    }

    fn gen_asm(&mut self, text: &str) {
        for line in text.lines() {
            self.emit(&format!("        {line}"));
        }
    }
}

impl Codegen {
    fn gen_if(
        &mut self,
        cond: &cc24_ast::Expr,
        then_body: &cc24_ast::Block,
        else_body: Option<&cc24_ast::Block>,
    ) {
        let else_label = self.new_label();
        let done_label = self.new_label();

        self.gen_expr(cond);
        self.emit("        ceq     r0,z");
        if else_body.is_some() {
            self.emit(&format!("        brt     {else_label}"));
        } else {
            self.emit(&format!("        brt     {done_label}"));
        }

        for s in &then_body.stmts {
            self.gen_stmt(s);
        }

        if let Some(eb) = else_body {
            self.emit(&format!("        bra     {done_label}"));
            self.emit(&format!("{else_label}:"));
            for s in &eb.stmts {
                self.gen_stmt(s);
            }
        }

        self.emit(&format!("{done_label}:"));
    }

    fn gen_while(&mut self, cond: &cc24_ast::Expr, body: &cc24_ast::Block) {
        let loop_label = self.new_label();
        let done_label = self.new_label();
        self.break_labels.push(done_label.clone());
        self.continue_labels.push(loop_label.clone());

        self.emit(&format!("{loop_label}:"));
        self.gen_expr(cond);
        self.emit("        ceq     r0,z");
        self.emit(&format!("        brt     {done_label}"));

        for s in &body.stmts {
            self.gen_stmt(s);
        }
        self.emit(&format!("        bra     {loop_label}"));
        self.emit(&format!("{done_label}:"));
        self.break_labels.pop();
        self.continue_labels.pop();
    }

    fn gen_for(
        &mut self,
        init: Option<&Stmt>,
        cond: Option<&cc24_ast::Expr>,
        inc: Option<&cc24_ast::Expr>,
        body: &cc24_ast::Block,
    ) {
        if let Some(init_stmt) = init {
            self.gen_stmt(init_stmt);
        }

        let loop_label = self.new_label();
        let cont_label = self.new_label();
        let done_label = self.new_label();
        self.break_labels.push(done_label.clone());
        self.continue_labels.push(cont_label.clone());

        self.emit(&format!("{loop_label}:"));
        if let Some(cond_expr) = cond {
            self.gen_expr(cond_expr);
            self.emit("        ceq     r0,z");
            self.emit(&format!("        brt     {done_label}"));
        }

        for s in &body.stmts {
            self.gen_stmt(s);
        }

        self.emit(&format!("{cont_label}:"));
        if let Some(inc_expr) = inc {
            self.gen_expr(inc_expr);
        }

        self.emit(&format!("        bra     {loop_label}"));
        self.emit(&format!("{done_label}:"));
        self.break_labels.pop();
        self.continue_labels.pop();
    }
}
