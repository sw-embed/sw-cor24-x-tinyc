//! COR24 assembly code generator.

mod binop;
mod emit;
mod expr;
mod func;
mod stmt;

use std::collections::{HashMap, HashSet};

use cc24_ast::Program;

/// Code generator state.
#[derive(Default)]
pub struct Codegen {
    pub(crate) out: String,
    pub(crate) label_counter: usize,
    pub(crate) locals: HashMap<String, i32>,
    pub(crate) locals_size: i32,
    pub(crate) globals: HashSet<String>,
    pub(crate) return_label: String,
}

impl Codegen {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate COR24 assembly for a program.
    pub fn generate(&mut self, program: &Program) -> String {
        for g in &program.globals {
            self.globals.insert(g.name.clone());
        }
        self.emit("        .text");
        self.emit("");
        self.emit_start();
        for func in &program.functions {
            self.emit("");
            self.gen_function(func);
        }
        self.emit_globals(program);
        self.out.clone()
    }
}
