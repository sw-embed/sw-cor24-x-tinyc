//! COR24 assembly code generator.

mod binop;
mod emit;
mod expr;
mod func;
mod runtime;
mod stmt;

use std::collections::{HashMap, HashSet};

use cc24_ast::{Program, Type};

/// Code generator state.
#[derive(Default)]
pub struct Codegen {
    pub(crate) out: String,
    pub(crate) label_counter: usize,
    pub(crate) locals: HashMap<String, i32>,
    pub(crate) local_types: HashMap<String, Type>,
    pub(crate) locals_size: i32,
    pub(crate) globals: HashSet<String>,
    pub(crate) global_types: HashMap<String, Type>,
    pub(crate) return_label: String,
    pub(crate) string_literals: Vec<String>,
    pub(crate) needs_div: bool,
    pub(crate) needs_mod: bool,
    pub(crate) break_labels: Vec<String>,
    pub(crate) continue_labels: Vec<String>,
}

impl Codegen {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate COR24 assembly for a program.
    pub fn generate(&mut self, program: &Program) -> String {
        for g in &program.globals {
            self.globals.insert(g.name.clone());
            self.global_types.insert(g.name.clone(), g.ty.clone());
        }
        self.emit("        .text");
        self.emit("");
        self.emit_start();
        for func in &program.functions {
            self.emit("");
            self.gen_function(func);
        }
        runtime::emit_runtime(self);
        self.emit_data_section(program);
        self.out.clone()
    }
}
