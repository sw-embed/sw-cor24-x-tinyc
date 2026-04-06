# Codegen Optimization

## Concern

The tc24r compiler produces correct but verbose assembly. Comparing
against hand-optimized COR24 reference code (see dplshift-example.md),
the output was ~1.9x larger (42 vs 22 instructions for a simple
function). Several patterns generated unnecessary instructions.

## Approach

Four peephole optimizations identified from the dplshift comparison,
implemented as targeted changes in the parser and codegen:

### 1. Negative literal folding (parser)

**Before:** `-1` parsed as `UnaryOp::Neg(IntLit(1))`, generating
`push r0; lc r0,0; pop r1; sub r0,r1` (5 instructions).

**After:** Fold `-N` to `IntLit(-N)` at parse time. Generates
`lc r0,-1` (1 instruction). Saves 4 instructions per negative literal.

**File:** `components/frontend/crates/tc24r-parser/src/expr.rs`

### 2. Shift-by-1 peephole (codegen)

**Before:** `x << 1` generates `lc r1,1; shl r0,r1` (2 instructions).

**After:** When RHS of `Shl` is `IntLit(1)`, emit `add r0,r0` instead
(1 instruction). Self-add is equivalent to left-shift-by-1 and avoids
loading the shift count.

**File:** `components/codegen-expr/crates/tc24r-expr-ops/src/binop.rs`

### 3. Redundant branch elimination (post-pass)

**Before:** Control flow codegen often produces `bra Lx` immediately
before `Lx:` (a branch to the next instruction — a no-op).

**After:** Post-pass scans the assembly output and removes `bra Lx`
when the next line is `Lx:`. This is a text-level optimization applied
after all code generation and branch resolution.

**File:** `components/codegen-emit/crates/tc24r-emit-core/src/emit.rs`

### 4. Compare against zero register (codegen)

**Before:** `if (x < 0)` generates `lc r1,0; cls r0,r1` (2 instructions)
because the RHS 0 is loaded into r1.

**After:** When comparing against `IntLit(0)`, use the COR24 `z`
register directly: `cls r0,z` (1 instruction). Handles `==`, `!=`,
`<`, `>=` against zero. `>` and `<=` fall back to r1 load (asymmetric
compare semantics).

**Files:**
- `components/codegen-ops/crates/tc24r-ops-compare/src/compare.rs`
- `components/codegen-stmt/crates/tc24r-stmt-control/src/condition.rs`

## Results

### dplshift.c benchmark

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Assembly lines | 58 | 50 | -14% |
| Instructions | ~42 | ~34 | -19% |
| Reference (hand-optimized) | — | 22 | 1.5x vs 1.9x |

Key improvements:
- `lc r0,-1` instead of 5-instruction negation sequence
- `add r0,r0` instead of 2-instruction shift-by-1 (3 occurrences)
- Removed 1 redundant `bra L0` before `L0:`

### Not yet implemented

- **Register allocation**: The reference code keeps variables in `r2`
  (honoring the `register` keyword). tc24r puts all locals on the stack.
  This is the largest remaining gap (accounts for most of the 1.5x vs
  1.0x difference). Requires a register allocator, which is a major
  architectural change.

- **Constant folding**: Expressions like `2 + 3` could be evaluated at
  compile time. Currently both operands are loaded and added at runtime.

- **Dead store elimination**: Zero-fill stores followed immediately by
  init stores could be omitted for elements that are explicitly initialized.
