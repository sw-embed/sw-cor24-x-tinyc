//! Statement and block parsing.

use tc24r_ast::{Block, Expr, Stmt, Type};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

use crate::control;
use crate::decl::parse_type;
use crate::expr::parse_expr;
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
    // Function pointer: int (*fp)(int, int) or int (*table[4])(int)
    if ts.check(&TokenKind::LParen) && matches!(ts.lookahead(1), TokenKind::Star) {
        return parse_fn_ptr_declarator(ts, ty);
    }
    let name = ts.expect_ident()?;
    // Check for array: int a[N] or int a[N][M]
    let mut ty = ty;
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
    // Struct/array brace initializer: struct s x = {1, 2};
    if ts.eat(TokenKind::Assign) {
        if ts.check(&TokenKind::LBrace) {
            return parse_brace_init(ts, name, ty);
        }
        let init = Some(parse_expr(ts)?);
        return Ok(Stmt::LocalDecl { name, ty, init });
    }
    Ok(Stmt::LocalDecl {
        name,
        ty,
        init: None,
    })
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
        let TokenKind::IntLit(size) = ts.peek().kind else {
            return Err(CompileError::new(
                "expected array size in function pointer",
                Some(ts.current_span()),
            ));
        };
        ts.advance();
        ts.expect(TokenKind::RBracket)?;
        is_array = Some(size as usize);
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

/// Parse brace initializer: struct s x = {1, 2};
/// Desugars to: LocalDecl x + member assignments.
fn parse_brace_init(ts: &mut TokenStream, name: String, ty: Type) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LBrace)?;
    let mut values = Vec::new();
    if !ts.check(&TokenKind::RBrace) {
        loop {
            values.push(parse_expr(ts)?);
            if !ts.eat(TokenKind::Comma) {
                break;
            }
            // Allow trailing comma: {1, 2,}
            if ts.check(&TokenKind::RBrace) {
                break;
            }
        }
    }
    ts.expect(TokenKind::RBrace)?;

    let mut stmts = vec![Stmt::LocalDecl {
        name: name.clone(),
        ty: ty.clone(),
        init: None,
    }];

    // Generate member assignments based on struct member order
    if let Type::Struct { members, .. } = &ty {
        for (i, val) in values.into_iter().enumerate() {
            if let Some(member) = members.get(i) {
                stmts.push(Stmt::Expr(Expr::MemberAssign {
                    object: Box::new(Expr::Ident(name.clone())),
                    member: member.name.clone(),
                    value: Box::new(val),
                }));
            }
        }
    }

    Ok(Stmt::Block(Block { stmts }))
}
