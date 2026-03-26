# tc24r Project Status

Last updated: 2026-03-25

## Current State

The compiler is functional and can compile real C programs to COR24 assembly
that runs on hardware (COR24-TB FPGA board) and the cor24-rs emulator.
48 working demos exercise all implemented features.

## Component Architecture

The compiler is organized into 14 components with ~50 crates total, each
component being its own Cargo workspace under `components/`:

| Component | Purpose |
|-----------|---------|
| core | Shared types: AST, tokens, spans, errors, traits |
| frontend | Preprocessing, lexing, parsing |
| backend | Legacy codegen and validation |
| codegen-emit | Assembly emission (core, data, load/store) |
| codegen-expr | Expression codegen (call, literal, ops, pointer, variable, struct) |
| codegen-ops | Operator codegen (arithmetic, bitwise, compare, divmod, incdec, logical, unary, type-infer) |
| codegen-state | Codegen state management |
| codegen-stmt | Statement codegen (control flow, simple statements) |
| codegen-structure | Function structure (ISR, locals, prologue) |
| config | Configuration and target definition |
| dispatch | Codegen dispatch layer |
| macros | Proc macros (asm-dsl, emit-macros, handler-macros) |
| testing | Test harnesses (as24, compile, cor24, golden) |
| cli | CLI binary entry point |

## Implemented Features

### Types
- int (24-bit), char (8-bit), void
- Pointers (int *, char **, arbitrary depth)
- Arrays (int a[N], char buf[N])
- struct (named and anonymous, dot and arrow access)
- enum (named and anonymous)
- typedef

### Expressions
- Decimal and hex integer literals, character literals
- String literals with escape sequences
- All C binary operators: + - * / % & | ^ << >> == != < > <= >= && ||
- Unary operators: - ~ ! & (address-of) * (dereference) ++ --
- Compound assignment: += -= *= /= %= &= |= ^= <<= >>=
- Ternary operator (? :)
- sizeof(type) and sizeof(expr)
- Full C operator precedence
- Pointer arithmetic with element-size scaling
- Type cast expressions
- Array indexing (read and write)
- Function call expressions
- Struct member access (dot and arrow)
- Statement expressions ({ ... })

### Statements
- Variable declarations with initialization
- Multi-declarations (int x, y, z;)
- Assignment (simple, array element, dereference, struct member)
- if/else, while, do...while, for (braced and braceless)
- switch/case/default (with fall-through and break)
- break, continue
- return (early return supported)
- Expression statements
- Inline asm("...")

### Functions
- Multiple parameters
- Recursion
- void functions
- Function prototypes (forward declarations)
- __attribute__((interrupt)) for ISR prologue/epilogue

### Globals
- int, char, and pointer global variables
- Correct .word/.byte emission by type

### Preprocessor
- #define (constant and function-like macro substitution)
- #include "..." (relative path)
- #include <...> (system path with -I flag)
- #pragma once

### Runtime
- _start entry point calling _main
- Software __tc24r_div and __tc24r_mod helpers
- ISR support with register save/restore and rti

## Demos

48 demos in the `demos/` directory, each with a run script:

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
| demo13.c | break, continue |
| demo14.c | Prefix/postfix ++, -- |
| demo15.c | Ternary operator (? :) |
| demo16.c | Character literals ('a', '\n') |
| demo17.c | Multi-declaration (int x, y, z;) |
| demo18.c | sizeof operator |
| demo19.c | static/extern keywords |
| demo20.c | Statement expressions ({ }) |
| demo21.c | Compound assignment (+=, -=, etc.) |
| demo22.c | Braceless control flow bodies |
| demo23.c | enum |
| demo24.c | typedef |
| demo25.c | struct (dot access, sizeof) |
| demo26.c | switch/case (break, fall-through) |
| demo27.c | Function prototypes (forward declarations, mutual recursion) |
| demo28.c | union (shared memory, sizeof) |
| demo29.c | sizeof with array types (int[4], int[3][4]) |
| demo30.c | Line continuation (backslash-newline) |
| demo31.c | Tentative definitions (int x; int x = 5;) |
| demo32.c | Multi-declarator typedef (typedef int A, B[4];) |
| demo33.c | Comma-separated struct/union members (int a, b;) |
| demo34.c | Multi-dimensional array declarations (int a[2][3]) |
| demo35.c | Struct/union array members (char a[3]) |
| demo36.c | Forward-declared struct tags, self-referential structs |
| demo37.c | Anonymous struct/union members (C11) |
| demo38.c | Struct brace initializer (struct s x = {1, 2}) |
| demo39.c | printf via stdio.h, long branches, varargs syntax |
| demo40.c | malloc/free/calloc via stdlib.h (bump allocator) |
| demo41.c | getc/atoi, strlen/strcmp/strcpy via string.h |
| demo42.c | Nested struct member access, linked list traversal |
| demo43.c | Lisp cons cells (struct pointer return, car/cdr chains) |
| demo44.c | Lisp Phase 1: constructors, predicates, S-expr printer |
| demo45.c | Lisp eval: reader + eval + builtins — (+ 40 2) => 42 |
| demo46.c | unsigned int: signed/unsigned shifts (sra/srl), comparisons (cls/clu) |
| demo47.c | ptr[i].member: struct pointer array indexing (BUG-010 fix) |
| demo48.c | Global struct array declaration and lookup (BUG-011 fix) |

## Test Suite

| Layer | Crate | Count | Notes |
|-------|-------|-------|-------|
| Lexer unit tests | tc24r-lexer-tests | 8 | Token types, operators, comments, hex, strings |
| Parser unit tests | tc24r-parser-tests | 11 | Statements, types, arrays, pointers, control flow |
| Preprocessor tests | tc24r-preprocess-tests | 8 | #define, #include, #pragma once |
| Golden file tests | tc24r-codegen-tests | 7 | Compile C, compare assembly output |
| Codegen checks | tc24r-codegen-tests | 2 | _start emission, ISR prologue/epilogue |
| cor24-run validation | tc24r-codegen-validate | 19 | Assemble output with cor24-run |
| as24 HTTP validation | tc24r-codegen-validate | 13 (ignored) | Validate with as24 service on port 7412 |

Run all active tests:

```
scripts/build-all.sh
```

Run ignored as24 tests (requires service on localhost:7412):

```
cargo test --manifest-path components/backend/Cargo.toml -- --ignored
```

## Architecture Decisions

- **Language**: Rust (not a chibicc fork -- same architecture pattern, written from scratch)
- **int size**: 24-bit (native COR24 word)
- **No IR**: AST directly to assembly text (chibicc style)
- **Expression evaluation**: push lhs, evaluate rhs, pop lhs into r1, operate
- **Local variables**: pre-pass collects all declarations, allocates stack at function entry
- **Labels**: L0, L1, L2... (monotonic counter)
- **Design constraints**: <5 functions/module, <5 modules/crate, prefer many small crates

## Known Limitations

- No float/double (COR24 has no FPU -- out of scope)
- No optimization passes
- Single translation unit only
