# cc24 Future Plan

Planned features and improvements, roughly in priority order.

## Language Features

### Increment and Decrement (++/--)

Add `i++`, `i--`, `++i`, `--i` as both expressions and statements. These are
the most common missing operators -- every demo currently writes `i = i + 1`.

### Compound Assignment (+=, -=, etc.)

Add `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`. These
desugar to `x = x op expr` with x evaluated once.

### sizeof

`sizeof(type)` and `sizeof(expr)`. Returns the size in bytes: 3 for int and
pointers, 1 for char, N*element_size for arrays. Evaluated at compile time.

### switch/case

```c
switch (x) {
    case 0: ... break;
    case 1: ... break;
    default: ...
}
```

Requires `break` statement support. Implementation: chain of compare-and-branch
instructions, with fallthrough between cases.

### break and continue

`break` exits the innermost loop or switch. `continue` jumps to the loop
condition check. Requires tracking the current loop's exit and continue labels
on a stack during codegen.

### struct

```c
struct point { int x; int y; };
struct point p;
p.x = 10;
int val = p.y;
```

Requires: field offset computation, member access (dot and arrow), struct size
for stack allocation, passing structs by pointer.

### typedef and enum

```c
typedef unsigned char uint8;
enum color { RED, GREEN, BLUE };
```

typedef maps names to types. enum defines integer constants starting at 0.

### Function Prototypes

Forward declarations to allow mutual recursion and out-of-order function
definitions:

```c
int foo(int n);  // prototype
int bar(int n) { return foo(n - 1); }
int foo(int n) { if (n == 0) return 1; return bar(n); }
```

## Code Generation Improvements

### Long Branches

Currently branches are limited to signed 8-bit offset (~127 bytes). Long
branches would emit a `bra` over a `la`+`jmp` sequence when the target is
out of range. This requires either a two-pass approach or conservative
always-long emission with a peephole pass to shorten.

### Chain-of-Responsibility for Binop Dispatch

The current binop codegen is a large match block in `binop.rs`. Refactor to
a chain-of-responsibility pattern where each handler (arithmetic, bitwise,
comparison, logical, pointer) checks whether it handles the operation and
either processes it or passes to the next handler. This would:

- Keep each handler focused on one category
- Make it easy to add new operator types
- Follow the existing <5 functions/module constraint
- Allow handlers to be tested independently

### Optimization Passes

No optimization passes exist currently. Potential passes:

- **Constant folding**: evaluate `3 + 4` at compile time
- **Dead store elimination**: remove stores that are immediately overwritten
- **Peephole**: replace `push r0 / lc r0,N / pop r1 / add r0,r1` with
  `add r0,N` for small constants
- **Redundant load elimination**: avoid reloading a value that is already in
  a register

### Source Line Comments

Emit source file and line number comments in the assembly output, matching
the MakerLisp convention:

```
; line 7, file "fib.c"
```

Useful for debugging generated assembly.

## Multi-file Compilation

Currently the compiler handles a single translation unit. Multi-file support
would require:

- Separate compilation to .o (object) files
- A linker (ld24) to combine them
- External symbol resolution
- Separate .data and .text sections per file

This is a large effort and low priority compared to language features.

## Diagnostics

- Better error messages with source line display and caret pointing to the error
- Warning for unused variables
- Warning for missing return statement in non-void functions
- Error recovery in the parser to report multiple errors per compilation
