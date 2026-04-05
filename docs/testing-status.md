# Testing Status

Last updated: 2026-04-04

## Summary

| Test Suite | Pass | Total | Coverage | Notes |
|-----------|------|-------|----------|-------|
| tc24r demos | 61 | 61 | 100% | End-to-end compiler + emulator |
| reg-rs regressions | 33 | 33 | 100% | Output stability checks |
| chibicc-subset | 5 | 5 | 100% | Curated subsets of chibicc tests |
| chibicc full | 14 | 41 | 34% | cast, commonsym, compat, const, control, decl, enum, extern, generic, pointer, pragma-once, sizeof, stdhdr, vla |
| beej-c-guide | 6 | 11 | 55% | hello_world, functions, pointers, pointers_arithmetic, strings, typedef |
| bgc examples | 41 | 117 | 35% | With stdio/stdlib/string/stdbool stubs |

## tc24r Demos (55/55)

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
| 34 | demo34.c | Multi-dimensional array declarations (int a[2][3]) |
| 35 | demo35.c | Struct/union array members (char a[3]) |
| 36 | demo36.c | Forward-declared struct tags, self-referential structs |
| 37 | demo37.c | Anonymous struct/union members (C11) |
| 38 | demo38.c | Struct brace initializer (struct s x = {1, 2}) |
| 39 | demo39.c | printf via stdio.h, long branches, varargs syntax |
| 40 | demo40.c | malloc/free/calloc via stdlib.h (bump allocator) |
| 41 | demo41.c | getc/atoi, strlen/strcmp/strcpy via string.h |
| 42 | demo42.c | Nested struct member access, linked list traversal |
| 43 | demo43.c | Lisp cons cells (struct pointer return, car/cdr chains) |
| 44 | demo44.c | Lisp Phase 1: constructors, predicates, S-expr printer |
| 45 | demo45.c | Lisp eval: reader + eval + builtins — (+ 40 2) => 42 |
| 46 | demo46.c | unsigned int: signed/unsigned shifts (sra/srl), comparisons (cls/clu) |
| 47 | demo47.c | ptr[i].member: struct pointer array indexing (BUG-010 fix) |
| 48 | demo48.c | Global struct array declaration and lookup (BUG-011 fix) |
| 49 | demo49.c | (ptr+offset)->member: paren ptr arithmetic arrow access (BUG-012 fix) |
| 50 | demo50.c | Large local array (char buf[256]) with nested calls (BUG-013 fix) |
| 51 | demo51.c | Function pointer: basic variable call |
| 52 | demo52.c | Function pointer: array dispatch table |
| 53 | demo53.c | Function pointer: passed as parameter |
| 54 | demo54.c | Global function pointer declaration |
| 55 | demo55.c | Constant expression in array size |

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

## chibicc Full Tests (14/41)

Testing against `~/github/softwarewrighter/chibicc/test/*.c`.

### Passing (14)

| Test | Notes |
|------|-------|
| cast | Cast expressions `(type)expr` |
| commonsym | Common symbol linkage (tentative definitions) |
| compat | `_Noreturn`, `restrict`, `volatile`, `auto` keywords |
| const | const type qualifiers |
| control | if/else, while, for, do-while, switch, break, continue, goto/labels |
| decl | Declarations with type modifiers |
| enum | enum declarations and usage |
| extern | `inline` function specifier, local function prototypes |
| generic | _Generic keyword support |
| pointer | Multi-dim arrays, reverse subscript (2[x]), commutative ptr+int |
| pragma-once | #pragma once inclusion guard |
| sizeof | sizeof on types and expressions |
| stdhdr | System header inclusion (stdalign.h, stdbool.h, stddef.h, stdnoreturn.h) |
| vla | Variable-length array declarations |

### Compile Fail (27) — Categorized

#### Out of Scope (5 tests)

| Test | Reason |
|------|--------|
| float | No FPU on COR24 |
| atomic | `<stdatomic.h>` — OS-level feature |
| tls | `<pthread.h>` — OS threading |
| varargs | `<stdarg.h>` — ABI-level variadic calling convention |
| unicode | UTF-8 identifiers — low priority for embedded |

#### Actionable: Parser/Codegen Features (20 tests)

Preprocessor stringification (#) is now supported. Remaining failures
are parser or codegen level. See `.agentrail/plan.md` for the saga
addressing these in priority order.

| Test | Blocking Feature | Saga Phase |
|------|-----------------|------------|
| arith | Octal literals, complex expressions | Phase 5 |
| builtin | `__builtin_types_compatible_p` | Phase 5 |
| ~~pointer~~ | ~~Multi-dim arrays, reverse subscript~~ | ~~Phase 5~~ → **PASS** |
| ~~sizeof~~ | ~~`sizeof expr` without parens~~ | ~~Phase 5~~ → **PASS** |
| attribute | `__attribute__`, compound literals, `_Alignof` | Phase 4-5 |
| offsetof | Struct offsets differ (COR24 3-byte int vs x86 4-byte) | x86-specific |
| variable | Complex declarations, for-scoped variables | Phase 2 |
| complit | Compound literals `(type){init}` | Phase 3 |
| initializer | Brace initializers in expressions | Phase 3 |
| alignof | `_Alignof` / `_Alignas` keywords | Phase 4 |
| ~~compat~~ | ~~`_Noreturn` specifier~~ | ~~Phase 4~~ → **PASS** |
| typeof | `typeof` operator | Phase 4 |
| ~~extern~~ | ~~`inline` function specifier~~ | ~~Phase 4~~ → **PASS** |
| bitfield | Struct bitfield syntax `int x : 5` | Phase 5 |
| asm | Extended asm syntax (multi-string) | Phase 5 |
| constexpr | Complex enum/const initializer expressions | Phase 5 |
| string | Multi-char / wide char literals | Phase 5 |
| literal | Numeric literal edge cases | Phase 5 |
| pragma-once | Relative include path resolution | Phase 5 |
| function | Return type parsing edge case | Phase 5 |
| ~~vla~~ | ~~Variable-length arrays~~ | ~~Phase 5~~ → **PASS** |

#### Compiles but not yet passing via test harness (6 tests)

These compile when tested directly but the chibicc test awk filter
strips needed declarations. Harness improvements in Phase 6.

| Test | Status |
|------|--------|
| alloca | Compiles after ASSERT macro fix |
| ~~commonsym~~ | ~~Compiles, passes (r0=0) when extern decls preserved~~ → **PASS** |
| line | Compiles but codegen panic on some patterns |
| macro | Compiles but stack overflow on recursive macro |
| usualconv | Compiles after float/long lines stripped |
| decl | Compiles after type modifier lines stripped |

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
- Type modifiers: long, short, signed (→ int); unsigned (→ unsigned int with srl/clu)
- inline keyword (accepted, ignored)
- Escape sequences: \v, \f, \e
- Unknown # directives silently skipped (#line, # nnn "file")
- Long branches (no 127-byte range limit)
- Varargs syntax (...) accepted in parameter lists
- Freestanding printf via include/stdio.h (codegen dispatches to __tc24r_printfN)
- Struct brace initializers (struct s x = {1, 2})
- Forward-declared struct tags and self-referential structs
- Anonymous struct/union members (C11)
- Comma-separated struct/union members
- Array members in structs (char buf[N])
- Multi-dimensional array declarations (int a[N][M])
- Chained postfix expressions (a[i].member)
- Tentative definitions (int x; int x = 5;)
- Multi-declarator typedef (typedef int A, B[4];)
- sizeof(type[N]) array type arguments
- Line continuation (backslash-newline)
- Unknown escape sequences accepted literally
- Function pointers: local/global declarations, arrays, parameter passing, indirect calls
- Constant expressions in array sizes (int buf[ROWS * COLS])
- Preprocessor stringification operator (#param → "arg")
- Cast expressions `(type)expr` with narrowing/widening codegen
- Implicit string literal concatenation ("a" "b" → "ab")
- Freestanding stub headers: stddef.h, stdalign.h, stdbool.h, stdnoreturn.h, stdarg.h
- `offsetof(type, member)` as compiler builtin
- Null statements (bare `;`) and comma operator
- Empty for-loop clauses (`for(;;)`, `for(;cond;)`)
- goto/labels (`goto label;` and `label:`)
- Fix: declarator initializers stop at comma (multi-decl regression)
- C11 `_Noreturn` and C99 `restrict` / `volatile` / `auto` keywords (accepted, ignored)
- Array parameter syntax `int a[restrict static N]` (decays to pointer)
- Local function prototype declarations (`int foo(int x);` inside blocks)
- Improved awk filter: strip extern symbol references in extern.c/commonsym.c
- `sizeof expr` without parentheses (`sizeof x`, `sizeof **x + 1`)
- `Expr::SizeofExpr` AST node with codegen type inference (no array decay)
- Awk filter: skip sizeof assertions assuming 32-bit ISA sizes
- Octal integer literals (`0777` = 511)
- Implicit array size from initializer (`int a[] = {1,2,3}`, `char s[] = "hello"`)
- Array brace initializer codegen (element-wise DerefAssign)
- Postfix operators on integer literals (`2[x]` reverse subscript)
- Commutative pointer+int arithmetic (int+ptr scales correctly)
- Local variable re-allocation for larger same-named types across stmt exprs
- Array-to-pointer decay in deref type inference (multidim array indexing)
- Compound literal codegen: `(type){init}` emits temp address, not value
- Multi-dimensional array dimension order fix (`int a[2][3]` → `Array(Array(Int,3),2)`)
- Nested brace initializers: `{{1,2},{3,4}}` for 2D arrays and struct arrays
- Designated initializers: `.field=val`, `[idx]=val`, chained `.a.b=val`
- Range designators: `[2 ... 10]='a'` (GCC extension)
- Flat initializer fill for nested types (auto-distribute across sub-objects)
- Empty initializers `{}` (zero-fill)
- Flexible array members in structs (`char b[]`)
- Global struct/union variable initializers with nested braces
- Type-aware global data emission (struct member sizes, byte/word layout)
- Signed integer division and modulo with negative operands (C99 truncation-toward-zero)
- Inline assembly: multi-string concatenation, `volatile`/`inline` qualifiers, extended asm operand skipping
- `__asm__` / `__asm` keyword aliases
- Wide/unicode string and char literal prefixes: `L"..."`, `L'x'`, `u"..."`, `U"..."`, `u8"..."`
- Multi-byte and multi-character char literals (`L'あ'`, `'abcd'`)
- Octal escape sequences in string and char literals (`\0`, `\101`, `\377`)
- String literal subscript: `"abc"[1]` (postfix chain on string literals)
- `register` storage class keyword (accepted and ignored)

## beej-c-guide Examples (6/11)

Testing against `~/github/softwarewrighter/beej-c-guide/src/*.c`.

### Compiling (6)

| Example | Notes |
|---------|-------|
| hello_world.c | printf via freestanding stdio.h |
| functions.c | printf with %d |
| pointers.c | printf with %d |
| pointers_arithmetic.c | Implicit array size, char[] = "string" |
| strings.c | Implicit array size, char[] = "string" |
| typedef.c | printf with %d |

### Blocked (5)

| Example | Blocker |
|---------|---------|
| arrays.c | Codegen panic (complex array init) |
| file_io.c | Complex lvalue assignment |
| memory_management.c | `<stdlib.h>` (malloc/free) |
| structs.c | `float` type in struct |
| variables_and_statements.c | `float` type in declarations |

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
- **Preprocessor**: No #line, __LINE__, __FILE__. No token pasting (##).
  Stringification (#) is supported. Recursive macro expansion may
  stack overflow on deeply nested patterns.
