//! Top-level and declaration parsing.

use tc24r_ast::{Function, GlobalDecl, Param, Program, Type};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

use crate::stmt::parse_block;
use tc24r_parse_stream::try_parse_interrupt_attr;

pub use tc24r_parser_types::parse_type;
use tc24r_parser_types::{is_base_type, is_storage_class, is_typedef_name};

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
        // Top-level struct/union definition: struct tag { ... };
        if matches!(ts.peek().kind, TokenKind::Struct | TokenKind::Union) {
            parse_type(ts)?; // registers the struct in ts.struct_types
            if ts.eat(TokenKind::Semicolon) {
                continue; // standalone definition, no variable
            }
            // Otherwise: struct tag { ... } var; -- need to parse declarator
            // Fall through won't work here since we consumed the type.
            // Parse the variable name and create a global.
            let name = ts.expect_ident()?;
            let ty = ts
                .struct_types
                .values()
                .last()
                .cloned()
                .unwrap_or(Type::Int);
            let init = None;
            ts.expect(TokenKind::Semicolon)?;
            globals.push(GlobalDecl { name, ty, init });
            continue;
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
    Ok(Program { functions, globals })
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
    i += 1;
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
    let name = ts.expect_ident()?;
    while ts.eat(TokenKind::LBracket) {
        let TokenKind::IntLit(size) = ts.peek().kind else {
            return Err(CompileError::new(
                "expected array size",
                Some(ts.current_span()),
            ));
        };
        ts.advance();
        ts.expect(TokenKind::RBracket)?;
        ty = Type::Array(Box::new(ty), size as usize);
    }
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
        let ty = parse_type(ts)?;
        // Unnamed parameters allowed in prototypes: int foo(int, int);
        let name = if ts.check(&TokenKind::Comma) || ts.check(&TokenKind::RParen) {
            String::new()
        } else {
            ts.expect_ident()?
        };
        params.push(Param { name, ty });
        if !ts.eat(TokenKind::Comma) {
            break;
        }
    }
    Ok(params)
}
