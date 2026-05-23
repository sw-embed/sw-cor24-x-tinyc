# Brief: opt-in heap-reclaim variant of `<stdlib.h>`

**Owner:** dcxtc
**Branch:** `pr/stdlib-heap-variant`
**Repo:** `sw-cor24-x-tinyc`
**Drafted by:** dcxtc (motivated by future demos that allocate and
free many small objects — Lisp-style cons cells, parser ASTs, etc.)

## Context

`include/stdlib.h` today is a bump allocator with two real bugs:

```c
void free(void *ptr) {
    // No-op bump allocator — memory is not reclaimed
}

void *realloc(void *ptr, int size) {
    // Simple implementation: allocate new block, no copy
    // (correct behavior requires knowing old size)
    return malloc(size);
}
```

`free` is genuinely a no-op (line 28 of stdlib.h). `realloc`
silently loses the old contents — programs that use it have
undefined-but-quiet data corruption. Both are admitted in
comments but they're sharp edges if a demo or downstream agent
unwittingly relies on standard semantics.

For short-running deterministic programs (most current demos),
the bump allocator is *actually a feature* — no fragmentation,
no search-order timing, predictable layout. We want to keep that
property as the default. But for longer-running programs that
allocate and discard objects (Lisp interp, parser, anything with
a workload), we need a real free with reclaim.

## Goal

After this saga:

- `<stdlib.h>` defaults to the existing bump+nop allocator (no
  behavior change for current programs).
- Defining `TC24R_HEAP_RECLAIM` *before* `#include <stdlib.h>`
  swaps in a real free-list allocator with working `free`,
  coalescing, and a working `realloc` that copies old data.
- The two implementations live in peer headers `<heap_bump.h>`
  and `<heap_reclaim.h>`, with `<stdlib.h>` dispatching between
  them.

## Scope

```c
// include/stdlib.h (sketch)
#pragma once
#define NULL 0
#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1
// ... atoi, abs, exit, etc. (always present, unchanged)

#ifdef TC24R_HEAP_RECLAIM
#include <heap_reclaim.h>
#else
#include <heap_bump.h>
#endif
```

```c
// include/heap_bump.h
#pragma once
// Current bump impl, extracted verbatim from today's stdlib.h.
// free is a no-op; realloc keeps its current
// "allocate-without-copy" wart (or removed entirely with a
// #error pointing at heap_reclaim.h — TBD in implementation).
```

```c
// include/heap_reclaim.h
#pragma once
// Free-list allocator:
//   - Header per allocation (size + next-free pointer)
//   - First-fit search on a singly-linked free list
//   - free() walks back, links into list
//   - Optional coalescing of adjacent blocks (v1: skip; v2: add)
//   - realloc() reads old size from header, copies if growing
```

| Function | Bump variant | Reclaim variant |
|---|---|---|
| `malloc(n)` | bump `_heap_ptr += n` | search free list, split if oversized |
| `free(p)` | nop | reads header for size, links into free list |
| `calloc(n, sz)` | malloc + zero (already correct) | malloc + zero (works either way) |
| `realloc(p, n)` | broken: returns new alloc, loses data | reads old size from header, copies, frees old |

**Out of scope:**
- Coalescing in v1 (add later if fragmentation bites).
- Best-fit / next-fit search (first-fit is fine to start).
- Thread-safety (single-core, no threads on COR24).
- Variable-sized headers / per-arena heaps.
- Wide-string interning, GC, refcounting, anything beyond
  basic malloc/free.

## Suggested implementation approach (don't have to follow this)

**Header layout** (3-byte aligned for COR24 24-bit words):

```
+-----------+-----------+
| size (3B) | next (3B) |    6-byte header
+-----------+-----------+
| user payload (size B) |
+-----------------------+
```

- `size` is the total *payload* size (excluding header).
- `next` is the pointer to the next free block when this block is
  free; unused (could be 0) when allocated.
- `malloc(n)` returns `header + 6` (skip past header).
- `free(p)` does `header = p - 6; link header into free list`.

**Free list initialization:** at first `malloc`, if the list is
empty, the entire range `[_heap_start, _heap_end)` becomes one
free block. `_heap_end` defaults to a fixed address (e.g.
`0x0F0000`, leaving 64KB headroom for stack-grows-down
collisions). User can override by `#define`'ing
`TC24R_HEAP_END` before include.

**First-fit search:** walk the free list, return the first block
≥ requested size. If the block is much larger, split: keep the
remainder as a smaller free block.

**`realloc(p, n)`:** read old size from header, allocate new
block, byte-copy old payload, free old. Now correct (and the
old comment-confessed bug is gone).

## Tests

In `components/frontend/crates/tc24r-parser-tests/` and as a
demo:

1. **Bump variant (default) regression:** every existing demo
   that uses `malloc` (demo45, others) compiles and runs
   unchanged.

2. **Reclaim variant — basic free + reuse:**
   ```c
   #define TC24R_HEAP_RECLAIM
   #include <stdlib.h>
   int main(void) {
       void *p = malloc(100);
       free(p);
       void *q = malloc(100);
       /* q should equal p (or at least not have advanced
          past where p was) — bump would advance, reclaim
          reuses */
       return p == q ? 0 : 1;
   }
   ```

3. **Reclaim variant — realloc preserves data:**
   ```c
   #define TC24R_HEAP_RECLAIM
   #include <stdlib.h>
   #include <string.h>
   int main(void) {
       char *s = (char *)malloc(4);
       strcpy(s, "abc");
       s = (char *)realloc(s, 8);
       return strcmp(s, "abc"); /* must be 0 */
   }
   ```

4. **Reclaim variant — many small alloc/free cycles:** allocate
   N=1000 small objects, free half, allocate another N/2;
   verify no collision with stack and `_heap_ptr` (or
   equivalent free-list watermark) doesn't unboundedly grow.

5. **demo64.c**: end-to-end demo with `TC24R_HEAP_RECLAIM` —
   build a small Lisp-style structure with cons/free cycles,
   prove memory is reclaimed (e.g. allocate 10000 cons cells in
   a loop with frees, observe heap doesn't blow past a fixed
   ceiling).

## Migration

Internal — adds new headers, refactors `<stdlib.h>`. No
downstream changes required. Programs continue compiling
unchanged unless they `#define TC24R_HEAP_RECLAIM`.

The `realloc` correctness fix is gated on the reclaim variant.
The bump variant's `realloc` is documented as broken-by-design
(or removed with a `#error` — TBD); programs that need
correct `realloc` opt in.

## Pre-existing concerns to handle

- **Multi-TU collision (future).** Both variants currently define
  `_heap_ptr` (or free-list head) as a non-static global. When
  multi-file compilation lands, every TU including `<stdlib.h>`
  produces a duplicate symbol. Two answers, decide in this saga:
  - `static` everywhere (each TU gets its own heap — wasteful,
    fragmenting, but compiles)
  - Split decl/def: `<stdlib.h>` declares, the user explicitly
    compiles a `stdlib_impl.c` that defines. Breaks the .h-only
    model but is the C-idiomatic answer.

  Recommendation: punt to a follow-up brief (`dcxtc-stdlib-multi-tu.md`)
  when multi-TU lands. Document the trade-off in the README for
  now.

- **Heap/stack collision.** Heap grows up from `0x080000`, stack
  grows down. They can meet silently. The reclaim variant
  should add a high-water-mark check that returns NULL if the
  free list can't satisfy a request. Bump variant could too,
  cheaply.

- **`int` vs `size_t`.** Standard prototype is
  `void *malloc(size_t)`. COR24 24-bit `int` is large enough
  (~16M, vs 1MB SRAM) but the type isn't standard. Worth
  fixing eventually for portability and for symmetry with the
  rest of `<stddef.h>`.

## What goes in this PR

1. Refactor `<stdlib.h>` to dispatch on `TC24R_HEAP_RECLAIM`.
2. Extract the existing bump impl into `<heap_bump.h>`.
3. New `<heap_reclaim.h>` with first-fit free-list allocator,
   working free, working realloc.
4. Tests above (parser-tests + demo64).
5. Update `README.md` "What Works" — note both variants and the
   opt-in macro.

## What does NOT go in this PR

- No coalescing (v2 saga).
- No multi-TU fix (separate brief when multi-TU lands).
- No best-fit / arena / GC / pool allocators.
- No `<malloc.h>` non-standard extensions.
- No changes to `cor24-asm` or any other repo.

## When done

Push `pr/stdlib-heap-variant` and signal. After relay, demos
that need real free/reclaim (Lisp eval, parser ASTs, anything
with churn) can opt in by defining the macro. Programs that
don't define the macro see no change.
