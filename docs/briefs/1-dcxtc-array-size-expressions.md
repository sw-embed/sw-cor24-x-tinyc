# Brief: support const-expr in array sizes in `tc24r`

**Owner:** dcxtc
**Branch:** `pr/array-size-expressions`
**Repo:** `sw-cor24-x-tinyc`

## Context

`tc24r` is the C-to-COR24 cross-transpiler. It currently parses array sizes as a **single token only** — a literal or `#define`'d constant. Anything beyond that errors.

Concrete failure observed in `sw-cor24-plsw/src/macro.h:387`:
```c
char  mac_arg_buf[MAC_ARG_MAX * MAC_ARG_VAL_MAX];
/* tc24r: error at offset 15032: expected RBracket, got Star */
```

Minimal repro:
```c
#define A 8
#define B 16
char buf[A * B];        /* fails: expected RBracket, got Star */
char buf2[BUF_SIZE];    /* OK */
```

Constant-expression array sizes are normal C (have been since C89). dcpls is shipping a stopgap `#define`-the-product workaround in PL/SW to unblock today, but the proper fix is here in `tc24r`.

## Goal

Extend `tc24r`'s array-size grammar to accept compile-time-evaluable integer expressions. After this saga, the failing repro above parses cleanly and `tc24r` emits the same machine code it would for the equivalent literal-sized array.

## Scope

Support these inside `[ ... ]` for array sizes:

| Construct | Example | Required |
|---|---|---|
| Integer literal | `[42]`, `[0x100]` | already works |
| Single identifier (constant) | `[BUF_SIZE]` | already works |
| Binary `*`, `+`, `-`, `/` | `[A * B]`, `[N + 1]`, `[N - 1]`, `[N / 2]` | **add** |
| Parens for grouping | `[(A + B) * C]`, `[A * (B + C)]` | **add** |
| Mix of literals and identifiers | `[N * 16]`, `[256 + N]` | **add** |
| Unary `-` | `[-1]` (probably nonsense in array context but parser-level OK) | nice-to-have |
| `sizeof(...)` | `[sizeof(int)]`, `[sizeof(struct X)]` | **out of scope** for this PR — separate concern |
| Function calls or runtime values | `[strlen(s)]` | out of scope (would be VLA — also out of scope) |

**Operator precedence:** standard C — `*` and `/` higher than `+` and `-`. Left-associative. Parens override. Match what a C99-conforming compiler would do for the same expression.

**Evaluation:** at parse/codegen time, fold the expression to a single integer constant. Reject (with a clear error) anything that can't be reduced to a compile-time integer (e.g. references to non-`#define`'d identifiers).

**Errors:** if an identifier inside the expression isn't a known compile-time constant, error with a message that names the offending identifier and clearly says "must be a compile-time integer constant."

## Suggested implementation approach (don't have to follow this)

If `tc24r`'s parser already has a general expression parser somewhere (e.g., for initializers like `int x = A * B;`), reuse it for array sizes — they should share the same const-expr machinery. If today the array-size path has its own one-token parser, replace it with a call into the general expression parser, gated to compile-time-evaluable nodes.

If `tc24r` has no general expression evaluator yet, this saga doesn't have to ship one — minimal const-expr parser supporting the operators above is enough.

## Tests

In `tests/` (or wherever your fixture pattern lives):

1. **Positive cases** — verify these produce the same output as their literal-sized equivalents:
   - `char buf[A * B];` where `A=8, B=16` should match `char buf[128];`
   - `int xs[N + 1];`
   - `char grid[ROWS * COLS];`
   - `char nested[(A + B) * 2];` (parens)
   - Mixed: `char cell[N * 16];` (identifier × literal)

2. **Negative cases**:
   - `int xs[i * 2];` where `i` is a runtime variable — error mentioning that `i` must be compile-time-constant.
   - `char ar[];` (incomplete array — should error or behave as today)
   - `char ar[A *];` (broken expression) — clear parse error.

3. **Regression**: existing fixtures with literal sizes and single-identifier sizes still pass.

## Migration

Once this lands and mike installs the new `tc24r` to `work/bin/`:

- dcpls can revert their `MAC_ARG_BUF_SIZE` workaround in `macro.h` and restore the cleaner `char mac_arg_buf[MAC_ARG_MAX * MAC_ARG_VAL_MAX]` form. That's a follow-up dcpls saga, not yours.
- Other repos may have similar workarounds (or `[A * B]` lines that haven't been hit because the file wasn't compiled yet); a project-wide grep follow-up is worth scheduling.

## What goes in this PR

1. Extend the array-size parser to support const-expr with `* / + -` and parens.
2. Compile-time integer evaluation that reduces these expressions to a single number.
3. Tests above.
4. Update `README.md` (or relevant docs) noting the supported expression forms.

## What does NOT go in this PR

- No `sizeof(...)` support — separate saga.
- No VLA or runtime-sized arrays.
- No general expression parser overhaul if `tc24r` doesn't have one — minimal const-expr only.
- No changes to `sw-cor24-plsw` — that's dcpls's stopgap saga and dcpls's later cleanup, not yours.

## When done

Push `pr/array-size-expressions` and signal. After relay, mike rebuilds and reinstalls `tc24r` to `work/bin/`. Existing PL/SW workarounds become redundant (dcpls will clean those up separately). Other downstream repos that may have hit similar issues unblock automatically.
