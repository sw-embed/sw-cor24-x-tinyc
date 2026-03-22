# cc24 -- C Compiler for the COR24 FPGA Soft CPU

cc24 is an open-source C compiler targeting the **COR24** (C-Oriented RISC, 24-bit) instruction set architecture. COR24 is an FPGA soft CPU designed by [MakerLisp](https://makerlisp.com) for efficient C code execution. The existing MakerLisp C compiler is proprietary -- cc24 provides an open-source alternative.

The approach is inspired by [chibicc](https://github.com/rui314/chibicc), a small educational C compiler. cc24 follows the same architecture pattern (recursive-descent parser, direct assembly emission) but is written from scratch in Rust.

## Project Status

The compiler is **functional** -- it compiles real C programs to COR24 assembly that runs on hardware and the cor24-rs emulator. 14 components, ~50 crates, 17 demos all passing. See [docs/status.md](docs/status.md) for detailed status and test counts.

### What Works

- Types: int (24-bit), char (8-bit), void, pointers, arrays
- All standard C operators with correct precedence
- Control flow: if/else, while, do...while, for, break, continue
- Ternary operator (? :), character literals, multi-declaration
- Prefix/postfix increment and decrement (++i, i--)
- Functions with multiple parameters, recursion, ISR support
- Globals, string constants, hex literals
- Pointer arithmetic with element-size scaling
- Preprocessor: #define, #include, #pragma once
- Inline assembly: asm("...")
- MMIO: LED, UART TX/RX, interrupt enable
- Software division and modulo runtime

### What Does Not Work Yet

- switch/case
- +=, -=, and other compound assignment
- sizeof, typedef, enum, struct, union
- Function prototypes (forward declarations)
- Multi-file compilation

## COR24 Architecture at a Glance

- **24-bit** registers and address space (16 MB)
- **8 registers**: r0-r2 (general purpose), fp, sp, z (zero), iv, ir
- **32 instructions**: arithmetic, logic, shift, compare, branch, load/store, stack ops
- **1/2/4-byte** variable-length instruction encoding
- Hardware multiply (24-cycle), no hardware divide
- Little-endian, byte-addressable memory
- UART and GPIO via memory-mapped I/O

## Component Structure

The compiler is organized into 14 components with ~50 crates, each component
being its own Cargo workspace under `components/`:

| Component | Purpose |
|-----------|---------|
| core | Shared types: AST, tokens, spans, errors, traits |
| frontend | Preprocessing, lexing, parsing |
| backend | Legacy codegen and validation |
| codegen-emit | Assembly emission (core, data, load/store) |
| codegen-expr | Expression codegen (call, literal, ops, pointer, variable) |
| codegen-ops | Operator codegen (arithmetic, bitwise, compare, divmod, incdec, logical, unary, type-infer) |
| codegen-state | Codegen state management |
| codegen-stmt | Statement codegen (control flow, simple statements) |
| codegen-structure | Function structure (ISR, locals, prologue) |
| config | Configuration and target definition |
| dispatch | Codegen dispatch layer |
| macros | Proc macros (asm-dsl, emit-macros, handler-macros) |
| testing | Test harnesses (as24, compile, cor24, golden) |
| cli | CLI binary entry point |

See [docs/architecture.md](docs/architecture.md) for data flow and design constraints.

## Building

Build all components:

```bash
scripts/build-all.sh
```

This runs each component's `scripts/build.sh` in dependency order. The release binary is produced at:

```
components/cli/target/release/cc24
```

To build a single component:

```bash
cargo build --manifest-path components/<name>/Cargo.toml
```

## Usage

```bash
cc24 <input.c> [-o output.s] [-I dir]
```

- `<input.c>` -- C source file to compile
- `-o output.s` -- output assembly file (default: stdout)
- `-I dir` -- add include search path (repeatable)

### Full Workflow

```bash
# Compile C to COR24 assembly
cc24 program.c -o program.s

# Assemble (requires as24 from COR24-TB archive)
as24 < program.s | longlgo > program.lgo

# Run on emulator (cor24-rs)
cd ~/github/sw-embed/cor24-rs
cargo run -- run path/to/program.s
```

## Demos

17 demos in `demos/`, all PASS on the cor24-rs emulator:

| # | Demo | Features Tested | Status |
|---|------|-----------------|--------|
| 1 | demo.c | Globals, function calls, recursion, if/else, while, for | PASS |
| 2 | demo2.c | char, pointers, casts, MMIO (LED, UART TX) | PASS |
| 3 | demo3.c | Hex literals, pointer arithmetic, string constants | PASS |
| 4 | demo4.c | Software division and modulo | PASS |
| 5 | demo5.c | Arrays (declaration and indexing) | PASS |
| 6 | demo6.c | Global char/pointer, .byte/.word emission | PASS |
| 7 | demo7.c | Pointer subtraction with scaling | PASS |
| 8 | demo8.c | Preprocessor #define | PASS |
| 9 | demo9.c | Interrupt attribute, ISR, UART RX interrupt | PASS |
| 10 | demo10.c | #include, #pragma once, -I flag | PASS |
| 11 | demo11.c | Logical && and || with short-circuit | PASS |
| 12 | demo12.c | do...while loop | PASS |
| 13 | demo13.c | break, continue (while, do...while, for) | PASS |
| 14 | demo14.c | Prefix/postfix ++, -- | PASS |
| 15 | demo15.c | Ternary operator (? :) | PASS |
| 16 | demo16.c | Character literals ('a', '\n', '\\') | PASS |
| 17 | demo17.c | Multi-declaration (int x, y, z;) | PASS |
| 18 | demo18.c | sizeof operator | PASS |
| 19 | demo19.c | static/extern keywords | PASS |
| 20 | demo20.c | Statement expressions ({ }) | PASS |
| 21 | demo21.c | Compound assignment (+=, -=, *=, /=, etc.) | PASS |

Run a demo:

```bash
demos/run-demo.sh     # runs demo.c through cc24 and cor24-rs
```

## chibicc Test Compatibility

chibicc coverage: **2/41 (5%)**

Testing against the 41 test files from [chibicc](https://github.com/rui314/chibicc).
2 currently compile and pass. Status and first blocker for each file listed below.

### Passing (2)

| Test | Notes |
|------|-------|
| pragma-once | #pragma once inclusion guard |
| stdhdr | System header inclusion (skips gracefully) |

### Out of Scope (7)

These tests require hosted C features, FPU, or threading not applicable to
a freestanding 24-bit target:

| Test | Reason |
|------|--------|
| alloca | Requires alloca() runtime |
| atomic | Requires _Atomic / hosted threading |
| float | Requires FPU (COR24 has no FPU) |
| stdhdr | Requires hosted standard headers |
| tls | Requires _Thread_local / OS threads |
| varargs | Requires va_list / stdarg.h runtime |
| vla | Requires variable-length array runtime |

### Compile Fail (34)

| Test | First Blocker |
|------|---------------|
| alignof | Missing _Alignof keyword |
| arith | Large int literals (overflow 24-bit) |
| asm | GNU asm syntax (extended asm) |
| attribute | Missing __attribute__ keyword |
| bitfield | Missing struct |
| builtin | Missing __builtin functions |
| cast | Statement expressions ({...}) |
| commonsym | Missing static/extern keywords |
| compat | Missing struct |
| complit | Compound literals |
| const | Missing const keyword |
| constexpr | Missing constexpr keyword |
| control | Missing switch/case |
| decl | Missing static/extern keywords |
| enum | Missing enum keyword |
| extern | Missing extern keyword |
| function | Missing static keyword |
| generic | Missing _Generic keyword |
| initializer | Missing struct |
| line | Missing __LINE__ / __FILE__ |
| literal | Large int literals (overflow 24-bit) |
| macro | Missing system headers |
| offsetof | Missing struct |
| pointer | Statement expressions ({...}) |
| pragma-once | Missing system headers |
| sizeof | Struct member access (.) |
| string | String concatenation |
| struct | Missing struct keyword |
| typedef | Missing typedef keyword |
| typeof | Missing typeof keyword |
| unicode | Unicode literals |
| union | Missing union keyword |
| usualconv | Struct member access (.) |
| variable | Statement expressions ({...}) |

### Blockers Fixed

- Ternary operator `? :` -- was blocking 8 tests (arith, control, sizeof, etc.)
- Character literals `'a'`, `'\n'` -- was blocking literal.c, complit.c
- Multi-declaration `int x, y;` -- was blocking variable.c
- Hex literals `0xFF` -- was blocking MMIO patterns
- Logical `&&` / `||` -- was blocking complex conditionals
- `break` / `continue` -- was blocking loop tests
- `++` / `--` -- was blocking for-loop increment patterns
- `sizeof` operator -- was blocking sizeof.c, decl.c, string.c
- `static` / `extern` keywords -- was blocking commonsym.c, compat.c, extern.c
- Statement expressions `({ })` -- was blocking 6 tests
- Compound assignment `+=`, `-=`, `*=`, `/=`, etc. -- was blocking arith.c, control.c
- Test runner adaptation -- strip printf/exit calls, return _test_fail

### Remaining Blockers (by impact)

| Blocker | Tests Affected | Effort |
|---------|---------------|--------|
| `struct` / `union` / `.` access | 10 tests | Large |
| `goto` / labels | 3 tests | Medium |
| `switch` / `case` / `default` | 3 tests | Medium |
| `typedef` / `enum` | 3 tests | Small-Medium |
| Large int literals (>24-bit) | 2 tests | Out of scope (24-bit target) |
| Missing system headers | 6 tests | Out of scope (freestanding) |

## Documentation

| Document | Description |
|----------|-------------|
| [Language Reference](docs/reference.md) | All supported C features with examples |
| [Architecture](docs/architecture.md) | Component layout, data flow, design constraints |
| [Project Status](docs/status.md) | Current progress, test counts, demo list |
| [Future Plan](docs/plan.md) | Planned features and improvements |
| [ISA Summary](docs/isa-summary.md) | COR24 instruction set reference |
| [ABI Proposal](docs/abi-proposal.md) | Calling convention, stack frame layout |
| [Memory Map](docs/memory-map.md) | Address space, MMIO ports, stack location |
| [Assembler Syntax](docs/assembler-syntax.md) | as24 assembly syntax |
| [C Data Model](docs/c-data-model.md) | Type sizes and promotions |
| [Interrupt Plan](docs/interrupt-plan.md) | ISR support design |

### Development

| Document | Description |
|----------|-------------|
| [AI Agent Instructions](docs/ai_agent_instructions.md) | Guidelines for AI coding agents |
| [Development Process](docs/process.md) | TDD workflow and commit conventions |
| [Tools](docs/tools.md) | Recommended development tools |

## Tests

Tests across all components:

```bash
# Run all tests (build-all.sh runs tests too)
scripts/build-all.sh

# Run tests for one component
cargo test --manifest-path components/frontend/Cargo.toml

# Run ignored as24 validation tests (requires service on localhost:7412)
cargo test --manifest-path components/backend/Cargo.toml -- --ignored
```

## License

MIT -- see [LICENSE](LICENSE) for details.

Copyright (c) 2026 Michael A Wright
