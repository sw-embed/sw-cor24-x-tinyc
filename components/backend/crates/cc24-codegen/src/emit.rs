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
            if let Some(Expr::IntLit(val)) = &g.init {
                self.emit(&format!("        .word   {val}"));
            } else {
                self.emit("        .word   0");
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
