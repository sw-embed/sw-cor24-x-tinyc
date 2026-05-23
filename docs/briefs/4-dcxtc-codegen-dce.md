# Brief: whole-program dead-code elimination in tc24r

**Owner:** dcxtc
**Branch:** `pr/codegen-dce`
**Repo:** `sw-cor24-x-tinyc`
**Drafted by:** dcxtc (observed while planning the heap-variant
saga: every `.h`-defined function ships whether used or not)

## Context

tc24r emits **every function** defined in any `#include`'d header,
regardless of whether the program ever calls it. Concrete: demo45
calls `malloc` and a few `<string.h>` / `<stdio.h>` helpers, but
its `.s` also contains:

```
_free:        # never called
_calloc:      # never called
_realloc:     # never called
_exit:        # never called
_abs:         # never called
_atoi:        # never called
```

(Verified: `./components/cli/target/release/tc24r demos/demo45.c -I include | grep '^_'`
shows all six.)

This is wasted text-section bytes in every demo, and it'll get
worse as the freestanding stdlib grows (e.g. when
`pr/stdlib-heap-variant` lands and the reclaim variant adds
~150 lines of free-list bookkeeping).

## Why now

tc24r is **single-TU** today (per README "What Does Not Work Yet:
Multi-file compilation"). That makes whole-program analysis
trivial: by the time codegen runs, the parser has the entire
program in memory. There is no linker between tc24r and `cor24-asm`
to do `--gc-sections` for us. Adding link-time DCE later would
mean teaching `cor24-asm` about dead-section pruning; doing it at
the tc24r AST level is simpler and lands the win now.

When multi-TU eventually arrives, the analysis will need to be
either per-TU + linker-level or a global pass over all TUs — but
the AST-pass machinery built here is the right starting point
either way.

## Goal

After this saga: every function whose symbol is not reachable
(directly or indirectly) from `main`, an interrupt handler, a
global initializer, or inline assembly is dropped from the
emitted `.s`. Globals (data) get the same treatment if it's
cheap; otherwise just functions.

Concretely, demo45's `.s` no longer contains `_free`, `_calloc`,
`_realloc`, `_exit`, `_abs`, `_atoi` — text section shrinks
proportionally.

## Scope

| Construct | Behavior |
|---|---|
| Direct call: `foo()` | `foo` is reachable |
| Function-pointer use: `int (*p)() = foo;` | `foo` is reachable (address taken) |
| Function name in any `Expr::Ident` outside a call position | `foo` is reachable |
| Reference from a global initializer | reachable |
| Reference from inline `asm("...")` string | conservative: keep all functions named with `_` prefix that match symbol-like tokens in any asm string. Or simpler: keep ALL functions if any asm appears in the program (heavy hammer; can refine later). |
| `main` | always a root |
| Interrupt handler (`is_interrupt: true`) | always a root |
| Functions referenced from struct/array initializers | reachable |
| Unreachable functions | **dropped from codegen** |

**Out of scope:**
- Dead globals/data (defer to a follow-up if we care).
- Cross-TU DCE (multi-TU isn't supported yet).
- Inlining or other optimizations.

## Suggested implementation approach (don't have to follow this)

Pipeline: `lex → parse → **dce_filter** → codegen`.

**Where it lives:** new analysis module, e.g.
`components/frontend/crates/tc24r-dce/` or as a method on
`Program`. Probably easier as a free function in a new crate,
called from `dispatch/` (or wherever the high-level pipeline is
orchestrated).

**Algorithm:**

```
fn dce(program: &mut Program) {
    let mut reachable: HashSet<String> = HashSet::new();
    let mut worklist: Vec<String> = Vec::new();

    // Roots
    worklist.push("main".into());
    for f in &program.functions {
        if f.is_interrupt { worklist.push(f.name.clone()); }
    }
    for g in &program.globals {
        collect_function_refs(&g.init, &mut worklist);
    }
    // Conservative: keep all functions if any inline asm exists
    // anywhere in the program (or scan asm strings for symbols).

    while let Some(name) = worklist.pop() {
        if !reachable.insert(name.clone()) { continue; }
        if let Some(f) = program.functions.iter().find(|f| f.name == name) {
            collect_function_refs_in_block(&f.body, &mut worklist);
        }
    }

    program.functions.retain(|f| reachable.contains(&f.name));
}

fn collect_function_refs(expr, worklist) {
    // Walk the expression: for every Expr::Call { name }, push name.
    // For every Expr::Ident(name), if name is a known function, push.
}
```

**Conservative roots to ALWAYS keep:**
- `main`, `_start` (synthesized), all `is_interrupt` functions
- Any function whose name appears in inline `asm("...")` strings
- Externs / declared-but-not-defined functions left as references
  (the AST should already handle: just don't drop a function
  that's also called)

**Edge case: function pointers in struct initializers.** If a
struct has a function-pointer field initialized to `foo`, that
should mark `foo` reachable. The collector needs to descend into
`Expr::InitList` and any nested initializer expressions.

**Edge case: function called only through a pointer.** Example:
```c
int (*ops[])() = { foo, bar };
int main() { return ops[0](); }
```
`foo` and `bar` are address-taken (Expr::Ident in an init list)
and thus reachable. The conservative rule "if address is taken,
keep" handles it — even if `bar` is never actually called.

## Tests

In `components/frontend/crates/tc24r-parser-tests/` (or a new
`tc24r-dce-tests/` crate):

1. **Direct calls keep / unreferenced drops:**
   ```c
   int used(void) { return 42; }
   int unused(void) { return 99; }
   int main(void) { return used(); }
   ```
   Assert `Program` after DCE contains `used` + `main`, not `unused`.

2. **Transitive reachability:**
   ```c
   int leaf(void) { return 1; }
   int mid(void) { return leaf(); }
   int main(void) { return mid(); }
   ```
   Assert all three present.

3. **Function pointer keeps target:**
   ```c
   int callme(void) { return 7; }
   int (*fp)(void) = callme;
   int main(void) { return fp(); }
   ```
   Assert `callme` is kept.

4. **Interrupt handler is a root:**
   ```c
   __attribute__((interrupt)) void isr(void) { ... }
   int main(void) { return 0; }
   ```
   Assert `isr` is kept.

5. **End-to-end size shrink:** compile demo45 before and after
   DCE; assert the symbol set shrinks (no `_abs`, `_atoi`, `_exit`,
   `_calloc`, `_realloc`, `_free`).

6. **Regression:** existing demos still pass behavior tests
   (`r0`, halt, UART output unchanged). Run reg-rs.

## Migration

Internal-only. Programs that previously compiled still compile
and run identically — the dropped symbols were never called. The
`.s` and assembled `.lgo` shrink; nothing observable changes.

When `pr/rebase-codegen-baselines` lands (or in conjunction with
this saga), the reg-rs `.out` baselines will update to reflect
the smaller code. Coordination point: run rebase **after** DCE
lands, not before, so both sets of changes flow into one rebase.

## What goes in this PR

1. AST analysis pass that builds the call graph, computes
   reachability, and filters `Program.functions`.
2. Conservative handling of inline `asm` (whichever option from
   "Suggested approach" is cleanest to implement).
3. Hook the pass into the compile pipeline.
4. Tests above.
5. Update `README.md` "What Works" with a one-line note.

## What does NOT go in this PR

- No data/global DCE (defer).
- No multi-TU machinery.
- No reg-rs baseline rebase (separate
  `pr/rebase-codegen-baselines`).
- No inlining or other codegen optimizations.

## When done

Push `pr/codegen-dce` and signal. After relay, every demo's `.s`
shrinks. The follow-up `pr/stdlib-heap-variant` then ships the
opt-in heap reclaim without bloating programs that don't use it.
