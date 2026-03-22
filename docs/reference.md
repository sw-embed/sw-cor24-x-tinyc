# cc24 Language Reference

Quick reference for the C subset supported by cc24, targeting the COR24 architecture.

## Types

| Type | Size | Description |
|------|------|-------------|
| `int` | 24-bit (3 bytes) | Native machine word, signed |
| `char` | 8-bit (1 byte) | Signed byte |
| `void` | -- | Return type only |
| `int *` | 24-bit | Pointer to int |
| `char *` | 24-bit | Pointer to char |
| `char **` | 24-bit | Pointer to pointer to char |

Pointers can be nested to arbitrary depth (`int **`, `char ***`, etc.).

## Literals

### Integer Literals

```c
int x = 42;         // decimal
int y = 0xFF;        // hexadecimal (0x or 0X prefix)
int z = 0xFF0100;    // 24-bit hex address
```

Small constants (-128 to 127) emit `lc r0,N`. Larger constants emit `la r0,N`.

### String Literals

```c
char *msg = "hello\n";
```

String constants are emitted in .data as `.byte` sequences with a NUL terminator.
Supported escape sequences: `\n`, `\t`, `\r`, `\\`, `\"`, `\0`.

### Character Values

There is no char literal syntax. Use integer values directly:

```c
int ch = 72;     // 'H' == 72
```

## Operators

All operators follow standard C precedence, from lowest to highest:

### Assignment

```c
x = expr;        // simple assignment
a[i] = expr;     // array element assignment
*p = expr;       // pointer dereference assignment
```

### Logical (short-circuit)

```c
a && b           // logical AND -- skips b if a is 0
a || b           // logical OR -- skips b if a is nonzero
```

### Bitwise

```c
a | b            // bitwise OR
a ^ b            // bitwise XOR
a & b            // bitwise AND
```

### Comparison

```c
a == b           // equal
a != b           // not equal
a < b            // less than
a > b            // greater than
a <= b           // less or equal
a >= b           // greater or equal
```

### Shift

```c
a << n           // left shift
a >> n           // right shift (arithmetic)
```

### Arithmetic

```c
a + b            // addition
a - b            // subtraction
a * b            // multiplication (hardware, 24-cycle)
a / b            // division (software __cc24_div)
a % b            // modulo (software __cc24_mod)
```

### Unary

```c
-x               // negate
~x               // bitwise NOT
!x               // logical NOT (0 -> 1, nonzero -> 0)
&x               // address-of (yields pointer to x)
*p               // dereference (reads value at pointer p)
```

## Variable Declarations

### Local Variables

```c
int x = 10;
char c = 65;
int *p = &x;
int arr[4];
char buf[16];
```

All locals are allocated on the stack at function entry. Array elements are
accessed by index.

### Global Variables

```c
int counter = 0;
char flag = 1;
int *port = (int *)0xFF0100;
```

Globals are emitted in the `.data` section. `int` globals use `.word`,
`char` globals use `.byte`. Pointer globals use `.word`.

## Arrays

```c
int a[4];
a[0] = 10;
a[1] = 20;
int x = a[0] + a[1];

char buf[8];
buf[0] = 72;
```

Arrays are stack-allocated for locals. Indexing computes
`base + index * element_size` and loads/stores the appropriate width.

## Pointers

### Address-of and Dereference

```c
int x = 42;
int *p = &x;
int y = *p;       // y == 42
*p = 100;         // x == 100
```

### Pointer Arithmetic

```c
char *p = (char *)0xFF0100;
char *q = p + 5;   // advances by 5 bytes (char is 1 byte)

int *ip = (int *)base;
int *jp = ip + 2;  // advances by 6 bytes (int is 3 bytes)

int diff = jp - ip; // yields 2 (element count, not byte count)
```

Pointer addition scales by element size. Pointer subtraction between two
pointers of the same type divides by element size.

### Casts

```c
char *uart = (char *)0xFF0100;
int *led = (int *)0xFF0000;
```

Cast expressions change the type of a pointer for load/store width selection.

## Control Flow

### if / else

```c
if (x > 0) {
    y = 1;
} else {
    y = 0;
}
```

### while

```c
while (i < 10) {
    i = i + 1;
}
```

### do...while

```c
do {
    x = x - 1;
} while (x > 0);
```

### for

```c
for (i = 0; i < n; i = i + 1) {
    sum = sum + a[i];
}
```

Supports `i++`, `i--`, `++i`, `--i` for increment/decrement.

### break and continue

`break` exits the innermost loop. `continue` jumps to the loop's
next iteration (increment for `for`, condition for `while`/`do...while`).

```c
while (1) {
    if (done) { break; }
    if (skip) { continue; }
    process();
}

for (int i = 0; i < 10; i = i + 1) {
    if (i % 2 == 0) { continue; }  // skip evens
    sum = sum + i;
}
```

## Functions

### Definition and Calls

```c
int add(int a, int b) {
    return a + b;
}

int main() {
    int result = add(3, 4);
    return result;
}
```

Arguments are pushed right-to-left and cleaned up by the caller.
Parameters are accessed at positive offsets from the frame pointer.

### Recursion

```c
int fib(int n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}
```

### void Functions

```c
void uart_putc(int ch) {
    *(char *)0xFF0100 = ch;
    return;
}
```

### Interrupt Service Routines

```c
void __attribute__((interrupt)) uart_isr() {
    int ch = *(char *)0xFF0101;
    // handle received byte
}
```

The `interrupt` attribute causes the compiler to emit a prologue that saves
all registers (r0, r1, r2) and an epilogue that restores them and uses
`rti` instead of the normal return sequence.

## Inline Assembly

```c
asm("ei");                    // enable interrupts
asm("di");                    // disable interrupts
asm("la r0,_uart_isr");      // load ISR address
asm("sw r0,0(iv)");          // set interrupt vector
```

The string is emitted verbatim into the assembly output. One instruction per
`asm()` call is recommended.

## Preprocessor

### #define

```c
#define LED_ADDR 0xFF0000
#define UART_TX  0xFF0100

int *led = (int *)LED_ADDR;
```

Simple constant substitution. No function-like macros. Substitution does not
occur inside string literals.

### #include

```c
#include "demo10_io.h"       // relative to source file directory
#include <cor24.h>           // searches -I directories
```

Use the `-I dir` flag to specify system include paths.

### #pragma once

```c
#pragma once
// header contents -- included at most once
```

## Startup and Runtime

The compiler emits a `_start` label that calls `_main` and halts on return:

```
_start:
    la r0,_main
    jal r1,(r0)
    halt
```

### Software Division

Division and modulo use runtime helper functions emitted automatically
when `/` or `%` are used:

- `__cc24_div` -- signed 24-bit division
- `__cc24_mod` -- signed 24-bit modulo

## Memory-Mapped I/O (COR24-TB)

### LED (D2, active-low)

```c
int *led = (int *)0xFF0000;
*led = 0;         // LED on (active low)
*led = 1;         // LED off
```

### UART TX

```c
void putc(int ch) {
    *(char *)0xFF0100 = ch;
}
```

### UART RX

```c
int getc() {
    return *(char *)0xFF0101;
}
```

### Interrupt Enable

```c
int *ie = (int *)0xFF0010;
*ie = 1;          // enable interrupts
```

Or using inline assembly:

```c
asm("ei");
```

## CLI Usage

```
cc24 <input.c> [-o output.s] [-I dir]
```

- `<input.c>` -- C source file to compile
- `-o output.s` -- output assembly file (default: stdout)
- `-I dir` -- add include search path (repeatable)

## Known Limitations

- No `switch`/`case`
- No `+=`, `-=`, or other compound assignment
- No `typedef`, `enum`, `struct`, `union`
- No function prototypes (forward declarations)
- `static` local variables are treated as regular locals (not persisted
  across calls). File-scope `static` and `static` functions work correctly
  for single-translation-unit compilation.
- `extern` is accepted and ignored (single translation unit).
- `sizeof(expr)` returns 3 (int size) for all expressions; only
  `sizeof(type)` returns accurate sizes.
- No multi-file compilation (single translation unit)
- No `float` or `double`
- Branch range limited to signed 8-bit offset (~127 bytes)
- No optimization passes
