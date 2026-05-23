# Brief: emit `.zero N` for zero-init globals in tc24r

**Owner:** dcxtc
**Branch:** `pr/emit-zero-fill`
**Repo:** `sw-cor24-x-tinyc`
**Discovered by:** dcstk during chunk-pool sizing review of the
chunk-storage scheme (2026-05-10) — `chunk_storage[16 * 4096]`
costs ~150 KB of `.lgo` for 64 KB of runtime SRAM, almost all of
which is encoding overhead from per-word zero-fill text.
**Depends on:** `dcxas-zero-fill-directive.md` — **already landed
and on PATH.** `cor24-asm` commit `fcc7f6c` (`feat: add .zero N
directive for bulk zero-fill`). No upstream wait.

## Symptom

For any global static array with no initializer (or `INIT(0)`-style
implicit zero), tc24r enumerates one `.word 0` line per 3-byte
word. Concrete example from the dcstk chunk-pool work:

```c
static char chunk_storage[16 * 4096];   /* 65,536 chars = 21,846 words */
```

emits 21,845 separate `.word   0` lines in the `.s`. The `.lgo`
encoder then carries roughly 7 bytes of text per `.s` line, so
**64 KB of physical SRAM zero-fill produces ~150 KB of `.lgo`
text**. That is the +145 KB overhead dcstk measured in their
chunk-pool review — it's not real bloat, it's an encoding artifact
of `tc24r`'s per-word emit.

The bloat is purely on the source / `.lgo` side. The assembled
`.bin` is the correct 64 KB. Nothing in the runtime image changes.
But the toolchain pipeline — `.s` parsing, `.lgo` encoding,
relay/transfer cost, downstream tools that scan the `.lgo` — pays
the 2.3x text inflation for every zero-init global.

## Direct blocker for dcstk (and any consumer of large zero arrays)

Anyone allocating a sizable static buffer hits this:

- The chunk-pool work above (~150 KB `.lgo` overhead per 64 KB pool).
- Future ring buffers, frame buffers, lookup tables zeroed at boot.
- Any port of a C program with `static char buf[N]` where N is
  large (most embedded/parser/IO code).

Without the fix, every such consumer has to either accept the
encoding bloat or hand-tune their globals to fit a budget that's
artificially 2-3x too tight.

## Root cause

`components/codegen-emit/crates/tc24r-emit-data/src/data.rs:118`
— `emit_zero_fill`:

```rust
fn emit_zero_fill(state: &mut CodegenState, ty: &Type) {
    let total = ty.size();
    let mut off = 0;
    while off + 3 <= total {
        emit!(state, "        .word   0");      // <-- one line per word
        off += 3;
    }
    while off < total {
        emit!(state, "        .byte   0");      // <-- one line per trailing byte
        off += 1;
    }
}
```

This is the path hit for `_ => emit_zero_fill(state, &g.ty)` at
`data.rs:44` — i.e. every global whose `init` is `None`,
`Expr::InitList(empty)`, or any other shape that falls through the
match.

`emit_typed_data` (`data.rs:61`) has the same shape at line 99
(`_ => emit!(state, "        .word   0")`) for individual elements
of a partially-initialized array, but that path only fires when an
explicit `InitList` is being walked element-by-element. The big
win is in `emit_zero_fill`, which handles the all-zero case in one
gulp.

## Goal

When `emit_zero_fill` would emit only zero bytes, emit a single
`.zero <total_byte_size>` directive instead. Identical bytes go
into the `.bin` and `.lgo`; the `.s` shrinks proportionally.

For the chunk-pool case: 21,846 lines collapse to 1.

## What changes

In `components/codegen-emit/crates/tc24r-emit-data/src/data.rs`:

```rust
fn emit_zero_fill(state: &mut CodegenState, ty: &Type) {
    let total = ty.size();
    if total > 0 {
        emit!(state, "        .zero   {total}");
    }
}
```

That is the entire change. The previous loop is replaced by one
emit call. `total == 0` (zero-sized type) is a no-op, matching the
old loop's behavior.

**Optional follow-up (recommend deferring):** the same compaction
applies in two other places:

1. `emit_typed_data` line 99 — when walking an `InitList` that
   ran out of values, the per-element `.word 0` could batch into
   `.zero N` for trailing-zero runs. Requires looking ahead in the
   value iterator.
2. `data.rs:33-38` — string init padding (`pad` zero bytes after
   a too-short `.byte` string) emits one `.byte 0` per pad byte;
   could be `.zero {pad}` in one line.

Both are smaller wins (mixed-init arrays and short-string padding
are usually tens of bytes, not 65 KB). Defer to a follow-up brief
if profiling shows them.

## Output spec (byte-identical `.lgo`)

After the change, the generated `.lgo` must be byte-identical to
the previous output for the same source. Smoke test:

```sh
# Build a demo with a large zero-init global with old tc24r, capture .lgo.
# Build the same demo with new tc24r, capture .lgo.
# diff .lgo's must be empty.
```

The `.s` will differ (one `.zero N` line vs N enumerated lines)
but the `.lgo` and the `--listing` output addresses must match.
This is how `cor24-asm` is wired (see `dcxas-zero-fill-directive.md`):
the `.zero N` directive emits N bytes of zero into the same
location counter, so downstream is unaffected.

## Tests to add

In `components/codegen-emit/crates/tc24r-emit-data/tests/` (or
wherever the existing data-section tests live):

1. **Byte-identical `.lgo` regression** — pick a demo with a
   zero-init global (or add one) and verify the `.lgo` matches
   before/after by assembling both `.s` outputs through `cor24-asm`
   and comparing.
2. **Big zero array** — `static char buf[65536];` emits one
   `.zero 65536`, not 21,846 `.word 0` + trailing `.byte 0` lines.
3. **Small zero array** — `static int x[3];` emits `.zero 9`
   (3 elements x 3 bytes), not three `.word 0` lines.
4. **Non-word-multiple zero array** — `static char buf[10];` emits
   `.zero 10`, not three `.word 0` + one `.byte 0`.
5. **Single zero scalar** — `static int x;` emits `.zero 3`, not
   `.word 0`. (Sanity check that scalar globals still work; the
   one-emit path covers them too.)
6. **Mixed-init unaffected** — `static int x[5] = {0, 1, 2, 0, 0};`
   keeps the existing per-element form (this brief doesn't touch
   `emit_typed_data`).
7. **String-init unaffected** — `static char s[8] = "hi";` keeps
   the existing `.byte 'h','i',0,0,0,0,0,0` form (deferred to
   follow-up).
8. **reg-rs** — run reg-rs against existing demos. Goldens will
   diff (the `.s` snapshots change shape) — that's the expected
   coordination point with `pr/rebase-codegen-baselines`. See
   "Migration" below.

## What does NOT go in this PR

- No changes to `cor24-asm` — `.zero N` is already there
  (`fcc7f6c`).
- No changes to `emit_typed_data` mixed-init handling — defer.
- No changes to string-init padding in `emit_data_section` — defer.
- No changes to struct padding (`emit_typed_data` line 87-89) —
  defer.
- No reg-rs baseline rebase — that's `pr/rebase-codegen-baselines`.
  Run rebase **after** this lands so both flow into one rebase.

## Migration

Internal-only. Programs that previously compiled still compile and
run identically — the assembled bytes are unchanged. Only the `.s`
intermediate and the `.lgo` text shrink. Nothing observable at
runtime.

The reg-rs `.s` snapshot baselines (the `.rgt` / `.tdb` files
under `work/reg-rs/`) will diff; rebase coordination is the same as
the DCE brief's: run `pr/rebase-codegen-baselines` once, after this
lands.

## When done

Push `pr/emit-zero-fill`. After mike relays + reinstalls `tc24r` to
`work/bin/`:

- dcstk's chunk-pool `.lgo` overhead drops from +145 KB to ~0 (one
  `.zero 65536` line plus the directive's tiny `.lgo` cost).
- Any tc24r consumer with zero-init globals gets the same shrink
  for free.
- Future briefs that add large zero buffers (ring buffers, frame
  buffers, lookup tables) stop fighting the encoding artifact.

This pairs with the dcpls / dcxas zero-fill work (already landed)
to give the whole COR24 toolchain — PL/SW, tinyc, and the assembler
they share — a consistent, source-density-friendly story for static
zero-fill. Roughly 5 lines of code changed.
