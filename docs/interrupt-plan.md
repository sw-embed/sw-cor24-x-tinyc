# COR24 Interrupt and Inline Assembly Plan

## COR24 Interrupt Hardware

COR24 has a simple interrupt model:

- One interrupt source: UART RX data ready
- **iv (r6)**: holds ISR address (set by software)
- **ir (r7)**: holds return address (set by CPU on interrupt entry)
- On interrupt: CPU executes `jal r7,(r6)` -- jumps to handler, saves PC in ir
- Return from ISR: `jmp (ir)` -- also clears interrupt-in-service flag
- Interrupts detected on control flow transfers (bra, brf, brt, jmp, jal)
- Enable/disable via MMIO write to 0xFF0010 (bit 0)

## ISR vs Normal Function

| Aspect | Normal function | Interrupt handler |
|--------|----------------|-------------------|
| Prologue | push fp, r2, r1 | push r0, r1, r2, save C flag, push fp |
| Epilogue | pop r1, r2, fp; jmp (r1) | pop fp, restore C flag, pop r2, r1, r0; jmp (ir) |
| Registers saved | Only callee-saved (r1, r2, fp) | ALL registers + condition flag |
| Return instruction | `jmp (r1)` | `jmp (ir)` -- clears interrupt flag |
| Entry mechanism | `jal r1,(r0)` from caller | CPU auto-executes `jal r7,(r6)` |

The key difference: a normal function knows its caller won't care about r0
(caller-saved), so it doesn't save it. An ISR can fire at any point, so it
must save everything the interrupted code might be using.

## Implementation Plan

### Phase 1: Basic inline asm (MVP)

Add `asm("...")` statement support. The string contents are emitted verbatim
as assembly lines. No operand substitution, no constraints.

**Syntax:**
```c
asm("la iv,_isr_entry");    // set interrupt vector
asm("jmp (ir)");             // return from interrupt
asm("mov r2,c");             // read condition flag
```

**Implementation:**
- Lexer: recognize `asm` keyword
- Parser: parse `asm ( "string literal" ) ;`
- Codegen: emit each line of the string as-is

**What this enables:**
```c
// ISR wrapper -- manual register save/restore
void isr_entry(void) {
    asm("push r0");
    asm("push r1");
    asm("push r2");
    asm("mov  r2,c");
    asm("push r2");

    handle_uart_rx();    // call normal C function for the logic

    asm("pop  r2");
    asm("clu  z,r2");
    asm("pop  r2");
    asm("pop  r1");
    asm("pop  r0");
    asm("jmp  (ir)");
}
```

This is ugly but functional. The programmer manages register save/restore.

### Phase 2: __attribute__((interrupt)) (post-MVP)

The compiler recognizes an interrupt attribute on a function and
automatically generates the correct ISR prologue/epilogue.

**Syntax:**
```c
__attribute__((interrupt))
void uart_isr(void) {
    volatile char *uart = (char *)0xFF0100;
    rx_char = *uart;
}
```

**Implementation:**
- Parser: recognize `__attribute__((interrupt))` before function definition
- AST: add `is_interrupt: bool` field to Function node
- Codegen: emit ISR prologue/epilogue when flag is set

**Generated prologue:**
```asm
_uart_isr:
        push    r0
        push    r1
        push    r2
        mov     r2,c            ; save condition flag
        push    r2
        push    fp
        mov     fp,sp
```

**Generated epilogue:**
```asm
        mov     sp,fp
        pop     fp
        pop     r2
        clu     z,r2            ; restore condition flag
        pop     r2
        pop     r1
        pop     r0
        jmp     (ir)            ; interrupt return
```

**Advantages over inline asm:**
- Correct by construction -- can't forget to save a register
- Can't accidentally use `jmp (r1)` instead of `jmp (ir)`
- Maintenance-free -- change the handler body without updating the wrapper
- About 30-40 lines of codegen change (COR24 only has 3 GP registers)

### Phase 3: Extended inline asm (future)

GCC-style extended asm with operand substitution and constraints.

**Syntax:**
```c
int val;
asm("mov %0,c" : "=r"(val));              // output: condition flag -> val
asm("la iv,%0" : : "i"(handler_addr));     // input: address -> iv register
```

**Constraint letters (COR24-specific):**
- `r` -- any of r0, r1, r2
- `i` -- immediate value
- `m` -- memory operand

This is significantly more complex to implement (register allocator
interaction, operand encoding) and is deferred until the need arises.

## Interrupt Setup Pattern

Regardless of which ISR mechanism is used, setting the interrupt vector
requires writing to the iv register. This is not memory-mapped, so it
always needs inline asm or a compiler builtin:

```c
// Using inline asm (MVP)
asm("la iv,_my_isr");

// Using a builtin (future alternative)
__builtin_set_iv(my_isr);
```

Enabling/disabling interrupts is just MMIO and works in plain C:
```c
*(volatile char *)0xFF0010 = 1;    // enable UART RX interrupt
*(volatile char *)0xFF0010 = 0;    // disable
```

## Sample Assembly Reference

See `docs/sample-asm/interrupt.s` for a complete working example showing:
- ISR with full register save/restore and condition flag handling
- Interrupt vector setup via `mov iv,r0`
- Interrupt enable via MMIO
- Polling a global variable set by the ISR

## Timeline

| Phase | Feature | When |
|-------|---------|------|
| MVP (Phase 3) | `asm("...")` basic passthrough | With function calls |
| Post-MVP | `__attribute__((interrupt))` | Soon after MVP |
| Future | Extended `asm("..." : "=r"(x))` | When needed |
| Future | `__builtin_set_iv(func)` | When needed |
