# cc24 -- C Compiler for the COR24 FPGA Soft CPU

cc24 is an open-source C compiler targeting the **COR24** (C-Oriented RISC, 24-bit) instruction set architecture. COR24 is an FPGA soft CPU designed by [MakerLisp](https://makerlisp.com) for efficient C code execution. The existing MakerLisp C compiler is proprietary -- cc24 provides an open-source alternative.

The approach is inspired by [chibicc](https://github.com/rui314/chibicc), a small educational C compiler. cc24 follows the same architecture pattern (recursive-descent parser, direct assembly emission) but is written from scratch in Rust.

## Project Status

The compiler is **functional** -- it compiles real C programs to COR24 assembly that runs on hardware and the cor24-rs emulator. 12 working demos exercise all implemented features including functions, recursion, pointers, arrays, MMIO, interrupts, and a preprocessor.

See [docs/status.md](docs/status.md) for detailed status and test counts.

### What Works

- Types: int (24-bit), char (8-bit), void, pointers, arrays
- All standard C operators with correct precedence
- Control flow: if/else, while, do...while, for
- Functions with multiple parameters, recursion, ISR support
- Globals, string constants, hex literals
- Pointer arithmetic with element-size scaling
- Preprocessor: #define, #include, #pragma once
- Inline assembly: asm("...")
- MMIO: LED, UART TX/RX, interrupt enable
- Software division and modulo runtime

### What Does Not Work Yet

- switch/case, break, continue
- ++, --, +=, and other compound assignment
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

The compiler is organized into 4 components with 14 crates:

```
components/
  core/           -- shared types (AST, tokens, spans, errors)
    cc24-ast
    cc24-error
    cc24-span
    cc24-token
  frontend/       -- preprocessing, lexing, parsing
    cc24-lexer
    cc24-lexer-tests
    cc24-parser
    cc24-parser-tests
    cc24-parse-stream
    cc24-preprocess
    cc24-preprocess-tests
  backend/        -- code generation and validation
    cc24-codegen
    cc24-codegen-tests
    cc24-codegen-validate
  cli/            -- compiler binary
    cc24
```

Each component is its own Cargo workspace. See [docs/architecture.md](docs/architecture.md) for data flow and design constraints.

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

12 demos in `demos/`, each with a run script:

| Demo | Features |
|------|----------|
| demo.c | Globals, function calls, recursion, if/else, while, for |
| demo2.c | char, pointers, casts, MMIO (LED, UART TX) |
| demo3.c | Hex literals, pointer arithmetic, string constants |
| demo4.c | Software division and modulo |
| demo5.c | Arrays (declaration and indexing) |
| demo6.c | Global char/pointer, .byte/.word emission |
| demo7.c | Pointer subtraction with scaling |
| demo8.c | Preprocessor #define |
| demo9.c | Interrupt attribute, ISR, UART RX interrupt |
| demo10.c | #include, #pragma once, -I flag |
| demo11.c | Logical && and || with short-circuit |
| demo12.c | do...while loop |

Run a demo:

```bash
demos/run-demo.sh     # runs demo.c through cc24 and cor24-rs
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

54 active tests across all components:

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
