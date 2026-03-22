use cc24_ast::Program;
use cc24_codegen_state::CodegenState;
use cc24_emit_core::emit;
use cc24_emit_data::{emit_data_section, emit_start};
use cc24_ops_divmod::emit_divmod_runtime;

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
        emit(&mut self.state, "        .text");
        emit(&mut self.state, "");
        emit_start(&mut self.state);
        for func in &program.functions {
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
