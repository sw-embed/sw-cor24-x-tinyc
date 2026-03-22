# Testing Status

Last updated: 2026-03-22

## Summary

| Test Suite | Pass | Total | Coverage | Notes |
|-----------|------|-------|----------|-------|
| cc24 demos | 21 | 21 | 100% | End-to-end compiler + emulator |
| reg-rs regressions | 21 | 21 | 100% | Output stability checks |
| chibicc-subset | 5 | 5 | 100% | Curated subsets of chibicc tests |
| chibicc full | 3 | 41 | 7% | generic, pragma-once, stdhdr |
| bgc examples | 1 | 117 | 1% | 116 blocked on stdio.h |

## cc24 Demos (21/21)

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

Run: `demos/run-demo<N>.sh`

## chibicc-Inspired Subset Tests (5/5)

Simplified tests based on chibicc patterns using only cc24-supported
features. Located in `tests/chibicc-subset/`.

| Test | Features Verified |
|------|------------------|
| arith.c | +, -, *, /, comparisons, &&, ||, !, ?:, +=/-= |
| control.c | if/else, while, for, do-while, break, continue |
| function.c | calls, arguments, recursion (fib) |
| pointer.c | &x, *p, *p=val, array indexing |
| variable.c | locals, globals, assignment, multi-decl |

Run: `scripts/run-subset-tests.sh`

## chibicc Full Tests (3/41)

Testing against `~/github/softwarewrighter/chibicc/test/*.c`.

### Passing (3)

| Test | Notes |
|------|-------|
| generic | Empty after stripping unsupported features |
| pragma-once | #pragma once inclusion guard |
| stdhdr | System header inclusion (skips gracefully) |

### Compile Fail (38)

Most common blockers:
- Braceless if/while/for bodies (`if (x) stmt;`) -- nearly all tests
- `goto` / labels -- control.c
- `switch` / `case` -- control.c
- `struct` / `union` / `.` member access -- 10+ tests
- Complex lvalue increment (`(*p)++`) -- arith.c
- Float/double literals -- arith.c, literal.c, cast.c
- Binary/octal literals -- literal.c
- `typedef` / `enum` keywords -- 3 tests

Run: `scripts/run-chibicc-tests.sh`

### Blockers Fixed

- Ternary `? :`, char literals, multi-decl, hex literals
- Logical `&&` / `||`, break/continue, ++/--
- sizeof, static/extern, statement expressions `({ })`
- Compound assignment `+=`, `-=`, etc.
- Function-like macros (#define FOO(x) ...)
- Integer suffix handling (U, L, LL)
- Large literal truncation to 24 bits
- Void return (`return;`)

## bgc (Beej's Guide to C) Examples (1/117)

Testing against `/home/mike/bgc_download/bgc_source/examples/*.c`.

### Status

- 116/117 blocked on `#include <stdio.h>` (no hosted C runtime)
- 1 compiles: `env_argvadder.c` (does not use stdio)
- All examples use `printf` and require a stdio.h implementation

### Path to Progress

Requires either:
1. A freestanding `stdio.h` stub with minimal printf (needs varargs or fixed-arg workaround)
2. Or a test adapter that strips printf calls and checks compilation only

Run: `scripts/run-beej-tests.sh`

## Known Limitations Affecting Tests

- **Braceless control flow**: `if (x) stmt;` without braces is not supported.
  Most real C code uses this. High priority to fix.
- **Local variable scoping**: Statement expression locals share stack
  with outer scope locals of the same name (flat allocation).
- **No varargs**: `printf` and similar functions cannot be implemented.
- **24-bit int**: Arithmetic wraps at 24 bits. Tests using 32/64-bit
  values will see different results.
- **No float/double**: Floating point tests are out of scope.
- **sed portability**: Test runner uses GNU sed extensions not available
  on macOS. See docs/known-issues.md.
