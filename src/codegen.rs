use std::collections::{HashMap, HashSet};

use crate::ast::{BinOp, Expr, Function, Program, Stmt, UnaryOp};

#[derive(Default)]
pub struct Codegen {
    out: String,
    label_counter: usize,
    /// Map from local variable name to fp-relative offset (negative).
    /// First local is at -3(fp), second at -6(fp), etc.
    locals: HashMap<String, i32>,
    /// Total bytes allocated for locals in current function.
    locals_size: i32,
    /// Set of global variable names.
    globals: HashSet<String>,
    /// Return label for the current function (for early returns).
    return_label: String,
}

impl Codegen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate(&mut self, program: &Program) -> String {
        // Collect global variable names
        for g in &program.globals {
            self.globals.insert(g.name.clone());
        }

        // .text section with all functions
        self.emit("        .text");
        for func in &program.functions {
            self.emit("");
            self.gen_function(func);
        }

        // .data section with global variables
        if !program.globals.is_empty() {
            self.emit("");
            self.emit("        .data");
            for g in &program.globals {
                self.emit(&format!("_{}:", g.name));
                if let Some(Expr::IntLit(val)) = &g.init {
                    self.emit(&format!("        .word   {val}"));
                } else {
                    self.emit("        .word   0");
                }
            }
        }

        self.out.clone()
    }

    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    fn gen_function(&mut self, func: &Function) {
        self.locals.clear();
        self.locals_size = 0;
        self.return_label = self.new_label();

        // First pass: collect all local variable declarations to compute stack size
        self.collect_locals_block(&func.body.stmts);
        // Also add parameters as locals (accessed at positive fp offsets)
        // Parameters are at fp+9, fp+12, fp+15, ...
        for (i, param) in func.params.iter().enumerate() {
            let offset = 9 + (i as i32) * 3;
            self.locals.insert(param.name.clone(), offset);
        }

        self.emit(&format!("        .globl  _{}", func.name));
        self.emit(&format!("_{}:", func.name));

        // Prologue
        self.emit("        push    fp");
        self.emit("        push    r2");
        self.emit("        push    r1");
        self.emit("        mov     fp,sp");

        // Allocate locals
        if self.locals_size > 0 {
            if self.locals_size <= 127 {
                self.emit(&format!("        add     sp,-{}", self.locals_size));
            } else {
                self.emit(&format!("        sub     sp,{}", self.locals_size));
            }
        }

        // Body
        for stmt in &func.body.stmts {
            self.gen_stmt(stmt);
        }

        // Epilogue at return label
        let ret_label = self.return_label.clone();
        self.emit(&format!("{ret_label}:"));
        self.emit("        mov     sp,fp");
        self.emit("        pop     r1");
        self.emit("        pop     r2");
        self.emit("        pop     fp");
        self.emit("        jmp     (r1)");
    }

    /// Pre-pass to allocate stack slots for local variables.
    fn collect_locals_block(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.collect_locals_stmt(stmt);
        }
    }

    fn collect_locals_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::LocalDecl { name, .. } => {
                if !self.locals.contains_key(name) {
                    self.locals_size += 3; // one 24-bit word
                    let offset = -self.locals_size;
                    self.locals.insert(name.clone(), offset);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                self.collect_locals_block(&then_body.stmts);
                if let Some(eb) = else_body {
                    self.collect_locals_block(&eb.stmts);
                }
            }
            Stmt::While { body, .. } => {
                self.collect_locals_block(&body.stmts);
            }
            Stmt::For { init, body, .. } => {
                if let Some(init_stmt) = init {
                    self.collect_locals_stmt(init_stmt);
                }
                self.collect_locals_block(&body.stmts);
            }
            _ => {}
        }
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Return(expr) => {
                self.gen_expr(expr);
                let ret_label = self.return_label.clone();
                self.emit(&format!("        bra     {ret_label}"));
            }
            Stmt::Expr(expr) => {
                self.gen_expr(expr);
            }
            Stmt::LocalDecl { name, init, .. } => {
                if let Some(init_expr) = init {
                    self.gen_expr(init_expr);
                    let offset = self.locals[name];
                    self.emit(&format!("        sw      r0,{offset}(fp)"));
                }
            }
            Stmt::If {
                cond,
                then_body,
                else_body,
            } => {
                let else_label = self.new_label();
                let done_label = self.new_label();

                self.gen_expr(cond);
                self.emit("        ceq     r0,z");
                if else_body.is_some() {
                    self.emit(&format!("        brt     {else_label}"));
                } else {
                    self.emit(&format!("        brt     {done_label}"));
                }

                // Then block
                for s in &then_body.stmts {
                    self.gen_stmt(s);
                }

                if let Some(eb) = else_body {
                    self.emit(&format!("        bra     {done_label}"));
                    self.emit(&format!("{else_label}:"));
                    for s in &eb.stmts {
                        self.gen_stmt(s);
                    }
                }

                self.emit(&format!("{done_label}:"));
            }
            Stmt::While { cond, body } => {
                let loop_label = self.new_label();
                let done_label = self.new_label();

                self.emit(&format!("{loop_label}:"));
                self.gen_expr(cond);
                self.emit("        ceq     r0,z");
                self.emit(&format!("        brt     {done_label}"));

                for s in &body.stmts {
                    self.gen_stmt(s);
                }
                self.emit(&format!("        bra     {loop_label}"));
                self.emit(&format!("{done_label}:"));
            }
            Stmt::For {
                init,
                cond,
                inc,
                body,
            } => {
                if let Some(init_stmt) = init {
                    self.gen_stmt(init_stmt);
                }

                let loop_label = self.new_label();
                let done_label = self.new_label();

                self.emit(&format!("{loop_label}:"));
                if let Some(cond_expr) = cond {
                    self.gen_expr(cond_expr);
                    self.emit("        ceq     r0,z");
                    self.emit(&format!("        brt     {done_label}"));
                }

                for s in &body.stmts {
                    self.gen_stmt(s);
                }

                if let Some(inc_expr) = inc {
                    self.gen_expr(inc_expr);
                }

                self.emit(&format!("        bra     {loop_label}"));
                self.emit(&format!("{done_label}:"));
            }
            Stmt::Asm(text) => {
                // Emit each line of the asm string verbatim
                for line in text.lines() {
                    self.emit(&format!("        {line}"));
                }
            }
        }
    }

    /// Generate code for an expression. Result is left in r0.
    fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLit(val) => {
                self.load_immediate(*val);
            }
            Expr::StringLit(_s) => {
                // TODO: string literals in .data section
                // For now, not needed (asm() takes strings at compile time)
            }
            Expr::Ident(name) => {
                if self.globals.contains(name) {
                    self.emit(&format!("        la      r1,_{name}"));
                    self.emit("        lw      r0,0(r1)");
                } else {
                    let offset = self.locals[name];
                    self.emit(&format!("        lw      r0,{offset}(fp)"));
                }
            }
            Expr::Assign { name, value } => {
                self.gen_expr(value);
                if self.globals.contains(name) {
                    self.emit(&format!("        la      r1,_{name}"));
                    self.emit("        sw      r0,0(r1)");
                } else {
                    let offset = self.locals[name];
                    self.emit(&format!("        sw      r0,{offset}(fp)"));
                }
            }
            Expr::Call { name, args } => {
                // Push arguments right-to-left
                for arg in args.iter().rev() {
                    self.gen_expr(arg);
                    self.emit("        push    r0");
                }
                // Call function
                self.emit(&format!("        la      r0,_{name}"));
                self.emit("        jal     r1,(r0)");
                // Clean up arguments
                if !args.is_empty() {
                    let cleanup = args.len() as i32 * 3;
                    self.emit(&format!("        add     sp,{cleanup}"));
                }
                // Result is in r0
            }
            Expr::UnaryOp { op, operand } => {
                self.gen_expr(operand);
                match op {
                    UnaryOp::Neg => {
                        // r0 = 0 - r0: save r0, load 0, sub
                        self.emit("        push    r0");
                        self.emit("        lc      r0,0");
                        self.emit("        pop     r1");
                        self.emit("        sub     r0,r1");
                    }
                    UnaryOp::BitNot => {
                        // r0 = r0 ^ 0xFFFFFF
                        self.emit("        lc      r1,-1");
                        self.emit("        xor     r0,r1");
                    }
                    UnaryOp::LogNot => {
                        // r0 = (r0 == 0) ? 1 : 0
                        self.emit("        ceq     r0,z");
                        self.emit("        mov     r0,c");
                    }
                }
            }
            Expr::BinOp { op, lhs, rhs } => {
                // Evaluate lhs into r0, push, evaluate rhs into r0,
                // pop lhs into r1, compute.
                self.gen_expr(lhs);
                self.emit("        push    r0");
                self.gen_expr(rhs);
                self.emit("        mov     r1,r0"); // r1 = rhs
                self.emit("        pop     r0"); // r0 = lhs
                self.gen_binop(*op);
            }
        }
    }

    fn gen_binop(&mut self, op: BinOp) {
        // r0 = lhs, r1 = rhs, result in r0
        match op {
            BinOp::Add => self.emit("        add     r0,r1"),
            BinOp::Sub => self.emit("        sub     r0,r1"),
            BinOp::Mul => self.emit("        mul     r0,r1"),
            BinOp::BitAnd => self.emit("        and     r0,r1"),
            BinOp::BitOr => self.emit("        or      r0,r1"),
            BinOp::BitXor => self.emit("        xor     r0,r1"),
            BinOp::Shl => self.emit("        shl     r0,r1"),
            BinOp::Shr => self.emit("        srl     r0,r1"),
            BinOp::Eq => {
                self.emit("        ceq     r0,r1");
                self.emit("        mov     r0,c");
            }
            BinOp::Ne => {
                self.emit("        ceq     r0,r1");
                self.emit("        mov     r0,c");
                // Invert: r0 = !r0
                self.emit("        ceq     r0,z");
                self.emit("        mov     r0,c");
            }
            BinOp::Lt => {
                self.emit("        cls     r0,r1");
                self.emit("        mov     r0,c");
            }
            BinOp::Gt => {
                // lhs > rhs  =>  rhs < lhs
                self.emit("        cls     r1,r0");
                self.emit("        mov     r0,c");
            }
            BinOp::Le => {
                // lhs <= rhs  =>  !(rhs < lhs)
                self.emit("        cls     r1,r0");
                self.emit("        mov     r0,c");
                self.emit("        ceq     r0,z");
                self.emit("        mov     r0,c");
            }
            BinOp::Ge => {
                // lhs >= rhs  =>  !(lhs < rhs)
                self.emit("        cls     r0,r1");
                self.emit("        mov     r0,c");
                self.emit("        ceq     r0,z");
                self.emit("        mov     r0,c");
            }
            BinOp::Div | BinOp::Mod => {
                // No hardware divide -- emit a placeholder call to runtime helper
                // TODO: implement __div24 / __mod24 runtime routines
                if op == BinOp::Div {
                    self.emit("        ; TODO: software divide r0/r1");
                } else {
                    self.emit("        ; TODO: software modulo r0%r1");
                }
            }
        }
    }

    fn load_immediate(&mut self, val: i32) {
        if (-128..=127).contains(&val) {
            self.emit(&format!("        lc      r0,{val}"));
        } else {
            self.emit(&format!("        la      r0,{val}"));
        }
    }

    fn emit(&mut self, line: &str) {
        self.out.push_str(line);
        self.out.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn compile(source: &str) -> String {
        let tokens = Lexer::new(source).tokenize().unwrap();
        let program = Parser::new(tokens).parse().unwrap();
        Codegen::new().generate(&program)
    }

    #[test]
    fn codegen_return_42() {
        let output = compile("int main() { return 42; }");
        let expected = include_str!("../tests/fixtures/return_42.expected.s");
        assert_eq!(output, expected);
    }

    #[test]
    fn codegen_return_large_constant() {
        let output = compile("int main() { return 1000; }");
        assert!(output.contains("        la      r0,1000"));
    }

    #[test]
    fn codegen_locals_add() {
        let output = compile("int main() { int a = 2; int b = 3; return a + b; }");
        assert!(output.contains("        add     sp,-6"));
        assert!(output.contains("sw      r0,-3(fp)"));
        assert!(output.contains("sw      r0,-6(fp)"));
        assert!(output.contains("        add     r0,r1"));
    }

    #[test]
    fn codegen_if_else() {
        let output = compile("int main() { if (1) { return 3; } else { return 4; } }");
        assert!(output.contains("        ceq     r0,z"));
        // L0 is return label, L1 is else label
        assert!(output.contains("        brt     L1"));
        assert!(output.contains("        bra     L0")); // return jump
    }

    #[test]
    fn codegen_while_loop() {
        let output = compile("int main() { int i = 0; while (i < 5) { i = i + 1; } return i; }");
        // L0 is return label, L1 is loop, L2 is done
        assert!(output.contains("L1:"));
        assert!(output.contains("        brt     L2"));
        assert!(output.contains("        bra     L1"));
        assert!(output.contains("L2:"));
    }

    #[test]
    fn codegen_unary_neg() {
        let output = compile("int main() { return -42; }");
        assert!(output.contains("        lc      r0,42"));
        assert!(output.contains("        sub     r0,r1"));
    }

    #[test]
    fn codegen_comparison_eq() {
        let output = compile("int main() { return 1 == 1; }");
        assert!(output.contains("        ceq     r0,r1"));
        assert!(output.contains("        mov     r0,c"));
    }

    #[test]
    fn codegen_function_call() {
        let output = compile(
            "int add(int a, int b) { return a + b; } int main() { return add(2, 5); }",
        );
        // Should have two functions
        assert!(output.contains("_add:"));
        assert!(output.contains("_main:"));
        // Caller pushes args right-to-left
        assert!(output.contains("        lc      r0,5"));
        assert!(output.contains("        lc      r0,2"));
        // Call sequence
        assert!(output.contains("        la      r0,_add"));
        assert!(output.contains("        jal     r1,(r0)"));
        assert!(output.contains("        add     sp,6"));
        // Callee accesses params at positive fp offsets
        assert!(output.contains("        lw      r0,9(fp)"));
        assert!(output.contains("        lw      r0,12(fp)"));
    }

    #[test]
    fn codegen_global_var() {
        let output = compile("int x = 10; int main() { x = x + 5; return x; }");
        // .data section
        assert!(output.contains("        .data"));
        assert!(output.contains("_x:"));
        assert!(output.contains("        .word   10"));
        // Read global: la + lw
        assert!(output.contains("        la      r1,_x"));
        assert!(output.contains("        lw      r0,0(r1)"));
        // Write global: la + sw
        assert!(output.contains("        sw      r0,0(r1)"));
    }
}
