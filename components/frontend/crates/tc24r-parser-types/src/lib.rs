//! tc24r parser.

mod type_detect;
mod type_parse;

pub use type_detect::{
    is_base_type, is_storage_class, is_type_keyword, is_type_start, is_typedef_name,
};
pub use type_parse::parse_type;

/// Skip a typeof expression (balanced parens). Called when typeof(expr)
/// is encountered and the content is not a type keyword.
pub fn parse_typeof_expr(ts: &mut tc24r_parse_stream::TokenStream) {
    // Already inside the parens — skip tokens until we reach the matching )
    // The caller will consume the closing )
    let mut depth = 0i32;
    loop {
        match ts.peek().kind {
            tc24r_token::TokenKind::LParen => {
                depth += 1;
                ts.advance();
            }
            tc24r_token::TokenKind::RParen => {
                if depth == 0 {
                    break;
                }
                depth -= 1;
                ts.advance();
            }
            _ => {
                ts.advance();
            }
        }
    }
}
