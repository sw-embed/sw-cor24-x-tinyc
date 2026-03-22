# cc24 Architecture

## Component Layout

The compiler is split into 4 components, each a Cargo workspace with
multiple crates under a `crates/` directory:

```
components/
  core/                          -- shared types
    crates/
      cc24-ast/                  -- AST node types (Program, Function, Stmt, Expr, Type)
      cc24-error/                -- CompileError type
      cc24-span/                 -- Source location (file, line, column)
      cc24-token/                -- TokenKind enum and Token struct

  frontend/                      -- source text to AST
    crates/
      cc24-preprocess/           -- #define, #include, #pragma once
      cc24-preprocess-tests/     -- preprocessor test suite
      cc24-lexer/                -- tokenizer (operators, readers, main loop)
      cc24-lexer-tests/          -- lexer test suite
      cc24-parse-stream/         -- token stream with expect/peek helpers
      cc24-parser/               -- recursive-descent parser
      cc24-parser-tests/         -- parser test suite

  backend/                       -- AST to assembly
    crates/
      cc24-codegen/              -- COR24 assembly emitter
      cc24-codegen-tests/        -- golden file tests and codegen checks
      cc24-codegen-validate/     -- assembler validation (cor24-run and as24)

  cli/                           -- binary entry point
    crates/
      cc24/                      -- main.rs, argument parsing, orchestration
```

14 crates total. Test crates are separated from library crates so that
test dependencies do not affect the library build.

## Data Flow

Source code flows through the compiler in a linear pipeline:

```
input.c
  |
  v
[preprocess] -- #define substitution, #include expansion, #pragma once
  |
  v
preprocessed source text
  |
  v
[lexer] -- tokenize into TokenKind stream
  |
  v
token stream
  |
  v
[parser] -- recursive descent, produces AST
  |
  v
AST (Program -> Vec<Function> + Vec<Global>)
  |
  v
[codegen] -- walk AST, emit COR24 assembly text
  |
  v
output.s
```

There is no intermediate representation (IR). The AST is walked directly
by the code generator to emit assembly, following the chibicc approach.

## Crate Dependencies

Dependencies flow in one direction: cli -> backend -> frontend -> core.

```
cc24 (cli)
  |
  +-- cc24-codegen
  |     +-- cc24-ast
  |     +-- cc24-error
  |     +-- cc24-span
  |
  +-- cc24-parser
  |     +-- cc24-ast
  |     +-- cc24-token
  |     +-- cc24-parse-stream
  |           +-- cc24-token
  |           +-- cc24-span
  |           +-- cc24-error
  |
  +-- cc24-lexer
  |     +-- cc24-token
  |     +-- cc24-span
  |
  +-- cc24-preprocess
        +-- cc24-span
```

## Design Constraints

The codebase follows size constraints to keep modules focused:

- **< 5 public functions per module** -- each .rs file has a narrow API
- **< 5 modules per crate** -- crates stay small and single-purpose
- **Prefer many small crates** over few large ones
- **Test crates are separate** -- cc24-lexer-tests, cc24-parser-tests, etc.
- **No circular dependencies** -- strictly layered

## Key Design Decisions

### No IR

The compiler emits assembly directly from the AST. This keeps the codebase
small and follows chibicc's architecture. The tradeoff is that optimization
passes would need to work on either the AST or the assembly text.

### Push/Pop Expression Evaluation

Binary expressions are evaluated by:
1. Evaluate left-hand side into r0
2. Push r0
3. Evaluate right-hand side into r0
4. Pop r1 (left-hand side)
5. Apply operator: r0 = r1 op r0

This is simple and correct but generates more push/pop pairs than necessary.
A future peephole pass could eliminate redundant stack traffic.

### Stack-Allocated Locals

At function entry, a pre-pass counts all local variable declarations
(including arrays) and allocates stack space with a single `sub sp,N`.
Variables are accessed at negative offsets from fp. Array elements are
contiguous starting from the variable's base offset.

### Label Generation

Labels are generated from a monotonic counter: L0, L1, L2, etc. Each
control-flow construct consumes one or more labels. The counter is global
across all functions in a translation unit.

### Software Division

COR24 has no hardware divide instruction. When the compiler encounters
`/` or `%`, it emits calls to `__cc24_div` or `__cc24_mod` helper
functions. These are emitted once at the end of the assembly output
if used.

## Module Breakdown

### cc24-codegen Modules

| Module | Purpose |
|--------|---------|
| lib.rs | Top-level codegen entry, _start, globals, runtime helpers |
| func.rs | Function prologue/epilogue, ISR variant |
| stmt.rs | Statement dispatch (if, while, for, return, decl, assign) |
| expr.rs | Expression dispatch (literals, variables, unary, call, cast) |
| binop.rs | Binary operator code generation |
| emit.rs | Assembly text emission helpers |
| runtime.rs | __cc24_div and __cc24_mod implementations |

### cc24-parser Modules

| Module | Purpose |
|--------|---------|
| lib.rs | Top-level parse entry, function and global parsing |
| decl.rs | Type parsing, variable declarations |
| stmt.rs | Statement parsing |
| expr.rs | Expression parsing, precedence climbing |
| arithmetic.rs | Arithmetic and comparison subexpressions |
| bitwise.rs | Bitwise operator subexpressions |
| control.rs | if, while, for, do...while parsing |

### cc24-lexer Modules

| Module | Purpose |
|--------|---------|
| lib.rs | Main tokenizer loop |
| operators.rs | Multi-character operator recognition |
| readers.rs | String, number, identifier readers |
