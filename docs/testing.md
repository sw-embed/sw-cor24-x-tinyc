



I want to validate a tiny C compiler for COR24 ISA.  Are there compatibility test suites for different versions of C?  e.g., what are some well-known versions, like "C89" or "C11" and are there test suites for chibicc or other "Tiny" C implementations?  Is there a set of interesting C programs, like Euler problems, or specifically embedded C programs.  I'm looking for a simplified subset of sample C programs that are "no-std" and do not need a crt.so or equivalent.  Are there recognized or documented "levels" of C support?


Thought for 27s
You want a validation strategy for a tiny/freestanding C compiler targeting COR24, with four things in scope:

Which C language versions matter

What compatibility / conformance suites exist

What small “interesting” program sets exist for validation

Whether there are recognized “levels” of C support, especially for bare-metal / no-runtime use

The key architectural point is this: for a tiny COR24 compiler, it is usually a mistake to aim first at “all of C.” A better plan is to define a precise target profile such as “freestanding C89 subset” or “freestanding C99 subset without floats, varargs, VLAs, or libc”, and validate against that profile in layers. The C standard itself distinguishes hosted vs freestanding implementations, and a conforming freestanding implementation only needs to support strictly conforming programs whose library use is limited to the freestanding headers your implementation provides. That is the closest C analogue to “no-std.” 

Recognized C versions
The well-known standard milestones are:

C89 / ANSI C

C90 / ISO C

C95 (amendment to C90)

C99

C11

C17 (mainly bug-fix / maintenance revision)

C23 (current revision, ISO/IEC 9899:2024) 

For a tiny compiler, the practical “tiers” people usually care about are:

C89/C90-like: smallest reasonable baseline

C99-ish: adds many features people expect today, but much harder

C11-ish: often selective, because parts are optional or painful for tiny targets

C23: not realistic as an initial target for a tiny educational compiler unless you intentionally implement only a narrow freestanding subset 

Are there recognized “levels” of C support?
There is no single universal ladder like “Level 1, Level 2, Level 3” baked into ISO C. The standard instead gives you more useful axes:

1) Hosted vs freestanding
This is the biggest formal distinction. For COR24, you almost certainly want freestanding first. In freestanding mode, the entry point and runtime environment are implementation-defined, which fits bare metal, kernels, ROM monitors, and FPGA soft CPUs. 

2) Language revision
You can truthfully say things like:

“freestanding C89 compiler”

“freestanding C99 subset compiler”

“mostly C11 language, minimal freestanding library”

That is how many small compilers are described in practice. For example, public descriptions commonly characterize chibicc as a C11 implementation, 8cc as targeting C11, cparser as C99, and lacc as C89. 

3) Optional features / feature-test macros
From C11 onward, some facilities are optional, and implementations can advertise absence of things like atomics or complex numbers with macros such as __STDC_NO_ATOMICS__ and __STDC_NO_COMPLEX__. That gives you another standards-based way to define your support boundary. 

So, for your COR24 project, a good documented support statement would be something like:

COR24C v0: freestanding C89 core, plus selected C99 integer/header conveniences, no floating point, no hosted runtime, no varargs, no recursion guarantee, no dynamic allocation, implementation-defined startup.

That is much clearer than claiming generic “C support.”

Compatibility and conformance test suites
There are real suites, but they differ in cost, size, and philosophy.

1) Plum Hall Validation Suite
This is one of the classic commercial C validation suites. It is widely used for compiler and library conformance evaluation. It is strong for serious validation, but it is commercial, not lightweight. 

Pros

Serious conformance focus

Well-known name in compiler validation

Cons

Commercial

Heavier than you probably want for an educational COR24 compiler

More useful later than at MVP stage

2) GCC testsuite / c-torture
GCC has longstanding C torture tests and C testsuites including compile/execute categories. These are very valuable as a source of real compiler stress cases. 

Pros

Large body of existing tests

Good at finding parser / optimizer / codegen mistakes

Easy to mine selectively

Cons

Many tests assume a fuller environment than your target may have

A lot of cases are not ideal for a tiny first-pass compiler

Some are designed around GCC infrastructure, not a standalone tiny compiler flow

3) Csmith
Csmith generates random valid C programs free of undefined behavior for differential compiler testing. It is extremely useful once you have a compiler that can compile a modest subset reliably. 

Pros

Great for finding weird parser / semantic / codegen bugs

Excellent once you can cross-check COR24 results against another compiler/interpreter/model

Cons

Not ideal as the very first validation layer

Generated programs are not pedagogical

You need a good oracle strategy, usually differential testing against a reference implementation or expected output

4) Compiler’s own regression suite
Small compilers often rely heavily on their own curated tests. chibicc, 8cc, and similar projects commonly validate themselves with internal regression tests and with the ability to compile substantial real code. Public descriptions of chibicc and 8cc both emphasize C11 support and self-hosting / real-world compilation goals. 

Pros

Directly aligned with the compiler’s actual supported subset

Easy to port to a new backend

Cons

May reflect the frontend’s assumptions more than your COR24 backend’s needs

Often not organized by “freestanding embedded subset”

Are there test suites specifically for chibicc or “tiny C” implementations?
There is no single universal “tiny C compiler test suite” that everyone uses. In practice, people combine:

the compiler’s own regression tests

selected GCC torture tests

Csmith for fuzzing/differential testing

a small set of hand-picked real programs

optionally a commercial conformance suite later 

For chibicc-derived work, the most practical route is usually:

Start from chibicc’s own tests

Add a COR24 backend smoke suite

Add a bare-metal freestanding sample suite

Then add selected GCC torture cases

Then add Csmith once semantics and codegen stabilize 

“No-std” style C: what does that mean in C terms?
In C, the closest concept is freestanding, not “no-std” in the Rust sense. A freestanding implementation does not require a hosted OS environment, and it can provide only the subset of library facilities required for freestanding operation. The startup model is implementation-defined, so you can have your own reset vector, hand-written startup, or direct jump to a known entry symbol. 

For your purposes, the simplest useful subset is probably:

integer types only

pointers

structs/unions

arrays

function calls

if/else, switch, loops

static globals / locals

simple initializers

volatile for MMIO

maybe enums and typedefs

no libc dependency

no heap

no floating point initially

no varargs initially

no complex numbers

no threads / atomics initially 

That subset is enough for a lot of embedded/bare-metal code.

Good validation program categories for COR24
Instead of jumping straight to “real applications,” define a layered corpus.

Layer A: compiler-unit microtests
Tiny one-file programs that exercise one thing only:

precedence / associativity

integer promotions

signed/unsigned comparisons

pointer arithmetic

struct layout

array decay

function calls and returns

recursion depth sanity

switch lowering

volatile loads/stores

local/global/static storage

These are your most important tests early on.

Layer B: freestanding algorithm tests
These avoid libc and runtime assumptions:

Euclid GCD

integer square root

CRC16 / CRC32

fixed-point multiply/divide

small-sort kernels

ring buffer

bitset operations

simple lexer/tokenizer on static strings

tiny state machines

linear congruential RNG

checksum / hash kernels

These are good because they stress control flow, arithmetic, memory addressing, and calling convention without needing printf or filesystem support.

Layer C: MMIO / embedded-style tests
For COR24, these may be more important than textbook programs:

GPIO read/write wrappers using volatile

polling UART TX/RX registers

timer compare loops

interrupt flag clear/set helpers

memory-mapped device struct overlays

fixed-address ROM/RAM copying

startup .data copy / .bss clear helpers

These validate exactly the kinds of code embedded users care about.

Layer D: selected “interesting” benchmark kernels
Project Euler itself is mixed: some problems are nice small integer kernels, but many are more about math tricks than compiler coverage. Better to cherry-pick just a few:

prime sieve

Fibonacci variants

modular arithmetic kernels

integer palindrome / digit-manipulation code

Collatz / loop-heavy integer code

These are useful as demonstrations, not as primary conformance tests.

Best sources of sample programs
For a tiny compiler, I would rank sources like this:

Best fit
your own hand-curated freestanding microtests

selected compiler regression tests

small embedded kernels you write specifically for COR24

Good second tier
selected GCC torture tests that do not depend on hosted runtime

selected chibicc/8cc tests that fit your subset

Later
Csmith-generated programs

larger “real code” self-hosting or library ports

That progression gives much better signal than trying to run random hosted C programs too early. 

Practical recommendation for COR24
Phase 0: declare the support contract
Pick one of these and document it:

Profile	Good for	Risk
Freestanding C89 core	simplest MVP	older language, fewer niceties
Freestanding C99 integer subset	better developer ergonomics	noticeably more implementation work
“chibicc subset for COR24”	faster porting if frontend reused	support boundary can get fuzzy
For a tiny educational compiler, I would start with freestanding C89 core + selected C99 conveniences.

Phase 1: build a four-bucket test suite
Parse-only

Compile-only

Compile + execute on emulator

Compile + compare generated assembly patterns

That last one matters for a new ISA backend because some bugs show up clearly in assembly long before they fail at runtime.

Phase 2: define your “recognized levels”
You asked whether there are documented levels. Since ISO C does not give you a clean numbered ladder, create your own project levels:

L0 Syntax core: expressions, statements, functions, globals

L1 Bare-metal core: pointers, structs, arrays, volatile, MMIO

L2 Embedded utility: enums, typedefs, initializers, separate compilation

L3 Modern extras: stdint.h subset, stdbool.h, inline, designated initializers

L4 Stretch goals: varargs, floats, wider library support

That is not an ISO standard, but it is a very effective engineering and documentation model.

My bottom-line recommendation
For validating a tiny COR24 C compiler, use this stack:

Primary definition: freestanding C89 or small C99 subset

Primary test source: your own curated bare-metal tests

Secondary source: chibicc regression tests adapted to your subset

Third source: selected GCC c-torture tests

Fuzzing later: Csmith

Serious conformance later: Plum Hall if you ever want stronger formal validation 

The most honest standards label for your early compiler is probably:

freestanding C89-compatible compiler for COR24, with selected C99 features

That gives you a defensible scope, good sample-program choices, and a realistic validation path.

If you want, I can turn this into a detailed validation matrix for COR24 with columns like: feature, C version, required? optional? test file, expected codegen pattern, runtime oracle.




