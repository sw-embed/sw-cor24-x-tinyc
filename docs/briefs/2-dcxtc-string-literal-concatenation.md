# Brief: support adjacent string-literal concatenation in `tc24r`

**Owner:** dcxtc
**Branch:** `pr/string-literal-concatenation`
**Repo:** `sw-cor24-x-tinyc`

## Context

`tc24r` does not concatenate adjacent string literals. Every C
standard since C89 requires this — it's translation phase 6
(ISO C99 §5.1.1.2 paragraph 1 step 6, ISO C11 §5.1.1.2 paragraph 1
step 6): "Adjacent string literal tokens are concatenated."

Concrete failure observed in `sw-cor24-plsw/src/main.c` (the PL/SW
compiler test suite uses this pattern heavily — 265 continuation
lines across the macro-parser, parser, codegen, and lexer test
fixtures):

```c
src = "MACRODEF GETMAIN;"
    "   REQUIRED LENGTH(expr);"
    "   OPTIONAL SUBPOOL(expr);"
    /* ... */
    "END;";
/* tc24r: error at offset 301059: expected Semicolon,
   got StringLit("   REQUIRED LENGTH(expr);") */
```

Minimal repro:

```c
char *foo() { return "abc" "def"; }
/* tc24r: error at offset 27: expected Semicolon, got StringLit("def") */
```

Backslash line-continuation inside a single string literal works
today (no error from `"abc\<newline>def"`), so the lexer can
already advance past line breaks within a string. The gap is in
the parser/lexer interaction *between* two complete string-literal
tokens.

This is blocking dcpls's `pr/plsw-test-harness` saga: PL/SW's
`src/main.c` can't compile after the `[A * B]` macro-buf
workaround lands because the next tc24r limitation surfaces
immediately. We're dogfooding the C compiler — no PL/SW-side
workaround. Fix it here.

## Goal

After this saga: tc24r concatenates adjacent string-literal tokens
(separated only by whitespace, including newlines, comments, or
nothing) into a single string-literal token, matching standard C
behavior. The minimal repro above and the
`sw-cor24-plsw/src/main.c` test fixtures parse cleanly.

## Scope

Support these forms inside any expression context where a string
literal can appear (initializers, function arguments, return
expressions, assignments, etc.):

| Construct | Example | Required |
|---|---|---|
| Two adjacent literals, same line | `"abc" "def"` → `"abcdef"` | **add** |
| Two adjacent literals, separated by newline | `"abc"\n  "def"` → `"abcdef"` | **add** |
| N adjacent literals (any number) | `"a" "b" "c" "d"` → `"abcd"` | **add** |
| Adjacent literals separated by comments | `"abc" /* hi */ "def"` → `"abcdef"` | **add** |
| Mix with backslash continuation inside a literal | `"abc\<nl>def" "ghi"` → `"abcdefghi"` | **add** |
| Adjacent wide-string literals (`L"..."`)  | `L"abc" L"def"` | **out of scope** (PL/SW doesn't use wide strings; revisit later) |
| Mixed narrow + wide | `"abc" L"def"` | **out of scope** (UB in C99, implementation-defined in C11) |

The concatenation must preserve embedded characters byte-for-byte:
escape sequences in each chunk evaluate independently, then the
resolved bytes are joined. Example:

```c
char *s = "ab\n" "cd";
/* s must equal {'a','b','\n','c','d','\0'} (6 bytes), NOT
                {'a','b','\\','n','c','d','\0'} */
```

## Suggested implementation approach (don't have to follow this)

The standard place is **translation phase 6**, immediately after
preprocessing and before parsing. If `tc24r`'s pipeline is:

```
source → lexer → parser → typer → codegen
```

then phase 6 typically belongs as a post-pass on the lexer's
token stream, *before* the parser sees it: scan the token stream,
collapse runs of `StringLit` tokens (with no other tokens between
them — whitespace and comments having already been stripped by the
lexer) into a single `StringLit` whose value is the concatenation
of the constituent values.

If the parser today expects exactly one `StringLit` token where a
string-literal expression appears, a simpler in-parser fix is also
fine: after consuming a `StringLit`, peek; while the next token is
also `StringLit`, consume and append. Either approach is correct;
pick whichever fits `tc24r`'s existing structure better.

The escape-sequence point above (concatenation happens *after*
each chunk's escapes are resolved, not before) is the only subtle
semantic — make sure the implementation matches that, with a test.

## Tests

In `tests/` (or wherever fixtures live):

1. **Positive cases** — these should compile and produce the same
   output as the equivalent single-literal form:
   - `char *s = "abc" "def";` ≡ `char *s = "abcdef";`
   - Multi-line: `char *s = "a"\n  "b"\n  "c";` ≡ `char *s = "abc";`
   - Escapes per chunk: `char *s = "ab\n" "cd";` ≡ `char *s = "ab\ncd";` (6 bytes)
   - With comments between: `char *s = "abc" /* x */ "def";` ≡ `char *s = "abcdef";`
   - In a function call: `puts("hello, " "world");`
   - As a return expression: `char *f() { return "ab" "cd"; }`
   - As an initializer in a multi-line list: each element of a
     `char *strings[] = { "a" "b", "c" "d" };` resolves correctly.

2. **Negative cases**:
   - `"abc" 42` (string followed by non-string token) should fail
     with the same error message it does today (no behavior change
     for non-adjacent-string cases).
   - Truly mismatched syntax stays an error.

3. **Regression**: existing fixtures (single-literal, escapes,
   `\<newline>` continuation inside one literal) keep passing.

4. **Real-world fixture**: copy
   `sw-cor24-plsw/src/main.c:4558-4567` (the GETMAIN macrodef
   test) into a fixture and verify it parses without error.

## Migration

Once this lands and mike installs the new `tc24r` to `work/bin/`:

- dcpls's `pr/plsw-test-harness` saga unblocks: `just build` and
  `just build-lgo` produce `build/plsw.s` and `build/plsw.lgo`,
  which lets the test harness bootstrap golden files for every
  `.plsw` example and run the full regression gauntlet.
- No source changes needed in `sw-cor24-plsw` — the existing
  `src = "..." "..." "...";` patterns become valid C as soon as
  tc24r supports them. (The `[A * B]` workaround in
  `src/macro.h:387` is independent and already deferred to
  `pr/array-size-expressions`.)

## What goes in this PR

1. Implement adjacent string-literal token concatenation per C99
   translation phase 6.
2. Tests above.
3. Update `README.md` (or relevant docs) noting the supported
   forms.

## What does NOT go in this PR

- No wide-string concatenation (`L"..." L"..."`) — separate saga
  if/when needed.
- No general C99-conformance audit — this is a targeted fix.
- No changes to `sw-cor24-plsw` — that's dcpls's repo, and dcpls
  has no workaround to revert (we are dogfooding; the only fix is
  here).

## When done

Push `pr/string-literal-concatenation` and signal. After relay,
mike rebuilds and reinstalls `tc24r` to `work/bin/`. dcpls can
then resume `pr/plsw-test-harness` and bootstrap golden outputs
for the regression suite.
