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

/// Compute the end offset (exclusive) of a struct member.
/// For bitfields, the end is the word boundary (offset + 3).
fn member_end_offset(m: &StructMember) -> i32 {
    if m.bit_width > 0 {
        m.offset + 3 // bitfield occupies (part of) a 3-byte word
    } else {
        m.offset + m.ty.size()
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
        members
            .iter()
            .map(|m| member_end_offset(m))
            .max()
            .unwrap_or(0)
    } else {
        members
            .iter()
            .map(|m| member_end_offset(m))
            .max()
            .unwrap_or(0)
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
    let mut offset: i32 = 0;
    // Bitfield packing state: current bit position within the word at `offset`
    let mut bit_pos: u8 = 0;
    let mut in_bitfield_word = false;
    while !ts.check(&TokenKind::RBrace) {
        let base_ty = parse_type_fn(ts)?;
        // Anonymous struct/union member: flatten inner members into parent
        if ts.check(&TokenKind::Semicolon) {
            if let Type::Struct { members: inner, .. } = &base_ty {
                // End any active bitfield word before flattening
                if in_bitfield_word && !is_union {
                    offset += 3; // word size
                    bit_pos = 0;
                    in_bitfield_word = false;
                }
                for m in inner {
                    let member_offset = if is_union { 0 } else { offset + m.offset };
                    members.push(StructMember {
                        name: m.name.clone(),
                        ty: m.ty.clone(),
                        offset: member_offset,
                        bit_width: m.bit_width,
                        bit_offset: m.bit_offset,
                    });
                }
                if !is_union {
                    offset += base_ty.size();
                }
                ts.expect(TokenKind::Semicolon)?;
                continue;
            }
        }
        // Parse first member name + optional array/bitfield suffix
        let name = ts.expect_ident()?;
        // Check for bitfield: int x : N;
        if ts.eat(TokenKind::Colon) {
            let width = parse_bitfield_width(ts)?;
            push_bitfield(
                &mut members,
                name,
                base_ty.clone(),
                width,
                &mut offset,
                &mut bit_pos,
                &mut in_bitfield_word,
                is_union,
            );
        } else {
            let ty = parse_member_array(ts, base_ty.clone())?;
            // Non-bitfield: end any active bitfield word
            if in_bitfield_word && !is_union {
                offset += 3;
                bit_pos = 0;
                in_bitfield_word = false;
            }
            let size = ty.size();
            let member_offset = if is_union { 0 } else { offset };
            members.push(StructMember {
                name,
                ty,
                offset: member_offset,
                bit_width: 0,
                bit_offset: 0,
            });
            if !is_union {
                offset += size;
            }
        }
        // Comma-separated members of same type: int a, b; or int a:2, b:3;
        while ts.eat(TokenKind::Comma) {
            let name = ts.expect_ident()?;
            if ts.eat(TokenKind::Colon) {
                let width = parse_bitfield_width(ts)?;
                push_bitfield(
                    &mut members,
                    name,
                    base_ty.clone(),
                    width,
                    &mut offset,
                    &mut bit_pos,
                    &mut in_bitfield_word,
                    is_union,
                );
            } else {
                let ty = parse_member_array(ts, base_ty.clone())?;
                if in_bitfield_word && !is_union {
                    offset += 3;
                    bit_pos = 0;
                    in_bitfield_word = false;
                }
                let size = ty.size();
                let member_offset = if is_union { 0 } else { offset };
                members.push(StructMember {
                    name,
                    ty,
                    offset: member_offset,
                    bit_width: 0,
                    bit_offset: 0,
                });
                if !is_union {
                    offset += size;
                }
            }
        }
        ts.expect(TokenKind::Semicolon)?;
    }
    // Close any trailing bitfield word (offset used by total_size via member_end_offset)
    let _ = (offset, in_bitfield_word);
    Ok(members)
}

/// Parse the integer constant after `:` in a bitfield declaration.
fn parse_bitfield_width(ts: &mut TokenStream) -> Result<u8, CompileError> {
    let TokenKind::IntLit(w) = ts.peek().kind else {
        return Err(CompileError::new(
            "expected integer constant for bitfield width",
            Some(ts.current_span()),
        ));
    };
    ts.advance();
    Ok(w as u8)
}

/// Add a bitfield member, packing into the current word.
/// COR24 word = 24 bits (3 bytes).
fn push_bitfield(
    members: &mut Vec<StructMember>,
    name: String,
    ty: Type,
    width: u8,
    offset: &mut i32,
    bit_pos: &mut u8,
    in_bitfield_word: &mut bool,
    is_union: bool,
) {
    const WORD_BITS: u8 = 24;
    // Zero-width bitfield: force next word boundary
    if width == 0 {
        if *in_bitfield_word && !is_union {
            *offset += 3;
            *bit_pos = 0;
            *in_bitfield_word = false;
        }
        return;
    }
    // Check if bitfield fits in the current word
    if !*in_bitfield_word || *bit_pos + width > WORD_BITS {
        // Start a new word
        if *in_bitfield_word && !is_union {
            *offset += 3;
        }
        *bit_pos = 0;
        *in_bitfield_word = true;
    }
    let member_offset = if is_union { 0 } else { *offset };
    members.push(StructMember {
        name,
        ty,
        offset: member_offset,
        bit_width: width,
        bit_offset: *bit_pos,
    });
    if !is_union {
        *bit_pos += width;
    }
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
