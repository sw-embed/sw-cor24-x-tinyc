use crate::ast::{BinOp, Block, Expr, Function, GlobalDecl, Param, Program, Stmt, Type, UnaryOp};
use crate::error::CompileError;
use crate::span::Span;
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, CompileError> {
        let mut functions = Vec::new();
        let mut globals = Vec::new();
        while !self.at_eof() {
            // Disambiguate: type ident '(' → function, type ident '='|';' → global
            if self.is_global_decl() {
                globals.push(self.parse_global_decl()?);
            } else {
                functions.push(self.parse_function()?);
            }
        }
        Ok(Program { functions, globals })
    }

    /// Peek ahead to determine if the current position is a global variable
    /// declaration (type ident = ...; or type ident ;) vs a function definition.
    fn is_global_decl(&self) -> bool {
        // Must start with a type keyword
        let kind0 = &self.tokens[self.pos].kind;
        if !matches!(kind0, TokenKind::Int | TokenKind::Void) {
            return false;
        }
        // Next must be an identifier
        if self.pos + 1 >= self.tokens.len() {
            return false;
        }
        if !matches!(&self.tokens[self.pos + 1].kind, TokenKind::Ident(_)) {
            return false;
        }
        // If followed by '(', it's a function; otherwise it's a global
        if self.pos + 2 >= self.tokens.len() {
            return false;
        }
        !matches!(&self.tokens[self.pos + 2].kind, TokenKind::LParen)
    }

    fn parse_global_decl(&mut self) -> Result<GlobalDecl, CompileError> {
        let ty = self.parse_type()?;
        let name = self.expect_ident()?;
        let init = if self.eat(TokenKind::Assign) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(GlobalDecl { name, ty, init })
    }

    fn parse_function(&mut self) -> Result<Function, CompileError> {
        let start = self.current_span();
        let return_ty = self.parse_type()?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LParen)?;
        let params = self.parse_params()?;
        self.expect(TokenKind::RParen)?;
        let body = self.parse_block()?;
        Ok(Function {
            name,
            return_ty,
            params,
            body,
            span: start,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, CompileError> {
        let mut params = Vec::new();
        if self.check(&TokenKind::RParen) {
            return Ok(params);
        }
        loop {
            let ty = self.parse_type()?;
            let name = self.expect_ident()?;
            params.push(Param { name, ty });
            if !self.eat(TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_block(&mut self) -> Result<Block, CompileError> {
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while !self.check(&TokenKind::RBrace) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Block { stmts })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, CompileError> {
        // return expr ;
        if self.eat(TokenKind::Return) {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            return Ok(Stmt::Return(expr));
        }

        // if ( expr ) block [else block]
        if self.eat(TokenKind::If) {
            self.expect(TokenKind::LParen)?;
            let cond = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            let then_body = self.parse_block()?;
            let else_body = if self.eat(TokenKind::Else) {
                Some(self.parse_block()?)
            } else {
                None
            };
            return Ok(Stmt::If {
                cond,
                then_body,
                else_body,
            });
        }

        // while ( expr ) block
        if self.eat(TokenKind::While) {
            self.expect(TokenKind::LParen)?;
            let cond = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            let body = self.parse_block()?;
            return Ok(Stmt::While { cond, body });
        }

        // for ( init ; cond ; inc ) block
        if self.eat(TokenKind::For) {
            self.expect(TokenKind::LParen)?;
            // init: either a local decl or expression or empty
            let init = if self.check(&TokenKind::Semicolon) {
                None
            } else if self.check(&TokenKind::Int) || self.check(&TokenKind::Void) {
                Some(Box::new(self.parse_local_decl()?))
            } else {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::Semicolon)?;
                Some(Box::new(Stmt::Expr(expr)))
            };
            // If init was a local decl, the semicolon is already consumed
            if init.is_none() {
                self.expect(TokenKind::Semicolon)?;
            }
            let cond = if self.check(&TokenKind::Semicolon) {
                None
            } else {
                Some(self.parse_expr()?)
            };
            self.expect(TokenKind::Semicolon)?;
            let inc = if self.check(&TokenKind::RParen) {
                None
            } else {
                Some(self.parse_expr()?)
            };
            self.expect(TokenKind::RParen)?;
            let body = self.parse_block()?;
            return Ok(Stmt::For {
                init,
                cond,
                inc,
                body,
            });
        }

        // asm("...") ;
        if self.eat(TokenKind::Asm) {
            self.expect(TokenKind::LParen)?;
            let TokenKind::StringLit(s) = &self.peek().kind else {
                return Err(CompileError::new(
                    "expected string literal after asm(",
                    Some(self.current_span()),
                ));
            };
            let s = s.clone();
            self.advance();
            self.expect(TokenKind::RParen)?;
            self.expect(TokenKind::Semicolon)?;
            return Ok(Stmt::Asm(s));
        }

        // local variable declaration: type ident [= expr] ;
        if self.check(&TokenKind::Int) || self.check(&TokenKind::Void) {
            return self.parse_local_decl();
        }

        // expression statement
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_local_decl(&mut self) -> Result<Stmt, CompileError> {
        let ty = self.parse_type()?;
        let name = self.expect_ident()?;
        let init = if self.eat(TokenKind::Assign) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::LocalDecl { name, ty, init })
    }

    // --- Expression parsing (precedence climbing) ---

    fn parse_expr(&mut self) -> Result<Expr, CompileError> {
        self.parse_assign()
    }

    fn parse_assign(&mut self) -> Result<Expr, CompileError> {
        let expr = self.parse_or()?;
        if self.eat(TokenKind::Assign) {
            let Expr::Ident(name) = expr else {
                return Err(CompileError::new(
                    "left side of assignment must be a variable",
                    None,
                ));
            };
            let value = self.parse_assign()?;
            return Ok(Expr::Assign {
                name,
                value: Box::new(value),
            });
        }
        Ok(expr)
    }

    fn parse_or(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_xor()?;
        while self.eat(TokenKind::Pipe) {
            let rhs = self.parse_xor()?;
            lhs = Expr::BinOp {
                op: BinOp::BitOr,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_xor(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_and()?;
        while self.eat(TokenKind::Caret) {
            let rhs = self.parse_and()?;
            lhs = Expr::BinOp {
                op: BinOp::BitXor,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_and(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_equality()?;
        while self.eat(TokenKind::Amp) {
            let rhs = self.parse_equality()?;
            lhs = Expr::BinOp {
                op: BinOp::BitAnd,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_equality(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_relational()?;
        loop {
            let op = if self.eat(TokenKind::EqEq) {
                BinOp::Eq
            } else if self.eat(TokenKind::BangEq) {
                BinOp::Ne
            } else {
                break;
            };
            let rhs = self.parse_relational()?;
            lhs = Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_relational(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_shift()?;
        loop {
            let op = if self.eat(TokenKind::Lt) {
                BinOp::Lt
            } else if self.eat(TokenKind::Gt) {
                BinOp::Gt
            } else if self.eat(TokenKind::LtEq) {
                BinOp::Le
            } else if self.eat(TokenKind::GtEq) {
                BinOp::Ge
            } else {
                break;
            };
            let rhs = self.parse_shift()?;
            lhs = Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_shift(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_additive()?;
        loop {
            let op = if self.eat(TokenKind::LShift) {
                BinOp::Shl
            } else if self.eat(TokenKind::RShift) {
                BinOp::Shr
            } else {
                break;
            };
            let rhs = self.parse_additive()?;
            lhs = Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_additive(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_multiplicative()?;
        loop {
            let op = if self.eat(TokenKind::Plus) {
                BinOp::Add
            } else if self.eat(TokenKind::Minus) {
                BinOp::Sub
            } else {
                break;
            };
            let rhs = self.parse_multiplicative()?;
            lhs = Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_unary()?;
        loop {
            let op = if self.eat(TokenKind::Star) {
                BinOp::Mul
            } else if self.eat(TokenKind::Slash) {
                BinOp::Div
            } else if self.eat(TokenKind::Percent) {
                BinOp::Mod
            } else {
                break;
            };
            let rhs = self.parse_unary()?;
            lhs = Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, CompileError> {
        if self.eat(TokenKind::Minus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
            });
        }
        if self.eat(TokenKind::Tilde) {
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnaryOp::BitNot,
                operand: Box::new(operand),
            });
        }
        if self.eat(TokenKind::Bang) {
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp {
                op: UnaryOp::LogNot,
                operand: Box::new(operand),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, CompileError> {
        // Parenthesized expression
        if self.eat(TokenKind::LParen) {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::RParen)?;
            return Ok(expr);
        }

        // Integer literal
        if let TokenKind::IntLit(_) = &self.peek().kind {
            let TokenKind::IntLit(val) = self.advance().kind else {
                unreachable!()
            };
            return Ok(Expr::IntLit(val));
        }

        // String literal
        if let TokenKind::StringLit(_) = &self.peek().kind {
            let TokenKind::StringLit(s) = self.advance().kind else {
                unreachable!()
            };
            return Ok(Expr::StringLit(s));
        }

        // Identifier or function call
        if let TokenKind::Ident(_) = &self.peek().kind {
            let TokenKind::Ident(name) = self.advance().kind else {
                unreachable!()
            };
            // Function call: ident '(' args ')'
            if self.eat(TokenKind::LParen) {
                let mut args = Vec::new();
                if !self.check(&TokenKind::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if !self.eat(TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RParen)?;
                return Ok(Expr::Call { name, args });
            }
            return Ok(Expr::Ident(name));
        }

        Err(CompileError::new(
            format!("expected expression, got {:?}", self.peek().kind),
            Some(self.current_span()),
        ))
    }

    fn parse_type(&mut self) -> Result<Type, CompileError> {
        match self.peek().kind {
            TokenKind::Int => {
                self.advance();
                Ok(Type::Int)
            }
            TokenKind::Void => {
                self.advance();
                Ok(Type::Void)
            }
            _ => Err(CompileError::new(
                format!("expected type, got {:?}", self.peek().kind),
                Some(self.current_span()),
            )),
        }
    }

    // --- Helper methods ---

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.check(&kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, CompileError> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(CompileError::new(
                format!("expected {:?}, got {:?}", kind, self.peek().kind),
                Some(self.current_span()),
            ))
        }
    }

    fn expect_ident(&mut self) -> Result<String, CompileError> {
        if let TokenKind::Ident(_) = &self.peek().kind {
            let TokenKind::Ident(name) = self.advance().kind else {
                unreachable!()
            };
            Ok(name)
        } else {
            Err(CompileError::new(
                format!("expected identifier, got {:?}", self.peek().kind),
                Some(self.current_span()),
            ))
        }
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn current_span(&self) -> Span {
        self.peek().span
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse_source(src: &str) -> Program {
        let tokens = Lexer::new(src).tokenize().unwrap();
        Parser::new(tokens).parse().unwrap()
    }

    #[test]
    fn parse_return_42() {
        let program = parse_source("int main() { return 42; }");
        assert_eq!(program.functions.len(), 1);
        let func = &program.functions[0];
        assert_eq!(func.name, "main");
        assert_eq!(func.return_ty, Type::Int);
        assert!(matches!(
            &func.body.stmts[0],
            Stmt::Return(Expr::IntLit(42))
        ));
    }

    #[test]
    fn parse_local_decl_and_return() {
        let program = parse_source("int main() { int a = 5; return a; }");
        let stmts = &program.functions[0].body.stmts;
        assert_eq!(stmts.len(), 2);
        assert!(matches!(
            &stmts[0],
            Stmt::LocalDecl {
                name,
                ty: Type::Int,
                init: Some(Expr::IntLit(5)),
            } if name == "a"
        ));
        assert!(matches!(&stmts[1], Stmt::Return(Expr::Ident(n)) if n == "a"));
    }

    #[test]
    fn parse_binary_ops() {
        let program = parse_source("int main() { return 2 + 3 * 4; }");
        let Stmt::Return(expr) = &program.functions[0].body.stmts[0] else {
            panic!()
        };
        // Should be Add(2, Mul(3, 4)) due to precedence
        assert!(matches!(expr, Expr::BinOp { op: BinOp::Add, .. }));
    }

    #[test]
    fn parse_if_else() {
        let program = parse_source("int main() { if (1) { return 3; } else { return 4; } }");
        assert!(matches!(
            &program.functions[0].body.stmts[0],
            Stmt::If {
                else_body: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn parse_while_loop() {
        let program = parse_source("int main() { int i = 0; while (i < 5) { i = i + 1; } }");
        assert!(matches!(
            &program.functions[0].body.stmts[1],
            Stmt::While { .. }
        ));
    }

    #[test]
    fn parse_for_loop() {
        let program = parse_source(
            "int main() { int s = 0; for (int i = 0; i < 10; i = i + 1) { s = s + i; } }",
        );
        assert!(matches!(
            &program.functions[0].body.stmts[1],
            Stmt::For { .. }
        ));
    }
}
