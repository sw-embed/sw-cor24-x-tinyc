# Changelog

## Multi-dimensional array support and pointer fixes (2026-04-03)

Multi-dimensional arrays (`int x[2][3]`) now work correctly end-to-end:
- Array-to-pointer decay in deref type inference for nested array indexing
- Commutative pointer+int arithmetic (`int + ptr` scales correctly)
- Postfix operators on integer literals (`2[x]` reverse subscript)
- Local variable re-allocation when same-named local redeclared with larger type

Chibicc: 13/41 → 14/41 (pointer test now passes). Refs: #11

## Implicit array size from initializer (2026-04-03)

Support inferring array size from initializer when brackets are empty:
- `int a[] = {1, 2, 3}` infers size 3
- `char s[] = "hello"` infers size 6 (includes null terminator)
- Brace initializers for arrays now generate element assignments (`a[i] = val`)

Beej test suite improved from 4/11 to 6/11 (pointers_arithmetic.c and strings.c
now compile). Refs: #2

## Octal integer literals (2026-04-03)

Added support for octal integer literals (e.g. `0777` = 511 decimal).
The lexer now recognizes a leading `0` followed by digits `0-7` as base-8.
Removed the octal-stripping line from the chibicc test awk filter since
the compiler handles them natively. Refs: #10

## Function pointer support (2026-03-31)

Added function pointer support to the compiler, enabling callback dispatch
tables, higher-order functions, and the idiomatic C patterns needed by the
sws scripting language interpreter.

### New capabilities

- Function pointer local variables: `int (*fp)(int, int) = add; fp(3, 4);`
- Arrays of function pointers: `int (*table[8])(int); table[n](arg);`
- Function pointers as parameters: `void apply(int (*f)(int), int x) { f(x); }`
- Typedef for function pointer types: `typedef int (*handler_t)(int);`
- Function names used as values produce their code address (like array decay)

### Implementation

- AST: added `Expr::IndirectCall { callee, args }` variant
- Parser: function pointer declarator syntax in locals, params, typedefs;
  postfix `()` on expressions produces `IndirectCall`
- Codegen: `gen_indirect_call` evaluates callee after pushing args, then
  `jal r1,(r0)`; `gen_call` detects variable-as-function-pointer and
  delegates to indirect call path; `gen_ident` loads function address
  when a function name is used in value context
- No extra register pressure: callee evaluated into r0 after args pushed,
  same pattern as direct calls

### Tests

- 7 parser unit tests (fn ptr locals, arrays, params, typedefs, indirect calls)
- 2 codegen structural tests + 3 assembly validation tests
- 3 end-to-end demos (51-53) with reg-rs regression baselines
- 33/33 regression tests pass

## Fork from sw-vibe-coding/tc24r (2026-03-30)

Forked `sw-vibe-coding/tc24r` to `sw-embed/sw-cor24-x-tinyc` as part of COR24
ecosystem consolidation under the `sw-embed` GitHub organization.

### Changes from fork

- Updated `cor24-isa` path dep to point to `../sw-cor24-emulator/isa`
- Updated `cor24-run` fallback path to `sw-cor24-emulator`
- Added `scripts/build.sh` (builds all components + runs tests)
- Updated documentation references from `cor24-rs` to `sw-cor24-emulator`
