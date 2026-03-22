# Testing Status

Last updated: 2026-03-22

## Summary

| Test Suite | Pass | Total | Coverage | Notes |
|-----------|------|-------|----------|-------|
| cc24 demos | 22 | 22 | 100% | End-to-end compiler + emulator |
| reg-rs regressions | 22 | 22 | 100% | Output stability checks |
| chibicc-subset | 5 | 5 | 100% | Curated subsets of chibicc tests |
| chibicc full | 3 | 41 | 7% | generic, pragma-once, stdhdr |
| beej-c-guide | 0 | 11 | 0% | All need stdio.h |
| bgc examples | 0 | 117 | 0% | All need stdio.h |
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

## beej-c-guide Examples (0/11)

Testing against `~/github/softwarewrighter/beej-c-guide/src/*.c`.

All 11 examples use `#include <stdio.h>` and `printf`. None compile
without a stdio.h stub.

| Example | Blocker |
|---------|---------|
| hello_world.c | printf (1 call) |
| functions.c | printf (2 calls) |
| pointers.c | printf (4 calls) |
| structs.c | printf (3 calls), struct |
| typedef.c | printf (1 call), typedef |
| arrays.c | printf (18 calls) |
| strings.c | printf, string.h |
| pointers_arithmetic.c | printf, string.h |
| variables_and_statements.c | printf, scanf, stdbool.h |
| memory_management.c | printf, malloc/free |
| file_io.c | printf, fopen/fclose/fgets/etc |

Run: `scripts/run-beej-tests.sh`

## bgc (Beej's Guide to C) Examples (0/117)

Testing against `/home/mike/bgc_download/bgc_source/examples/*.c`.

- 116/117 blocked on `#include <stdio.h>`
- 1/117 blocked on `#include <stdalign.h>`
- All 117 examples use printf and require a stdio.h implementation

### Path to Progress for beej + bgc

Both suites require a freestanding `stdio.h` with at least:
- `printf(fmt, ...)` mapped to UART output
- `NULL` and `EOF` defines
- Optionally: `puts`, `putchar`, `getchar`

This requires either varargs support or a fixed-arg printf workaround.
The Arduino approach: provide `putchar` mapped to UART, then a minimal
printf that calls putchar for each formatted character.

## Known Limitations Affecting Tests

- **Braceless control flow**: Now supported (demo22).
- **Local variable scoping**: Statement expression locals share stack
  with outer scope locals of the same name (flat allocation).
- **No varargs**: `printf` and similar functions cannot be implemented.
- **24-bit int**: Arithmetic wraps at 24 bits. Tests using 32/64-bit
  values will see different results.
- **No float/double**: Floating point tests are out of scope.
- **sed portability**: Test runner uses GNU sed extensions not available
  on macOS. See docs/known-issues.md.
