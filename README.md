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

## Testing

| Test Suite | Pass | Total | Notes |
|-----------|------|-------|-------|
| cc24 demos | 21 | 21 | End-to-end compiler + emulator |
| chibicc-subset | 5 | 5 | Curated subsets of chibicc tests |
| chibicc full | 3 | 41 | generic, pragma-once, stdhdr |
| bgc examples | 1 | 117 | 116 blocked on stdio.h |

See [Testing Status](docs/testing-status.md) for detailed tables and blocker analysis.

```bash
demos/run-demo.sh              # run a single demo
scripts/run-subset-tests.sh    # run chibicc-subset tests
scripts/run-chibicc-tests.sh   # run full chibicc tests
```

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
