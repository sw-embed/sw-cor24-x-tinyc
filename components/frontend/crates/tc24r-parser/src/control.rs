//! Control flow statement parsing (if, while, for, asm).

use tc24r_ast::{Stmt, SwitchCase};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

use crate::expr::parse_expr;
use crate::stmt::{parse_block, parse_body, parse_local_decl};
use tc24r_parser_types::is_type_start;

pub fn parse_if(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let cond = parse_expr(ts)?;
    ts.expect(TokenKind::RParen)?;
    let then_body = parse_body(ts)?;
    let else_body = if ts.eat(TokenKind::Else) {
        Some(parse_body(ts)?)
    } else {
        None
    };
    Ok(Stmt::If {
        cond,
        then_body,
        else_body,
    })
}

pub fn parse_while(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let cond = parse_expr(ts)?;
    ts.expect(TokenKind::RParen)?;
    let body = parse_body(ts)?;
    Ok(Stmt::While { cond, body })
}

pub fn parse_for(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let init = parse_for_init(ts)?;
    let cond = if ts.check(&TokenKind::Semicolon) {
        None
    } else {
        Some(parse_expr(ts)?)
    };
    ts.expect(TokenKind::Semicolon)?;
    let inc = if ts.check(&TokenKind::RParen) {
        None
    } else {
        Some(parse_expr(ts)?)
    };
    ts.expect(TokenKind::RParen)?;
    let body = parse_body(ts)?;
    Ok(Stmt::For {
        init,
        cond,
        inc,
        body,
    })
}

fn parse_for_init(ts: &mut TokenStream) -> Result<Option<Box<Stmt>>, CompileError> {
    if ts.check(&TokenKind::Semicolon) {
        ts.expect(TokenKind::Semicolon)?;
        return Ok(None);
    }
    if is_type_start(ts) {
        // local decl consumes its own semicolon
        return Ok(Some(Box::new(parse_local_decl(ts)?)));
    }
    let expr = parse_expr(ts)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Some(Box::new(Stmt::Expr(expr))))
}

pub fn parse_asm(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    // Accept optional qualifiers: asm inline volatile (...)
    while matches!(ts.peek().kind, TokenKind::Volatile | TokenKind::Inline) {
        ts.advance();
    }
    ts.expect(TokenKind::LParen)?;
    // Parse template: one or more concatenated string literals
    let TokenKind::StringLit(first) = &ts.peek().kind else {
        return Err(CompileError::new(
            "expected string literal after asm(",
            Some(ts.current_span()),
        ));
    };
    let mut s = first.clone();
    ts.advance();
    // Concatenate adjacent string literals
    while let TokenKind::StringLit(next) = &ts.peek().kind {
        s.push_str(next);
        ts.advance();
    }
    // Skip GCC extended asm operands: asm("tmpl" : outputs : inputs : clobbers)
    while ts.eat(TokenKind::Colon) {
        skip_asm_operands(ts);
    }
    ts.expect(TokenKind::RParen)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::Asm(s))
}

/// Skip operands in an extended asm constraint section.
/// Each section contains comma-separated items like `"=r"(var)` or `"r"(expr)`.
fn skip_asm_operands(ts: &mut TokenStream) {
    // Stop at `)` (end of asm) or `:` (next section)
    while !ts.check(&TokenKind::RParen) && !ts.check(&TokenKind::Colon) {
        // Skip string constraint: "=r", "+m", etc.
        if matches!(ts.peek().kind, TokenKind::StringLit(_)) {
            ts.advance();
        }
        // Skip parenthesized expression: (var)
        if ts.eat(TokenKind::LParen) {
            let mut depth = 1;
            while depth > 0 {
                if ts.eat(TokenKind::LParen) {
                    depth += 1;
                } else if ts.eat(TokenKind::RParen) {
                    depth -= 1;
                } else {
                    ts.advance();
                }
            }
        }
        // Skip comma between operands
        if !ts.eat(TokenKind::Comma) {
            break;
        }
    }
}

pub fn parse_switch(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let expr = parse_expr(ts)?;
    ts.expect(TokenKind::RParen)?;
    ts.expect(TokenKind::LBrace)?;

    let mut cases = Vec::new();
    let mut default = None;

    while !ts.check(&TokenKind::RBrace) {
        if ts.eat(TokenKind::Case) {
            let value = parse_expr(ts)?;
            ts.expect(TokenKind::Colon)?;
            let stmts = parse_case_body(ts)?;
            cases.push(SwitchCase { value, stmts });
        } else if ts.eat(TokenKind::Default) {
            ts.expect(TokenKind::Colon)?;
            let stmts = parse_case_body(ts)?;
            default = Some(stmts);
        } else {
            return Err(CompileError::new(
                "expected case or default in switch",
                Some(ts.current_span()),
            ));
        }
    }

    ts.expect(TokenKind::RBrace)?;
    Ok(Stmt::Switch {
        expr,
        cases,
        default,
    })
}

/// Parse statements inside a case/default until the next case, default, or `}`.
fn parse_case_body(ts: &mut TokenStream) -> Result<Vec<Stmt>, CompileError> {
    let mut stmts = Vec::new();
    while !ts.check(&TokenKind::Case)
        && !ts.check(&TokenKind::Default)
        && !ts.check(&TokenKind::RBrace)
    {
        stmts.push(crate::stmt::parse_stmt(ts)?);
    }
    Ok(stmts)
}

pub fn parse_do_while(ts: &mut TokenStream) -> Result<Stmt, CompileError> {
    let body = parse_block(ts)?;
    ts.expect(TokenKind::While)?;
    ts.expect(TokenKind::LParen)?;
    let cond = parse_expr(ts)?;
    ts.expect(TokenKind::RParen)?;
    ts.expect(TokenKind::Semicolon)?;
    Ok(Stmt::DoWhile { body, cond })
}
