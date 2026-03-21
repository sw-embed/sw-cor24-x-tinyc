# COR24 C ABI Proposal (v0)

This ABI is based on MakerLisp's proprietary COR24 C compiler conventions,
as described in direct feedback from the COR24 hardware designer and
confirmed by examining MakerLisp compiler output (.s files from fib.c,
sieve.c, etc.).

## Register Roles

| Register | Saved By | Role |
|----------|----------|------|
| r0 | Caller (not saved) | Return value, scratch, first temp |
| r1 | Callee | Return address (from `jal`) |
| r2 | Callee | Register variable (preserved across calls) |
| fp (r3) | Callee | Frame pointer |
| sp (r4) | - | Stack pointer (maintained by convention) |
| z (r5) | - | Zero constant (read-only) |
| iv (r6) | - | Interrupt vector (set by startup code) |
| ir (r7) | - | Interrupt return (set by CPU) |

### Key Points

- **r0** is freely clobbered by any function. Used for return values.
- **r1** holds the return address after `jal`. Callee must save/restore it
  if the function makes any calls (non-leaf).
- **r2** is the only general-purpose callee-saved register. Use it as a
  "register variable" -- it survives across function calls.
- **fp** is always saved/restored. Used to access arguments and locals.

## Argument Passing

Arguments are passed **on the stack**, pushed **right-to-left** before the
call. The caller cleans up arguments after the call returns.

```asm
; calling func(a, b, c):
        push    c_value         ; arg 3 (rightmost, pushed first)
        push    b_value         ; arg 2
        push    a_value         ; arg 1 (leftmost, pushed last)
        la      r0,_func
        jal     r1,(r0)         ; call
        add     sp,9            ; clean up 3 args (3 bytes each)
```

Each argument occupies one 24-bit stack slot (3 bytes), regardless of the
C type. `char` and `short` values are widened to 24 bits before pushing.

### Accessing Arguments in Callee

After the standard prologue, arguments are at positive offsets from fp:

```
        lw      r0,9(fp)        ; first argument
        lw      r0,12(fp)       ; second argument
        lw      r0,15(fp)       ; third argument
        ; general: arg N is at fp + 9 + (N-1)*3
```

The offset of 9 accounts for the three saved registers (fp, r2, r1) that
the prologue pushes before setting fp = sp.

### Byte-Sized Arguments

When a `char` argument is passed, it occupies a full 24-bit stack slot.
The callee loads it with `lb` (sign-extended) or `lbu` (zero-extended)
at the appropriate byte offset within the slot:

```asm
        lb      r0,9(fp)        ; load char argument (sign-extended)
```

Note: The MakerLisp compiler loads `char` args with `lb` at the base of
the slot (byte 0 of the 3-byte word), relying on little-endian layout
where the low byte is at the lowest address.

## Return Values

- **char or int (24-bit):** returned directly in r0.
- **Larger types (long, float, double, structs):** r0 contains a pointer
  to a function-specific static storage area in .bss. The caller must copy
  the value out before calling the function again, as the next call
  overwrites the return area. (This follows the Portable C Compiler model.)

For the initial cc24 compiler (demo subset), only `char` and `int` return
values need to be supported -- both directly in r0.

## Function Prologue

Standard prologue for non-leaf functions:

```asm
_func:
        push    fp              ; save caller's frame pointer
        push    r2              ; save register variable
        push    r1              ; save return address
        mov     fp,sp           ; establish new frame
        sub     sp,N            ; allocate N bytes for locals (if needed)
```

If N is small enough (1..127), use `add sp,-N` instead of `sub sp,N`.
`sub sp,dddddd` is a 4-byte instruction for larger allocations.

If N is 0 (no locals), the `sub sp` / `add sp` can be omitted --
`mov sp,fp` in the epilogue handles restoration.

## Function Epilogue

```asm
        mov     sp,fp           ; deallocate locals
        pop     r1              ; restore return address
        pop     r2              ; restore register variable
        pop     fp              ; restore caller's frame pointer
        jmp     (r1)            ; return to caller
```

## Stack Frame Layout

```
Higher addresses (toward initial SP)
+---------------------+
| Argument N          |  fp + 9 + (N-1)*3
| ...                 |
| Argument 2          |  fp + 12
| Argument 1          |  fp + 9
+---------------------+
| Saved r1 (ret addr) |  fp + 6
| Saved r2            |  fp + 3
| Saved fp            |  fp + 0  <-- fp points here
+---------------------+
| Local variable 1    |  fp - 3
| Local variable 2    |  fp - 6
| ...                 |
| Local variable M    |  fp - M*3
+---------------------+  <-- sp (after sub sp,M*3)
Lower addresses (stack grows down)
```

## Leaf Function Optimization

If a function does not call any other function (leaf routine), it may
skip saving registers it does not clobber:

- If r1 is not overwritten, skip `push r1` / `pop r1`.
- If r2 is not used, skip `push r2` / `pop r2`.
- If fp is not needed (no locals accessed via fp), skip frame setup.

MakerLisp recommends limiting these optimizations to leaf routines only,
to avoid complexity in analyzing callee register usage.

## Stack Growth and Alignment

- Stack grows **downward** (push decrements sp by 3).
- Each stack slot is 3 bytes (one 24-bit word).
- sp should remain 3-byte aligned at all times.
- Initial sp is 0xFEEC00 (set by CPU reset).

## Function Call Sequence Summary

### Caller Responsibilities

1. Evaluate arguments.
2. Push arguments right-to-left onto stack.
3. Load target function address: `la r0,_func`
4. Call: `jal r1,(r0)`
5. Clean up arguments: `add sp,N*3`
6. Result is in r0.

### Callee Responsibilities

1. Execute prologue (push fp, r2, r1; set fp).
2. Allocate locals if needed.
3. Access arguments at positive fp offsets.
4. Place return value in r0.
5. Execute epilogue (restore sp, pop r1, r2, fp; jmp (r1)).

## Tail Calls

A tail call can skip the epilogue and use `jmp` instead of `jal`:

```asm
        ; instead of: la r0,_other; jal r1,(r0)
        ; followed by epilogue, use:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        la      r0,_other
        jmp     (r0)            ; tail call (r1 still has original return addr)
```

## Interrupt Service Routine ABI

ISRs must save and restore ALL registers they use, including the
condition flag:

```asm
isr:
        push    r0
        push    r1
        push    r2
        mov     r2,c            ; save condition flag in r2
        push    r2

        ; ... ISR body ...

        pop     r2
        clu     z,r2            ; restore condition flag
        pop     r2
        pop     r1
        pop     r0
        jmp     (ir)            ; return from interrupt (clears intis)
```

## Source

This ABI is based on:
- Direct email from MakerLisp (COR24 designer), March 2026
- MakerLisp C compiler output (fib.s, sieve.s from COR24-TB demo programs)
- The Portable C Compiler (pcc) calling conventions that MakerLisp adapted
