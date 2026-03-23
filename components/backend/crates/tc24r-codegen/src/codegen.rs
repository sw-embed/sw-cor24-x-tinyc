use tc24r_ast::Program;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::emit;
use tc24r_emit_data::{emit_data_section, emit_start};
use tc24r_ops_divmod::emit_divmod_runtime;

use crate::gen_function::gen_function;

pub struct Codegen {
    pub state: CodegenState,
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            state: CodegenState::default(),
        }
    }

    pub fn generate(&mut self, program: &Program) -> String {
        for g in &program.globals {
            self.state.globals.insert(g.name.clone());
            self.state.global_types.insert(g.name.clone(), g.ty.clone());
        }
        self.state.struct_types = program.struct_types.clone();
        emit(&mut self.state, "        .text");
        emit(&mut self.state, "");
        emit_start(&mut self.state);
        for func in &program.functions {
            if func.body.is_none() {
                continue; // skip prototypes
            }
            emit(&mut self.state, "");
            gen_function(&mut self.state, func);
        }
        emit_divmod_runtime(&mut self.state);
        emit_data_section(&mut self.state, program);
        self.state.out.clone()
    }
}

impl Default for Codegen {
    fn default() -> Self {
        Self::new()
    }
}
