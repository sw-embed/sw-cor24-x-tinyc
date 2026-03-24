# Bug Tracking

## Fixed

### BUG-001: Nested function-like macro not expanded

**Filed by:** tml24c
**Fixed:** 2026-03-23
**Component:** `tc24r-preprocess` (substitute.rs)

When an object-like `#define` references a function-like macro, the
preprocessor did not expand the inner macro. The compiler treated it
as a function call instead.

```c
#define MAKE_SYMBOL(idx) (((idx) << 2) | 2)
#define NIL_VAL MAKE_SYMBOL(0)
int x = NIL_VAL;  // generated: jal _MAKE_SYMBOL (wrong)
```

**Root cause:** `expand_ident()` pushed simple `#define` replacements
verbatim without re-expansion. The function-macro path already called
`expand_line()` recursively, but the simple define path did not.

**Fix:** Added `expand_line()` re-expansion after simple define
substitution in `substitute.rs:expand_ident()`.

---

### BUG-002: Compiler panic on two-level #define constant expression

**Filed by:** tml24c
**Fixed:** 2026-03-23
**Component:** `tc24r-preprocess` (substitute.rs)

When one object-like macro references another in an expression, the
compiler panics with "no entry found for key" in `tc24r-emit-load-store`.

```c
#define TAG_SYMBOL 2
#define NIL_VAL ((0 << 2) | TAG_SYMBOL)
arr[0] = NIL_VAL;  // panic: TAG_SYMBOL treated as undefined variable
```

**Root cause:** Same as BUG-001 — simple `#define` replacement was not
recursively expanded, so `TAG_SYMBOL` inside `NIL_VAL` reached the
parser as an identifier instead of being expanded to `2`.

**Fix:** Same one-line fix as BUG-001.

---

### BUG-003: Nested array indexing fails to parse

**Filed by:** tml24c
**Fixed:** 2026-03-23
**Component:** `tc24r-parser` (expr.rs)

`&pool[offsets[i]]` caused "expected Semicolon, got LBracket". Even
simple `&pool[0]` failed.

```c
char pool[100];
int offsets[10];
char *get(int i) {
    return &pool[offsets[i]];  // parse error
}
```

**Root cause:** The `&` (address-of) parser only accepted a bare
identifier (`ts.expect_ident()`), not postfix expressions like
`name[index]`. After consuming the identifier, leftover `[...]`
tokens caused the parse error.

**Fix:** After parsing the identifier in `&name`, check for postfix
operators (`[`, `.`, `->`). If present, parse the full postfix chain.
Since `arr[i]` desugars to `*(arr + i)`, applying `&` to that yields
`&*(arr + i)` which simplifies to `arr + i` — the Deref wrapper is
stripped, leaving the address expression.

---

### BUG-004: Short branch emitted for far forward target

**Filed by:** tml24c
**Fixed:** 2026-03-23
**Component:** `tc24r-emit-core` (emit.rs)

Compiling tml24c's `main.c` produced `bra L36` where L36 was ~91
instructions away — beyond the COR24 ±127 byte short branch range.
The assembler rejected it with "Branch target too far".

```
build/tml24c.s line 574:  bra     L36
build/tml24c.s line 665:  L36:
```

**Root cause:** The cor24-isa branch range check (commit fc82730)
correctly handled backward branches by checking instruction distance,
but forward branches (where the target label hasn't been emitted yet)
optimistically defaulted to short form. Large function bodies like
tml24c's eval exceeded the short branch range.

**Fix:** Implemented deferred branch resolution. Forward branches are
emitted as short form optimistically, then validated in a post-pass
(`resolve_branches()`) after the full function is generated and all
label positions are known. Out-of-range branches are expanded to long
form by text replacement in the output buffer.

Also fixed the `emit!` macro (tc24r-emit-macros) to track instruction
counts and label positions — previously only the `emit()` function
in emit-core did this, but most codegen uses the macro.

Result: small functions keep short branches (demo2: 140 instructions),
large functions auto-expand far branches (tml24c: assembles cleanly).

---

### BUG-005: Global arrays allocated as single word regardless of declared size

**Filed by:** tml24c
**Fixed:** 2026-03-23
**Component:** `tc24r-emit-data` (data.rs)

Global array declarations like `int arr[100]` were emitted as `.word 0`
(3 bytes) instead of 100 × `.word 0` (300 bytes). All global arrays
overlapped in memory.

```c
int arr[100];
// generated: .word 0  (3 bytes — wrong)
// expected: 100 × .word 0  (300 bytes)
```

**Root cause:** `emit_data_section()` only checked for `Type::Char` vs
everything else, without handling `Type::Array`. Arrays were treated
as a single word.

**Fix:** Added `Type::Array` match arm that emits the correct number
of `.word` or `.byte` directives based on element type and count.

---

### BUG-006: Global char array treated as pointer instead of array

**Filed by:** tml24c
**Fixed:** 2026-03-23
**Component:** `tc24r-expr-variable` (load.rs)

Indexing into a global `char` array generated `lw` (load word — treats
as pointer) instead of `la` (load address — array decay). Writes went
to wrong addresses.

```c
char pool[100];
pool[0] = 65;
// generated: la r1,_pool; lw r0,0(r1)  ← loads *pool (wrong)
// expected:  la r0,_pool               ← address of pool (correct)
```

**Root cause:** `gen_ident()` checked `state.local_types` for array
decay but not `state.global_types`. Global arrays fell through to the
regular global load path which dereferences with `lw`/`lbu`.

**Fix:** Added `state.global_types` check for `Type::Array` alongside
the existing `local_types` check.

---

### BUG-007: Array store with global-derived index always writes to index 0

**Filed by:** tml24c
**Fixed:** 2026-03-23
**Component:** `tc24r-expr-pointer` (deref.rs)

`offsets[idx] = counter` where both `offsets` and `counter` are globals
always stored to index 0. The computed target address was clobbered
before the store.

```c
int offsets[10]; int counter;
void do_store() { int idx = counter; offsets[idx] = counter; }
// actual output:  "0 0 0"
// expected:       "0 1 2"
```

**Root cause:** The simple-value optimization in `gen_deref_assign()`
moved the target address into r1, then loaded the value via
`gen_expr_fn(value)` — but loading a global variable uses r1 as
scratch (`la r1,_counter; lw r0,0(r1)`), clobbering the target address.

**Fix:** Excluded global variables from the `gen_deref_assign` simple
path (they require r1 as scratch when loaded into r0). Globals now
use the safe push/pop path, preserving the target address in r1.

---

### BUG-008: Right shift uses logical instead of arithmetic on signed integers

**Filed by:** tml24c (analysis of `fixnum_val()` workaround)
**Fixed:** 2026-03-23
**Component:** `tc24r-ops-bitwise` (bitwise.rs)

The `>>` operator on signed `int` values generated `srl` (shift right
logical, zero-fills) instead of `sra` (shift right arithmetic,
sign-extends). This produced incorrect results for negative values:
`-84 >> 2` returned 4194283 instead of -21.

```c
int neg = -84;
neg >> 2;  // generated: srl r0,r1 (logical) → 4194283
           // expected:  sra r0,r1 (arithmetic) → -21
```

**Root cause:** `gen_shr()` unconditionally emitted `srl`. COR24 has
both `srl` (logical, opcode 0x19) and `sra` (arithmetic, opcode 0x18)
but the compiler only used `srl`.

**Analysis:** The emulator is correct — `sra` sign-extends and `srl`
zero-fills, matching their documented semantics. This was a compiler
bug, not an emulator bug.

**Fix:** Changed `gen_shr()` to emit `sra` (arithmetic shift). Added
`gen_shr_logical()` for future `unsigned int` support. Per C99 §6.5.7,
right shift of signed values is implementation-defined; we choose
arithmetic (matching GCC/Clang convention).

---

## Open

(none)
