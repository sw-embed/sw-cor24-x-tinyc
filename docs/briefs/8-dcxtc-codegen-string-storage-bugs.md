# Brief: fix `char *global = "..."` and `char local[] = "..."` codegen

**Owner:** dcxtc
**Branch:** `pr/codegen-string-storage-bugs`
**Repo:** `sw-cor24-x-tinyc`
**Tracked in:** [`sw-embed/sw-cor24-x-tinyc#21`](https://github.com/sw-embed/sw-cor24-x-tinyc/issues/21) (GitHub issue mirror — this brief is canonical)
**Drafted by:** dcxtc (surfaced while building demo63 for `pr/string-literal-concatenation`)

## Context

While verifying string-literal concatenation end-to-end, two
pre-existing codegen bugs blocked the obvious demo path:

1. **Global pointer init** — `char *g = "abc";` stores the string
   bytes *literally at the symbol address*, instead of allocating an
   anonymous label for the bytes and storing the address into `g`.
   So `g[2]` (which compiles to "load pointer-value from `_g`, add 2,
   load byte") reads from address `0x6261` (the first three string
   bytes interpreted as a pointer), not from the string.

2. **Local array init from string literal** — `char a[] = "abc";`
   reserves `sizeof("abc") + 1` bytes on the stack but stores a
   *pointer to rodata* in the slot at `0(fp)` instead of copying the
   bytes in. Subscripting `a[N]` then computes `&fp_slot + N` and
   reads from there — bytes of the pointer's representation, not the
   string contents.

Both are symmetric mismatches between init-side codegen and read-side
codegen. The init side picks one storage model; the read side picks
the other.

## Concrete repros

Both fail today against the freshly-installed (post-Phase-1) tc24r:

```c
/* Bug 1 — global char pointer */
char *g = "abc";
int main(void) {
    return g[2] != 99;  /* expect 0; actual: 1 (g[2] is garbage byte) */
}
```

Disassembly shows the data layout is wrong:
```
_g:
        .byte   97,98,99,0   /* string bytes — should be a 3-byte pointer */
```
…and `_main` does `lw r0,0(_g)` (treats `_g` as a pointer slot),
giving inconsistency between init and use.

```c
/* Bug 2 — local char array */
int main(void) {
    char a[] = "abc";
    return a[2] != 99;  /* expect 0; actual: 1 */
}
```

Disassembly:
```
_main:
        add     sp,-4
        la      r0,_S0
        sw      r0,-4(fp)   /* stores POINTER to rodata in stack slot */
        ...
        lc      r0,-4
        add     r0,fp       /* &slot + 2 */
        lc      r1,2
        add     r0,r1
        lbu     r0,0(r0)    /* reads byte 2 of the stored pointer */
```

`sizeof(a)` correctly reports 4 (after `pr/string-literal-concatenation`,
also 7 for `char a[] = "abc" "def"`). Only the storage/access codegen
is wrong.

## Why both belong in the same PR

They're the same class of bug — a mismatch between which storage
model the compiler picks. Fixing one without the other leaves the
symmetric form broken in a confusing way:

- Fix bug 1, leave bug 2: pointer globals work, char-array locals
  still don't.
- Fix bug 2, leave bug 1: char-array locals work, pointer globals
  still don't.

The two are also likely to share infrastructure (a "store/copy a
string literal into a destination of type T" helper).

## Goal

After this saga:

```c
char *g = "abc";              /* g holds address of anonymous "abc" rodata */
char a[] = "abc";             /* a is 4 stack bytes containing 'a','b','c','\0' */
char a2[10] = "abc";          /* explicitly-sized: copy bytes, zero-pad rest */
char g2[] = "abc" "def";      /* with concat: 7-byte global of 'a','b',...,'f','\0' */
```

…all read back the bytes you'd expect from a C99-conforming compiler.

## Scope

| Construct | Today | Required |
|---|---|---|
| `char *g = "abc";` (global) | broken: bytes at `_g` directly | fix: anon rodata + pointer in `_g` |
| `char *g = "abc" "def";` (with concat) | same | same |
| `char a[] = "abc";` (local) | broken: pointer-in-slot | fix: byte-copy into stack |
| `char a[] = "abc" "def";` (local, with concat) | same | same |
| `char a[10] = "abc";` (explicitly sized local) | likely same — verify | fix; zero-pad remaining bytes |
| `char ga[] = "abc";` (global) | works (verified during Phase 1) | regression coverage only |
| `char *fn() { return "abc"; }` (function return) | works | regression coverage only |

Wide-string handling (`L"..."`) and multi-character literals stay
out of scope — separate sagas if/when needed.

## Suggested implementation approach (don't have to follow this)

The bug pattern smells like the parser/analyzer producing the same
`Expr::StringLit(s)` AST node for two semantically different
positions, and the codegen guessing the storage model from
context. Two places to look:

- `components/codegen-*` — find where `Expr::StringLit` is lowered
  for global init (works for `char ga[]`, broken for `char *g`) and
  for local init (broken for `char a[]`).
- `components/frontend/crates/tc24r-parser/src/decl.rs` and
  `stmt.rs` — make sure the AST distinguishes "init pointer with
  rodata address" vs "init array with rodata bytes". After
  `pr/string-literal-concatenation`, the parser already concatenates
  before storing the literal, so the AST shape is the right size;
  the question is what the codegen does with it.

The right fix is usually: emit the rodata bytes once at a stable
anonymous label, then have the init code generate either a pointer
store or a multi-byte copy depending on the destination type.

## Tests

In `components/frontend/crates/tc24r-parser-tests/` and as a
demo:

1. **Positive** — verify byte-equality at runtime (these read into
   `r0` and check):
   - Global pointer: `char *g = "abc"; main returns g[2];` → 99
   - Global pointer with concat: `char *g = "ab" "cd"; main returns g[3];` → 100
   - Local array: `char a[] = "abc"; return a[2];` → 99
   - Local array with concat: `char a[] = "ab" "cd"; return a[3];` → 100
   - Explicitly-sized: `char a[10] = "abc"; return a[5];` → 0 (zero-padded)
   - Per-chunk escape: `char a[] = "ab\n" "cd"; return a[2];` → 10

2. **Regression** — these already work; lock them in:
   - Global char array `char ga[] = "abc"; return ga[2];` → 99
   - Function return: `char *f(){return "abc";} return f()[2];` → 99
   - demo62, demo63 (both still pass)

3. **End-to-end** — extend `demo63.c` to remove the workaround
   comments, OR add a new `demo64.c` that exercises all of the
   above and produces a single PASS/FAIL.

## Migration

This is purely an internal codegen fix. No source changes in
downstream repos. After mike rebuilds + reinstalls tc24r, downstream
PL/SW patterns like `char *err = "expected " "identifier";` start
working in any context, not just function returns and char-array
globals.

## What goes in this PR

1. Fix global `char *` initialization with string literal.
2. Fix local `char[]` initialization with string literal.
3. Tests above, including a refreshed/expanded demo.
4. Update `README.md` "What Works" / remove the demo63 NOTE that
   currently apologises for both bugs.

## What does NOT go in this PR

- No wide-string support (`L"..."`).
- No general initializer overhaul beyond the two specific cases.
- No reg-rs baseline rebase (separate
  `pr/rebase-codegen-baselines`).

## When done

Push `pr/codegen-string-storage-bugs` and signal. After relay,
mike rebuilds and reinstalls. Any agent that's been working around
"can't index char locals from a string literal" can stop.
