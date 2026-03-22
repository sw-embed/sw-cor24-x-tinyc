//! Function prologue, epilogue, and local variable collection.

use cc24_ast::{Function, Stmt};

use crate::Codegen;

impl Codegen {
    pub(crate) fn gen_function(&mut self, func: &Function) {
        self.locals.clear();
        self.locals_size = 0;
        self.return_label = self.new_label();

        self.collect_locals_block(&func.body.stmts);
        for (i, param) in func.params.iter().enumerate() {
            let offset = 9 + (i as i32) * 3;
            self.locals.insert(param.name.clone(), offset);
        }

        self.emit(&format!("        .globl  _{}", func.name));
        self.emit(&format!("_{}:", func.name));
        self.emit_prologue();

        for stmt in &func.body.stmts {
            self.gen_stmt(stmt);
        }

        self.emit_epilogue();
    }

    fn emit_prologue(&mut self) {
        self.emit("        push    fp");
        self.emit("        push    r2");
        self.emit("        push    r1");
        self.emit("        mov     fp,sp");
        if self.locals_size > 0 {
            if self.locals_size <= 127 {
                self.emit(&format!("        add     sp,-{}", self.locals_size));
            } else {
                self.emit(&format!("        sub     sp,{}", self.locals_size));
            }
        }
    }

    fn emit_epilogue(&mut self) {
        let ret_label = self.return_label.clone();
        self.emit(&format!("{ret_label}:"));
        self.emit("        mov     sp,fp");
        self.emit("        pop     r1");
        self.emit("        pop     r2");
        self.emit("        pop     fp");
        self.emit("        jmp     (r1)");
    }

    pub(crate) fn collect_locals_block(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.collect_locals_stmt(stmt);
        }
    }
}

impl Codegen {
    fn collect_locals_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalDecl { name, .. } => {
                if !self.locals.contains_key(name) {
                    self.locals_size += 3;
                    let offset = -self.locals_size;
                    self.locals.insert(name.clone(), offset);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                self.collect_locals_block(&then_body.stmts);
                if let Some(eb) = else_body {
                    self.collect_locals_block(&eb.stmts);
                }
            }
            Stmt::While { body, .. } => {
                self.collect_locals_block(&body.stmts);
            }
            Stmt::For { init, body, .. } => {
                if let Some(init_stmt) = init {
                    self.collect_locals_stmt(init_stmt);
                }
                self.collect_locals_block(&body.stmts);
            }
            _ => {}
        }
    }
}
