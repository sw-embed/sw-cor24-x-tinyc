//! Local variable declaration code generation.

use tc24r_ast::{Expr, Type};
use tc24r_codegen_state::CodegenState;
use tc24r_emit_core::fp_store_word_r0;
use tc24r_emit_macros::emit;
use tc24r_type_infer::GenExprFn;

/// Generate code for a local variable declaration with optional initializer.
///
/// If the variable is missing from the locals map (e.g., inside a StmtExpr
/// whose scope was saved/restored), re-inserts it. The always-allocate
/// collect pass ensures there's enough frame space; we just need the
/// correct mapping.
pub fn gen_local_decl(
    state: &mut CodegenState,
    name: &str,
    ty: &Type,
    init: Option<&Expr>,
    gen_expr_fn: GenExprFn,
) {
    // Ensure the variable has a mapping. Inside StmtExpr scopes where
    // the outer map was saved/restored, the name may be missing.
    // The collect pass already reserved frame space via always-allocate,
    // so we allocate at the next available offset (within the frame).
    if !state.locals.contains_key(name) {
        let alloc = ty.size().max(3);
        state.locals_size += alloc;
        let offset = -state.locals_size;
        state.locals.insert(name.to_string(), offset);
        state.local_types.insert(name.to_string(), ty.clone());
    }

    if let Some(init_expr) = init {
        // `char a[N] = "literal";` must copy the bytes into the
        // stack array (zero-padding the rest), not store a pointer
        // to rodata. Without this special case, gen_expr_fn would
        // load the rodata address into r0 and we'd store that
        // address as a single word at slot offset 0 — subscripting
        // `a[i]` would then read bytes of the stored pointer instead
        // of the string.
        if let (Type::Array(elem, count), Expr::StringLit(s)) = (ty, init_expr) {
            if matches!(**elem, Type::Char | Type::UnsignedChar) {
                gen_local_char_array_string_init(state, name, *count, s);
                return;
            }
        }
        gen_expr_fn(init_expr, state);
        let offset = state.locals[name];
        fp_store_word_r0(state, offset);
    }
}

/// Emit per-byte stores for `char a[N] = "literal"` (or `char a[] = "literal"`
/// where the parser already inferred N).
///
/// Writes the literal's bytes plus a null terminator, then zero-pads up to
/// `count` total bytes (matches C semantics for `char a[10] = "abc"` which
/// is 'a','b','c','\0','\0','\0','\0','\0','\0','\0').
fn gen_local_char_array_string_init(
    state: &mut CodegenState,
    name: &str,
    count: usize,
    s: &str,
) {
    let base_offset = state.locals[name];
    let bytes = s.as_bytes();
    let null_terminated_len = bytes.len() + 1;
    for i in 0..count {
        let val: i32 = if i < bytes.len() {
            bytes[i] as i32
        } else if i < null_terminated_len {
            0
        } else {
            0
        };
        let off = base_offset + i as i32;
        emit!(state, "        lc      r0,{val}");
        if (-128..=127).contains(&off) {
            emit!(state, "        sb      r0,{off}(fp)");
        } else {
            emit!(state, "        la      r1,{off}");
            emit!(state, "        add     r1,fp");
            emit!(state, "        sb      r0,0(r1)");
        }
    }
}
