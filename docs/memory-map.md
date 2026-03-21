# COR24 Memory Map

## Address Space

24-bit address space (16 MB total). Byte-addressable. Little-endian.

```
0x000000 +---------------------------+
         |                           |
         |  External SRAM            |
         |  (up to 16 MB)            |
         |  512 KB on testboard      |
         |                           |
0xFE0000 +---------------------------+
         |  Embedded Block RAM       |
         |  (4 KB populated)         |
         |                           |
0xFEE000 |  Boot ROM / Program Start | <-- Reset vector (PC starts here)
         |                           |
0xFEEC00 |  Initial Stack Pointer    | <-- SP starts here, grows DOWN
         |                           |
0xFF0000 +---------------------------+
         |  Memory-Mapped I/O        |
         |  (64 KB region)           |
0xFFFFFF +---------------------------+
```

## Memory Regions

### External SRAM (0x000000 - 0xFDFFFF)

- Up to 16 MB addressable.
- COR24-TB testboard has 512 KB populated.
- 2-cycle access per byte; word access takes 6+ cycles.
- Used for: heap, large arrays, program code after relocation.

### Embedded Block RAM (0xFE0000 - 0xFEFFFF)

- 4 KB populated on testboard (0xFEE000 - 0xFEFFFF).
- Faster than SRAM (2 cycles from PC load to first byte).
- Used for: boot code, small programs, stack.
- Dual-port: can be read by instruction fetch and data access.

### Boot / Program Entry (0xFEE000)

- CPU begins execution here on reset.
- The monitor program (`loadngo`) typically resides here.
- `loadngo` can load user programs into SRAM and jump to them.

### Stack (Initial SP = 0xFEEC00)

- Stack pointer initialized to 0xFEEC00 on CPU reset.
- Grows **downward** (push decrements SP by 3).
- Stack is in embedded block RAM for speed.
- Stack size depends on available EBR below 0xFEEC00.

### Memory-Mapped I/O (0xFF0000 - 0xFFFFFF)

See I/O Ports section below.

## I/O Port Map

| Address | Read | Write | Description |
|---------|------|-------|-------------|
| 0xFF0000 | Button state (bit 0) | LED state (bit 0) | GPIO |
| 0xFF0010 | Interrupt enable (bit 0) | Set interrupt enable | Interrupt control |
| 0xFF0100 | UART RX data (8-bit) | UART TX data (8-bit) | Serial data |
| 0xFF0101 | UART status | - | Serial status |

### UART Status Register (0xFF0101) -- Read Only

| Bit | Name | Meaning |
|-----|------|---------|
| 7 | TX busy | 1 = transmitter is sending a byte |
| 2 | RX overflow | 1 = receive buffer overflowed |
| 1 | CTS | 1 = clear to send (flow control) |
| 0 | RX ready | 1 = received byte available |

### UART Usage Pattern

**Transmit a byte:**
```asm
        la      r1,0xFF0100     ; UART base
.wait:  lb      r2,1(r1)        ; read status (bit 7 = TX busy)
        cls     r2,z            ; negative = bit 7 set = busy
        brt     .wait           ; spin while busy
        sb      r0,0(r1)        ; transmit byte from r0[7:0]
```

**Receive a byte:**
```asm
        la      r1,0xFF0100     ; UART base
        lb      r0,0(r1)        ; read RX data (auto-acknowledges interrupt)
```

### Interrupt Enable (0xFF0010)

- Write bit 0 = 1 to enable UART RX interrupt.
- Write bit 0 = 0 to disable.
- Read returns current enable state.
- Only one interrupt source exists (UART RX ready).

## Memory Model for C Programs

### Freestanding (Bare Metal)

Programs run without an OS. The compiler generates:

- `.text` section: code, placed at program start address
- `.data` section: initialized global/static data
- `.comm` symbols: uninitialized (BSS) global/static data

### Typical Program Memory Layout

```
0xFEE000 +---------------------------+
         |  .text (code)             |
         |  Functions, string lits   |
         +---------------------------+
         |  .data (initialized)      |
         |  Global variables         |
         +---------------------------+
         |  .comm / .bss             |
         |  Uninitialized globals    |
         +---------------------------+
         |  (free space)             |
         +---------------------------+
         |  Stack (grows down)       |
0xFEEC00 +---------------------------+ <-- Initial SP
```

For larger programs loaded via the monitor into SRAM:

```
0x000000 +---------------------------+
         |  .text (code)             |
         +---------------------------+
         |  .data                    |
         +---------------------------+
         |  .bss                     |
         +---------------------------+
         |  (free / heap)            |
         +---------------------------+
         |  Stack (grows down)       |
         |  (SP set by startup code) |
         +---------------------------+
```

### Stack Allocation

- Each push/pop moves 3 bytes (one word).
- Local variables allocated by `sub sp,N` or `add sp,-N`.
- Deallocated by `mov sp,fp` (frame pointer restoration).
- No hardware stack overflow detection.

### Global Variable Placement

- Initialized globals go in `.data` section.
- Uninitialized globals declared with `.comm name,size`.
- String literals emitted as `.byte` sequences in `.data`.

## Hardware Notes

- UART baud rate: 921600 (at 101.6 MHz clock).
- UART format: 8N1.
- UART RX has 4-entry FIFO.
- LED is active-low on hardware (writing 1 turns LED off).
- Reading UART RX data auto-acknowledges the interrupt.
