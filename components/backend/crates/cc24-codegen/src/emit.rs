//! Output helpers.

use cc24_ast::{Expr, Program};

use crate::Codegen;

impl Codegen {
    pub(crate) fn emit(&mut self, line: &str) {
        self.out.push_str(line);
        self.out.push('\n');
    }

    pub(crate) fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    pub(crate) fn load_immediate(&mut self, val: i32) {
        if (-128..=127).contains(&val) {
            self.emit(&format!("        lc      r0,{val}"));
        } else {
            self.emit(&format!("        la      r0,{val}"));
        }
    }

    pub(crate) fn emit_globals(&mut self, program: &Program) {
        if program.globals.is_empty() {
            return;
        }
        self.emit("");
        self.emit("        .data");
        for g in &program.globals {
            self.emit(&format!("_{}:", g.name));
            if let Some(Expr::IntLit(val)) = &g.init {
                self.emit(&format!("        .word   {val}"));
            } else {
                self.emit("        .word   0");
            }
        }
    }
}
