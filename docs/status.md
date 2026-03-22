# cc24 Project Status

Last updated: 2026-03-21 (Phase 3 complete)

## Completed

### Phase 0 -- Specification (complete)

All COR24 machine specification documents written:

- `docs/isa-summary.md` -- 32 instructions, encoding, registers
- `docs/assembler-syntax.md` -- as24 syntax with examples
- `docs/abi-proposal.md` -- calling convention, stack frame layout
- `docs/c-data-model.md` -- 24-bit int, type sizes, promotions
- `docs/memory-map.md` -- address space, MMIO, stack
- `docs/toolchain-commands.md` -- as24, ld24, longlgo, cor24-rs workflow
- `docs/interrupt-plan.md` -- inline asm, `__attribute__((interrupt))` roadmap
- `docs/cor_24_chibicc_planning_doc.md` -- original planning doc

13 sample assembly programs in `docs/sample-asm/`:

| Sample | Feature |
|--------|---------|
| return_const.s | Constant return |
| add.s | Local variables, arithmetic |
| branch.s | if/else |
| loop.s | while loop |
| call.s | Function call with arguments |
| fib.s | Recursion (MakerLisp gold standard) |
| pointer.s | Address-of, dereference |
| globals.s | Global variable in .data |
| mmio.s | UART TX polling |
| uart_rx.s | UART RX polling |
| led.s | LED GPIO output |
| button.s | Button GPIO input |
| interrupt.s | UART RX interrupt with ISR |

### Phase 1 -- Backend Skeleton (complete)

Compiler written in Rust (not a chibicc fork -- same architecture, from scratch):

- `src/token.rs` -- token types
- `src/span.rs` -- source locations
- `src/error.rs` -- compiler error type
- `src/lexer.rs` -- tokenizer
- `src/parser.rs` -- recursive descent parser
- `src/ast.rs` -- AST types
- `src/codegen.rs` -- COR24 assembly emitter
- `src/lib.rs` -- module declarations
- `src/main.rs` -- CLI: `cc24 input.c [-o output.s]`

### Phase 2 -- Expressions, Locals, Control Flow (complete)

The compiler handles:

- Integer constants (lc for -128..127, la for larger)
- Local variable declarations with initialization
- Binary operators: `+` `-` `*` `&` `|` `^` `<<` `>>`
- Comparison operators: `==` `!=` `<` `>` `<=` `>=`
- Unary operators: `-` `~` `!`
- Assignment: `x = expr`
- if/else
- while loops
- for loops
- Parenthesized expressions
- Operator precedence (C standard order)

### Phase 3 -- Function Calls and Globals (complete)

The compiler handles:

- **Function calls**: `func(arg1, arg2)` expressions with right-to-left argument push, `la r0,_func`, `jal r1,(r0)`, `add sp,N*3` cleanup
- **Multiple function definitions**: codegen emits `.text` once, then all functions
- **Function parameters**: accessed via positive fp offsets (fp+9, fp+12, etc.)
- **Global variables**: `.data` section with `.word` initializers, `la r1,_varname` / `lw`/`sw` for access
- **String literals**: lexer tokenizes `"..."` with escape sequences
- **Basic `asm("...")`**: lexer recognizes `asm` keyword, parser grabs string literal, codegen emits verbatim
- **Early returns**: per-function return label with `bra` to epilogue

### Test Infrastructure (complete)

Three test layers, 42 active + 13 ignored:

| Layer | File | Auto? | Count |
|-------|------|-------|-------|
| Unit tests | `src/*.rs` | yes | 20 |
| Golden file comparison | `tests/golden.rs` | yes | 9 |
| cor24-run assembler validation | `tests/cor24_assemble.rs` | yes | 13 |
| as24 HTTP service validation | `tests/as24_validate.rs` | `--ignored` | 13 |

Golden fixture files: `tests/fixtures/*.c` and `*.expected.s`
Captured assembler listings: `tests/fixtures/listings/*.lst`

cor24-run assembler: `cor24-run` (in PATH via sw-install)

as24 HTTP service (dev-only): POST http://localhost:7412/assemble
Run with: `cargo test -- --ignored`

### Phase 4 -- Pointers and Arrays

Required for MVP programs 8-11.

- Address-of operator (`&x`) -- compute fp+offset into r0
- Pointer dereference (`*p`) -- `lw r0,0(r0)`
- Pointer arithmetic (scale by element size)
- Array indexing (`a[i]`)
- Pointer casts for MMIO (`*(volatile char *)0xFF0100 = val`)
- `char` type (8-bit load/store with `lb`/`lbu`/`sb`)
- Type casting expressions

Key sample assembly to match: pointer.s, mmio.s, led.s, button.s, uart_rx.s

### Phase 5 -- Refinement

- `__attribute__((interrupt))` for ISR prologue/epilogue
- Software divide/modulo runtime helpers (`__div24` / `__mod24`)
- Peephole optimizations (e.g. `add r0,N` instead of push/lc/pop/add for small constants)
- Better error messages with line/column
- Source-line comments in output (like MakerLisp's `; line 7, file "fib.c"`)

### MVP Gate

Program 12 on the test ladder: compile a C program that prints "Hello\n" over UART.
Requires all of Phases 3 + 4 working together.

## Architecture Decisions Made

- **Language**: Rust (not a chibicc C fork)
- **int size**: 24-bit (native machine word)
- **Pointer size**: 24-bit
- **Expression evaluation**: push lhs, evaluate rhs, pop lhs into r1, operate
- **Local variables**: pre-pass collects all declarations, allocates stack slots at function entry
- **Labels**: L0, L1, L2... (monotonic counter)
- **No IR**: AST directly to assembly text (chibicc style)

## Key Files

```
src/
  main.rs          -- CLI entry point
  lib.rs           -- module declarations
  token.rs         -- TokenKind enum
  span.rs          -- source location
  error.rs         -- CompileError type
  lexer.rs         -- tokenizer
  parser.rs        -- recursive descent parser
  ast.rs           -- Program, Function, Stmt, Expr, Type
  codegen.rs       -- COR24 assembly emitter
tests/
  golden.rs        -- golden file comparison tests
  cor24_assemble.rs -- cor24-run assembler validation
  as24_validate.rs  -- as24 HTTP service validation (#[ignore])
  fixtures/        -- .c inputs and .expected.s golden files
    listings/      -- captured as24 assembler listings
docs/
  sample-asm/      -- 13 reference assembly programs
```
