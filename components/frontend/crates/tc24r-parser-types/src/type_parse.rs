//! Type parsing: consume tokens and produce a Type AST node.

use tc24r_ast::Type;
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

/// Consume and discard storage-class / type-qualifier keywords.
/// Returns `true` if at least one qualifier was consumed.
fn consume_qualifiers(ts: &mut TokenStream) -> bool {
    let mut had = false;
    while matches!(
        ts.peek().kind,
        TokenKind::Static
            | TokenKind::Extern
            | TokenKind::Const
            | TokenKind::Inline
            | TokenKind::Noreturn
            | TokenKind::Volatile
            | TokenKind::Auto
            | TokenKind::Register
    ) {
        ts.advance();
        had = true;
    }
    had
}

/// Parse a base type keyword (char/int/void/enum/struct/typedef alias).
fn parse_base_type(ts: &mut TokenStream, had_qualifier: bool) -> Result<Type, CompileError> {
    // Consume type modifiers: long, short, signed, unsigned
    // On COR24, all integer types are 24-bit; long/short are accepted and treated as int.
    let mut had_modifier = false;
    let mut is_unsigned = false;
    while matches!(
        ts.peek().kind,
        TokenKind::Long | TokenKind::Short | TokenKind::Signed | TokenKind::Unsigned
    ) {
        if ts.peek().kind == TokenKind::Unsigned {
            is_unsigned = true;
        }
        ts.advance();
        had_modifier = true;
    }

    match ts.peek().kind {
        TokenKind::Char => {
            ts.advance();
            if is_unsigned {
                Ok(Type::UnsignedChar)
            } else {
                Ok(Type::Char)
            }
        }
        TokenKind::Int => {
            ts.advance();
            if is_unsigned {
                Ok(Type::UnsignedInt)
            } else {
                Ok(Type::Int)
            }
        }
        _ if had_modifier => {
            if is_unsigned {
                Ok(Type::UnsignedInt)
            } else {
                Ok(Type::Int)
            }
        }
        TokenKind::Void => {
            ts.advance();
            Ok(Type::Void)
        }
        TokenKind::Enum => {
            ts.advance();
            if matches!(ts.peek().kind, TokenKind::Ident(_)) {
                ts.advance();
            }
            Ok(Type::Int)
        }
        TokenKind::Struct => {
            ts.advance();
            tc24r_parser_struct::parse_struct_type(ts, parse_type, false)
        }
        TokenKind::Union => {
            ts.advance();
            tc24r_parser_struct::parse_struct_type(ts, parse_type, true)
        }
        TokenKind::Typeof => {
            ts.advance();
            ts.expect(TokenKind::LParen)?;
            // typeof(type) or typeof(expr) — try type first
            let ty = if super::is_base_type(&ts.peek().kind)
                || super::is_storage_class(&ts.peek().kind)
                || super::is_typedef_name(ts, &ts.peek().kind)
            {
                parse_type(ts)?
            } else {
                // typeof(expr) — infer type at parse time
                // For now, treat as Int (sufficient for most uses)
                let _expr = crate::parse_typeof_expr(ts);
                Type::Int
            };
            ts.expect(TokenKind::RParen)?;
            Ok(ty)
        }
        TokenKind::Ident(ref name) if ts.type_aliases.contains_key(name) => {
            let resolved = ts.type_aliases[name].clone();
            ts.advance();
            Ok(resolved)
        }
        _ if had_qualifier => Ok(Type::Int),
        _ => Err(CompileError::new(
            format!("expected type, got {:?}", ts.peek().kind),
            Some(ts.current_span()),
        )),
    }
}

/// Consume pointer stars and trailing const qualifiers after a base type.
fn consume_pointers(ts: &mut TokenStream, base: Type) -> Type {
    while ts.eat(TokenKind::Const) || ts.eat(TokenKind::Restrict) || ts.eat(TokenKind::Volatile) {}
    let mut ty = base;
    while ts.eat(TokenKind::Star) {
        while ts.eat(TokenKind::Const) || ts.eat(TokenKind::Restrict) || ts.eat(TokenKind::Volatile)
        {
        }
        ty = Type::Ptr(Box::new(ty));
    }
    ty
}

pub fn parse_type(ts: &mut TokenStream) -> Result<Type, CompileError> {
    let had_qualifier = consume_qualifiers(ts);
    let base = parse_base_type(ts, had_qualifier)?;
    Ok(consume_pointers(ts, base))
}
