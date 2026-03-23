# Testing Status

Last updated: 2026-03-22

## Summary

| Test Suite | Pass | Total | Coverage | Notes |
|-----------|------|-------|----------|-------|
| tc24r demos | 33 | 33 | 100% | End-to-end compiler + emulator |
| reg-rs regressions | 25 | 25 | 100% | Output stability checks |
| chibicc-subset | 5 | 5 | 100% | Curated subsets of chibicc tests |
| chibicc full | 6 | 41 | 14% | const, decl, enum, generic, pragma-once, stdhdr |
| beej-c-guide | 0 | 11 | 0% | All need stdio.h |
| bgc examples | 1 | 117 | 1% | 116 blocked on stdio.h |

## tc24r Demos (33/33)

| # | Demo | Features Tested |
|---|------|----------------|
| 1 | demo.c | All Phase 1-3 features combined |
| 2 | demo2.c | char, pointers, casts, MMIO (LED, UART) |
| 3 | demo3.c | Hex literals, pointer arithmetic, strings |
| 4 | demo4.c | Software division and modulo |
| 5 | demo5.c | Arrays (declaration and indexing) |
| 6 | demo6.c | Global char/pointer, .byte/.word emission |
| 7 | demo7.c | Pointer subtraction with scaling |
| 8 | demo8.c | Preprocessor #define |
| 9 | demo9.c | Interrupt attribute, ISR, UART RX interrupt |
| 10 | demo10.c | #include, #pragma once, -I flag |
| 11 | demo11.c | Logical && and || with short-circuit |
| 12 | demo12.c | do...while loop |
| 13 | demo13.c | break, continue |
| 14 | demo14.c | Prefix/postfix ++, -- |
| 15 | demo15.c | Ternary operator (? :) |
| 16 | demo16.c | Character literals ('a', '\n') |
| 17 | demo17.c | Multi-declaration (int x, y, z;) |
| 18 | demo18.c | sizeof operator |
| 19 | demo19.c | static/extern keywords |
| 20 | demo20.c | Statement expressions ({ }) |
| 21 | demo21.c | Compound assignment (+=, -=, etc.) |
| 22 | demo22.c | Braceless control flow bodies |
| 23 | demo23.c | enum |
| 24 | demo24.c | typedef |
| 25 | demo25.c | struct (dot access, sizeof) |
| 26 | demo26.c | switch/case (break, fall-through) |
| 27 | demo27.c | Function prototypes (forward declarations, mutual recursion) |
| 28 | demo28.c | union (shared memory, sizeof) |
| 29 | demo29.c | sizeof with array types (int[4], int[3][4]) |
| 30 | demo30.c | Line continuation (backslash-newline) |
| 31 | demo31.c | Tentative definitions (int x; int x = 5;) |
| 32 | demo32.c | Multi-declarator typedef (typedef int A, B[4];) |
| 33 | demo33.c | Comma-separated struct/union members (int a, b;) |

Run: `demos/run-demo<N>.sh`

## chibicc-Inspired Subset Tests (5/5)

Simplified tests based on chibicc patterns using only tc24r-supported
features. Located in `tests/chibicc-subset/`.

| Test | Features Verified |
|------|------------------|
| arith.c | +, -, *, /, comparisons, &&, ||, !, ?:, +=/-= |
| control.c | if/else, while, for, do-while, break, continue |
| function.c | calls, arguments, recursion (fib) |
| pointer.c | &x, *p, *p=val, array indexing |
| variable.c | locals, globals, assignment, multi-decl |

Run: `scripts/run-subset-tests.sh`

## chibicc Full Tests (6/41)

Testing against `~/github/softwarewrighter/chibicc/test/*.c`.

### Passing (6)

| Test | Notes |
|------|-------|
| const | const type qualifiers |
| decl | Declarations with type modifiers |
| enum | enum declarations and usage |
| generic | _Generic keyword support |
| pragma-once | #pragma once inclusion guard |
| stdhdr | System header inclusion |

### Compile Fail (36) — Categorized

#### Out of Scope: Floating Point (3 tests)

COR24 has no FPU. These tests are permanently out of scope.

| Test | Notes |
|------|-------|
| cast | float/double cast operations |
| constexpr | float/double constant expressions |
| float | Dedicated float/double test suite |

#### Out of Scope: Hosted C / Standard Library (4 tests)

tc24r is a freestanding compiler. Tests requiring hosted headers are out of scope.

| Test | Blocker |
|------|---------|
| atomic | `<stdatomic.h>` |
| attribute | `"stddef.h"` |
| tls | `<stdio.h>`, _Thread_local |
| varargs | `<stdarg.h>`, variadic functions |

#### Partially Blocked by Float (9 tests)

These tests contain some float/double usage mixed with integer tests.
Progress possible by stripping float portions.

| Test | Primary Blocker | Also Uses Float |
|------|----------------|-----------------|
| arith | Preprocessor test macros | float/double arithmetic |
| builtin | __builtin functions | float types |
| control | Preprocessor test macros | float literals |
| function | Preprocessor test macros | float params |
| generic | Passes (float parts skipped) | float in _Generic |
| offsetof | `<stddef.h>` | double in struct |
| sizeof | Preprocessor test macros | sizeof(float) |
| stdhdr | Passes (float in headers) | float typedefs |
| usualconv | Preprocessor test macros | float promotions |

#### Actionable: Missing Language Features (20 tests)

| Test | Blocking Feature |
|------|-----------------|
| alignof | _Alignof, _Alignas keywords |
| alloca | Preprocessor test macros (#define ASSERT) |
| asm | Extended inline asm syntax |
| bitfield | Struct bitfield syntax (int x : 5) |
| compat | _Noreturn, restrict, volatile qualifiers |
| commonsym | Preprocessor test macros |
| complit | Compound literals, complex struct init |
| control | goto/labels, switch enhancements |
| decl | Complex declaration parsing |
| extern | inline keyword |
| initializer | Designated initializers, short type |
| line | #line directive, __LINE__, __FILE__ |
| literal | Line continuation (backslash-newline) |
| macro | Complex macro expansion, #include errors |
| pointer | Array of pointers declaration |
| sizeof | Preprocessor test macros |
| string | \v escape, wide/unicode string literals |
| typedef | Multiple declarators in typedef |
| typeof | typeof operator |
| unicode | UTF-8/Unicode character handling |
| variable | Complex declarations, scoping |
| vla | Variable-length arrays |

Run: `scripts/run-chibicc-tests.sh`

### Blockers Fixed (cumulative)

- Ternary `? :`, char literals, multi-decl, hex literals
- Logical `&&` / `||`, break/continue, ++/--
- sizeof, static/extern, statement expressions `({ })`
- Compound assignment `+=`, `-=`, etc.
- Function-like macros (#define FOO(x) ...)
- Integer suffix handling (U, L, LL)
- Braceless control flow (`if (x) stmt;`)
- enum, typedef, struct (dot and arrow access)
- switch/case with break and fall-through
- Function prototypes (forward declarations)
- union (shared memory layout)
- Conditional compilation (#if, #ifdef, #ifndef, #elif, #else, #endif, #undef)
- Type modifiers: long, short, signed, unsigned (→ int on COR24)
- inline keyword (accepted, ignored)
- Escape sequences: \v, \f, \e
- Unknown # directives silently skipped (#line, # nnn "file")

## beej-c-guide Examples (0/11)

Testing against `~/github/softwarewrighter/beej-c-guide/src/*.c`.

All 11 examples use `#include <stdio.h>` and `printf`. None compile
without a stdio.h stub.

| Example | Blocker |
|---------|---------|
| hello_world.c | printf (1 call) |
| functions.c | printf (2 calls) |
| pointers.c | printf (4 calls) |
| structs.c | printf (3 calls) |
| typedef.c | printf (1 call) |
| arrays.c | printf (18 calls) |
| strings.c | printf, string.h |
| pointers_arithmetic.c | printf, string.h |
| variables_and_statements.c | printf, scanf, stdbool.h |
| memory_management.c | printf, malloc/free |
| file_io.c | printf, fopen/fclose/fgets/etc |

Run: `scripts/run-beej-tests.sh`

## bgc (Beej's Guide to C) Examples (1/117)

Testing against `/home/mike/bgc_download/bgc_source/examples/*.c`.

- 116/117 blocked on `#include <stdio.h>`
- 1/117 blocked on `#include <stdalign.h>`
- All 117 examples use printf and require a stdio.h implementation

## Known Limitations Affecting Tests

- **No float/double**: COR24 has no FPU. Float tests are out of scope.
- **No varargs**: `printf` and similar functions cannot be implemented.
- **24-bit int**: Arithmetic wraps at 24 bits. Tests using 32/64-bit
  values will see different results.
- **Local variable scoping**: Statement expression locals share stack
  with outer scope locals of the same name (flat allocation).
- **Preprocessor**: No #line, __LINE__, __FILE__. No complex macro
  expansion (stringification, token pasting).
