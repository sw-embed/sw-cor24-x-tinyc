# Brief: rebase tc24r reg-rs codegen baselines after legitimate codegen drift

**Owner:** dcxtc
**Branch:** `pr/rebase-codegen-baselines`
**Repo:** `sw-cor24-x-tinyc`
**Drafted by:** dcxtc (observed while running the test suite during
`pr/array-size-expressions` and `pr/string-literal-concatenation`)

## Context

Running `reg-rs run -p tc24r` against current HEAD on this repo
reports **77 failed of 82 total**. The five passing are the recent
demos (demo55, demo56, demo62, demo63, plus a couple of others). All
77 failures are baseline-mismatch on `.out` files in
`work/reg-rs/tc24r-*.out`, not a behavioral change in the test
itself.

CLAUDE.md notes: *"reg-rs baselines (`.out` files) contain absolute
paths. After switching machines, run `reg-rs rebase -p tc24r` to
update baselines."* — but this round of drift isn't from a machine
switch, it's from real codegen improvements landed since the
baselines were captured. Examples seen during Phase 1:

- `demo10.s`:
  ```
  L13:
          lc      r0,0
  -        bra     L7        /* dead unconditional branch immediately before L7 */
  L7:
          mov     sp,fp
  ```
  Looks like a dead-branch peephole.

- `demo1.s`:
  ```
  -        lc      r1,1
  -        shl     r0,r1
  +        add     r0,r0
  ```
  `x << 1` rewritten as `x + x` — also a peephole.

- Byte counts shifted (e.g. assembled-bytes line: 390 → 367).

These are improvements, not regressions: programs still produce
identical observable behavior (`r0`, halt state, UART output —
all unchanged where the demo runner checks them). Only the
intermediate assembly text and bytecode bytes drifted, and only
the `.out` baseline files notice.

## The risk in just blindly rebasing

`reg-rs rebase -p tc24r` would happily accept any new output as
the baseline, including a real correctness regression that
happens to compile. If a future change breaks codegen, the next
rebase makes the regression invisible.

So this saga is *deliberate, audited rebase*, not a one-line
script run.

## Goal

After this saga:

- Every `work/reg-rs/tc24r-*.out` reflects current correct
  codegen.
- `reg-rs run -p tc24r` is back to **82 passed of 82 total** (or
  whatever the new baseline count is; demo63 etc. count too).
- Each baseline change in the diff has been visually inspected
  and the diff is documented as an intentional improvement, not a
  regression.

## Scope

For each currently-failing reg-rs test (run `reg-rs run -p tc24r`
to enumerate):

1. Run the test in verbose mode (`reg-rs run -p <name> -vv`) to
   capture the diff between current output and the stored
   baseline.
2. Classify each diff line as one of:
   - **Peephole / DCE / strength-reduction**: legitimate
     improvement. Acceptable.
   - **Byte-count or instruction-count shift**: usually downstream
     of (1). Acceptable if only counts changed.
   - **Behavioral**: `r0` value, halt state, or UART output
     changed. **STOP — escalate to mike.** This is a regression
     and should not be silently rebased away.
3. Once every failing test is classified as (1) or (2), run
   `reg-rs rebase -p tc24r` to capture new baselines.
4. Re-run `reg-rs run -p tc24r` to confirm green.
5. Add a short note to `docs/testing-status.md` summarizing the
   classes of changes accepted (e.g. *"Rebased 2026-05-08:
   peephole — dead branch removal; strength reduction —
   `lc 1; shl` → `add r,r`; byte counts down ~5% across demos
   from these"*).

## Tests

Implicit: `reg-rs run -p tc24r` must pass after the rebase. The
saga succeeds iff that's green. Spot-check one or two demos by
hand to confirm no behavioral drift:

- Pick three demos at random. Run each demo's `bash demos/run-demoNN.sh`.
  Confirm it still produces the expected `r0` and UART output.

## Suggested order of operations

1. Fresh build: `cargo build --manifest-path components/cli/Cargo.toml --release`
2. Survey failures: `reg-rs run -p tc24r > /tmp/before.txt`
3. Walk failures, classify each (script-friendly: a small
   `for name in $(reg-rs run -p tc24r 2>&1 | awk '/^  FAIL/{print $2}'); do reg-rs run -p "$name" -vv > /tmp/diffs/$name; done`
   gives a per-test diff dump for review).
4. After audit: `reg-rs rebase -p tc24r`
5. Verify: `reg-rs run -p tc24r` → all pass
6. Manual spot-check: `bash demos/run-demo10.sh`, `run-demo1.sh`,
   one chibicc test.
7. Commit in one go (or split per-suite if it's cleaner).

## Migration

Internal-only. No downstream effect. After this lands and mike
rebuilds, `reg-rs run -p tc24r` is once again a useful regression
gate — right now it's a no-op (everything fails, so any new
failure goes unnoticed).

## What goes in this PR

1. Refreshed `work/reg-rs/tc24r-*.out` files for every previously-
   failing test.
2. A short `docs/testing-status.md` note summarizing the
   accepted-change classes (peephole, DCE, etc.).
3. Optional: a `scripts/audit-rgt-drift.sh` helper for future
   rebases that walks failures, dumps `-vv` diffs, and reports
   anything looking behavioral.

## What does NOT go in this PR

- No codegen changes. If the audit surfaces a real regression,
  open a separate `pr/<bugfix>` saga first; rebase only after
  that's fixed.
- No `.rgt` (test definition) edits — only `.out` baselines.
- No reg-rs tool changes.

## When done

Push `pr/rebase-codegen-baselines` and signal. After relay, the
test suite is once again green and meaningful, and future
codegen changes that *aren't* improvements get caught.
