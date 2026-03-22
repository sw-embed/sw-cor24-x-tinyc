//! Output helpers.

use cc24_ast::{Expr, Program, Type};

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

    /// Load a named variable into r0.
    pub(crate) fn gen_load_by_name(&mut self, name: &str) {
        if self.globals.contains(name) {
            self.emit(&format!("        la      r1,_{name}"));
            self.emit("        lw      r0,0(r1)");
        } else {
            let offset = self.locals[name];
            self.emit(&format!("        lw      r0,{offset}(fp)"));
        }
    }

    /// Store r0 into a named variable.
    pub(crate) fn gen_store_by_name(&mut self, name: &str) {
        if self.globals.contains(name) {
            self.emit(&format!("        la      r1,_{name}"));
            self.emit("        sw      r0,0(r1)");
        } else {
            let offset = self.locals[name];
            self.emit(&format!("        sw      r0,{offset}(fp)"));
        }
    }

    /// Emit startup code that calls _main and halts.
    pub(crate) fn emit_start(&mut self) {
        self.emit("        .globl  _start");
        self.emit("_start:");
        self.emit("        la      r0,_main");
        self.emit("        jal     r1,(r0)");
        self.emit("_halt:");
        self.emit("        bra     _halt");
    }

    pub(crate) fn emit_data_section(&mut self, program: &Program) {
        let has_globals = !program.globals.is_empty();
        let has_strings = !self.string_literals.is_empty();
        if !has_globals && !has_strings {
            return;
        }
        self.emit("");
        self.emit("        .data");
        for g in &program.globals {
            self.emit(&format!("_{}:", g.name));
            let val = match &g.init {
                Some(Expr::IntLit(v)) => *v,
                _ => 0,
            };
            if g.ty == Type::Char {
                self.emit(&format!("        .byte   {val}"));
            } else {
                self.emit(&format!("        .word   {val}"));
            }
        }
        for (i, s) in self.string_literals.clone().iter().enumerate() {
            self.emit(&format!("_S{i}:"));
            let bytes: Vec<String> = s
                .bytes()
                .chain(std::iter::once(0))
                .map(|b| b.to_string())
                .collect();
            self.emit(&format!("        .byte   {}", bytes.join(",")));
        }
    }
}
