# COR24 Assembler Syntax Reference

This document describes the assembly language syntax accepted by the COR24
assembler (`as24`) and used in MakerLisp's C compiler output.

## Source Format

Assembly source is plain text. Each line contains at most one instruction
or directive. Blank lines are ignored.

## Comments

Semicolon (`;`) starts a comment that extends to end of line.

```asm
        lc      r0,42           ; load the answer
; this entire line is a comment
```

The MakerLisp C compiler also emits source-line annotations as comments:

```asm
; line 7, file "fib.c"
```

## Labels

Labels end with a colon (`:`) and define a symbol at the current address.

### Global Labels

```asm
_main:
_fib:
putc:
halt:
```

Convention from MakerLisp C compiler: C function names are prefixed with
underscore (`_main`, `_fib`, `_printf`).

### Local Labels

Local labels start with a dot (`.`) and are scoped to the enclosing
global label:

```asm
print_num:
        push    r1
.div:
        lc      r2,10
        clu     r0,r2
        brt     .ones
        sub     r0,r2
        add     r1,1
        bra     .div
.ones:
        ; ...
```

The MakerLisp C compiler uses `L` prefixed labels for compiler-generated
targets (these are essentially local but without the dot):

```asm
L17:
L16:
L20:
```

## Instructions

### General Format

```
        mnemonic    operands        ; optional comment
```

Instructions are indented (convention: 8-space tab). Labels are not indented
(or at column 0).

### Register Operands

Register names: `r0`, `r1`, `r2`, `fp`, `sp`, `z`, `iv`, `ir`

Alternative names used in some contexts:
- `r3` = `fp`
- `r4` = `sp`
- `r5` = `z`
- `r6` = `iv`
- `r7` = `ir`
- `c` = condition flag (in `mov ra,c` context)

### Immediate Operands

Decimal:
```asm
        lc      r0,42           ; positive
        lc      r0,-1           ; negative (sign-extended to 0xFFFFFF)
        add     r0,10           ; 8-bit signed immediate
```

Hexadecimal:
```asm
        la      r0,0xFF0100     ; 24-bit hex address
        lcu     r1,0xDF         ; 8-bit unsigned hex
```

Negative addresses via signed interpretation:
```asm
        la      r1,-65280       ; equivalent to 0xFF0100 (two's complement)
```

Character values as decimal ASCII codes:
```asm
        lc      r0,72           ; 'H' = 0x48
        lc      r0,10           ; '\n' = 0x0A
```

### Memory Operands (Base + Offset)

Format: `offset(base_register)`

```asm
        lw      r0,9(fp)        ; load word at fp+9
        sw      r0,-3(fp)       ; store word at fp-3
        lb      r0,0(r1)        ; load byte at r1+0
        sb      r0,1(r2)        ; store byte at r2+1
        lw      r0,(r2)         ; offset 0 can be omitted in some assemblers
```

Offset is 8-bit signed: -128 to +127.
Valid base registers: r0, r1, r2, fp.

### Indirect Register (Jump/Call)

```asm
        jmp     (r1)            ; jump to address in r1
        jal     r1,(r0)         ; call: save return addr in r1, jump to (r0)
        jal     r1,(r2)         ; call: target in r2
```

## Directives

### Section Directives

```asm
        .text                   ; switch to code section
        .data                   ; switch to data section
```

### Symbol Visibility

```asm
        .globl  _main           ; make symbol globally visible
        .globl  _fib
```

### Data Emission

```asm
        .byte   70,105,98       ; emit bytes (decimal values)
        .byte   0               ; null terminator
        .word   1000            ; emit 24-bit word
        .word   100
```

String literals are emitted as sequences of `.byte` values:

```asm
; "Fibonacci 33\n" becomes:
L20:
        .byte   70,105,98,111,110,97,99,99
        .byte   105,32,51,51,10,0
```

### Uninitialized Storage (BSS)

```asm
        .comm   _flags,8191     ; reserve 8191 bytes of uninitialized storage
```

## Complete Example: MakerLisp C Compiler Output

This is actual output from MakerLisp's COR24 C compiler for a fibonacci
function, showing all syntax conventions:

```asm
        .text

        .globl  _fib
_fib:
        push    fp
        push    r2
        push    r1
        mov     fp,sp
        add     sp,-3
        lw      r2,9(fp)
; line 7, file "fib.c"
        lc      r0,2
        cls     r2,r0
        brf     L17
; line 8, file "fib.c"
        lc      r0,1
        bra     L16
L17:
; line 11, file "fib.c"
        mov     r0,r2
        add     r0,-1
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        sw      r0,-3(fp)
        mov     r0,r2
        add     r0,-2
        push    r0
        la      r0,_fib
        jal     r1,(r0)
        add     sp,3
        lw      r1,-3(fp)
        add     r0,r1
L16:
        mov     sp,fp
        pop     r1
        pop     r2
        pop     fp
        jmp     (r1)

        .data
L20:
        .byte   70,105,98,111,110,97,99,99
        .byte   105,32,51,51,10,0
```

## Assembler Tool

The reference assembler is `as24`, a simple C program from MakerLisp.

### Usage

```bash
as24 -l < program.s              # assemble with listing output
as24 < program.s | longlgo       # assemble and create load-and-go file
```

The assembler reads from stdin and writes binary or listing to stdout.

### Listing Format

The `-l` flag produces a listing showing address, hex bytes, and source:

```
000000 80              push    fp
000001 7f              push    r2
000002 7e              push    r1
000003 65              mov     fp,sp
000004 29 00 00 00     la      r0,__iob
000008 09 06           add     r0,6
00000a 7d              push    r0
```

## Key Syntax Patterns for C Compiler Code Generation

### Function Call

```asm
        push    r0              ; push argument (right-to-left)
        la      r0,_target      ; load function address
        jal     r1,(r0)         ; call (return addr saved in r1)
        add     sp,3            ; caller cleans up 1 argument (3 bytes)
```

### Loading Constants

```asm
        lc      r0,42           ; -128..127 (sign-extended)
        lcu     r0,200          ; 0..255 (zero-extended)
        la      r0,1000         ; any 24-bit value (4-byte instruction)
```

### Conditional Branch

```asm
        ceq     r0,z            ; test r0 == 0
        brt     is_zero         ; branch if true
        ; fall through if r0 != 0
```

### Infinite Halt Loop

```asm
halt:
        bra     halt            ; spin forever
```
