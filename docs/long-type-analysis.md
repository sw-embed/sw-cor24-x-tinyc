# COR24 `long` Type Analysis

## Purpose

This document describes how tc24r currently handles the C `long` type,
evaluates possible 32-, 48-, and 64-bit representations on the COR24 ISA,
and recommends a staged implementation plan for exact-width and wide integer
support.

tc24r is a clean-room Tiny C compiler that targets only COR24. Full ISO C
conformance is not a project requirement, but conventional C type behavior and
portable exact-width types remain useful when importing existing embedded C
code.

## Current implementation

The current compiler accepts `long` syntax but does not represent it as a
distinct type. The type parser consumes `long`, `short`, `signed`, and
`unsigned` as modifiers and maps the result to `Type::Int` or
`Type::UnsignedInt`. The AST contains no `Long` or `UnsignedLong` variants.

Consequently, the current data model is:

| C type | Width | Size | Representation |
|---|---:|---:|---|
| `char` | 8 bits | 1 byte | Byte |
| `int` | 24 bits | 3 bytes | Native COR24 word |
| `unsigned int` | 24 bits | 3 bytes | Native COR24 word |
| `long` | 24 bits | 3 bytes | Identical to `int` |
| `unsigned long` | 24 bits | 3 bytes | Identical to `unsigned int` |
| pointer | 24 bits | 3 bytes | Native address |

This behavior can be seen in:

- `components/frontend/crates/tc24r-parser-types/src/type_parse.rs`
- `components/core/crates/tc24r-ast/src/types.rs`
- `docs/c-data-model.md`

Integer literal suffixes also do not currently carry type information. The
lexer accepts suffixes such as `U`, `L`, `LL`, and combinations of them, then
discards the suffix and truncates the value to 24 bits. For example,
`0x12345678L` currently becomes the 24-bit literal `0x345678`. This behavior is
in `components/frontend/crates/tc24r-lexer/src/readers.rs`.

Thus current `long` support is syntactic 24-bit support, rather than a distinct
wide integer implementation. ISO C requires `long` to provide at least 32
value bits, although tc24r does not claim full ISO C conformance.

## Relevant COR24 constraints

COR24 has eight 24-bit registers, but only `r0`, `r1`, and `r2` are practical
general-purpose arithmetic registers in normal generated code. `r1` is also
the link register used by calls. The ISA has:

- Native 24-bit addition, subtraction, multiplication, logic, shifts, and
  comparisons.
- 24-bit word loads and stores.
- Byte loads and stores.
- Three-byte stack pushes and pops.
- No arithmetic carry or overflow flag.
- No high-half multiply result.
- No hardware division.

The absence of a carry flag is important for multi-word addition and
subtraction. Carry or borrow must be detected explicitly with unsigned
comparisons or handled in a runtime helper.

The current ABI and backend assume that every scalar expression result fits in
`r0`, every argument consumes one three-byte stack slot, and each parameter is
three bytes. Wide integers therefore require an explicit multi-word expression
and ABI convention; changing only `Type::size()` is not sufficient.

## A library cannot create a primitive integer width

A `typedef` or preprocessor macro can rename an existing compiler type, but it
cannot create a new primitive width. This works only if `unsigned long` is
already exactly 32 bits:

```c
typedef unsigned long uint32_t;
```

A library can represent a wide value as a structure:

```c
typedef struct {
    unsigned int lo;
    unsigned int hi;
} tc24_uint48;

typedef struct {
    unsigned int lo;
    unsigned int mid;
    unsigned int hi;
} tc24_uint64;
```

The 64-bit representation uses all 24 bits of `lo` and `mid`, but only 16 bits
of `hi`. Library functions can implement arithmetic:

```c
tc24_uint48 tc24_u48_add(tc24_uint48 a, tc24_uint48 b);
int tc24_u48_cmp(tc24_uint48 a, tc24_uint48 b);

tc24_uint64 tc24_u64_add(tc24_uint64 a, tc24_uint64 b);
tc24_uint64 tc24_u64_mul(tc24_uint64 a, tc24_uint64 b);
```

The disadvantage is that C operators do not work on structure values. Code
must use `tc24_u48_add(a, b)` rather than `a + b`, and similarly for shifts,
comparisons, increment, and compound assignment. Such types should not be
called standard `uint48_t` or `uint64_t` until they are true compiler integer
types, because users reasonably expect integer operators to work.

This library approach remains valuable as an implementation prototype. If a
wide primitive is later added, the tested library routines can become compiler
runtime helpers.

## Exact-width portable types

Portable C commonly exposes exact-width types through `<stdint.h>`:

```c
int8_t    uint8_t
int16_t   uint16_t
int32_t   uint32_t
int64_t   uint64_t
```

An exact-width typedef is valid only when the compiler has a suitable
underlying type of exactly that width. The standard also defines least-width
and fast-width families, which can express the COR24 tradeoffs more honestly:

```c
int_least32_t
uint_least32_t
int_fast16_t
uint_fast16_t
```

COR24-specific native types are also useful:

```c
typedef signed int int24_t;
typedef unsigned int uint24_t;
```

`int24_t` and `uint24_t` are implementation extensions rather than standard
`<stdint.h>` names, but accurately document the target's native word size.

## Representation options

### Option 1: 32-bit `long`

Use the following model:

| Type | Width | Object size |
|---|---:|---:|
| `int` | 24 bits | 3 bytes |
| `long` | 32 bits | 4 bytes |
| pointer | 24 bits | 3 bytes |

A packed little-endian 32-bit object consists of a low 24-bit word and a high
byte. A temporary would normally use two registers or stack words:

```text
low part:   bits 0..23
high part:  bits 24..31
```

Object representation and ABI representation need not be identical. Objects
should have `sizeof(long) == 4`; function arguments may occupy two native
three-byte stack slots if that materially simplifies the ABI.

Advantages:

- Meets the conventional minimum width for C `long`.
- Provides natural underlying types for `int32_t` and `uint32_t`.
- Best compatibility with imported embedded C, protocols, file formats,
  checksums, and device interfaces.
- Keeps native `int` and pointers efficient at 24 bits.

Disadvantages:

- Packed values are not naturally word-sized on COR24.
- Loads and stores require a 24-bit word operation plus a byte operation.
- Temporaries still require two native words, with most of the high word
  unused.

### Option 2: 48-bit `long`

Use two complete COR24 words:

```text
low word:   bits 0..23
high word:  bits 24..47
```

This gives a six-byte object with a very natural load, store, stack, spill, and
argument representation. A signed 48-bit value ranges from `-2^47` through
`2^47 - 1`; an unsigned value ranges from zero through `2^48 - 1`.

Advantages:

- Excellent match for the COR24 word and stack architecture.
- No partially occupied word or packed four-byte special case.
- Likely somewhat simpler than 32-bit support in memory and ABI code.
- Substantially greater range than 32-bit integers.

Disadvantages:

- Cannot serve as the underlying type for exact `int32_t` or `uint32_t`.
- Portable code often assumes, correctly or incorrectly, that embedded
  `long` is 32 bits.
- Serialized structures and binary interfaces commonly need exact 32-bit
  fields.

Forty-eight-bit `long` is a valid C design because `long` may be wider than 32
bits. It is architecturally elegant but less compatible with existing code.

### Option 3: 64-bit `long` or `long long`

A 64-bit value requires three COR24 words during computation:

```text
word 0: bits 0..23
word 1: bits 24..47
word 2: bits 48..63; upper 8 bits unused or padding
```

Packed objects should preferably occupy eight bytes, even if expression
temporaries and ABI slots use three words. A nine-byte representation with
padding is technically possible for a compiler-defined ABI, but is surprising
and poor for binary interoperability.

Three-word values are awkward because normal code generation has only about
three practical arithmetic registers, one of which is also the link register.
Nontrivial expressions would spill frequently or call runtime helpers.

This is nevertheless how wider integers are commonly supported on narrower
machines. Conventional 32-bit compilers use two words for 64-bit values,
inline simple operations, and call runtime helpers for expensive operations.
The same approach applies to COR24 with three words.

### Option 4: compiler-specific primitive widths

The compiler could introduce internal types such as:

```c
__int24   __uint24
__int32   __uint32
__int48   __uint48
__int64   __uint64
```

Headers would then expose selected aliases. This separates the C data model
from exact-width facilities and can support all desired widths. It is also the
largest design because every primitive width must participate correctly in
conversions, promotions, arithmetic, storage, calls, and initializers.

For a single-target Tiny C compiler, implementing every width immediately
would add substantial complexity without demonstrated application demand.

## Compiler and runtime responsibilities

Wide integer support is primarily a compiler-backend and ABI feature, assisted
by a runtime library. A runtime library alone cannot implement conventional
integer syntax because the compiler must understand:

- Type identity, signedness, rank, promotions, and conversions.
- Literal type and value preservation.
- `sizeof` and object alignment.
- Local and global storage.
- Arrays and pointer arithmetic.
- Struct and union layout.
- Loads, stores, dereference, and assignment.
- Function parameters and return values.
- Comparisons and conversion to Boolean.
- Unary, binary, increment, and compound-assignment operators.

Runtime helpers are well suited to operations that the ISA cannot perform
efficiently:

- Wide multiplication.
- Signed and unsigned division and modulo.
- General variable-count shifts, initially.
- Possibly comparisons and add/subtract in a correctness-first prototype.

Addition, subtraction, bitwise operations, equality, relational comparisons,
and common constant shifts are good candidates for later inline generation.

## Backend and ABI design

Before implementation, tc24r needs a documented convention for wide
temporaries. Plausible conventions include:

- `r0` holds the low 24 bits and `r2` holds the high word.
- Wide intermediate values are always spilled to compiler-managed stack
  storage.
- All wide operations initially call helpers that receive and return values in
  designated stack slots.

The helper-heavy convention is least efficient but easiest to make correct
with the limited register set. It can later be optimized without changing the
C data model.

The ABI must separately specify:

- Argument word order and stack-slot consumption.
- Wide return values, such as a register pair or caller-provided result area.
- Signed high-word normalization.
- Packed object layout versus expanded argument/temporary layout.
- Alignment of four-, six-, and eight-byte objects.
- Behavior at calls, including preservation of `r2` and use of link register
  `r1`.

For correctness and simplicity, a caller-provided result area is worth
considering for 64-bit values and possibly all wide values. A two-word 32- or
48-bit result can instead use `r0:r2` if the ABI carefully defines it.

## Required compiler changes

A true 32-bit `long` implementation would require at least the following work.
The same areas apply to 48- and 64-bit types, with different word counts.

1. Add distinct signed and unsigned wide variants to the AST type system.
2. Preserve integer literal suffixes and values wider than 24 bits.
3. Implement integer rank, signedness, promotions, and usual conversions.
4. Make expression type inference consistently propagate scalar result types.
5. Implement packed wide global data and zero initialization.
6. Allocate and align wide locals.
7. Implement wide variable, member, array, and pointer loads and stores.
8. Update struct and union layout and initializer emission.
9. Scale pointer arithmetic by the new element sizes.
10. Define and implement a multi-word expression-result convention.
11. Implement casts between byte, 24-bit, pointer, and wide types.
12. Implement Boolean conversion and signed/unsigned comparisons.
13. Implement unary, arithmetic, bitwise, shift, and compound operations.
14. Make argument placement and cleanup depend on parameter types.
15. Implement wide return values.
16. Add runtime helpers for expensive arithmetic.
17. Update `limits.h`, add a target `<stdint.h>`, and extend formatting support
    such as `%ld` and `%lu` where appropriate.

The major architectural work is multi-word expression representation and the
ABI, rather than the arithmetic helper algorithms themselves.

## Estimated effort

These are rough estimates for a developer already familiar with this Rust
codebase:

| Scope | Estimate |
|---|---:|
| Minimal useful wide type: storage, constants, casts, comparisons, add/subtract, calls, helper-based complex arithmetic | 1-2 focused weeks |
| Coherent support across most expression forms already handled by tc24r | 3-5 weeks |
| Well-tested and optimized implementation with robust ABI and external-suite compatibility | 6-10 weeks |

Forty-eight-bit `long` has almost the same frontend effort as 32-bit `long` and
may be approximately 10-20 percent simpler in storage and ABI handling.
Three-word 64-bit support may require approximately 1.5-2 times the
backend/runtime effort of a two-word 32- or 48-bit implementation.

These estimates are planning ranges, not commitments. Actual effort depends
strongly on how much of the current single-register expression architecture
can be generalized cleanly.

## Test-suite and demo impact

Existing demos mostly use `int`, `char`, pointers, arrays, and structures. The
only demo currently found with `long` in its text is `demo39.c`, where it means
long branch distances rather than the C data type. Existing application
behavior should therefore remain unchanged if `int` and pointers remain 24
bits.

There is still regression risk in shared machinery:

- Type parsing and inference.
- Local stack allocation.
- Global initializer emission.
- Array and structure layout.
- Casts, assignment, and pointer arithmetic.
- Function argument offsets and cleanup.

The chibicc test harness currently removes lines containing `long` and many
`sizeof` assertions in `scripts/run-chibicc-test.sh`. Real `long` support would
allow these filters to be narrowed and would expose additional tests, although
tests that assume a host-specific 64-bit `long` would still require
target-specific expectations.

New focused tests should cover:

- `sizeof(long)` and arrays/structures containing longs.
- Minimum, maximum, overflow, and sign-extension behavior.
- Decimal, octal, and hexadecimal literals with all relevant suffixes.
- Mixed `char`, `int`, unsigned, pointer, and long conversions.
- Global, local, pointer, array, and member loads/stores.
- Every unary and binary operator.
- Function arguments and return values, including nested calls.
- Runtime helper boundary cases, especially carry and borrow across bit 23.
- Packed byte layout and round trips through serialized memory.

## Recommended approach

Because tc24r targets only COR24, the implementation should retain the native
24-bit strengths of the ISA and add wider facilities incrementally.

### Stage 1: document and expose native widths

Keep:

```text
char       8 bits
int        24 bits
pointer    24 bits
```

Provide COR24 extension aliases:

```c
typedef signed int int24_t;
typedef unsigned int uint24_t;
```

If 16-bit primitive support is not real, do not claim exact `int16_t` merely
because documentation describes a nominal short size. Either implement it or
provide only honest least/fast types.

### Stage 2: implement exact 32-bit `long`

Use:

```text
long          32 bits, 4-byte packed object
unsigned long 32 bits, 4-byte packed object
```

Then provide a minimal target `<stdint.h>` with genuine `int8_t`, `uint8_t`,
`int32_t`, and `uint32_t`, plus COR24 `int24_t` and `uint24_t`. Add 16-bit names
only when their compiler semantics and object representation are implemented.

This choice is less mechanically elegant than 48-bit `long`, but exact 32-bit
compatibility is likely more valuable for imported portable embedded code.

Use a correctness-first, helper-heavy backend initially. Once tests pass,
inline common addition, subtraction, comparisons, bitwise operations, and
constant shifts.

### Stage 3: add library-based 48- and 64-bit values

Introduce clearly COR24-specific structure types such as `tc24_uint48` and
`tc24_uint64`, with focused conversion and arithmetic APIs. Do not initially
present structure types as standard `uint64_t`, because they lack normal
integer operator behavior.

This stage provides practical wide counters and serialization support without
requiring the compiler to solve three-word general expression evaluation.

### Stage 4: promote demonstrated needs into compiler primitives

If real applications are significantly burdened by the function-based API,
add compiler-supported 64-bit `long long`. Lower expensive operations to the
already-tested library routines, which then become the compiler runtime.

A compiler-native 48-bit extension can also be added if COR24 applications
demonstrate a strong need for it. It should be exposed as an explicit extension
rather than replacing exact 32-bit `long`.

## Recommended target data model

The recommended eventual model is:

| Type | Width | Notes |
|---|---:|---|
| `char` | 8 bits | Native byte |
| `int` | 24 bits | Native and efficient |
| pointer | 24 bits | Full COR24 address space |
| `long` | 32 bits | Exact portable/interchange width |
| COR24 `int24_t` / `uint24_t` | 24 bits | Target extension aliases for `int` |
| `tc24_uint48` | 48 bits | Library structure initially |
| `tc24_uint64` | 64 bits | Library structure initially |
| `long long` | 64 bits | Optional later compiler primitive |

This design preserves native COR24 efficiency, supplies the commonly needed
exact 32-bit type, and leaves 48- and 64-bit support proportional to actual
application demand. It also creates a low-risk evolution path: wide library
routines can later serve unchanged as compiler runtime helpers.
