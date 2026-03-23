



Looking for more sources of C examples for a free-standing or embedded C compiler under construction.  Are the C programs in the C Puzzle Book or the original K&R C book still valid and relevant for C89/C90 or newer compilers.  Is there a list of such examples documented anywhere?  Also there was a site rosettacode that compared how to solve problems in different programming languages.  Are the C examples there easy to download, compile, and run?  How can we validate that they are correct and our compiler produces correct code that runs ans is correct?  


Thought for 47s
You’re looking for sources of C example programs that are useful for a freestanding / embedded C compiler under construction, and also for a way to turn those examples into a real validation pipeline rather than just a pile of sample code.

High-level view
For this kind of compiler work, I would split sources into four buckets:

Language-learning examples
Good for parser/semantics bring-up, but weak as correctness oracles.

Compiler test suites
Better for conformance and edge cases.

Algorithm / task collections
Good for “real programs,” but often require adaptation and independent output checking.

Generated tests and differential testing
Best for finding miscompilations once the compiler is able to compile and run nontrivial code.

That split matters because a cute C example is not automatically a good compiler test. For a compiler under construction, the most valuable programs are the ones where you can say exactly what behavior is expected, especially under freestanding constraints. 

K&R and The C Puzzle Book: still useful?
K&R
K&R 1st edition (1978) is historically important, but it reflects pre-standard “K&R C”, not standardized C89/C90. By contrast, K&R 2nd edition (1988) was updated to cover the then-new ANSI C standard, so it is much closer to C89/C90. That makes the second edition still useful for a C89-oriented compiler; the first edition is more useful as historical background and for understanding older code you may encounter. 

The C Puzzle Book
The C Puzzle Book can still be useful, but mainly as a source of interesting corner cases and programmer traps, not as a clean correctness oracle. The book itself notes that some outputs differed across machines such as the PDP-11/70 and VAX 11/780, which is a sign that some examples depend on implementation details or non-portable behavior. In modern compiler validation, that means you should treat many puzzle-style examples as diagnostic / exploratory tests, not as pass-fail conformance tests unless you first classify them as well-defined under your chosen subset. 

Practical conclusion
So:

K&R 2nd edition: yes, still relevant for a C89/C90-oriented subset, especially for basic syntax, expressions, declarations, control flow, functions, arrays, pointers, and small library examples. 

K&R 1st edition: useful historically, but not a strong baseline for a new C89/C90 compiler. 

C Puzzle Book: useful as a source of edge cases and “gotchas,” but not automatically valid as a correctness suite. 

Is there a documented list of such examples?
There is no single authoritative public catalog of “all classic K&R/C89 example programs” that functions like a standard conformance list. But there are several practical indexes:

Rosetta Code’s C category is an index of tasks with C solutions. That gives you a large documented collection of small-to-medium examples. 

GCC’s torture tests are a long-running, documented source of compiler tests, including tests that should compile and tests that should compile/link/run under different optimization settings. 

For embedded-specific language extensions, WG14’s Embedded C Technical Report (TR 18037) documents extensions aimed at embedded processors, though that is more about language features than example repositories. 

So the answer is really: yes, partial lists exist, but not one canonical list of “classic sample programs”. In practice you assemble your own corpus from books, compiler test suites, Rosetta Code, and generated tests. 

Is Rosetta Code good for this?
Yes, with caution.

Rosetta Code has a Category:C page listing tasks solved in C, and the site is fundamentally organized around many small, isolated programming tasks. That makes it a good source of programs for:

expressions and control flow

strings and arrays

recursion

data structures

numeric tasks

algorithmic mini-benchmarks

But many tasks are not freestanding-friendly out of the box because they may rely on:

stdio

dynamic allocation

file I/O

floating-point library behavior

command-line arguments

OS services

Also, quality varies because Rosetta Code is community-edited rather than a formal conformance suite. Some tasks even note that examples need review or that a task is draft/clarified. 

Are the C examples easy to download?
Not as a polished bulk-download dataset, at least not in the way a dedicated compiler test suite usually is. But Rosetta Code is MediaWiki-based, and there are task pages and examples showing use of the MediaWiki API / raw page retrieval. That means you can script extraction, but you should expect some cleanup work rather than a ready-made compiler-validation pack. 

Are they easy to compile and run?
Some are. Many are not, without adaptation. For a freestanding compiler, I would assume you will need a curation / normalization step. Rosetta Code is best treated as a harvest source, not as a turnkey validation suite. 

How to validate that programs are correct, and that your compiler generates correct code?
This is the important part.

Recommended validation architecture
1. Define your supported subset first
For a compiler under construction, especially freestanding, define a matrix like this:

Area	Status
Core language syntax	yes/no
Integer types	exact widths?
Pointers	near/full support
struct/union/enum	yes/no
Recursion	yes/no
Function pointers	yes/no
volatile	yes/no
Floating point	yes/no
Hosted library	none / tiny subset
Freestanding entrypoint	custom start symbol / main adapter
In freestanding C, startup/termination and required library surface differ from hosted environments. Embedded C work also often introduces target-specific concerns. So you want every test tagged against the subset it exercises. 

2. Classify programs into tiers
I’d use five tiers:

Tier A — parser/semantic sanity
Tiny single-file tests:

declarations

arithmetic

precedence

if/while/for/switch

pointers

arrays

structs

function calls

These are best sourced from K&R 2nd edition, small hand-written cases, and selected book examples. 

Tier B — well-defined executable examples
Programs whose output is obvious and stable:

Fibonacci

sieve

sorting

CRC/checksum

integer arithmetic identities

fixed-string processing

simple state machines

Rosetta Code can contribute here, but only after curation. 

Tier C — compiler test suites
Use GCC torture tests and similar tests that focus on language/compiler behavior. These are far better than puzzle books for serious validation. 

Tier D — differential testing
Compile the same program with:

your compiler

GCC/Clang targeting a reference machine

Then compare outputs. This is one of the strongest practical techniques once you can avoid undefined behavior. Csmith was built specifically for this style of testing. 

Tier E — metamorphic / fuzz / reduction
Use generators such as Csmith and YARPGen, then reduce failures with test-case reduction tools. This is how modern compiler bugs are found at scale. 

What is the actual oracle?
For compiler validation, there are several oracles, each with pros and cons.

Oracle 1: exact stdout / memory image
Best for curated deterministic programs.

Pros

simple

easy to automate

ideal for freestanding harnesses

Cons

limited coverage

many interesting programs need adaptation

Oracle 2: differential testing
Compile/run the same well-defined source with multiple compilers and compare results.

Pros

very effective

scales well

finds real miscompilations

Cons

only valid if the source avoids undefined / unspecified behavior

reference compilers can also have bugs

Csmith explicitly exists for this model and tries to generate programs free of undefined behavior. 

Oracle 3: property-based checks
Instead of comparing exact output, verify invariants:

sort output is nondecreasing

checksum matches known value

encode/decode round-trips

parser(print(x)) == x

Pros

great for embedded/freestanding

less dependence on I/O formatting

Cons

requires hand-designed checkers

Oracle 4: metamorphic testing
Transform a program or input in a way that should preserve behavior, and verify it still does.

This family includes EMI-style ideas discussed in compiler-testing literature. 

What I would do for your compiler
A practical pipeline
Phase 1 — “classic small C”
Use:

K&R 2nd edition examples

a small subset of C Puzzle Book examples that you manually mark as well-defined

hand-written “expected result” tests

Goal:

parser

type checker

codegen for integers, pointers, control flow

Phase 2 — “curated Rosetta subset”
Build a script that harvests C entries from Rosetta Code, then classify each task:

Green: pure integer / array / string / recursion / no libc except maybe minimal printf

Yellow: needs adaptation

Red: depends on OS/files/floats/threads/large library surface

Then normalize Green tasks into your own repo:

one task per file

expected output file

tags for required features

note whether freestanding shim is needed

Rosetta Code is a good raw source, but you want your own frozen curated mirror. 

Phase 3 — “compiler-grade tests”
Add:

GCC torture tests you can support

hand-selected conformance tests

ABI tests

register allocator stress tests

volatile / sequence-point / aliasing tests, but only after core bring-up

GCC’s torture infrastructure is specifically meant for this sort of stress testing. 

Phase 4 — “generated tests”
Once the compiler can handle enough C:

run Csmith

differential-test against GCC/Clang

reduce failing cases

keep minimized regressions forever

This is where you stop relying on human-written samples alone and start finding real compiler bugs. 

Special issue: freestanding / embedded
For a freestanding compiler, many otherwise good tests assume hosted C. So I would create a tiny test ABI:

custom entrypoint, not necessarily main

memory-mapped output area or semihosting trap

deterministic exit code convention

minimal support library:

memcpy

memset

memcmp

maybe a tiny puts or print_hex

no file I/O

no malloc initially

no locale/time/threads

Then convert tests to one of two styles:

return code only

write result bytes to a known buffer / port

That makes your test harness target-neutral and much easier to automate. Freestanding behavior and startup are implementation-defined, so this custom harness is the right place to pin down your model. 

My direct answers
Are K&R and C Puzzle Book examples still valid?
K&R 2nd edition: largely yes for C89/C90-era core language work. 

K&R 1st edition: historical, not the right baseline for standard C89/C90 validation. 

C Puzzle Book: useful, but many examples should be treated as portability/UB traps, not correctness oracles. 

Is there a documented list?
Not one canonical list of all such classic examples, but there are usable indexes:

Rosetta Code’s C category

GCC torture tests

embedded C documentation via WG14/TR 18037 for extension areas 

Are Rosetta Code C examples easy to download, compile, and run?
Download: scriptable, yes, but not as a polished official test bundle. 

Compile/run: some yes, many need curation/adaptation. 

How do we validate correctness?
Use a layered strategy:

curated deterministic tests

differential testing

property/metamorphic testing

generated tests with Csmith/YARPGen

minimized regression corpus 

Best sources to add next
If your goal is free-standing compiler under construction, my ranking would be:

K&R 2nd edition examples for early bring-up

Your own tiny deterministic tests for each feature

Selected Rosetta Code C tasks, after curation

GCC torture tests, as your support grows

Csmith for automated bug hunting

YARPGen later, for more aggressive compiler fuzzing 

If you want, I can turn this into a concrete validation plan with:

a folder layout,

test tiers,

pass/fail conventions for a freestanding target,

and an initial curated list of about 50–100 example programs to start with.


Are there any public domain PDFs on learning C or on writing compilers that have C samples I can test against?  Is there a subset of ANSI C that is "free standing" and documented/listed/described anywhere?  I want to ensure my test suite is complete


Thought for 1m 58s
You want three things:

Legitimate PDFs you can use as sample sources.

A documented freestanding subset of ANSI/ISO C.

A way to make the suite as complete as practical.

Direct answer
Yes—there are a few legitimate free PDFs that are useful, but most classic C and compiler books are still copyrighted, so random PDFs of K&R, Appel, or similar books are often not something I’d recommend relying on. A safer route is to use public-domain or openly licensed material plus the public WG14/NIST documents for the language definition. 

Also, “freestanding C” is documented, but it is not a separate standard-defined language level beyond hosted vs. freestanding. WG14’s rationale explicitly says that beyond that two-level scheme, no additional subsetting is defined for C. 

And one more important point: there is no official complete validation suite from the C standard itself. WG14’s rationale says proposals for a validation suite were rejected. So “complete” in practice means layered coverage, not a single blessed corpus. 

Good public/open PDFs to start with
For learning C, with lots of sample code
C Handbook (Flavio Copes)
Internet Archive marks it with Public Domain Mark 1.0, and it’s downloadable as PDF. This is a good source of many small tutorial-style examples, though it is more modern tutorial material than a standards-conformance corpus. 

Beej’s Guide to C Programming
The document itself is under CC BY-NC-ND, but Beej explicitly states that the C source code presented in the guide is granted to the public domain. That makes it unusually useful for harvesting example programs legally. 

GNU C Language Introduction and Reference Manual
This is a freely redistributable GNU manual under the GNU Free Documentation License. It is useful as a source of examples, but it describes GNU C, not just ISO C, so for compiler testing you would want to filter out GNU extensions. 

For compiler construction, with C-oriented examples
GNU Bison manual
It is freely redistributable under the GNU Free Documentation License and includes explained example parsers, such as calculator examples, with generated C parsers and C support code. It is more parser-focused than full compiler-focused, but it is a solid source of small language-tool examples. 

Compiler Construction (Waite & Goos)
This one is free to access, but it is not public domain; the PDF itself says “all rights reserved” and that it is made available on the web by permission of the authors for educational purposes. Still useful, but not public-domain. 

A very useful standards document
FIPS PUB 160 / ANSI C adoption document
NIST’s FIPS publication for C publicly announces adoption of ANSI X3.159-1989 as a federal standard and is available as a public PDF. It is useful as a public historical standards anchor for ANSI C / C89. 

What is the freestanding subset?
Standards model
The C standard defines two execution environments:

hosted

freestanding

In a freestanding environment, the startup function name and type are implementation-defined, and any library facilities beyond the required minimum are also implementation-defined. The macro __STDC_HOSTED__ is the standard signal for hosted vs. freestanding. 

Important nuance: C89/C90 vs newer C
For modern C23, the required freestanding surface is larger. C23 requires the fully freestanding headers:

<float.h>, <iso646.h>, <limits.h>, <stdalign.h>, <stdarg.h>, <stdbool.h>, <stddef.h>, <stdint.h>, <stdnoreturn.h>, and <stdbit.h>, with conditional or partial support for parts of <math.h>, <fenv.h>, <stdlib.h>, and <string.h>. 

For ANSI C / C89, the classic freestanding minimum is much smaller: effectively just the facilities in:

<float.h>, <limits.h>, <stdarg.h>, and <stddef.h>. GCC’s standards documentation summarizes that progression clearly across C89, C95, C99, and C11. 

So if your target is a compiler “under construction” and you want a conservative ANSI C freestanding core, that four-header model is the right baseline. 

Is there a standard-defined “freestanding subset list” beyond that?
Not really in the way people often want.

WG14’s rationale is explicit: the committee recognized hosted and freestanding, but did not define further official sub-levels because too many levels would dilute the standard. 

That means you should define your own project subset matrix, for example:

C89 freestanding core

no floating point initially

no library calls except macros/types from required headers

optional hand-supplied intrinsics for memcpy/memset

optional implementation-defined startup ABI

That is the practical way to make the suite enforceable.

Can your test suite be “complete”?
Not in an absolute sense.

The standard committee explicitly rejected shipping a validation suite, so completeness has to be engineered by combining several test families. 

What a practical “complete enough” suite looks like
I’d break it into five layers.

1. Language-core microtests
Very small one-feature programs:

declarations

integer arithmetic

conversions

precedence

control flow

arrays

pointers

structs

enums

function calls

varargs if supported

These should be handwritten and fully deterministic.

2. Curated textbook/tutorial examples
Harvest from:

C Handbook

Beej

GNU C manual, but only ISO-compatible examples

These are good for parser/typechecker/codegen bring-up. 

3. Freestanding profile tests
Programs that use only the headers/facilities allowed by your chosen level:

for C89 freestanding: test only <float.h>, <limits.h>, <stdarg.h>, <stddef.h>

for C23 freestanding: add the wider required surface if you later choose to support it 

4. Negative tests
Programs that must fail:

syntax errors

type errors

invalid initializers

redeclarations

constraint violations

These are essential for a compiler under construction.

5. Differential and semantic tests
For well-defined programs, compare:

parse success/failure

diagnostics class

output

return code

memory image / checksum

against GCC or Clang in the matching language mode where possible.

Best concrete recommendation for you
If your target is freestanding ANSI C / C89-like, I would define the suite like this:

Phase A: “strict C89 freestanding core”
Allowed headers only:

<float.h>

<limits.h>

<stdarg.h>

<stddef.h> 

Allowed language surface:

integers

pointers

arrays

structs/unions/enums

control flow

function calls

old-style declarations only if you choose to support them

no hosted library assumptions

implementation-defined startup allowed

Phase B: “portable sample corpus”
Source examples from:

Beej code examples first

C Handbook examples second

selected Bison examples only where they reduce to ordinary C testable fragments 

Phase C: “completeness matrix”
Track coverage by feature:

lexical

preprocessing

declarators

integer types

expressions

statements

objects/storage duration

initialization

function calls/ABI

diagnostics

implementation-defined behavior

forbidden/unsupported features

That matrix is what will tell you whether the suite is complete for your chosen subset.

Bottom line
Yes, there are legitimate PDFs you can mine:

C Handbook — public-domain-marked, lots of examples. 

Beej’s Guide to C — document restricted, but code is public domain. 

GNU C manual — freely redistributable, but filter out GNU extensions. 

Bison manual — freely redistributable, useful for parser/C examples. 

For the language definition, use:

FIPS PUB 160 as a public ANSI C historical reference. 

WG14 drafts / cppreference for the modern freestanding rules. 

And for “complete”: the standard does not give you an official complete suite, and WG14 explicitly chose not to provide one. You’ll need a layered corpus plus a coverage matrix. 

I can turn this into a starter freestanding C89 test plan with categories, folder layout, and an initial list of 75–100 concrete test cases.





