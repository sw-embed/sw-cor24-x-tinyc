//! Top-level and declaration parsing.

use tc24r_ast::{Function, GlobalDecl, Param, Program, Type};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

use crate::stmt::{parse_block, skip_fn_ptr_params};
use tc24r_parse_stream::try_parse_interrupt_attr;

pub use tc24r_parser_types::parse_type;
use tc24r_parser_types::{is_base_type, is_storage_class, is_typedef_name};

/// Parse a constant integer expression between array brackets.
/// Supports integer literals, +, -, *, /, parentheses.
pub(crate) fn parse_const_array_size(ts: &mut TokenStream) -> Result<usize, CompileError> {
    let val = const_add_sub(ts)?;
    if val < 0 {
        return Err(CompileError::new(
            "array size must be positive",
            Some(ts.current_span()),
        ));
    }
    Ok(val as usize)
}

fn const_add_sub(ts: &mut TokenStream) -> Result<i32, CompileError> {
    let mut left = const_mul_div(ts)?;
    loop {
        if ts.eat(TokenKind::Plus) {
            left += const_mul_div(ts)?;
        } else if ts.eat(TokenKind::Minus) {
            left -= const_mul_div(ts)?;
        } else {
            break;
        }
    }
    Ok(left)
}

fn const_mul_div(ts: &mut TokenStream) -> Result<i32, CompileError> {
    let mut left = const_primary(ts)?;
    loop {
        if ts.eat(TokenKind::Star) {
            left *= const_primary(ts)?;
        } else if ts.eat(TokenKind::Slash) {
            let right = const_primary(ts)?;
            if right == 0 {
                return Err(CompileError::new(
                    "division by zero in constant expression",
                    Some(ts.current_span()),
                ));
            }
            left /= right;
        } else {
            break;
        }
    }
    Ok(left)
}

fn const_primary(ts: &mut TokenStream) -> Result<i32, CompileError> {
    if let TokenKind::IntLit(v) = ts.peek().kind {
        ts.advance();
        return Ok(v);
    }
    if ts.eat(TokenKind::LParen) {
        let val = const_add_sub(ts)?;
        ts.expect(TokenKind::RParen)?;
        return Ok(val);
    }
    if ts.eat(TokenKind::Minus) {
        return Ok(-const_primary(ts)?);
    }
    Err(CompileError::new(
        "expected integer constant in array size",
        Some(ts.current_span()),
    ))
}

/// Check if current position is an enum definition (`enum {` or `enum tag {`).
fn is_enum_definition(ts: &TokenStream) -> bool {
    if !matches!(ts.peek().kind, TokenKind::Enum) {
        return false;
    }
    match ts.lookahead(1) {
        TokenKind::LBrace => true,
        TokenKind::Ident(_) => matches!(ts.lookahead(2), TokenKind::LBrace),
        _ => false,
    }
}

/// Parse a full program (sequence of functions and globals).
pub fn parse_program(ts: &mut TokenStream) -> Result<Program, CompileError> {
    let mut functions = Vec::new();
    let mut globals = Vec::new();
    while !ts.at_eof() {
        // Top-level enum definition: consume and register constants
        if is_enum_definition(ts) {
            ts.advance(); // consume `enum`
            tc24r_parser_enum::parse_enum_decl(ts)?;
            continue;
        }
        // Top-level struct/union: definition, variable, or function return type.
        // Only intercept struct definitions (with { body }) and struct variable
        // declarations. Functions returning struct pointers fall through to
        // is_global_decl / parse_function.
        if matches!(ts.peek().kind, TokenKind::Struct | TokenKind::Union) {
            if is_struct_def_or_var(ts) {
                parse_type(ts)?; // registers the struct in ts.struct_types
                if ts.eat(TokenKind::Semicolon) {
                    continue; // standalone definition: struct tag { ... };
                }
                let name = ts.expect_ident()?;
                let mut ty = ts
                    .struct_types
                    .values()
                    .last()
                    .cloned()
                    .unwrap_or(Type::Int);
                // Handle array suffixes: struct symbol symtab[8];
                while ts.eat(TokenKind::LBracket) {
                    let size = parse_const_array_size(ts)?;
                    ts.expect(TokenKind::RBracket)?;
                    ty = Type::Array(Box::new(ty), size);
                }
                let init = None;
                ts.expect(TokenKind::Semicolon)?;
                globals.push(GlobalDecl { name, ty, init });
                continue;
            }
            // Otherwise: function returning struct pointer — fall through
        }
        // Top-level typedef
        if ts.eat(TokenKind::Typedef) {
            tc24r_parser_typedef::parse_typedef_decl(ts)?;
            continue;
        }
        let is_interrupt = try_parse_interrupt_attr(ts);
        if is_global_decl(ts) {
            parse_global_decls(ts, &mut globals)?;
        } else {
            functions.push(parse_function(ts, is_interrupt)?);
        }
    }
    Ok(Program {
        functions,
        globals,
        struct_types: ts.struct_types.clone(),
    })
}

/// Check if a struct/union at top level is a definition or variable
/// (not a function returning struct *).
/// struct tag { ... };  → true (definition)
/// struct tag var;      → true (variable)
/// struct tag *func();  → false (function returning pointer)
fn is_struct_def_or_var(ts: &TokenStream) -> bool {
    let mut i = 1; // skip struct/union keyword
    // Skip optional tag name
    if matches!(ts.lookahead(i), TokenKind::Ident(_)) {
        i += 1;
    }
    // If next is { → definition (handle it)
    if matches!(ts.lookahead(i), TokenKind::LBrace) {
        return true;
    }
    // If next is ; → standalone forward declaration
    if matches!(ts.lookahead(i), TokenKind::Semicolon) {
        return true;
    }
    // If next is an ident (variable name) → variable declaration
    if matches!(ts.lookahead(i), TokenKind::Ident(_)) {
        return true;
    }
    // Otherwise (e.g., * → function returning struct pointer) → false
    false
}

fn is_global_decl(ts: &TokenStream) -> bool {
    let mut i = 0;
    // Skip storage-class specifiers
    while is_storage_class(ts.lookahead(i)) {
        i += 1;
    }
    if !is_base_type(ts.lookahead(i)) && !is_typedef_name(ts, ts.lookahead(i)) {
        return false;
    }
    // Skip the base type tokens: struct/union/enum + optional tag name
    if matches!(
        ts.lookahead(i),
        TokenKind::Struct | TokenKind::Union | TokenKind::Enum
    ) {
        i += 1;
        // Skip tag name if present
        if matches!(ts.lookahead(i), TokenKind::Ident(_)) {
            i += 1;
        }
        // Skip struct body { ... } if present
        if matches!(ts.lookahead(i), TokenKind::LBrace) {
            // Can't easily count braces in lookahead; not a global decl if body follows
            return false;
        }
    } else {
        i += 1;
        // Skip additional type keywords: long int, unsigned char, etc.
        while is_base_type(ts.lookahead(i)) {
            i += 1;
        }
    }
    // Function pointer: int (*fp)(int, int)
    if matches!(ts.lookahead(i), TokenKind::LParen)
        && matches!(ts.lookahead(i + 1), TokenKind::Star)
    {
        return true;
    }
    // Skip pointer stars: int **ptr
    while matches!(ts.lookahead(i), TokenKind::Star) {
        i += 1;
    }
    matches!(ts.lookahead(i), TokenKind::Ident(_))
        && !matches!(ts.lookahead(i + 1), TokenKind::LParen)
}

fn parse_global_decls(
    ts: &mut TokenStream,
    globals: &mut Vec<GlobalDecl>,
) -> Result<(), CompileError> {
    let base_ty = parse_type(ts)?;
    merge_global(globals, parse_one_global(ts, base_ty.clone())?);
    while ts.eat(TokenKind::Comma) {
        merge_global(globals, parse_one_global(ts, base_ty.clone())?);
    }
    ts.expect(TokenKind::Semicolon)?;
    Ok(())
}

/// Merge a global declaration, handling tentative definitions.
/// `int x; int x = 5;` keeps the initialized version.
fn merge_global(globals: &mut Vec<GlobalDecl>, new: GlobalDecl) {
    if let Some(existing) = globals.iter_mut().find(|g| g.name == new.name) {
        if new.init.is_some() {
            existing.init = new.init;
        }
    } else {
        globals.push(new);
    }
}

fn parse_one_global(ts: &mut TokenStream, base_ty: Type) -> Result<GlobalDecl, CompileError> {
    let mut ty = base_ty;
    while ts.eat(TokenKind::Star) {
        ty = Type::Ptr(Box::new(ty));
    }
    // Function pointer: int (*fp)(int, int)
    if ts.check(&TokenKind::LParen) && matches!(ts.lookahead(1), TokenKind::Star) {
        return parse_global_fn_ptr(ts, ty);
    }
    let name = ts.expect_ident()?;
    while ts.eat(TokenKind::LBracket) {
        let size = parse_const_array_size(ts)?;
        ts.expect(TokenKind::RBracket)?;
        ty = Type::Array(Box::new(ty), size);
    }
    let init = if ts.eat(TokenKind::Assign) {
        Some(crate::expr::parse_expr(ts)?)
    } else {
        None
    };
    Ok(GlobalDecl { name, ty, init })
}

/// Parse a global function pointer: (*name)(params) or (*name[N])(params)
fn parse_global_fn_ptr(ts: &mut TokenStream, return_ty: Type) -> Result<GlobalDecl, CompileError> {
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
        Some(crate::expr::parse_expr(ts)?)
    } else {
        None
    };
    Ok(GlobalDecl { name, ty, init })
}

fn parse_function(ts: &mut TokenStream, is_interrupt: bool) -> Result<Function, CompileError> {
    let span = ts.current_span();
    let return_ty = parse_type(ts)?;
    let name = ts.expect_ident()?;
    ts.expect(TokenKind::LParen)?;
    let params = parse_params(ts)?;
    ts.expect(TokenKind::RParen)?;
    // Prototype: int foo(int n); — no body
    let body = if ts.eat(TokenKind::Semicolon) {
        None
    } else {
        Some(parse_block(ts)?)
    };
    Ok(Function {
        name,
        return_ty,
        params,
        body,
        span,
        is_interrupt,
    })
}

fn parse_params(ts: &mut TokenStream) -> Result<Vec<Param>, CompileError> {
    let mut params = Vec::new();
    if ts.check(&TokenKind::RParen) {
        return Ok(params);
    }
    // (void) means no parameters
    if ts.check(&TokenKind::Void) && matches!(ts.lookahead(1), TokenKind::RParen) {
        ts.advance();
        return Ok(params);
    }
    loop {
        // Varargs: ... (accept and ignore)
        if ts.check(&TokenKind::Dot) {
            ts.advance(); // .
            ts.advance(); // .
            ts.advance(); // .
            break;
        }
        let ty = parse_type(ts)?;
        // Function pointer parameter: int (*f)(int)
        if ts.check(&TokenKind::LParen) && matches!(ts.lookahead(1), TokenKind::Star) {
            ts.advance(); // (
            ts.advance(); // *
            let name = if ts.check(&TokenKind::RParen) {
                String::new() // unnamed: int (*)(int)
            } else {
                ts.expect_ident()?
            };
            ts.expect(TokenKind::RParen)?;
            // Consume parameter type list
            ts.expect(TokenKind::LParen)?;
            skip_fn_ptr_params(ts)?;
            ts.expect(TokenKind::RParen)?;
            params.push(Param {
                name,
                ty: Type::Ptr(Box::new(ty)),
            });
        } else {
            // Unnamed parameters allowed in prototypes: int foo(int, int);
            let name = if ts.check(&TokenKind::Comma) || ts.check(&TokenKind::RParen) {
                String::new()
            } else {
                ts.expect_ident()?
            };
            params.push(Param { name, ty });
        }
        if !ts.eat(TokenKind::Comma) {
            break;
        }
    }
    Ok(params)
}
