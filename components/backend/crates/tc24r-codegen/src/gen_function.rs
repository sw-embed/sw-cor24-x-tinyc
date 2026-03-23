use tc24r_ast::Function;
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::{emit, new_label};
use tc24r_struct_isr::{emit_isr_epilogue, emit_isr_prologue};
use tc24r_struct_locals::collect_locals_block;
use tc24r_struct_prologue::{emit_epilogue, emit_prologue};

use crate::gen_stmt::gen_stmt;

pub fn gen_function(state: &mut CodegenState, func: &Function) {
    state.locals.clear();
    state.local_types.clear();
    state.locals_size = 0;
    state.return_label = new_label(state);

    collect_locals_block(state, &func.body.stmts);
    for (i, param) in func.params.iter().enumerate() {
        let offset = 9 + (i as i32) * 3;
        state.locals.insert(param.name.clone(), offset);
        state
            .local_types
            .insert(param.name.clone(), param.ty.clone());
    }

    let name = &func.name;
    emit(state, &format!("        .globl  _{name}"));
    emit(state, &format!("_{name}:"));

    if func.is_interrupt {
        emit_isr_prologue(state);
    } else {
        emit_prologue(state);
    }

    for stmt in &func.body.stmts {
        gen_stmt(stmt, state);
    }

    if func.is_interrupt {
        emit_isr_epilogue(state);
    } else {
        emit_epilogue(state);
    }
}
