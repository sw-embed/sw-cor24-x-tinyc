//! Struct and union type parsing.

use tc24r_ast::{StructMember, Type};
use tc24r_error::CompileError;
use tc24r_parse_stream::TokenStream;
use tc24r_token::TokenKind;

/// Callback for parsing member types (avoids circular dependency).
pub type ParseTypeFn = fn(&mut TokenStream) -> Result<Type, CompileError>;

/// Parse a struct or union type after the keyword has been consumed.
/// When `is_union` is true, all members share offset 0 and total_size
/// is the maximum member size (union semantics).
pub fn parse_struct_type(
    ts: &mut TokenStream,
    parse_type_fn: ParseTypeFn,
    is_union: bool,
) -> Result<Type, CompileError> {
    let tag = parse_optional_tag(ts);
    if ts.check(&TokenKind::LBrace) {
        let ty = parse_body(ts, &tag, parse_type_fn, is_union)?;
        if let Some(ref name) = tag {
            ts.struct_types.insert(name.clone(), ty.clone());
        }
        return Ok(ty);
    }
    lookup_named_struct(ts, &tag)
}

fn parse_optional_tag(ts: &mut TokenStream) -> Option<String> {
    if matches!(ts.peek().kind, TokenKind::Ident(_)) {
        let TokenKind::Ident(name) = ts.advance().kind else {
            unreachable!()
        };
        Some(name)
    } else {
        None
    }
}

fn parse_body(
    ts: &mut TokenStream,
    tag: &Option<String>,
    parse_type_fn: ParseTypeFn,
    is_union: bool,
) -> Result<Type, CompileError> {
    ts.expect(TokenKind::LBrace)?;
    let members = parse_members(ts, parse_type_fn, is_union)?;
    ts.expect(TokenKind::RBrace)?;
    let total_size = if is_union {
        members.iter().map(|m| m.ty.size()).max().unwrap_or(0)
    } else {
        members.last().map_or(0, |m| m.offset + m.ty.size())
    };
    Ok(Type::Struct {
        tag: tag.clone(),
        members,
        total_size,
    })
}

fn parse_members(
    ts: &mut TokenStream,
    parse_type_fn: ParseTypeFn,
    is_union: bool,
) -> Result<Vec<StructMember>, CompileError> {
    let mut members = Vec::new();
    let mut offset = 0;
    while !ts.check(&TokenKind::RBrace) {
        let base_ty = parse_type_fn(ts)?;
        // Anonymous struct/union member: flatten inner members into parent
        if ts.check(&TokenKind::Semicolon) {
            if let Type::Struct { members: inner, .. } = &base_ty {
                for m in inner {
                    let member_offset = if is_union { 0 } else { offset + m.offset };
                    members.push(StructMember {
                        name: m.name.clone(),
                        ty: m.ty.clone(),
                        offset: member_offset,
                    });
                }
                if !is_union {
                    offset += base_ty.size();
                }
                ts.expect(TokenKind::Semicolon)?;
                continue;
            }
        }
        // Parse first member name + optional array suffix
        let name = ts.expect_ident()?;
        let ty = parse_member_array(ts, base_ty.clone())?;
        let size = ty.size();
        let member_offset = if is_union { 0 } else { offset };
        members.push(StructMember {
            name,
            ty,
            offset: member_offset,
        });
        if !is_union {
            offset += size;
        }
        // Comma-separated members of same type: int a, b;
        while ts.eat(TokenKind::Comma) {
            let name = ts.expect_ident()?;
            let ty = parse_member_array(ts, base_ty.clone())?;
            let size = ty.size();
            let member_offset = if is_union { 0 } else { offset };
            members.push(StructMember {
                name,
                ty,
                offset: member_offset,
            });
            if !is_union {
                offset += size;
            }
        }
        ts.expect(TokenKind::Semicolon)?;
    }
    Ok(members)
}

/// Parse optional array suffix on a struct member: char a[3];
fn parse_member_array(ts: &mut TokenStream, mut ty: Type) -> Result<Type, CompileError> {
    while ts.eat(TokenKind::LBracket) {
        // Flexible array member: char b[];
        if ts.check(&TokenKind::RBracket) {
            ts.expect(TokenKind::RBracket)?;
            ty = Type::Array(Box::new(ty), 0);
            continue;
        }
        let TokenKind::IntLit(size) = ts.peek().kind else {
            return Err(CompileError::new(
                "expected array size in struct member",
                Some(ts.current_span()),
            ));
        };
        ts.advance();
        ts.expect(TokenKind::RBracket)?;
        ty = Type::Array(Box::new(ty), size as usize);
    }
    Ok(ty)
}

fn lookup_named_struct(ts: &mut TokenStream, tag: &Option<String>) -> Result<Type, CompileError> {
    let name = tag.as_ref().ok_or_else(|| {
        CompileError::new("expected struct body or tag name", Some(ts.current_span()))
    })?;
    if let Some(ty) = ts.struct_types.get(name) {
        return Ok(ty.clone());
    }
    // Forward declaration: register an incomplete struct placeholder.
    // This allows `struct foo *p;` before the struct body is defined.
    // The placeholder will be replaced when the full definition is parsed.
    let placeholder = Type::Struct {
        tag: Some(name.clone()),
        members: Vec::new(),
        total_size: 0,
    };
    ts.struct_types.insert(name.clone(), placeholder.clone());
    Ok(placeholder)
}
