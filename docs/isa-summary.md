# COR24 ISA Summary

C-Oriented RISC, 24-bit Instruction Set Architecture.
Designed by MakerLisp for efficient C code execution on FPGA.

## Registers

8 registers, all 24-bit wide, plus PC and a 1-bit condition flag.

| Index | Name | Role | Notes |
|-------|------|------|-------|
| r0 | r0 | General purpose / return value | Not saved across calls; scratch |
| r1 | r1 | Link register (return address) | `jal` always saves return addr here |
| r2 | r2 | General purpose / register variable | Callee-saved |
| r3 | fp | Frame pointer | Callee-saved |
| r4 | sp | Stack pointer | Grows downward; init 0xFEEC00 |
| r5 | z | Zero register | Always reads 0; used in compares |
| r6 | iv | Interrupt vector | Holds ISR address |
| r7 | ir | Interrupt return address | CPU sets on interrupt entry |
| - | PC | Program counter | 24-bit |
| - | C | Condition flag | 1-bit; set only by ceq/cls/clu |

### Register Constraints

- Only r0, r1, r2 are truly general-purpose for arithmetic/logic.
- fp and sp have dedicated roles but can appear in some instructions.
- r5 (z) is read-only zero; only usable as an operand in compare instructions.
- r6 (iv) and r7 (ir) are for interrupt handling.
- The C flag is not set by arithmetic -- only by compare instructions.

## Instruction Formats

All instructions are 1, 2, or 4 bytes. Never 3 bytes.

| Format | Bytes | Structure | Used By |
|--------|-------|-----------|---------|
| Register-register | 1 | `[opcode+regs]` | add, sub, mul, and, or, xor, shifts, compares, mov, push, pop, jal, jmp, sxt, zxt |
| Register + imm8 | 2 | `[opcode+reg] [imm8]` | add imm, lc, lcu, lb, lbu, lw, sb, sw, bra, brf, brt |
| Register + imm24 | 4 | `[opcode+reg] [low] [mid] [high]` | la, sub sp |

The first byte encodes both the opcode (5 bits) and register operand(s) (3 bits).
A 256-entry decode ROM maps each first-byte value to (opcode, ra, rb).

## Complete Instruction Set (32 Instructions)

### Arithmetic

| Mnemonic | Bytes | Operation | Notes |
|----------|-------|-----------|-------|
| `add ra,rb` | 1 | ra = ra + rb | Wraps at 2^24 |
| `add ra,dd` | 2 | ra = ra + sign_ext(dd) | dd is 8-bit signed (-128..127) |
| `sub ra,rb` | 1 | ra = ra - rb | Modulo 2^24 |
| `sub sp,dddddd` | 4 | sp = sp - imm24 | Stack allocation; 24-bit unsigned |
| `mul ra,rb` | 1 | ra = (ra * rb)[23:0] | 24-cycle hardware multiply; low 24 bits |

### Logical

| Mnemonic | Bytes | Operation |
|----------|-------|-----------|
| `and ra,rb` | 1 | ra = ra & rb |
| `or ra,rb` | 1 | ra = ra \| rb |
| `xor ra,rb` | 1 | ra = ra ^ rb |

### Shift

| Mnemonic | Bytes | Operation | Notes |
|----------|-------|-----------|-------|
| `shl ra,rb` | 1 | ra = ra << rb[4:0] | Left shift |
| `srl ra,rb` | 1 | ra = ra >> rb[4:0] | Logical right (zero fill) |
| `sra ra,rb` | 1 | ra = ra >>> rb[4:0] | Arithmetic right (sign fill) |

Shift amount uses only the low 5 bits of rb.

### Compare (Set Condition Flag)

| Mnemonic | Bytes | Operation |
|----------|-------|-----------|
| `ceq ra,rb` | 1 | C = (ra == rb) |
| `cls ra,rb` | 1 | C = (ra < rb) signed |
| `clu ra,rb` | 1 | C = (ra < rb) unsigned |

To test if a register is zero: `ceq r0,z` (compare with r5).
To test if negative (signed): `cls r0,z` (less than zero).

### Branch (PC-Relative)

| Mnemonic | Bytes | Operation |
|----------|-------|-----------|
| `bra dd` | 2 | PC = PC + sign_ext(dd) -- unconditional |
| `brf dd` | 2 | if (!C) PC = PC + sign_ext(dd) -- branch if false |
| `brt dd` | 2 | if (C) PC = PC + sign_ext(dd) -- branch if true |

Branch offset dd is 8-bit signed (-128..+127 bytes).
PC is 2 bytes ahead at execution (past the branch instruction itself).

### Jump / Call

| Mnemonic | Bytes | Operation | Notes |
|----------|-------|-----------|-------|
| `jal ra,(rb)` | 1 | r1 = PC+1; PC = rb | Return addr always saved to r1 |
| `jmp (ra)` | 1 | PC = ra | If ra=r7, clears interrupt flag |

Three `jal` encodings exist: target in (r0), (r1), or (r2).
The return address is always stored in r1 regardless of encoding.

### Load

| Mnemonic | Bytes | Operation | Notes |
|----------|-------|-----------|-------|
| `la ra,dddddd` | 4 | ra = imm24 | If ra=r7, acts as direct jump |
| `lc ra,dd` | 2 | ra = sign_ext(dd) | 8-bit signed constant |
| `lcu ra,dd` | 2 | ra = zero_ext(dd) | 8-bit unsigned constant |
| `lb ra,dd(rb)` | 2 | ra = sign_ext(mem[rb+sign_ext(dd)]) | Load byte, sign-extend |
| `lbu ra,dd(rb)` | 2 | ra = zero_ext(mem[rb+sign_ext(dd)]) | Load byte, zero-extend |
| `lw ra,dd(rb)` | 2 | ra = mem24[rb+sign_ext(dd)] | Load 24-bit word |

### Store

| Mnemonic | Bytes | Operation |
|----------|-------|-----------|
| `sb ra,dd(rb)` | 2 | mem[rb+sign_ext(dd)] = ra[7:0] |
| `sw ra,dd(rb)` | 2 | mem24[rb+sign_ext(dd)] = ra |

### Move / Extension

| Mnemonic | Bytes | Operation | Notes |
|----------|-------|-----------|-------|
| `mov ra,rb` | 1 | ra = rb | If rb=r5(z), loads C flag: ra={0..0,C} |
| `sxt ra,rb` | 1 | ra = sign_ext(rb[7:0]) | Sign-extend byte to 24 bits |
| `zxt ra,rb` | 1 | ra = zero_ext(rb[7:0]) | Zero-extend byte to 24 bits |

### Stack

| Mnemonic | Bytes | Operation |
|----------|-------|-----------|
| `push ra` | 1 | sp -= 3; mem24[sp] = ra |
| `pop ra` | 1 | ra = mem24[sp]; sp += 3 |

Stack operations always move 3 bytes (one 24-bit word).

## Addressing Modes

| Mode | Syntax | Description |
|------|--------|-------------|
| Register direct | `add r0,r1` | Operands in registers |
| Immediate 8-bit | `lc r0,42` | 8-bit signed/unsigned constant |
| Immediate 24-bit | `la r0,0x1234` | Full 24-bit constant/address |
| Base + offset | `lw r0,6(fp)` | Base register + 8-bit signed offset |
| Register indirect | `jmp (r1)` | Jump to address in register |

Base registers for load/store: r0, r1, r2, fp.
sp cannot be used as a base register in lb/lbu/lw/sb/sw -- use fp instead.

## Immediate Value Ranges

| Context | Size | Signed Range | Unsigned Range |
|---------|------|-------------|----------------|
| lc, add imm, branch offset, load/store offset | 8-bit | -128..127 | 0..255 (lcu only) |
| la, sub sp | 24-bit | -8388608..8388607 | 0..16777215 |

## Key Semantic Details

### Overflow
Silent wraparound at 24 bits (modulo 2^24). No overflow flag.

### Condition Flag
- Set only by ceq, cls, clu.
- All other instructions preserve C.
- `mov ra,c` (where c means the condition flag via r5 encoding) loads 0 or 1 into ra.

### Endianness
Little-endian. Multi-byte values stored LSB first.
4-byte instruction `la r0,0x123456` encodes as: `[opcode] [0x56] [0x34] [0x12]`.

### Multiply
Hardware multiply via `mul ra,rb`. Takes 24 clock cycles.
Returns low 24 bits of the full product. No high-word access.

### Divide
No hardware divide instruction. Must be implemented in software
via repeated subtraction or lookup table.

### NOP
No dedicated NOP instruction. Common idioms:
- `zxt z,z` (opcode 0xFF) -- harmless no-op
- Careful: `add r0,r0` doubles r0, not a true NOP

## Interrupts

- Interrupt vector address stored in r6 (iv).
- On interrupt: CPU executes `jal r7,(r6)` automatically.
  - Return address saved in r7 (ir).
  - Jumps to address in r6.
- Return from ISR: `jmp (r7)` -- also clears the interrupt-in-service flag.
- Currently only one interrupt source: UART RX data ready.
- Interrupts are detected on control flow transfers (bra, brf, brt, jmp, jal).

## References

- Hardware Verilog: `cor24_cpu.v` in COR24-TB archive
- Decode ROM: `dis_rom_init.mem` (256-entry lookup)
- ISA manual: `COR24-TB-MAN.pdf`
- MakerLisp: https://makerlisp.com
- License: MIT
