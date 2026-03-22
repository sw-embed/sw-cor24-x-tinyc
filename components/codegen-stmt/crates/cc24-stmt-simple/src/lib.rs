//! Simple statement code generation (Return, Expr, LocalDecl, Break, Continue, Asm).

mod expr_stmt;
mod jump;
mod local_decl;
mod ret;

pub use expr_stmt::gen_expr_stmt;
pub use jump::{gen_asm, gen_break, gen_continue};
pub use local_decl::gen_local_decl;
pub use ret::gen_return;

/// Callback type for recursive statement code generation.
///
/// Used by control-flow statements that contain nested statements.
/// The caller at a higher DAG level passes its `gen_stmt` function.
pub type GenStmtFn = fn(&cc24_ast::Stmt, &mut cc24_codegen_state::CodegenState);
