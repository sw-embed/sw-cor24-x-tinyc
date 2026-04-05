# Double-Precision Left Shift — Compiler Comparison

This document compares tc24r's output against a hand-optimized COR24
reference assembly for a double-precision (48-bit) integer left shift
implemented in C.

COR24 has 24-bit registers. A 48-bit value is stored across two 24-bit
halves: `x0` (low) and `x1` (high). A left shift of the pair requires
shifting `x1` left by 1 and pulling the top bit of `x0` into the bottom
bit of `x1`.

## Source: dplshift.c

```c
int main()
{
    register unsigned int x0, x1;

    /* Initial conditions */
    x0 = x1 = -1;

    /* Try double precision integer left shift, this way */
    x1 = (x1 << 1) | (x0 >> 23);

    /* Now this way */
    x1 = (x1 << 1) | ((int)x0 < 0);

    /* Shifting the low part is the same */
    x0 <<= 1;

    return 0;
}
```

Two techniques are shown for extracting the carry bit from `x0`:

- **Method 1**: `x0 >> 23` — logical right shift by 23 isolates bit 23
  (the MSB of a 24-bit value) into bit 0.
- **Method 2**: `(int)x0 < 0` — cast to signed and compare against zero.
  On COR24, `cls rN, z` sets the condition flag if the signed value is
  negative (bit 23 set), then `mov rN, c` captures the flag as 0 or 1.

Both produce the same result; method 2 is one instruction shorter.

## Reference Assembly: dplshift.s-cor24

Hand-written/optimized for COR24. Uses `register` to keep `x0` in `r2`
(a callee-saved register), `x1` on the stack at `-3(fp)`.

```asm
        .text

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
; line 6, file "dplshift.c-cor24"
        lc      r0,-1
        sw      r0,-3(fp)
        mov     r2,r0
; line 9, file "dplshift.c-cor24"
        mov     r0,r2
        lc      r1,23
        srl     r0,r1
        lw      r1,-3(fp)
        add     r1,r1
        or      r1,r0
        sw      r1,-3(fp)
; line 12, file "dplshift.c-cor24"
        lw      r0,-3(fp)
        add     r0,r0
        cls     r2,z
        mov     r1,c
        or      r0,r1
        sw      r0,-3(fp)
; line 15, file "dplshift.c-cor24"
        add     r2,r2
; line 17, file "dplshift.c-cor24"
        lc      r0,0
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
```

**Key features**: 3-byte stack frame (only `x1` on stack), `x0` lives
in `r2` throughout, `add rN,rN` used for shift-by-1, line-number
comments for debugging.

## tc24r Output: dplshift.s

Compiled with `tc24r docs/dplshift.c -o docs/dplshift.s`.

```asm
        .text

        .globl  _start
_start:
        la      r0,_main
        jal     r1,(r0)
_halt:
        bra     _halt

        .globl  _main
_main:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-6
        lc      r0,1
        push    r0
        lc      r0,0
        pop     r1
        sub     r0,r1
        sw      r0,-6(fp)
        sw      r0,-3(fp)
        lw      r0,-6(fp)
        lc      r1,1
        shl     r0,r1
        push    r0
        lw      r0,-3(fp)
        lc      r1,23
        srl     r0,r1
        mov     r1,r0
        pop     r0
        or      r0,r1
        sw      r0,-6(fp)
        lw      r0,-6(fp)
        lc      r1,1
        shl     r0,r1
        push    r0
        lw      r0,-3(fp)
        lc      r1,0
        cls     r0,r1
        mov     r0,c
        mov     r1,r0
        pop     r0
        or      r0,r1
        sw      r0,-6(fp)
        lw      r0,-3(fp)
        lc      r1,1
        shl     r0,r1
        sw      r0,-3(fp)
        lc      r0,0
        bra     L0
L0:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)
```

## Line-by-Line Comparison

### Prologue and Stack Frame

| Aspect | Reference | tc24r |
|--------|-----------|-------|
| Entry stub | none | `_start` calls `_main`, halts on return |
| Frame size | 3 bytes (`x1` only) | 6 bytes (`x0` and `x1` both on stack) |
| `register` effect | `x0` kept in `r2` | `register` accepted, ignored; both vars on stack |

The reference allocates `x0` to register `r2` (honoring the `register`
hint). tc24r accepts the keyword but does not perform register
allocation — all locals live on the stack.

### Line 6: `x0 = x1 = -1`

| | Reference | tc24r |
|-|-----------|-------|
| `-1` literal | `lc r0,-1` (1 instr) | `lc r0,1; push r0; lc r0,0; pop r1; sub r0,r1` (5 instr) |
| Chained assign | `sw r0,-3(fp); mov r2,r0` | `sw r0,-6(fp); sw r0,-3(fp)` |

tc24r computes `-1` as `0 - 1` because the parser treats `-1` as unary
negation of `1`, not as a negative literal. The reference compiler
recognizes `-1` as a single constant.

### Line 9: `x1 = (x1 << 1) | (x0 >> 23)` (method 1)

| | Reference | tc24r |
|-|-----------|-------|
| `x1 << 1` | `lw r1,-3(fp); add r1,r1` | `lw r0,-6(fp); lc r1,1; shl r0,r1` |
| `x0 >> 23` | `mov r0,r2; lc r1,23; srl r0,r1` | `lw r0,-3(fp); lc r1,23; srl r0,r1` |
| OR + store | `or r1,r0; sw r1,-3(fp)` | `mov r1,r0; pop r0; or r0,r1; sw r0,-6(fp)` |

Both produce the correct result. The reference avoids a push/pop because
`x0` lives in `r2` (no need to save it before computing the shift). The
reference also uses `add r1,r1` for shift-by-1, which is a peephole
optimization over `shl r0,r1` with `r1=1`.

### Line 12: `x1 = (x1 << 1) | ((int)x0 < 0)` (method 2)

| | Reference | tc24r |
|-|-----------|-------|
| Sign test | `cls r2,z; mov r1,c` | `lw r0,-3(fp); lc r1,0; cls r0,r1; mov r0,c` |
| `x1 << 1` | `lw r0,-3(fp); add r0,r0` | `lw r0,-6(fp); lc r1,1; shl r0,r1; push r0` |

The reference tests `r2 < 0` directly (one instruction: `cls r2,z`)
since `x0` is in a register. tc24r loads `x0` from the stack and
compares against a loaded zero. Both use `cls` + `mov c` correctly.

### Line 15: `x0 <<= 1`

| | Reference | tc24r |
|-|-----------|-------|
| Shift | `add r2,r2` (1 instr) | `lw r0,-3(fp); lc r1,1; shl r0,r1; sw r0,-3(fp)` (4 instr) |

The reference shifts `r2` in-place with a self-add. tc24r loads from
stack, shifts, and stores back.

### Epilogue

Both are identical: `lc r0,0` (return value), restore `sp`, pop
`r1/r2/fp`, `jmp (r1)`. tc24r has a redundant `bra L0` before `L0:`
(a no-op branch to the next instruction).

## Summary

| Metric | Reference | tc24r | Notes |
|--------|-----------|-------|-------|
| Correctness | Yes | Yes | Both produce identical runtime results |
| Instructions | 22 | 42 | ~1.9x code size |
| Stack usage | 3 bytes | 6 bytes | No register allocation in tc24r |
| Execution | Runs | Runs | `r0 = 0` on both |

### What tc24r gets right
- Correct shift/OR/compare logic for both carry extraction methods
- `register` keyword accepted (no error)
- Unsigned right shift uses `srl` (not `sra`)
- `(int)x0 < 0` correctly uses signed compare `cls`
- Chained assignment `x0 = x1 = -1` handled
- Compound assignment `x0 <<= 1` handled

### Optimization opportunities
1. **Negative literal folding**: `-1` could emit `lc r0,-1` instead of computing `0 - 1`
2. **Register allocation**: honor `register` hint or implement general regalloc to keep hot variables in `r2`
3. **Shift-by-1 peephole**: `shl r0,1` could emit `add r0,r0`
4. **Redundant branch elimination**: `bra L0` immediately before `L0:` is a no-op
5. **Compare against zero**: `cls r0,r1` where `r1=0` could use `cls r0,z` (z register is always 0)
