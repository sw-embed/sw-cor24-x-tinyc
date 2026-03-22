# COR24 + chibicc Planning Document

## Purpose

This document lays out a practical plan for creating a **demo-quality C-to-COR24 compiler path** by adapting **chibicc** to emit `cor24.s` directly. The goal is educational and incremental: get a small but real subset of C compiling to your FPGA soft CPU without first building a production-grade toolchain.

This plan is designed to answer one central question:

**What exactly do I need to provide in order for a chibicc-based backend effort to proceed efficiently?**

---

# 1. Project Goal and Scope

## 1.1 Primary goal

Create a modified version of chibicc that can:

- parse a useful subset of C
- lower it to COR24 assembly text
- support enough ABI and runtime behavior to run simple demo programs
- produce understandable, inspectable assembly for learning and debugging

## 1.2 Non-goals for the first version

The first version should **not** attempt to be:

- production-ready
- fully optimizing
- GCC-compatible
- libc-compatible in any serious way
- ELF/object-format aware unless you already need that
- complete for the C language

## 1.3 First milestone

The first successful milestone should be:

- compile simple freestanding C into `cor24.s`
- assemble and run on the COR24 environment
- support simple integer expressions, locals, functions, branches, loops, pointer dereference, and memory-mapped I/O helpers

---

# 2. Recommended High-Level Architecture

## 2.1 Architecture choice

The cleanest educational approach is:

**C source -> chibicc frontend + parser + type system -> custom COR24 code generator -> `cor24.s`**

This avoids translating from another ISA and gives full control over the emitted assembly.

## 2.2 Why chibicc

chibicc is a good fit because:

- it is small enough to understand
- it already handles parsing, typing, AST construction, and much of the front-end work
- its code generation is simple enough to replace or adapt
- it is intentionally educational rather than industrial

## 2.3 Likely internal strategy

There are two practical implementation patterns:

### Pattern A: Replace the existing backend directly

- fork chibicc
- keep most parsing/type logic intact
- replace or heavily edit codegen to emit COR24 assembly

### Pattern B: Add a new target mode

- keep existing codegen for reference
- add a new `codegen_cor24.c` or equivalent
- switch backend with a flag or build config

For learning and maintainability, **Pattern B** is better.

---

# 3. What You Need to Provide

This is the most important section.

In order to design or implement a COR24 backend, the missing information is not mostly about C syntax. It is mostly about the **machine model**, **ABI**, and **toolchain boundary**.

## 3.1 Required deliverables from you

You should provide the following, ideally as separate text files.

### A. ISA summary

A concise but precise summary of the COR24 ISA.

Minimum needed:

- register names and count
- register width
- special registers (PC, SP, flags, link register, etc.)
- instruction formats
- arithmetic instructions
- logical instructions
- compare/test instructions
- branch/jump/call/return instructions
- load/store instructions
- immediate forms and immediate size limits
- shift instructions
- sign-extension / zero-extension instructions, if any
- multiply/divide support, if any
- byte/halfword/word access rules
- endianness
- alignment requirements
- stack-related instructions, if any

Best format:

- markdown file plus one or two short code examples
- PDF/manual excerpt is also useful if available

### B. Assembly syntax reference

Describe the exact syntax expected by your assembler.

Need to know:

- comment syntax
- label syntax
- directive syntax (`.text`, `.data`, `.word`, etc.)
- numeric literal formats
- string literal support in assembly
- symbol visibility rules
- function label conventions
- how addresses are materialized
- how constants are emitted
- how uninitialized storage is declared

If you already have sample handwritten COR24 assembly that assembles successfully, that is ideal.

### C. Calling convention / ABI proposal

This is essential. Even for a demo compiler, this must be decided early.

Need to know or decide:

- where function arguments go
- where return values go
- which registers are caller-saved
- which registers are callee-saved
- whether there is a frame pointer
- stack growth direction
- stack alignment rule
- how local variables are laid out
- how temporaries spill to stack
- how structs would be passed later, even if postponed now
- how pointers are represented
- whether code/data share one address space
- whether function pointers and data pointers are same width and format

If you do not already have an ABI, that is okay. But you need to choose one.

### D. C data model choice

You need to decide what C types mean on COR24.

At minimum:

- `char` size and signedness
- `short` size
- `int` size
- `long` size
- `long long` support or no support initially
- pointer size
- enum size rule
- `_Bool` size
- alignment rules for each type

Suggested initial model for simplicity:

- `char` = 8 bits
- `short` = 16 bits
- `int` = 24 bits or 32 bits (must choose)
- `long` = same as `int` initially, or 32 bits if you want more C compatibility
- pointer = 24 bits

This choice affects nearly everything in codegen.

### E. Memory model

Need a map or summary of how memory works.

Please provide:

- RAM address range
- ROM/flash/code range
- memory-mapped I/O ranges
- whether loads/stores are byte-addressed or word-addressed
- whether unaligned access is legal
- whether code can read constant data from program space like ordinary memory
- startup/reset execution address
- stack default location

### F. Runtime environment

Need to know what the generated program runs under.

Please describe:

- bare metal or monitor or simulator
- who sets up stack pointer
- whether there is startup code already
- whether global data is copied/zeroed before `main`
- whether there is an existing linker script or fixed memory layout
- how a program exits, halts, or returns control
- whether there is any existing C runtime support from the proprietary compiler

### G. Toolchain interface

Please provide the exact tools and commands currently used for COR24.

Need:

- assembler name and invocation
- linker name and invocation
- object format, if any
- whether you assemble from plain text directly to memory image
- how symbols are resolved
- how programs are loaded into the FPGA environment
- whether there is a simulator/test runner
- whether there is a disassembler

### H. Small assembly test corpus

Please provide a small set of known-good COR24 assembly programs.

Ideal minimum set:

1. arithmetic-only example
2. simple function call example
3. loop example
4. stack-using example
5. memory load/store example
6. conditional branch example
7. memory-mapped I/O example
8. global/static data example

This will make backend design dramatically easier.

### I. Existing MSP430-to-COR24 translation notes (optional but very helpful)

Since you already translate MSP430-flavored assembly into COR24, any notes you have on instruction mapping are valuable.

Please provide:

- which MSP430 constructs map cleanly to COR24
- which constructs are painful
- known compiler codegen patterns you already handle
- examples of pass-through comments or inline opcode tricks

This can speed up backend design because it exposes natural code templates.

---

# 4. Recommended Initial Feature Set

To keep the project achievable, the first version should support only a subset of C.

## 4.1 Recommended supported subset

### Expressions

- integer literals
- local variable references
- global variable references
- unary `-`, `~`, `!`, `&`, `*`
- binary `+`, `-`, `*` if hardware allows or helper routine exists
- binary `&`, `|`, `^`
- shifts `<<`, `>>`
- comparison operators
- assignment
- comma operator optionally later

### Statements

- expression statement
- `if`
- `while`
- `for`
- `return`
- block statements

### Functions

- ordinary function definitions
- function calls with small fixed number of arguments
- recursion optional, not required first

### Types

- integer types
- pointers
- one-dimensional arrays
- no structs/unions initially
- no floating point
- no varargs
- no bitfields

### Storage

- locals
- globals
- string literals
- simple static storage

## 4.2 Features to defer

Defer these until basic execution works:

- `switch`
- `goto`
- structs/unions
- initializer complexity
- 64-bit arithmetic
- floating point
- variadic functions
- function pointers if awkward
- preprocessor completeness beyond what chibicc already supports

---

# 5. Backend Design Choices You Will Need to Make

## 5.1 Register allocation strategy

For the first version, use a **simple register discipline** rather than trying to build a sophisticated allocator.

Recommended approach:

- reserve a few registers for expression evaluation
- reserve one register as stack pointer
- optionally reserve one as frame pointer
- spill aggressively to stack if needed

The first version should favor correctness and clarity over performance.

## 5.2 Expression evaluation model

Two common choices:

### Choice A: stack-machine-like lowering

- evaluate subexpressions one at a time
- push intermediate values to stack
- simple but verbose

### Choice B: fixed temporary register discipline

- use a small number of temp registers for left/right operands
- spill when nesting is deep
- better assembly, modest complexity

Recommended: **Choice B with frequent spilling**.

## 5.3 Prologue/epilogue format

You need a standard function frame layout.

Typical questions:

- save return address manually or use link register?
- save callee-saved regs always or lazily?
- frame pointer mandatory or optional?
- locals addressed off SP or FP?

Recommended initial design:

- always create a frame for non-leaf functions
- use FP if it makes codegen simpler
- treat leaf optimization as future work

## 5.4 Handling globals and literals

Need conventions for:

- string literal placement
- read-only versus writable data sections
- global symbol naming
- relocation model or absolute-address model

For a demo, it is acceptable to use a very simple symbol model if your assembler/linker tolerates it.

---

# 6. Practical Work Breakdown

## Phase 0 -- Feasibility and specification

Goal:

- define enough of COR24 to make code generation deterministic

Tasks:

- collect ISA summary
- collect assembler syntax
- decide C data model
- decide ABI
- prepare 5-10 golden assembly examples

Exit criteria:

- enough information exists to hand-compile simple C functions into COR24 assembly consistently

## Phase 1 -- Backend skeleton

Goal:

- produce valid assembly file structure

Tasks:

- fork chibicc
- isolate existing codegen entry points
- add a COR24 target switch or file
- emit `.text`, labels, function labels, simple prologue/epilogue
- compile empty `main` and `return 0`

Exit criteria:

- can compile trivial function to syntactically valid COR24 assembly

## Phase 2 -- Integer expressions and locals

Goal:

- support computation inside one function

Tasks:

- local stack slots
- integer constants
- local loads/stores
- add/sub/bitwise/compare
- unary operators
- if/while/for lowering

Exit criteria:

- can compile and run arithmetic/loop test programs

## Phase 3 -- Calls and globals

Goal:

- support multi-function programs

Tasks:

- call sequence
- argument passing
- return value handling
- global variable emission
- string literals

Exit criteria:

- can compile small multi-function demo programs

## Phase 4 -- Pointers and arrays

Goal:

- support low-level C idioms useful on embedded systems

Tasks:

- address-of and dereference
- pointer arithmetic
- array decay
- indexed load/store patterns

Exit criteria:

- can compile memory-mapped I/O helpers and simple array code

## Phase 5 -- Refinement

Goal:

- make the tool pleasant enough for demos

Tasks:

- better diagnostics
- cleaner assembly formatting
- optional peephole cleanups
- helper runtime routines for multiply/divide if needed
- optional inline assembly or builtins for COR24-specific features

---

# 7. Test-Driven Development Plan

## 7.1 Why TDD matters here

Compiler work becomes unmanageable without a growing corpus of tiny tests. The risk is not just parser bugs; it is silent miscompilation.

## 7.2 Recommended test layers

### Layer 1: AST/codegen unit-style checks

- compile tiny snippets
- inspect emitted assembly text
- verify labels, directives, instruction patterns

### Layer 2: golden assembly comparison

- for tiny inputs, compare output to expected assembly skeletons
- allow some flexibility for generated label names

### Layer 3: execution tests

- compile C source
- assemble/load/run
- capture memory/register outputs
- verify expected results

## 7.3 First 15 suggested test programs

1. `int main() { return 0; }`
2. `int main() { return 42; }`
3. `int main() { int a=2; int b=3; return a+b; }`
4. `int main() { int a=7; a=a-2; return a; }`
5. `int main() { if (1) return 3; else return 4; }`
6. `int main() { int i=0; while (i<5) i=i+1; return i; }`
7. `int add(int a,int b){ return a+b; } int main(){ return add(2,5); }`
8. `int main() { int x=1; int *p=&x; return *p; }`
9. `int main() { int a[3]; a[0]=4; return a[0]; }`
10. global variable read/write test
11. string literal address test
12. nested expression test
13. comparison operators test
14. function with 3+ args test
15. memory-mapped I/O intrinsic/builtin test

---

# 8. Questions That Must Be Answered Early

These are the decisions most likely to block progress.

## 8.1 Is `int` 24-bit or 32-bit?

This is probably the single biggest language-model question.

### 24-bit `int`
Pros:

- maps naturally to the machine
- simpler arithmetic and storage
- educationally elegant

Cons:

- less standard in hosted-C expectations
- may expose odd edge cases in chibicc assumptions
- might require more edits if code assumes 32-bit `int`

### 32-bit `int`
Pros:

- more conventional C behavior
- may align better with frontend assumptions

Cons:

- more expensive runtime and codegen
- every arithmetic op becomes multiword or helper-based

For a demo compiler, **24-bit `int` may be the most honest and practical choice**, unless chibicc internals make that too awkward.

## 8.2 Are pointers 24 bits?

Probably yes, but confirm.

Need to know whether:

- all memory is addressable with 24-bit pointers
- code and data use the same pointer representation
- function pointers need special treatment

## 8.3 Does COR24 support byte-addressed memory?

If yes, life is easier for C.

If no, then `char *` semantics and string handling become more complex and need explicit policy decisions.

## 8.4 Are there compare instructions or only subtract-and-branch patterns?

This shapes how conditional code generation should be written.

## 8.5 Is multiply/divide native or runtime-assisted?

If absent in hardware, plan helper routines and constrain initial test programs.

---

# 9. Suggested File Bundle You Should Prepare

To move this project forward efficiently, create a folder like this:

```text
cor24-compiler-inputs/
  README.md
  isa-summary.md
  assembler-syntax.md
  abi-proposal.md
  c-data-model.md
  memory-map.md
  toolchain-commands.md
  sample-asm/
    add.s
    call.s
    loop.s
    branch.s
    globals.s
    mmio.s
  optional/
    proprietary-compiler-output/
    msp430-to-cor24-notes.md
    translated-examples/
```

## 9.1 What each file should contain

### `README.md`

- one-paragraph overview
- where the docs came from
- which things are authoritative versus guessed

### `isa-summary.md`

- instruction list
- encoding notes if available
- examples

### `assembler-syntax.md`

- exact syntax rules with examples

### `abi-proposal.md`

- function call and frame conventions

### `c-data-model.md`

- sizes, alignments, signedness, pointer model

### `memory-map.md`

- code/data/stack/MMIO layout

### `toolchain-commands.md`

- exact commands you run today for assemble, link, load, test

### `sample-asm/*`

- minimal working assembly snippets

---

# 10. Risks and Likely Pain Points

## 10.1 Frontend assumptions versus 24-bit machine reality

Many compiler components are written assuming 8/16/32/64-bit-friendly targets. A 24-bit machine is unusual enough that type sizes, alignment, and constant handling need careful review.

## 10.2 C semantics around promotions

Even a tiny compiler quickly runs into:

- integer promotions
- signed versus unsigned compares
- shift behavior
- pointer arithmetic scaling

You do not need perfect standards compliance for a demo, but you do need consistency.

## 10.3 Stack frame bugs

Most early compiler failures are ABI/frame/layout bugs, not parser issues.

## 10.4 Global initialization

If there is no runtime startup code, then even simple globals and string literals require explicit placement and startup behavior decisions.

## 10.5 Toolchain mismatch

If chibicc emits nice assembly but your assembler/linker expects odd directives or symbol conventions, progress can stall. That is why working assembly samples matter so much.

---

# 11. Recommended Next Step Sequence

## Immediate next steps for you

1. Prepare the COR24 input bundle described above.
2. Decide whether `int` should be 24-bit or 32-bit for version 1.
3. Decide whether pointer size is 24 bits.
4. Write down a provisional calling convention.
5. Provide 5-10 known-good assembly files.

## Immediate next steps for implementation after that

1. inspect chibicc codegen structure
2. define an internal target description for COR24
3. emit function skeletons and constant returns
4. add locals and arithmetic
5. add branches and loops
6. add function calls
7. add globals and pointers

---

# 12. What Would Make This Much Easier

The single most helpful things you could provide are:

- a short COR24 ISA summary written in your own words
- three to ten tiny assembly examples that definitely assemble and run
- a clear statement of register roles and stack behavior
- a decision on the C type model

Those four items alone would remove most ambiguity.

---

# 13. Minimal Questionnaire

If you want a compact checklist instead of the full document, answer these questions:

1. How many general-purpose registers does COR24 have, and what are their names?
2. Which register is SP, PC, FP, LR, or equivalent?
3. Is memory byte-addressed?
4. Is endianness little or big?
5. What load/store widths exist?
6. What immediate widths exist?
7. How are calls and returns implemented?
8. Does hardware support multiply/divide?
9. What is the stack growth direction?
10. Which registers must a callee preserve?
11. What sizes should `char`, `short`, `int`, `long`, and pointers have?
12. How does assembly declare code, data, constants, strings, and labels?
13. What commands assemble and load a program today?
14. What does a minimal hand-written `main` look like in working COR24 assembly?
15. How should a program terminate?

---

# 14. Final Recommendation

The best way to make a chibicc-based COR24 path succeed is to treat this as three mini-projects:

1. **Machine specification project** -- define ISA, ABI, data model, assembly syntax.
2. **Backend skeleton project** -- teach chibicc to emit basic COR24 function assembly.
3. **Validation project** -- grow a test suite of tiny C programs and known-good execution results.

If you provide the machine-specification inputs cleanly, the rest becomes a manageable compiler-learning exercise rather than an open-ended reverse-engineering task.

