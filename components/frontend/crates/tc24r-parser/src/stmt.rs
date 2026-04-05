//! Statement and block parsing.

use tc24r_ast::{BinOp, Block, Expr, Stmt, Type};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

use crate::control;
use crate::decl::{parse_const_array_size, parse_type};
use crate::expr::{parse_assign, parse_expr};
use tc24r_parser_types::is_type_start;

/// Parse a brace-delimited block.
pub fn parse_block(ts: &mut TokenStream) -> Result<Block, CompileError> {
    ts.expect(TokenKind::LBrace)?;
    let mut stmts = Vec::new();
    while !ts.check(&TokenKind::RBrace) {
        stmts.push(parse_stmt(ts)?);
    }
    ts.expect(TokenKind::RBrace)?;
    Ok(Block { stmts })
}

/// Parse either a brace-delimited block or a single statement.
/// Enables braceless control flow: `if (x) stmt;`
pub fn parse_body(ts: &mut TokenStream) -> Result<Block, CompileError> {
    if ts.check(&TokenKind::LBrace) {
        parse_block(ts)
    } else {
        let stmt = parse_stmt(ts)?;
        Ok(Block { stmts: vec![stmt] })
    }
}

/// Parse a single statement.
pub fn parse_stmt(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    // Null statement: bare `;`
    if ts.eat(TokenKind::Semicolon) {
        return Ok(Stmt::Expr(Expr::IntLit(0)));
    }
    // Bare block: { ... } used as scope
    if ts.check(&TokenKind::LBrace) {
        let block = parse_block(ts)?;
        return Ok(Stmt::Block(block));
    }
    if ts.eat(TokenKind::Return) {
        return parse_return(ts);
    }
    if ts.eat(TokenKind::If) {
        return control::parse_if(ts);
    }
    if ts.eat(TokenKind::While) {
        return control::parse_while(ts);
    }
    if ts.eat(TokenKind::For) {
        return control::parse_for(ts);
    }
    if ts.eat(TokenKind::Do) {
        return control::parse_do_while(ts);
    }
    if ts.eat(TokenKind::Switch) {
        return control::parse_switch(ts);
    }
    if ts.eat(TokenKind::Break) {
        ts.expect(TokenKind::Semicolon)?;
        return Ok(Stmt::Break);
    }
    if ts.eat(TokenKind::Continue) {
        ts.expect(TokenKind::Semicolon)?;
        return Ok(Stmt::Continue);
    }
    if ts.eat(TokenKind::Goto) {
        let name = ts.expect_ident()?;
        ts.expect(TokenKind::Semicolon)?;
        return Ok(Stmt::Goto(name));
    }
    // Label: `ident:` (but not `default:` or `case X:`)
    if let TokenKind::Ident(_) = ts.peek().kind {
        if matches!(ts.lookahead(1), TokenKind::Colon) {
            let name = ts.expect_ident()?;
            ts.expect(TokenKind::Colon)?;
            return Ok(Stmt::Label(name));
        }
    }
    if ts.eat(TokenKind::Asm) {
        return control::parse_asm(ts);
    }
    if ts.check(&TokenKind::Enum) && is_enum_definition(ts) {
        ts.advance(); // consume `enum`
        return tc24r_parser_enum::parse_enum_decl(ts);
    }
    if ts.eat(TokenKind::Typedef) {
        return tc24r_parser_typedef::parse_typedef_decl(ts);
    }
    if is_type_start(ts) {
        return parse_local_decl(ts);
    }
    let expr = parse_expr(ts)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Expr(expr))
}

/// Check if `enum` starts a definition (`enum {` or `enum tag {`)
/// as opposed to a variable declaration (`enum tag x;`).
fn is_enum_definition(ts: &TokenStream) -> bool {
    // Current token is Enum
    match ts.lookahead(1) {
        TokenKind::LBrace => true, // enum { ... }
        TokenKind::Ident(_) => {
            matches!(ts.lookahead(2), TokenKind::LBrace) // enum tag { ... }
        }
        _ => false,
    }
}

fn parse_return(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    // void return: return;
    if ts.eat(TokenKind::Semicolon) {
        return Ok(Stmt::Return(Expr::IntLit(0)));
    }
    let expr = parse_expr(ts)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Return(expr))
}

pub fn parse_local_decl(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    let base_ty = parse_type(ts)?;
    // Standalone struct definition: `struct tag { ... };`
    if ts.check(&TokenKind::Semicolon) {
        ts.advance();
        return Ok(Stmt::Expr(Expr::IntLit(0)));
    }
    let first = parse_one_declarator(ts, base_ty.clone())?;
    // Check for comma-separated additional declarators
    if !ts.check(&TokenKind::Comma) {
        ts.expect(TokenKind::Semicolon)?;
        return Ok(first);
    }
    let mut stmts = vec![first];
    while ts.eat(TokenKind::Comma) {
        stmts.push(parse_one_declarator(ts, base_ty.clone())?);
    }
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Block(Block { stmts }))
}

/// Parse a single variable declarator (name, optional array suffix,
/// optional initializer). Does NOT consume a trailing semicolon --
/// the caller (parse_local_decl) handles that after all declarators
/// are parsed. This separation is critical for multi-declarations
/// like `int x = 1, y = 2;` where commas separate declarators.
fn parse_one_declarator(ts: &mut TokenStream, base_ty: Type) -> Result<Stmt, CompileError> {
    // Handle pointer stars on each declarator: int *p, *q;
    let mut ty = base_ty;
    while ts.eat(TokenKind::Star) {
        ty = Type::Ptr(Box::new(ty));
    }
    // Parenthesized declarator: (*name)[N] (pointer to array) or (*fp)(params) (function pointer)
    if ts.check(&TokenKind::LParen) && matches!(ts.lookahead(1), TokenKind::Star) {
        // Peek ahead to distinguish: after (*name), is it [ (ptr-to-array) or ( (fn ptr)?
        // Find the matching ) after (*name...
        let after_star = ts.lookahead(2); // token after *
        match after_star {
            TokenKind::Ident(_) => {
                // (*name) — check what follows the closing )
                // lookahead(3) should be RParen for (*name)
                if matches!(ts.lookahead(3), TokenKind::RParen) {
                    if matches!(ts.lookahead(4), TokenKind::LBracket) {
                        // (*name)[N] — pointer to array
                        return parse_ptr_to_array_declarator(ts, ty);
                    }
                }
                // Otherwise: function pointer
                return parse_fn_ptr_declarator(ts, ty);
            }
            _ => return parse_fn_ptr_declarator(ts, ty),
        }
    }
    // Parenthesized declarator without star: (name)[N] equivalent to name[N]
    if ts.check(&TokenKind::LParen) && matches!(ts.lookahead(1), TokenKind::Ident(_)) {
        if matches!(ts.lookahead(2), TokenKind::RParen) {
            // (name) — skip parens, parse as normal declarator
            ts.advance(); // (
            let name = ts.expect_ident()?;
            ts.expect(TokenKind::RParen)?;
            // Continue with array suffix parsing below
            let mut ty = ty;
            let mut implicit_size = false;
            let mut dims = Vec::new();
            while ts.eat(TokenKind::LBracket) {
                if ts.check(&TokenKind::RBracket) {
                    ts.expect(TokenKind::RBracket)?;
                    dims.push(0usize);
                    implicit_size = true;
                } else {
                    let size = parse_const_array_size(ts)?;
                    ts.expect(TokenKind::RBracket)?;
                    dims.push(size);
                }
            }
            for &size in dims.iter().rev() {
                ty = Type::Array(Box::new(ty), size);
            }
            if ts.eat(TokenKind::Assign) {
                if ts.check(&TokenKind::LBrace) {
                    return parse_brace_init(ts, name, ty, implicit_size);
                }
                let init = Some(parse_assign(ts)?);
                return Ok(Stmt::LocalDecl { name, ty, init });
            }
            return Ok(Stmt::LocalDecl {
                name,
                ty,
                init: None,
            });
        }
    }
    let name = ts.expect_ident()?;
    // Local function prototype: int foo(int x); — accept and ignore
    if ts.check(&TokenKind::LParen) {
        ts.advance();
        skip_fn_ptr_params(ts)?;
        ts.expect(TokenKind::RParen)?;
        return Ok(Stmt::Expr(Expr::IntLit(0)));
    }
    // Check for array: int a[N], int a[N][M], or int a[] = {...}
    // Collect dimensions left-to-right, then apply in reverse so that
    // int a[2][3] → Array(Array(Int, 3), 2), not Array(Array(Int, 2), 3).
    let mut ty = ty;
    let mut implicit_size = false;
    let mut dims = Vec::new();
    while ts.eat(TokenKind::LBracket) {
        if ts.check(&TokenKind::RBracket) {
            ts.expect(TokenKind::RBracket)?;
            dims.push(0usize);
            implicit_size = true;
        } else {
            let size = parse_const_array_size(ts)?;
            ts.expect(TokenKind::RBracket)?;
            dims.push(size);
        }
    }
    for &size in dims.iter().rev() {
        ty = Type::Array(Box::new(ty), size);
    }
    // Struct/array brace initializer: struct s x = {1, 2};
    if ts.eat(TokenKind::Assign) {
        if ts.check(&TokenKind::LBrace) {
            return parse_brace_init(ts, name, ty, implicit_size);
        }
        // char s[] = "hello" — infer size from string length + null
        if implicit_size {
            if let Type::Array(elem, 0) = &ty {
                if let TokenKind::StringLit(s) = &ts.peek().kind {
                    let len = s.len() + 1; // +1 for null terminator
                    ty = Type::Array(elem.clone(), len);
                }
            }
        }
        let init = Some(parse_assign(ts)?);
        return Ok(Stmt::LocalDecl { name, ty, init });
    }
    Ok(Stmt::LocalDecl {
        name,
        ty,
        init: None,
    })
}

/// Parse a pointer-to-array declarator: (*name)[N]
/// `char (*y)[3]` produces LocalDecl { name: "y", ty: Ptr(Array(Char, 3)) }.
fn parse_ptr_to_array_declarator(
    ts: &mut TokenStream,
    elem_ty: Type,
) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?; // (
    ts.expect(TokenKind::Star)?; // *
    let name = ts.expect_ident()?;
    ts.expect(TokenKind::RParen)?; // )
                                   // Parse array dimensions: [N], [N][M], etc.
    let mut dims = Vec::new();
    while ts.eat(TokenKind::LBracket) {
        if ts.check(&TokenKind::RBracket) {
            ts.expect(TokenKind::RBracket)?;
            dims.push(0usize);
        } else {
            let size = parse_const_array_size(ts)?;
            ts.expect(TokenKind::RBracket)?;
            dims.push(size);
        }
    }
    // Build type: Ptr(Array(elem_ty, dims))
    let mut inner = elem_ty;
    for &size in dims.iter().rev() {
        inner = Type::Array(Box::new(inner), size);
    }
    let ty = Type::Ptr(Box::new(inner));
    // Optional initializer
    let init = if ts.eat(TokenKind::Assign) {
        Some(parse_assign(ts)?)
    } else {
        None
    };
    Ok(Stmt::LocalDecl { name, ty, init })
}

/// Parse a function pointer declarator: (*name)(params) or (*name[N])(params)
/// Produces LocalDecl with Ptr(return_ty) or Array(Ptr(return_ty), N).
fn parse_fn_ptr_declarator(ts: &mut TokenStream, return_ty: Type) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?; // (
    ts.expect(TokenKind::Star)?; // *
    let name = ts.expect_ident()?;
    // Optional array suffix: (*table[4])
    let mut is_array = None;
    if ts.eat(TokenKind::LBracket) {
        let size = parse_const_array_size(ts)?;
        ts.expect(TokenKind::RBracket)?;
        is_array = Some(size);
    }
    ts.expect(TokenKind::RParen)?; // )
                                   // Consume parameter list: (int, int, ...)
    ts.expect(TokenKind::LParen)?;
    skip_fn_ptr_params(ts)?;
    ts.expect(TokenKind::RParen)?;
    let ptr_ty = Type::Ptr(Box::new(return_ty));
    let ty = if let Some(size) = is_array {
        Type::Array(Box::new(ptr_ty), size)
    } else {
        ptr_ty
    };
    // Optional initializer
    let init = if ts.eat(TokenKind::Assign) {
        Some(parse_expr(ts)?)
    } else {
        None
    };
    Ok(Stmt::LocalDecl { name, ty, init })
}

/// Skip a function pointer parameter list (balanced parens).
pub(crate) fn skip_fn_ptr_params(ts: &mut TokenStream) -> Result<(), CompileError> {
    if ts.check(&TokenKind::RParen) {
        return Ok(());
    }
    loop {
        while !ts.check(&TokenKind::RParen) && !ts.check(&TokenKind::Comma) {
            if ts.eat(TokenKind::LParen) {
                skip_fn_ptr_params(ts)?;
                ts.expect(TokenKind::RParen)?;
            } else {
                ts.advance();
            }
        }
        if !ts.eat(TokenKind::Comma) {
            break;
        }
    }
    Ok(())
}

/// Parse brace initializer for a local variable.
/// Handles nested braces, designated initializers, flat fill, and unions.
/// Desugars to: LocalDecl + zero-fill + byte-offset stores.
fn parse_brace_init(
    ts: &mut TokenStream,
    name: String,
    ty: Type,
    implicit_size: bool,
) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LBrace)?;
    let mut stores = Vec::new();
    let max_top_idx = init_braced(ts, &ty, 0, &mut stores)?;
    ts.expect(TokenKind::RBrace)?;

    // Infer array size from highest element index
    let ty = if implicit_size {
        if let Type::Array(elem, 0) = &ty {
            Type::Array(elem.clone(), max_top_idx)
        } else {
            ty
        }
    } else {
        ty
    };

    let total_size = ty.size();
    let mut stmts = vec![Stmt::LocalDecl {
        name: name.clone(),
        ty: ty.clone(),
        init: None,
    }];

    // Zero-fill entire variable
    stmts.extend(zero_fill_stmts(&name, total_size));

    // Apply init stores (last write wins for designated overwrites)
    for (off, elem_ty, val) in stores {
        stmts.push(make_byte_store(&name, off, &elem_ty, val));
    }

    Ok(Stmt::Block(Block { stmts }))
}

// ===== Recursive initializer parsing =====

/// Parse elements inside braces for an aggregate (or scalar) type.
/// Returns max top-level element count for size inference.
pub(crate) fn init_braced(
    ts: &mut TokenStream,
    ty: &Type,
    base_offset: i32,
    stores: &mut Vec<(i32, Type, Expr)>,
) -> Result<usize, CompileError> {
    // Scalar in braces: {42} or {"foo"}
    if is_scalar(ty) {
        if !ts.check(&TokenKind::RBrace) {
            let val = parse_assign(ts)?;
            stores.push((base_offset, ty.clone(), val));
        }
        return Ok(1);
    }

    let mut next_off = base_offset;
    let mut max_top_idx = 0usize;

    while !ts.check(&TokenKind::RBrace) {
        // Track top-level element for size inference
        let top_idx = element_index_at(ty, base_offset, next_off);
        if top_idx + 1 > max_top_idx {
            max_top_idx = top_idx + 1;
        }

        // Range designator: [start ... end] = val
        if ts.check(&TokenKind::LBracket) && is_range_designator(ts) {
            let (start, end) = parse_range_designator(ts)?;
            ts.eat(TokenKind::Assign);
            let val = parse_assign(ts)?;
            for idx in start..=end {
                let (elem_ty, elem_off) = element_at_index(ty, idx, base_offset);
                stores.push((elem_off, elem_ty, copy_simple_expr(&val)));
            }
            let (end_ty, end_off) = element_at_index(ty, end, base_offset);
            next_off = end_off + end_ty.size();
            if end + 1 > max_top_idx {
                max_top_idx = end + 1;
            }
        } else if is_designator_start(ts) {
            // Designator chain: .field, [idx], .field.field, [i][j], etc.
            let (desig_ty, desig_off, desig_top) = resolve_designators(ts, ty, base_offset)?;
            if desig_top + 1 > max_top_idx {
                max_top_idx = desig_top + 1;
            }
            ts.eat(TokenKind::Assign);

            if ts.check(&TokenKind::LBrace) {
                // Braced sub-init for designated aggregate
                ts.advance();
                init_braced(ts, &desig_ty, desig_off, stores)?;
                ts.expect(TokenKind::RBrace)?;
                next_off = desig_off + desig_ty.size();
            } else if is_scalar(&desig_ty) {
                let val = parse_assign(ts)?;
                stores.push((desig_off, desig_ty.clone(), val));
                next_off = desig_off + desig_ty.size();
            } else {
                // Aggregate without braces after designator: fill first leaf
                next_off = desig_off;
                let leaf_ty = leaf_type_at(ty, base_offset, next_off);
                let val = parse_assign(ts)?;
                stores.push((next_off, leaf_ty.clone(), val));
                next_off += leaf_ty.size();
            }
        } else if ts.check(&TokenKind::LBrace) {
            // Braced sub-init at current position
            let top = element_index_at(ty, base_offset, next_off);
            let (elem_ty, elem_base) = element_at_index(ty, top, base_offset);

            if top + 1 > max_top_idx {
                max_top_idx = top + 1;
            }
            ts.advance();
            if is_scalar(&elem_ty) {
                let val = parse_assign(ts)?;
                stores.push((elem_base, elem_ty.clone(), val));
            } else {
                init_braced(ts, &elem_ty, elem_base, stores)?;
            }
            ts.expect(TokenKind::RBrace)?;
            next_off = elem_base + elem_ty.size();
        } else {
            // Value at current position
            let top = element_index_at(ty, base_offset, next_off);
            let (elem_ty, elem_base) = element_at_index(ty, top, base_offset);
            if top + 1 > max_top_idx {
                max_top_idx = top + 1;
            }

            // String literal initializes char array
            if is_char_array(&elem_ty) && matches!(&ts.peek().kind, TokenKind::StringLit(_)) {
                let TokenKind::StringLit(mut s) = ts.advance().kind else {
                    unreachable!()
                };
                // Implicit concatenation
                while let TokenKind::StringLit(_) = &ts.peek().kind {
                    let TokenKind::StringLit(next) = ts.advance().kind else {
                        unreachable!()
                    };
                    s.push_str(&next);
                }
                emit_string_stores(&s, &elem_ty, elem_base, stores);
                next_off = elem_base + elem_ty.size();
            } else {
                // Scalar value at leaf position
                let leaf_ty = leaf_type_at(ty, base_offset, next_off);
                let val = parse_assign(ts)?;
                stores.push((next_off, leaf_ty.clone(), val));
                next_off += leaf_ty.size();
            }
        }

        if !ts.eat(TokenKind::Comma) {
            break;
        }
        if ts.check(&TokenKind::RBrace) {
            break;
        }
    }

    Ok(max_top_idx)
}

// ===== Init helper functions =====

/// Check if the next token starts a designator (.field or [idx]).
fn is_designator_start(ts: &TokenStream) -> bool {
    if matches!(ts.peek().kind, TokenKind::Dot) {
        matches!(ts.lookahead(1), TokenKind::Ident(_))
    } else {
        matches!(ts.peek().kind, TokenKind::LBracket)
    }
}

/// Check for range designator: [int . . . int]
fn is_range_designator(ts: &TokenStream) -> bool {
    matches!(ts.peek().kind, TokenKind::LBracket) && matches!(ts.lookahead(2), TokenKind::Dot)
}

/// Parse range designator [start ... end] and return (start, end).
fn parse_range_designator(ts: &mut TokenStream) -> Result<(usize, usize), CompileError> {
    ts.expect(TokenKind::LBracket)?;
    let start = parse_const_array_size(ts)?;
    ts.expect(TokenKind::Dot)?;
    ts.expect(TokenKind::Dot)?;
    ts.expect(TokenKind::Dot)?;
    let end = parse_const_array_size(ts)?;
    ts.expect(TokenKind::RBracket)?;
    Ok((start, end))
}

/// Check if type is scalar (not array or struct/union).
fn is_scalar(ty: &Type) -> bool {
    !matches!(ty, Type::Array(..) | Type::Struct { .. })
}

/// Check if type is a char/unsigned char array.
fn is_char_array(ty: &Type) -> bool {
    matches!(ty, Type::Array(elem, _) if matches!(elem.as_ref(), Type::Char | Type::UnsignedChar))
}

/// Get (type, byte_offset) of element at index within aggregate.
fn element_at_index(ty: &Type, idx: usize, base: i32) -> (Type, i32) {
    match ty {
        Type::Array(elem, _) => {
            let off = base + (idx as i32) * elem.size();
            ((**elem).clone(), off)
        }
        Type::Struct { members, .. } => {
            if let Some(m) = members.get(idx) {
                (m.ty.clone(), base + m.offset)
            } else {
                (Type::Int, base)
            }
        }
        _ => (ty.clone(), base),
    }
}

/// Find which top-level element index an offset belongs to.
fn element_index_at(ty: &Type, base: i32, offset: i32) -> usize {
    match ty {
        Type::Array(elem, _) => {
            let elem_size = elem.size();
            if elem_size == 0 {
                return 0;
            }
            ((offset - base) / elem_size) as usize
        }
        Type::Struct { members, .. } => members
            .iter()
            .rposition(|m| offset >= base + m.offset)
            .unwrap_or(0),
        _ => 0,
    }
}

/// Find the index of a named field in a struct/union type.
fn find_field_index(ty: &Type, name: &str) -> Result<usize, CompileError> {
    if let Type::Struct { members, .. } = ty {
        for (i, m) in members.iter().enumerate() {
            if m.name == name {
                return Ok(i);
            }
        }
    }
    Err(CompileError::new(
        format!("no field '{name}' in type for designated initializer"),
        None,
    ))
}

/// Get the leaf (scalar) type at a specific byte offset within a type.
fn leaf_type_at(ty: &Type, base: i32, offset: i32) -> Type {
    match ty {
        Type::Array(elem, _) => {
            let elem_size = elem.size();
            if elem_size == 0 {
                return Type::Int;
            }
            let elem_base = base + ((offset - base) / elem_size) * elem_size;
            leaf_type_at(elem, elem_base, offset)
        }
        Type::Struct { members, .. } => {
            for m in members.iter().rev() {
                if offset >= base + m.offset {
                    return leaf_type_at(&m.ty, base + m.offset, offset);
                }
            }
            Type::Int
        }
        _ => ty.clone(),
    }
}

/// Parse a designator chain and resolve to (type, byte_offset, top_level_index).
fn resolve_designators(
    ts: &mut TokenStream,
    ty: &Type,
    base_offset: i32,
) -> Result<(Type, i32, usize), CompileError> {
    let mut current_ty = ty.clone();
    let mut current_off = base_offset;
    let mut top_idx = 0usize;
    let mut first = true;

    while is_designator_start(ts) {
        if matches!(ts.peek().kind, TokenKind::Dot) {
            ts.advance();
            let field = ts.expect_ident()?;
            let idx = find_field_index(&current_ty, &field)?;
            if first {
                top_idx = idx;
                first = false;
            }
            let (sub_ty, sub_off) = element_at_index(&current_ty, idx, current_off);
            current_ty = sub_ty;
            current_off = sub_off;
        } else {
            ts.advance(); // [
            let idx = parse_const_array_size(ts)?;
            ts.expect(TokenKind::RBracket)?;
            if first {
                top_idx = idx;
                first = false;
            }
            let (sub_ty, sub_off) = element_at_index(&current_ty, idx, current_off);
            current_ty = sub_ty;
            current_off = sub_off;
        }
    }

    Ok((current_ty, current_off, top_idx))
}

/// Emit stores for a string literal initializing a char array.
fn emit_string_stores(s: &str, ty: &Type, base_offset: i32, stores: &mut Vec<(i32, Type, Expr)>) {
    let array_size = if let Type::Array(_, sz) = ty { *sz } else { 0 };
    for (i, byte) in s.bytes().enumerate() {
        if i >= array_size {
            break;
        }
        stores.push((
            base_offset + i as i32,
            Type::Char,
            Expr::IntLit(byte as i32),
        ));
    }
    let null_pos = s.len();
    if null_pos < array_size {
        stores.push((base_offset + null_pos as i32, Type::Char, Expr::IntLit(0)));
    }
}

/// Copy a simple expression (for range designators).
fn copy_simple_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::IntLit(v) => Expr::IntLit(*v),
        Expr::StringLit(s) => Expr::StringLit(s.clone()),
        _ => Expr::IntLit(0),
    }
}

/// Generate zero-fill stores covering total_size bytes.
pub(crate) fn zero_fill_stmts(name: &str, total_size: i32) -> Vec<Stmt> {
    let mut stmts = Vec::new();
    let mut off = 0;
    while off + 3 <= total_size {
        stmts.push(make_byte_store(name, off, &Type::Int, Expr::IntLit(0)));
        off += 3;
    }
    while off < total_size {
        stmts.push(make_byte_store(name, off, &Type::Char, Expr::IntLit(0)));
        off += 1;
    }
    stmts
}

/// Generate a store at byte_offset from variable base.
/// Uses (char*)&var + byte_offset, cast to appropriate pointer type.
pub(crate) fn make_byte_store(name: &str, byte_offset: i32, elem_ty: &Type, value: Expr) -> Stmt {
    let addr = Expr::AddrOf(Box::new(Expr::Ident(name.to_string())));
    let byte_ptr = Expr::Cast {
        ty: Type::Ptr(Box::new(Type::Char)),
        expr: Box::new(addr),
    };
    let ptr = if byte_offset == 0 {
        byte_ptr
    } else {
        Expr::BinOp {
            op: BinOp::Add,
            lhs: Box::new(byte_ptr),
            rhs: Box::new(Expr::IntLit(byte_offset)),
        }
    };
    let typed_ptr = if matches!(elem_ty, Type::Char | Type::UnsignedChar) {
        ptr
    } else {
        Expr::Cast {
            ty: Type::Ptr(Box::new(elem_ty.clone())),
            expr: Box::new(ptr),
        }
    };
    Stmt::Expr(Expr::DerefAssign {
        ptr: Box::new(typed_ptr),
        value: Box::new(value),
    })
}
