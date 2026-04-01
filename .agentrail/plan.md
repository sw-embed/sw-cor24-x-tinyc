# chibicc Compatibility Saga

Goal: maximize tc24r's ability to compile real C programs from three
external test suites (chibicc, beej-c-guide, bgc). Current baseline:
chibicc 6/41, beej 4/11, bgc 41/117.

Five tests are permanently out of scope (float, atomic, tls, varargs,
unicode). Of the remaining 24 actionable chibicc failures, this plan
addresses them in priority order by number of tests unlocked.

## Phase Overview

| Phase | Description                          | Tests Unlocked | Cumulative |
|-------|--------------------------------------|----------------|------------|
| 0     | Cast expressions `(type)expr`        | 5              | 11/41      |
| 1     | Stub headers (stddef.h, stdalign.h)  | 3              | 14/41      |
| 2     | goto/labels, empty for(;;)           | 2              | 16/41      |
| 3     | Compound literals, brace init        | 2              | 18/41      |
| 4     | C99/C11 keywords and specifiers      | 4              | 22/41      |
| 5     | Remaining parser features            | 5              | 27/41      |
| 6     | Test harness improvements            | —              | validate   |

---

## Phase 0: Cast Expressions

Cast expressions `(type)expr` are the single highest-value feature,
blocking 5 chibicc tests (arith, builtin, cast, pointer, sizeof).

### Step 0.1: Parse cast expressions
- Detect `(type)` prefix in expression parser (expr.rs)
- Distinguish from parenthesized expressions: `(int)x` vs `(x+1)`
- Lookahead: if token after `(` is a type keyword, parse as cast
- AST: add `Expr::Cast { ty, expr }` node
- Test: `(int)x`, `(char *)p`, `(void *)0`

### Step 0.2: Codegen for cast expressions
- Emit appropriate narrowing/widening for int↔char casts
- Pointer casts are no-ops (same size on COR24)
- Handle `(void *)0` as null pointer constant
- Test: compile and run cast demo

### Step 0.3: Add demo and verify chibicc tests
- Create demo56.c exercising cast expressions
- Re-run chibicc tests, verify arith/builtin/cast/pointer/sizeof progress
- Update testing-status.md counts

---

## Phase 1: Stub Headers

Missing freestanding headers block 3 tests (attribute, offsetof, stdhdr).

### Step 1.1: Add include/stddef.h stub
- Define NULL, size_t, ptrdiff_t
- Define offsetof() macro
- Test: `#include <stddef.h>` compiles

### Step 1.2: Add include/stdalign.h stub
- Define alignof/alignas macros (map to _Alignof/_Alignas or no-op)
- Test: `#include <stdalign.h>` compiles

### Step 1.3: Add include/stdbool.h stub
- Define bool, true, false
- Test: `#include <stdbool.h>` compiles
- Also unblocks beej-c-guide variables_and_statements.c

### Step 1.4: Verify chibicc and beej test progress
- Re-run all three test suites
- Update testing-status.md

---

## Phase 2: goto/labels and Empty For

Blocks 2 chibicc tests (control, variable) and likely several bgc examples.

### Step 2.1: Support empty clauses in for loops
- `for(;;)` — empty init, condition, increment
- `for(; i < n;)` — empty init and increment
- Parser: allow missing expressions in for statement
- Test: `for(;;) { break; }` compiles

### Step 2.2: Support goto and labels
- Parser: recognize `label:` and `goto label;`
- AST: add `Stmt::Label(name)` and `Stmt::Goto(name)`
- Codegen: emit labels and unconditional branches
- Test: demo with goto-based state machine

### Step 2.3: Verify test progress
- Re-run test suites, update docs

---

## Phase 3: Compound Literals and Brace Initializers

Blocks 2 chibicc tests (complit, initializer).

### Step 3.1: Support array brace initializers
- `int a[] = {1, 2, 3};` — infer size from initializer count
- `int a[3] = {1, 2, 3};` — brace init for arrays (not just structs)
- Parser + codegen changes

### Step 3.2: Support compound literals
- `(int){42}`, `(int[]){1, 2, 3}`, `(struct point){.x=1, .y=2}`
- Parser: `(type){init}` in expression position
- Codegen: allocate temporary, initialize, return address
- Test: demo with compound literals

### Step 3.3: Verify test progress

---

## Phase 4: C99/C11 Keywords and Specifiers

Each unlocks 1 chibicc test. 4 keywords total.

### Step 4.1: Support _Alignof / _Alignas
- Parser: recognize keywords in type/expression position
- _Alignof(type) returns alignment (always 1 on COR24 byte-addressed)
- _Alignas accepted and ignored
- Unlocks: alignof test

### Step 4.2: Support _Noreturn specifier
- Parser: accept _Noreturn before function return type
- Ignore for codegen (informational only)
- Unlocks: compat test

### Step 4.3: Support typeof operator
- Parser: `typeof(expr)` and `typeof(type)` in type position
- Evaluate type of expression at parse time
- Unlocks: typeof test

### Step 4.4: Support inline specifier on function definitions
- Parser: accept `inline` before function return type
- Ignore for codegen (no inlining)
- Unlocks: extern test

### Step 4.5: Verify test progress

---

## Phase 5: Remaining Parser Features

Each unlocks 1 chibicc test.

### Step 5.1: Struct bitfield declarations
- Parser: `int x : N` in struct member position
- Codegen: pack bitfields into words
- Unlocks: bitfield test

### Step 5.2: Extended asm syntax
- Parser: accept multi-string asm, operand constraints
- Codegen: emit string content, ignore constraints
- Unlocks: asm test

### Step 5.3: Enum/const complex initializer expressions
- Const-fold more complex expressions in enum values
- Unlocks: constexpr test

### Step 5.4: Multi-character and wide character literals
- Lexer: handle 'abcd' multi-char and L'x' wide char
- Unlocks: string, literal tests

### Step 5.5: Include path resolution for relative paths
- `#include "test/foo.c"` resolves relative to source file
- Unlocks: pragma-once test

### Step 5.6: Verify test progress and update docs

---

## Phase 6: Test Harness Improvements

### Step 6.1: Improve chibicc test awk filter
- Reduce aggressive line stripping (e.g. common_ext declarations)
- Add targeted per-test fixups where needed
- Re-evaluate all 41 tests with improved harness

### Step 6.2: Re-run beej-c-guide and bgc suites
- Test with all new features
- Update testing-status.md with final counts

### Step 6.3: Update README and docs
- Final summary table
- Feature coverage documentation

---

## Out of Scope

| Test | Reason |
|------|--------|
| float | No FPU on COR24 |
| atomic | `<stdatomic.h>` — OS-level feature |
| tls | `<pthread.h>` — OS threading |
| varargs | `<stdarg.h>` — ABI-level variadic support |
| unicode | UTF-8 identifiers — low priority for embedded |
