# cc24 Project Status

Last updated: 2026-03-21

## Current State

The compiler is functional and can compile real C programs to COR24 assembly
that runs on hardware (COR24-TB FPGA board) and the cor24-rs emulator.
12 working demos exercise all implemented features.

## Component Architecture

The compiler is organized into 4 components with 14 crates total, each
component being its own Cargo workspace under `components/`:

| Component | Crates | Purpose |
|-----------|--------|---------|
| core | cc24-ast, cc24-error, cc24-span, cc24-token | Shared types: AST, tokens, spans, errors |
| frontend | cc24-lexer, cc24-lexer-tests, cc24-parser, cc24-parser-tests, cc24-parse-stream, cc24-preprocess, cc24-preprocess-tests | Preprocessing, lexing, parsing |
| backend | cc24-codegen, cc24-codegen-tests, cc24-codegen-validate | Code generation and validation |
| cli | cc24 | CLI binary entry point |

## Implemented Features

### Types
- int (24-bit), char (8-bit), void
- Pointers (int *, char **, arbitrary depth)
- Arrays (int a[N], char buf[N])

### Expressions
- Decimal and hex integer literals
- String literals with escape sequences
- All C binary operators: + - * / % & | ^ << >> == != < > <= >= && ||
- Unary operators: - ~ ! & (address-of) * (dereference)
- Full C operator precedence
- Pointer arithmetic with element-size scaling
- Type cast expressions
- Array indexing (read and write)
- Function call expressions

### Statements
- Variable declarations with initialization
- Assignment (simple, array element, dereference)
- if/else, while, do...while, for
- return (early return supported)
- Expression statements
- Inline asm("...")

### Functions
- Multiple parameters
- Recursion
- void functions
- __attribute__((interrupt)) for ISR prologue/epilogue

### Globals
- int, char, and pointer global variables
- Correct .word/.byte emission by type

### Preprocessor
- #define (constant substitution)
- #include "..." (relative path)
- #include <...> (system path with -I flag)
- #pragma once

### Runtime
- _start entry point calling _main
- Software __cc24_div and __cc24_mod helpers
- ISR support with register save/restore and rti

## Demos

12 demos in the `demos/` directory, each with a run script:

| Demo | Features Exercised |
|------|--------------------|
| demo.c | Globals, function calls, recursion (fib), if/else, while, for |
| demo2.c | char type, pointers, casts, MMIO (LED, UART TX) |
| demo3.c | Hex literals, pointer arithmetic, string constants |
| demo4.c | Software division and modulo |
| demo5.c | Array declarations, array indexing (read/write) |
| demo6.c | Global char, global pointer, .byte/.word emission |
| demo7.c | Pointer subtraction (ptr-int, ptr-ptr with scaling) |
| demo8.c | Preprocessor #define |
| demo9.c | __attribute__((interrupt)), ISR, UART RX interrupt |
| demo10.c | #include, #pragma once, system headers, -I flag |
| demo11.c | Logical && and || with short-circuit evaluation |
| demo12.c | do...while loop |

## Test Suite

54 active tests, 13 ignored (require as24 HTTP service):

| Layer | Crate | Count | Notes |
|-------|-------|-------|-------|
| Lexer unit tests | cc24-lexer-tests | 8 | Token types, operators, comments, hex, strings |
| Parser unit tests | cc24-parser-tests | 11 | Statements, types, arrays, pointers, control flow |
| Preprocessor tests | cc24-preprocess-tests | 8 | #define, #include, #pragma once |
| Golden file tests | cc24-codegen-tests | 6 | Compile C, compare assembly output |
| Codegen checks | cc24-codegen-tests | 2 | _start emission, ISR prologue/epilogue |
| cor24-run validation | cc24-codegen-validate | 19 | Assemble output with cor24-run |
| as24 HTTP validation | cc24-codegen-validate | 13 (ignored) | Validate with as24 service on port 7412 |

Run all active tests:

```
scripts/build-all.sh
```

Run ignored as24 tests (requires service on localhost:7412):

```
cargo test --manifest-path components/backend/Cargo.toml -- --ignored
```

## Architecture Decisions

- **Language**: Rust (not a chibicc fork -- same architecture, written from scratch)
- **int size**: 24-bit (native COR24 word)
- **No IR**: AST directly to assembly text (chibicc style)
- **Expression evaluation**: push lhs, evaluate rhs, pop lhs into r1, operate
- **Local variables**: pre-pass collects all declarations, allocates stack at function entry
- **Labels**: L0, L1, L2... (monotonic counter)
- **Design constraints**: <5 functions/module, <5 modules/crate, prefer many small crates

## Known Limitations

- No switch/case, break/continue, ++/--, sizeof, typedef, enum, struct
- No function prototypes (forward declarations)
- Branch range limited to signed 8-bit offset (~127 bytes)
- No optimization passes
- Single translation unit only
