# cc24 -- C Compiler for the COR24 FPGA Soft CPU

cc24 is an open-source C compiler targeting the **COR24** (C-Oriented RISC, 24-bit) instruction set architecture. COR24 is an FPGA soft CPU designed by [MakerLisp](https://makerlisp.com) for efficient C code execution. The existing MakerLisp C compiler is proprietary -- cc24 aims to provide an open-source alternative, starting as an educational demo-quality tool and growing from there.

The approach is inspired by [chibicc](https://github.com/rui314/chibicc), a small educational C compiler. cc24 follows the same architecture pattern (recursive-descent parser, direct assembly emission) but is written from scratch in Rust.

## Project Status

**Phase 1 complete** -- the compiler can compile trivial C programs to valid COR24 assembly.

- Phase 0 (specification): Complete -- ISA, ABI, data model, memory map, assembler syntax, and 13 sample assembly programs documented
- Phase 1 (backend skeleton): Complete -- lexer, parser, codegen, golden-file test harness; compiles `int main() { return 42; }`
- Phase 2 (expressions and locals): In progress

## COR24 Architecture at a Glance

- **24-bit** registers and address space (16 MB)
- **8 registers**: r0-r2 (general purpose), fp, sp, z (zero), iv, ir
- **32 instructions**: arithmetic, logic, shift, compare, branch, load/store, stack ops
- **1/2/4-byte** variable-length instruction encoding
- Hardware multiply (24-cycle), no hardware divide
- Little-endian, byte-addressable memory
- UART and GPIO via memory-mapped I/O

## C Data Model

| Type | Size | Notes |
|------|------|-------|
| `char` | 8-bit | signed |
| `short` | 16-bit | |
| `int` | 24-bit | native machine word |
| `long` | 24-bit | same as int (v1) |
| pointer | 24-bit | flat address space |

No `long long`, `float`, or `double` in the initial version.

## Documentation

Specification and design documents are in the `docs/` directory:

| Document | Description |
|----------|-------------|
| [ISA Summary](docs/isa-summary.md) | Complete COR24 instruction set reference |
| [Assembler Syntax](docs/assembler-syntax.md) | Assembly language syntax accepted by as24 |
| [ABI Proposal](docs/abi-proposal.md) | Calling convention, stack frame layout, register roles |
| [C Data Model](docs/c-data-model.md) | Type sizes, alignment, integer promotions |
| [Memory Map](docs/memory-map.md) | Address space layout, MMIO ports, stack location |
| [Toolchain Commands](docs/toolchain-commands.md) | as24, ld24, longlgo, te, and development workflow |
| [Compiler Planning](docs/cor_24_chibicc_planning_doc.md) | Detailed chibicc adaptation plan and phased roadmap |
| [Project Status](docs/status.md) | Current progress, completed phases, next steps |
| [Interrupt Plan](docs/interrupt-plan.md) | Inline asm, __attribute__((interrupt)), ISR support roadmap |
| [Sample Assembly](docs/sample-asm/) | 13 known-good COR24 assembly programs (see below) |

### Research and Process

| Document | Description |
|----------|-------------|
| [Research Notes](docs/research.txt) | Original ChatGPT research on compiler options for COR24 |
| [AI Agent Instructions](docs/ai_agent_instructions.md) | Development process guidelines for AI coding agents |
| [Development Process](docs/process.md) | TDD workflow, pre-commit quality gates, commit conventions |
| [Tools](docs/tools.md) | Recommended development tools |

### Sample Assembly Programs

Each sample has a C equivalent in comments and serves as a golden reference for compiler output:

| Sample | C Feature | Hardware |
|--------|-----------|----------|
| return_const.s | Return constant | -- |
| add.s | Local variables, arithmetic | -- |
| branch.s | if/else conditional | -- |
| loop.s | while loop | -- |
| call.s | Function call with arguments | -- |
| fib.s | Recursion (MakerLisp compiler output) | -- |
| pointer.s | Address-of, pointer dereference | -- |
| globals.s | Global variable in .data section | -- |
| mmio.s | UART TX (polling putc) | UART |
| uart_rx.s | UART RX (polling getc) | UART |
| led.s | LED on/off | GPIO |
| button.s | Read button state | GPIO |
| interrupt.s | UART RX interrupt handler with ISR | UART, interrupt |

## Building

```bash
cargo build
```

## Planned Development Workflow

Once the compiler is functional:

```bash
# Compile C to COR24 assembly
cargo run -- program.c -o program.s

# Assemble (requires as24 from COR24-TB archive)
as24 < program.s | longlgo > program.lgo

# Run on emulator (cor24-rs)
cd ~/github/sw-embed/cor24-rs
cargo run -- run path/to/program.s
```

## Implementation Roadmap

The compiler follows a phased plan based on the [chibicc planning doc](docs/cor_24_chibicc_planning_doc.md):

1. **Phase 1 -- Backend skeleton**: Emit valid `.text`, labels, prologue/epilogue. Compile `return 0;`
2. **Phase 2 -- Expressions and locals**: Integer constants, local variables, arithmetic, if/while/for
3. **Phase 3 -- Calls and globals**: Function calls, argument passing, global variables, string literals
4. **Phase 4 -- Pointers and arrays**: Address-of, dereference, pointer arithmetic, indexed access
5. **Phase 5 -- Refinement**: Diagnostics, peephole optimizations, runtime helpers (divide, etc.)

## MVP Definition

The compiler reaches MVP when it can compile the "Hello" UART demo -- a
program that computes something and prints it over the serial port. This
requires all core features working together:

| # | Test Program | What it proves |
|---|-------------|----------------|
| 1 | `return 42` | Constants, prologue/epilogue (done) |
| 2 | `int a=2; int b=3; return a+b;` | Locals, stack allocation |
| 3 | Binary operators (`+`, `-`, `*`, `&`, `\|`, `<<`) | Arithmetic/logic codegen |
| 4 | `if (1) return 3; else return 4;` | Conditional branches |
| 5 | `while(i<10) i=i+1;` | Loops |
| 6 | `int add(int a,int b){...}` | Function calls, arguments |
| 7 | Recursive fibonacci | Recursion (gold standard: fib.s) |
| 8 | `int *p=&x; return *p;` | Pointers |
| 9 | Global variables | .data section |
| 10 | `*(volatile char*)0xFF0100 = 'H';` | MMIO, pointer casts |
| 11 | `char *s="Hello"; s[0]` | String literals, indexing |
| 12 | Print "Hello\n" over UART | **MVP gate** -- ties everything together |

## Next Steps

1. **Phase 2**: Local variables, binary/unary operators, if/else, while, for
2. **Phase 3**: Function calls, argument passing, globals, string literals, basic `asm("...")`
3. **Phase 4**: Pointers, arrays, pointer casts (enables MMIO)
4. **Phase 5**: Refinement, `__attribute__((interrupt))`, peephole optimizations

## License

MIT -- see [LICENSE](LICENSE) for details.

Copyright (c) 2026 Michael A Wright
