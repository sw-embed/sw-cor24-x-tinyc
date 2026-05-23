# Brief: investigate 3 tc24r demo tests with `exit_code = 1`

**Owner:** dcxtc
**Branch:** start as `feat/audit-negative-demos`; `dg-mark-pr` when ready
**Repo:** `sw-cor24-x-tinyc`
**Drafted by:** mike (regression-suite audit, 2026-05-09)

## What's worth checking

Three demo tests in `work/reg-rs/` declare `exit_code = 1`:

| Test | What the demo claims to test |
|---|---|
| `tc24r-demo2.rgt` | `=== tc24r Demo 2: Pointers, Char, MMIO ===` |
| `tc24r-demo8.rgt` | `=== tc24r Demo 8: Preprocessor #define ===` |
| `tc24r-demo10.rgt` | `=== tc24r Demo 10: #include, #pragma once, System Headers ===` |

`exit_code = 1` is unusual for demos — by the demo header comments, these are positive demos of language features the transpiler should support. There are two possibilities:

- **Legitimate negative test**: each demo deliberately exercises a known-incomplete code path that returns non-zero, and the test asserts the transpiler refuses cleanly.
- **Locked-in regression**: each demo is supposed to work but currently exits non-zero due to a transpiler bug, and the test is asserting "this demo continues to fail."

If it's the second, the test is fighting the demo's stated goal — the regression suite would actively oppose any future fix.

This is the pattern the user flagged on 2026-05-09: agents capturing reg-rs goldens by blindly running the harness against the current repo state, including failures, instead of verifying the goldens reflect intended behavior.

## Investigation steps (per demo)

1. **Read the demo's `.rgt`/`.err`/`.out` triple in full.** Note: the .err files appear empty per a quick scan; the .out files contain the demo header output. So the demo prints its banner and source, then exits 1 — what happens between the banner and the exit?
2. **Run the demo manually**: `bash demos/run-demo<N>.sh` (or whatever the .rgt's `command` field is). Where does it actually fail?
3. **Compare against the demo's intent**: the demo header says it's testing pointers/char/MMIO (demo2), `#define` (demo8), or `#include + #pragma once` (demo10). If the failure is in those features, that's a real transpiler bug.

## Three acceptable outcomes per demo

| Outcome | Action |
|---|---|
| Demo legitimately tests a known-rejection path | Add a comment in the demo source explaining what's being rejected and why; nothing else to fix. |
| Demo is supposed to work but transpiler can't compile it yet | File a precise saga to fix the transpiler feature (your repo); meanwhile move the demo to `demos/known-failures/` and remove the `.rgt` from active `just test` until fixed. |
| Demo is broken and not worth fixing | Delete the demo + its goldens. |

Don't ship a state where the regression suite asserts a transpiler bug as expected behavior.

## What does NOT belong in this saga

- Fixing all three demos in one saga; pick one to actually fix and document the others as "investigated, see follow-up X."
- Refactoring `work/reg-rs/` harness.
- Adding new demos.

## Verification

After the saga lands, each of the three negative demos has either:
- A comment explaining why it's a legitimate negative test (and the test stays), OR
- A follow-up saga tracking the real transpiler fix, with the test moved out of the active suite, OR
- The demo is removed entirely.

`just test` should not be asserting transpiler bugs as expected behavior.
