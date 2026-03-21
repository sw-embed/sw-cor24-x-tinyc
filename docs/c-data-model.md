# COR24 C Data Model

Type sizes, alignment, signedness, and pointer model for the cc24 compiler.

## Type Sizes

| C Type | Size (bits) | Size (bytes) | Signedness | Notes |
|--------|-------------|-------------|------------|-------|
| `char` | 8 | 1 | signed | Matches C default; use `unsigned char` for unsigned |
| `unsigned char` | 8 | 1 | unsigned | |
| `short` | 16 | 2 | signed | Stored as 2 bytes in memory |
| `unsigned short` | 16 | 2 | unsigned | |
| `int` | 24 | 3 | signed | Native machine word |
| `unsigned int` | 24 | 3 | unsigned | |
| `long` | 24 | 3 | signed | Same as int for initial version |
| `unsigned long` | 24 | 3 | unsigned | |
| `long long` | - | - | - | Not supported initially |
| `float` | - | - | - | Not supported initially |
| `double` | - | - | - | Not supported initially |
| `_Bool` | 8 | 1 | unsigned | 0 or 1 |
| pointer | 24 | 3 | - | Same as int; full address space |
| `enum` | 24 | 3 | signed | Same as int |

### Rationale for 24-bit int

COR24 is a 24-bit machine. Making `int` 24 bits:
- Maps naturally to the hardware word size
- Avoids expensive multi-word arithmetic for every int operation
- Matches pointer size (simplifies pointer/int conversions)
- Is honest about the machine's capabilities

The MakerLisp C compiler uses this same model.

### What About 32-bit int?

A 32-bit int would require:
- Every arithmetic operation to use two registers or helper routines
- Load/store pairs for every int access
- Significant codegen complexity for a demo compiler

This could be revisited later, but for cc24 v1, 24-bit int is correct.

## Alignment

| Type | Alignment |
|------|-----------|
| `char` | 1 byte |
| `short` | 1 byte (no alignment requirement) |
| `int` / pointer | 3 bytes (word-aligned preferred) |
| struct members | Natural alignment of each member |

COR24 hardware supports unaligned word access, but aligned access is more
efficient. The compiler should emit word-aligned locals and globals where
practical.

Stack slots are always 3-byte aligned (the stack operates in 24-bit words).

## Endianness

**Little-endian.** Least significant byte at lowest address.

A 24-bit value 0x123456 stored at address A:
- A+0: 0x56 (low byte)
- A+1: 0x34 (mid byte)
- A+2: 0x12 (high byte)

This matches `lw`/`sw` instruction behavior.

## Integer Promotions

For the initial compiler, integer promotion rules are simplified:

1. `char` is promoted to `int` (24-bit) in expressions.
2. `short` is promoted to `int` (24-bit) in expressions.
3. Operations produce `int` (24-bit) results.
4. When storing back to `char`, truncate to low 8 bits.
5. When storing back to `short`, truncate to low 16 bits.

Sign extension / zero extension is handled by:
- `sxt ra,rb` -- sign-extend byte to 24-bit
- `zxt ra,rb` -- zero-extend byte to 24-bit
- `lb` -- load byte with sign extension
- `lbu` -- load byte with zero extension

For `short`, there are no 16-bit load/store instructions. Use:
- Load short: `lw` then mask/sign-extend (or two `lb`/`lbu` instructions)
- Store short: `sb` low byte, shift right 8, `sb` high byte

### Practical Note on short

Given the added complexity of 16-bit types on a 24-bit machine with no
native 16-bit load/store, the initial compiler may treat `short` as
equivalent to `int` (24-bit) for simplicity. This is a pragmatic choice
for a demo compiler.

## Pointer Representation

- All pointers are 24 bits (3 bytes).
- Code and data share a single flat address space.
- Function pointers and data pointers have the same representation.
- `sizeof(void *)` == `sizeof(int)` == 3.
- NULL pointer is 0x000000.

## sizeof Summary

| Expression | Value |
|-----------|-------|
| `sizeof(char)` | 1 |
| `sizeof(short)` | 2 |
| `sizeof(int)` | 3 |
| `sizeof(long)` | 3 |
| `sizeof(void *)` | 3 |
| `sizeof(int *)` | 3 |

## Array Layout

Arrays are laid out contiguously in memory with elements at stride
`sizeof(element)`:

```
int arr[4]:     arr+0, arr+3, arr+6, arr+9  (stride 3)
char buf[8]:    buf+0, buf+1, buf+2, ...     (stride 1)
```

Pointer arithmetic scales by element size:
- `int *p; p+1` advances by 3 bytes
- `char *p; p+1` advances by 1 byte

## Struct Layout (Deferred)

Structs are not supported in the initial compiler version. When added:
- Members laid out in declaration order
- Each member at its natural alignment
- Total struct size padded to alignment of largest member
- Passed by pointer, returned via static storage pointer in r0

## Limits

| Constant | Value | Hex |
|----------|-------|-----|
| `CHAR_BIT` | 8 | |
| `CHAR_MIN` | -128 | 0x80 |
| `CHAR_MAX` | 127 | 0x7F |
| `UCHAR_MAX` | 255 | 0xFF |
| `INT_MIN` | -8388608 | 0x800000 |
| `INT_MAX` | 8388607 | 0x7FFFFF |
| `UINT_MAX` | 16777215 | 0xFFFFFF |
| `LONG_MIN` | -8388608 | 0x800000 |
| `LONG_MAX` | 8388607 | 0x7FFFFF |

## Features Deferred

The following are not supported in cc24 v1:

- `long long` (would require 48-bit multi-word arithmetic)
- `float`, `double` (no FPU; would need software emulation)
- Bitfields
- `_Complex`
- Variable-length arrays
- `volatile` semantics (all memory accesses are currently volatile-like)
