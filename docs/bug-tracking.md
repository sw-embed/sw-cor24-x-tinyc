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

### BUG-009: Large local char arrays corrupt stack (from tml24c docs/bugs.md)

**Filed by:** tml24c
**Status:** Not a tc24r bug — mis-diagnosed GC bug in tml24c
**Component:** n/a

Originally reported as `char buf[128]` corrupting the stack. The actual
root cause was a garbage collection bug in tml24c, not a compiler issue.
The tc24r prologue handles large frames correctly (`sub sp,N` for N > 127).

---

### BUG-010: `ptr[i].member` panics — struct pointer array indexing

**Filed by:** p24p
**Fixed:** 2026-03-25
**Component:** `tc24r-expr-struct` (member.rs)

Array subscript on a struct pointer followed by member access panicked
because `object_type()` resolved `arr[i]` to `Int` instead of the struct
type. The parser desugars `arr[i]` into `*(arr + i)`, and `object_type()`
lacked a `BinOp` case for pointer arithmetic.

```c
struct pair { int key; int val; };
struct pair *arr = (struct pair *)malloc(6);
arr[0].key = 10;   // PANIC: unknown struct member 'key' in type Int
```

**Root cause:** `object_type()` only handled `Ident`, `Deref`, `Call`,
and `MemberAccess`. When it recursed into `Deref(BinOp(Add, ptr, i))`,
the `BinOp` fell through to the default `_ => Type::Int` case.

**Fix:** Added `BinOp::Add` / `BinOp::Sub` arms to `object_type()` that
preserve pointer types through pointer arithmetic, matching the existing
`expr_type()` implementation in `tc24r-type-infer`.

---

### BUG-011: Global struct array fails to parse

**Filed by:** p24p
**Fixed:** 2026-03-25
**Component:** `tc24r-parser` (decl.rs), `tc24r-emit-data` (data.rs)

`struct symbol symtab[8];` at global scope failed with "expected
Semicolon, got LBracket". Global `int` arrays worked fine.

```c
struct symbol { int name_char; int value; };
struct symbol symtab[8];   // ERROR: expected Semicolon, got LBracket
```

**Root cause (parser):** The struct variable declaration path in
`parse_program()` consumed the struct name and variable name but
immediately expected a semicolon without handling array bracket suffixes.
The regular global declaration path in `parse_one_global()` already
handled `[N]` but struct declarations were intercepted before reaching it.

**Root cause (data emission):** `emit_data_section()` emitted one
`.word` per array element regardless of element type. For struct arrays,
each element spans multiple words (e.g. 2-word struct needs 2 `.word`
directives per element, not 1).

**Fix:** Added array bracket parsing loop to the struct variable
declaration path (matching `parse_one_global()`). Fixed data emission
to calculate words per element using `ceil(elem.size() / 3)`.

---

### BUG-012: `(ptr + offset)->member` fails to parse

**Filed by:** p24p
**Fixed:** 2026-03-25
**Component:** `tc24r-parser` (expr.rs)

Parenthesized pointer arithmetic followed by arrow access failed with
"expected Semicolon, got Arrow".

```c
struct pair *arr = (struct pair *)malloc(6);
(arr + 1)->key = 30;   // ERROR: expected Semicolon, got Arrow
```

**Root cause:** `parse_primary()` had three return paths for parenthesized
expressions (statement expressions, casts, and regular parenthesized
expressions) that all returned without calling `parse_postfix_chain()`.
The identifier path correctly called it, but the paren paths bypassed
it, leaving `->`, `.`, and `[` tokens unconsumed.

**Fix:** Added `parse_postfix_chain()` calls to all three parenthesized
expression return paths in `parse_primary()`.

---

### BUG-013: Large stack frame offset overflow corrupts locals

**Filed by:** tml24c (`format` function stack corruption)
**Fixed:** 2026-03-28
**Component:** `tc24r-emit-core` (immediate.rs), `tc24r-emit-load-store` (load.rs, store.rs), `tc24r-stmt-simple` (local_decl.rs), `tc24r-type-infer` (simple.rs)

Functions with large stack frames (locals totaling > 127 bytes, e.g.
`char buf[256]`) generated `lw`/`sw` instructions with offsets that
overflowed the 8-bit signed immediate range (-128..127). The assembler
silently truncated the offset, causing reads/writes to hit wrong memory
locations — typically the saved frame pointer at `fp+0`.

```c
void apply_primitive(...) {
    char buf[256];  // offset -256 → truncated to 0 → clobbers saved fp
    int sentinel;   // offset -259 → also truncated
    // nested function calls corrupt buf[0] and sentinel
}
```

**Root cause (part 1 — offset overflow):** Four emission sites emitted
fp-relative offsets directly into `lw`/`sw` instructions without range
checking: `gen_load_by_name`, `gen_store_by_name`, `gen_local_decl`,
and `gen_simple_into_r1`.

**Root cause (part 2 — r1 clobber regression):** The expanded offset
sequence (`la r1,offset` / `add r1,fp` / `lw r0,0(r1)`) clobbers r1.
The `gen_deref_assign` "simple" optimization saved the pointer address
in r1 then loaded the value via `gen_expr_fn`, which now destroyed r1
for large-offset variables. Symptom: `number->string 42` returned "44"
because the char swap in the digit-reversal loop read wrong bytes.

**Fix (part 1):** Added `fp_load_word_r0`, `fp_store_word_r0`, and
`fp_load_word_r1` helpers to `tc24r-emit-core` that expand to
`la r1,{offset}` / `add r1,fp` / `lw/sw r0,0(r1)` when the offset
is outside -128..127. All four emission sites now use these helpers.

**Fix (part 2):** Updated `is_simple_expr` to return false for local
variables at offsets outside -128..127, forcing the safe push/pop path
instead of the r1-clobbering shortcut.

---

### BUG-014: `#define` values include `//` line comments

**Filed by:** manual testing
**Fixed:** 2026-03-29
**Component:** `tc24r-preprocess` (preprocess.rs, func_macro.rs)

`#define FOO 10 // comment` expanded `FOO` to `10 // comment` instead
of just `10`. When `FOO` appeared mid-expression, the `//` commented
out everything after it on that line, causing silent parse failures or
wrong results.

```c
#define SIZE 10 // array size
int arr[SIZE];  // expanded to: int arr[10 // array size];
                // the ]; is commented out → parse error
```

**Root cause:** The preprocessor's `parse_simple_define` took everything
after the macro name as the replacement value. Comment stripping happens
in the lexer (post-preprocessing), but macro expansion happens during
preprocessing — so the comment text gets baked into the expanded output.
The same issue affected function-like macro bodies.

**Fix:** Added `strip_line_comment()` that scans for `//` outside of
string and char literals. Applied to both `parse_simple_define` values
and `parse_params_and_body` (function-like macro bodies).

---

## Open

(none)
