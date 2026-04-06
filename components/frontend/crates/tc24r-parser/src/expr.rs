//! Expression entry point, assignment, unary, and primary parsing.

use tc24r_ast::{BinOp, Block, Expr, Stmt, Type, UnaryOp};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_parser_compound::{desugar_compound, eat_compound_assign, make_assign};
use tc24r_parser_types::{is_type_start, parse_type};
use tc24r_token::TokenKind;

use crate::bitwise::parse_log_or;
use crate::decl::parse_const_array_size;
use crate::stmt::parse_block;

/// Parse an expression (includes comma operator at lowest precedence).
pub fn parse_expr(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let mut lhs = parse_assign(ts)?;
    while ts.eat(TokenKind::Comma) {
        let rhs = parse_assign(ts)?;
        lhs = Expr::BinOp {
            op: BinOp::Comma,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
    }
    Ok(lhs)
}

/// Parse an assignment expression (used in function args, initializers).
pub fn parse_assign(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let expr = parse_ternary(ts)?;
    if ts.eat(TokenKind::Assign) {
        let value = parse_assign(ts)?;
        return make_assign(expr, value);
    }
    if let Some(op) = eat_compound_assign(ts) {
        let rhs = parse_assign(ts)?;
        return desugar_compound(expr, op, rhs);
    }
    Ok(expr)
}

fn parse_ternary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let cond = parse_log_or(ts)?;
    if ts.eat(TokenKind::Question) {
        let then_expr = parse_expr(ts)?;
        ts.expect(TokenKind::Colon)?;
        let else_expr = parse_ternary(ts)?;
        return Ok(Expr::Ternary {
            cond: Box::new(cond),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        });
    }
    Ok(cond)
}

pub fn parse_unary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    if ts.eat(TokenKind::Sizeof) {
        return parse_sizeof(ts);
    }
    if ts.eat(TokenKind::Offsetof) {
        return parse_offsetof(ts);
    }
    if ts.eat(TokenKind::BuiltinTypesCompatible) {
        return parse_builtin_types_compatible(ts);
    }
    if ts.eat(TokenKind::Plus) {
        // Unary + is identity, just parse the operand
        return parse_unary(ts);
    }
    if ts.eat(TokenKind::Minus) {
        let operand = parse_unary(ts)?;
        // Fold -N → IntLit(-N) at parse time (avoids 0-N codegen)
        if let Expr::IntLit(n) = &operand {
            return Ok(Expr::IntLit(-n));
        }
        return Ok(Expr::UnaryOp {
            op: UnaryOp::Neg,
            operand: Box::new(operand),
        });
    }
    if ts.eat(TokenKind::Tilde) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::UnaryOp {
            op: UnaryOp::BitNot,
            operand: Box::new(operand),
        });
    }
    if ts.eat(TokenKind::Bang) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::UnaryOp {
            op: UnaryOp::LogNot,
            operand: Box::new(operand),
        });
    }
    if ts.eat(TokenKind::PlusPlus) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::PreInc(Box::new(operand)));
    }
    if ts.eat(TokenKind::MinusMinus) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::PreDec(Box::new(operand)));
    }
    if ts.eat(TokenKind::Amp) {
        let operand = parse_unary(ts)?;
        // &*(ptr) simplifies to ptr
        if let Expr::Deref(inner) = operand {
            return Ok(*inner);
        }
        // &expr — wrap in AddrOf
        return Ok(Expr::AddrOf(Box::new(operand)));
    }
    if ts.eat(TokenKind::Star) {
        let operand = parse_unary(ts)?;
        return Ok(Expr::Deref(Box::new(operand)));
    }
    parse_primary(ts)
}

fn parse_primary(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    if ts.eat(TokenKind::LParen) {
        // Statement expression: ({ stmt; ... expr; })
        if ts.check(&TokenKind::LBrace) {
            let block = parse_block(ts)?;
            ts.expect(TokenKind::RParen)?;
            return parse_postfix_chain(ts, Expr::StmtExpr(block));
        }
        // Cast: (type)expr  OR  Compound literal: (type){init}
        if is_type_start(ts) {
            let ty = parse_type(ts)?;
            // Check for array suffix: (int[3]) or (int[])
            let ty = parse_optional_array_suffix(ts, ty)?;
            ts.expect(TokenKind::RParen)?;
            // Compound literal: (type){init}
            if ts.check(&TokenKind::LBrace) {
                let lit = parse_compound_literal(ts, ty)?;
                return parse_postfix_chain(ts, lit);
            }
            let operand = parse_unary(ts)?;
            let cast = Expr::Cast {
                ty,
                expr: Box::new(operand),
            };
            return parse_postfix_chain(ts, cast);
        }
        let expr = parse_expr(ts)?;
        ts.expect(TokenKind::RParen)?;
        return parse_postfix_chain(ts, expr);
    }
    if let TokenKind::IntLit(_) = &ts.peek().kind {
        let TokenKind::IntLit(val) = ts.advance().kind else {
            unreachable!()
        };
        // Support postfix on literals: 2[x] is valid C (equivalent to x[2])
        return parse_postfix_chain(ts, Expr::IntLit(val));
    }
    if let TokenKind::StringLit(_) = &ts.peek().kind {
        let TokenKind::StringLit(mut s) = ts.advance().kind else {
            unreachable!()
        };
        // C89 implicit string literal concatenation: "a" "b" → "ab"
        while let TokenKind::StringLit(_) = &ts.peek().kind {
            let TokenKind::StringLit(next) = ts.advance().kind else {
                unreachable!()
            };
            s.push_str(&next);
        }
        return parse_postfix_chain(ts, Expr::StringLit(s));
    }
    if let TokenKind::Ident(_) = &ts.peek().kind {
        return parse_ident_or_call(ts);
    }
    Err(CompileError::new(
        format!("expected expression, got {:?}", ts.peek().kind),
        Some(ts.current_span()),
    ))
}

fn parse_ident_or_call(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    let TokenKind::Ident(name) = ts.advance().kind else {
        unreachable!()
    };
    // Resolve enum constants to integer literals
    if let Some(&val) = ts.enum_constants.get(&name) {
        return Ok(Expr::IntLit(val));
    }
    // Function call: foo(args...)
    if ts.eat(TokenKind::LParen) {
        let mut args = Vec::new();
        if !ts.check(&TokenKind::RParen) {
            loop {
                args.push(parse_assign(ts)?);
                if !ts.eat(TokenKind::Comma) {
                    break;
                }
            }
        }
        ts.expect(TokenKind::RParen)?;
        let call = Expr::Call { name, args };
        return parse_postfix_chain(ts, call);
    }
    // Start with the identifier, then chain postfix ops: [], ., ->, ++, --
    let mut result = Expr::Ident(name);
    result = parse_postfix_chain(ts, result)?;
    Ok(result)
}

/// Parse chained postfix operations: a[i].b->c[j] etc.
fn parse_postfix_chain(ts: &mut TokenStream, mut expr: Expr) -> Result<Expr, CompileError> {
    loop {
        if ts.eat(TokenKind::LBracket) {
            let index = parse_expr(ts)?;
            ts.expect(TokenKind::RBracket)?;
            expr = Expr::Deref(Box::new(Expr::BinOp {
                op: tc24r_ast::BinOp::Add,
                lhs: Box::new(expr),
                rhs: Box::new(index),
            }));
        } else if ts.eat(TokenKind::LParen) {
            // Indirect call: expr(args) — for table[n](arg), (*fp)(arg), etc.
            let mut args = Vec::new();
            if !ts.check(&TokenKind::RParen) {
                loop {
                    args.push(parse_assign(ts)?);
                    if !ts.eat(TokenKind::Comma) {
                        break;
                    }
                }
            }
            ts.expect(TokenKind::RParen)?;
            expr = Expr::IndirectCall {
                callee: Box::new(expr),
                args,
            };
        } else if ts.eat(TokenKind::Dot) {
            let member = ts.expect_ident()?;
            expr = Expr::MemberAccess {
                object: Box::new(expr),
                member,
            };
        } else if ts.eat(TokenKind::Arrow) {
            let member = ts.expect_ident()?;
            expr = Expr::MemberAccess {
                object: Box::new(Expr::Deref(Box::new(expr))),
                member,
            };
        } else if ts.eat(TokenKind::PlusPlus) {
            expr = Expr::PostInc(Box::new(expr));
        } else if ts.eat(TokenKind::MinusMinus) {
            expr = Expr::PostDec(Box::new(expr));
        } else {
            break;
        }
    }
    Ok(expr)
}

/// Parse `sizeof(type)`, `sizeof(expr)`, or `sizeof expr` after the `sizeof` token.
fn parse_sizeof(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    if ts.eat(TokenKind::LParen) {
        if is_type_start(ts) {
            let mut ty = parse_type(ts)?;
            // Handle array suffix: sizeof(int[4]), sizeof(char[16])
            while ts.eat(TokenKind::LBracket) {
                let TokenKind::IntLit(size) = ts.peek().kind else {
                    return Err(CompileError::new(
                        "expected array size in sizeof",
                        Some(ts.current_span()),
                    ));
                };
                ts.advance();
                ts.expect(TokenKind::RBracket)?;
                ty = Type::Array(Box::new(ty), size as usize);
            }
            ts.expect(TokenKind::RParen)?;
            return Ok(Expr::IntLit(ty.size()));
        }
        // sizeof(expr) — parenthesized expression
        let expr = parse_expr(ts)?;
        ts.expect(TokenKind::RParen)?;
        return Ok(Expr::SizeofExpr(Box::new(expr)));
    }
    // sizeof expr — unary expression without parentheses
    let expr = parse_unary(ts)?;
    Ok(Expr::SizeofExpr(Box::new(expr)))
}

/// Parse `offsetof(type, member)` after the `offsetof` token.
fn parse_offsetof(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let ty = parse_type(ts)?;
    ts.expect(TokenKind::Comma)?;
    let member = ts.expect_ident()?;
    ts.expect(TokenKind::RParen)?;
    match ty.find_member(&member) {
        Some(m) => Ok(Expr::IntLit(m.offset)),
        None => Err(CompileError::new(
            format!("no member '{member}' in type"),
            Some(ts.current_span()),
        )),
    }
}

/// Parse `__builtin_types_compatible_p(type1, type2)` — returns 1 if compatible, 0 otherwise.
fn parse_builtin_types_compatible(ts: &mut TokenStream) -> Result<Expr, CompileError> {
    ts.expect(TokenKind::LParen)?;
    let ty1 = parse_type(ts)?;
    // Skip function pointer suffix: (*)(params)
    skip_fn_ptr_type_suffix(ts);
    ts.expect(TokenKind::Comma)?;
    let ty2 = parse_type(ts)?;
    skip_fn_ptr_type_suffix(ts);
    ts.expect(TokenKind::RParen)?;
    // Types are compatible if structurally equal (ignoring const/volatile qualifiers).
    // Anonymous structs with identical layout are NOT compatible in C.
    let compatible = types_compatible(&ty1, &ty2);
    Ok(Expr::IntLit(if compatible { 1 } else { 0 }))
}

/// Check if two types are compatible for __builtin_types_compatible_p.
/// Ignores const/volatile qualifiers (already stripped by parse_type).
fn types_compatible(a: &Type, b: &Type) -> bool {
    match (a, b) {
        (Type::Int, Type::Int) => true,
        (Type::Char, Type::Char) => true,
        (Type::UnsignedChar, Type::UnsignedChar) => true,
        (Type::UnsignedInt, Type::UnsignedInt) => true,
        (Type::Void, Type::Void) => true,
        (Type::Ptr(a), Type::Ptr(b)) => types_compatible(a, b),
        (Type::Array(a, n1), Type::Array(b, n2)) => n1 == n2 && types_compatible(a, b),
        (Type::Struct { tag: Some(a), .. }, Type::Struct { tag: Some(b), .. }) => a == b,
        _ => false,
    }
}

/// Skip optional function pointer type suffix: (*)(params)
/// Used in __builtin_types_compatible_p where types like int(*)(void) appear.
fn skip_fn_ptr_type_suffix(ts: &mut TokenStream) {
    // Check for (*) pattern
    if ts.check(&TokenKind::LParen) && matches!(ts.lookahead(1), TokenKind::Star) {
        ts.advance(); // (
        ts.advance(); // *
        ts.expect(TokenKind::RParen).ok(); // )
                                           // Skip parameter list: (params)
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
    }
}

/// Parse optional array suffix after a type in cast/compound-literal position.
/// E.g., (int[3]) or (int[]) — returns the modified type.
fn parse_optional_array_suffix(ts: &mut TokenStream, mut ty: Type) -> Result<Type, CompileError> {
    while ts.check(&TokenKind::LBracket) {
        ts.advance(); // eat [
        if ts.check(&TokenKind::RBracket) {
            ts.advance(); // eat ]
            ty = Type::Array(Box::new(ty), 0); // size inferred from init
        } else {
            let size = parse_const_array_size(ts)?;
            ts.expect(TokenKind::RBracket)?;
            ty = Type::Array(Box::new(ty), size);
        }
    }
    Ok(ty)
}

/// Counter for generating unique compound literal temp names.
static COMPLIT_COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Parse a compound literal: (type){init-list}
/// Desugars to a statement expression that allocates a temp, initializes it,
/// and evaluates to the temp's value (or address for arrays).
fn parse_compound_literal(ts: &mut TokenStream, ty: Type) -> Result<Expr, CompileError> {
    use crate::stmt::{init_braced, make_byte_store, zero_fill_stmts};

    let id = COMPLIT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let tmp_name = format!("__complit_{id}");

    ts.expect(TokenKind::LBrace)?;
    let mut stores = Vec::new();
    let max_top_idx = init_braced(ts, &ty, 0, &mut stores)?;
    ts.expect(TokenKind::RBrace)?;

    // Infer array size
    let actual_ty = if let Type::Array(elem, 0) = &ty {
        Type::Array(elem.clone(), max_top_idx)
    } else {
        ty.clone()
    };

    let total_size = actual_ty.size();
    let mut stmts = vec![Stmt::LocalDecl {
        name: tmp_name.clone(),
        ty: actual_ty.clone(),
        init: None,
    }];

    // Zero-fill + apply init stores
    stmts.extend(zero_fill_stmts(&tmp_name, total_size));
    for (off, elem_ty, val) in stores {
        stmts.push(make_byte_store(&tmp_name, off, &elem_ty, val));
    }

    // Result expression: arrays decay to pointer, structs return address
    match &actual_ty {
        Type::Array(..) => stmts.push(Stmt::Expr(Expr::Ident(tmp_name))),
        Type::Struct { .. } => {
            stmts.push(Stmt::Expr(Expr::AddrOf(Box::new(Expr::Ident(tmp_name)))));
        }
        _ => stmts.push(Stmt::Expr(Expr::Ident(tmp_name))),
    }

    Ok(Expr::StmtExpr(Block { stmts }))
}
