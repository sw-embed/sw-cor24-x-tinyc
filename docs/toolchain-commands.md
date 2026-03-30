# COR24 Toolchain Commands

## Overview

The COR24 toolchain consists of:

1. **as24** -- Assembler (C source in COR24-TB archive: `asld24/as24.c`)
2. **ld24** -- Linker (C source: `asld24/ld24.c`)
3. **longlgo** -- Load-and-go file generator (C source: `asld24/longlgo.c`)
4. **loadngo** -- On-target monitor/bootloader (runs on COR24 hardware)
5. **te** -- Terminal emulator for host-to-board communication (`tools/te.c`)
6. **MakerLisp C compiler** -- Proprietary C-to-COR24 compiler (not open source)

## Assembler (as24)

Reads COR24 assembly from stdin, produces object output.

```bash
# Assemble with listing output
as24 -l < program.s

# Assemble to object (pipe to linker)
as24 < program.s > program.o
```

### Listing Output Format

```
000000 80              push    fp
000001 7f              push    r2
000002 7e              push    r1
000003 65              mov     fp,sp
000004 29 00 00 00     la      r0,_fib
```

Columns: address (hex), instruction bytes (hex), source line.

## Linker (ld24)

Links assembled objects. Details TBD -- the MakerLisp toolchain handles
this internally.

## Load-and-Go Generator (longlgo)

Converts assembled output to `.lgo` format for loading onto the COR24
testboard via the monitor.

```bash
as24 < program.s | longlgo > program.lgo
```

The `.lgo` format is a hex text format understood by the `loadngo` monitor.

## Monitor (loadngo)

The monitor program resides in boot ROM (0xFEE000) on the COR24-TB.
It communicates via UART and supports:

- **L** -- Load a `.lgo` file into memory
- **G** -- Go (jump to loaded program's entry point)

## Terminal Emulator (te)

Host-side tool for communicating with the COR24-TB via USB serial.

```bash
# Connect to board (typical)
te /dev/ttyUSB0
```

## Typical Development Workflow

### On Hardware (Linux)

```bash
# 1. Write C source
vim program.c

# 2. Compile (MakerLisp compiler -- proprietary)
tc24r program.c -o program.s

# 3. Assemble
as24 < program.s | longlgo > program.lgo

# 4. Connect to board
te /dev/ttyUSB0

# 5. In terminal: load and run
# L (load command)
# <paste program.lgo contents>
# G (go command)
```

### For tc24r Development (This Project)

Since tc24r development happens on Mac with testing deferred to Linux:

```bash
# 1. Write test C source
vim tests/return_42.c

# 2. Compile with tc24r (our compiler, under development)
cargo run -- tests/return_42.c -o tests/return_42.s

# 3. Inspect generated assembly
cat tests/return_42.s

# 4. Compare with expected assembly (golden file testing)
diff tests/return_42.s tests/expected/return_42.s

# 5. (On Linux, later) Assemble and run on hardware or simulator
as24 < tests/return_42.s | longlgo > tests/return_42.lgo
```

### For Emulator Testing (sw-cor24-emulator)

The sw-cor24-emulator project provides a COR24 emulator written in Rust:

```bash
# Assemble and run in emulator
cd ~/github/sw-embed/sw-cor24-emulator
cargo run -- run path/to/program.s
```

This enables testing compiler output without physical hardware.

## Assembler Source Code

The assembler (`as24.c`) and linker (`ld24.c`) source are available in
the COR24-TB archive at `asld24/`. They are simple C programs that can
be compiled on Linux:

```bash
cd asld24
make
```

The assembler is a single-pass assembler (~500 lines of C). It supports:
- All 32 COR24 instructions
- Labels (global and local)
- `.text`, `.data`, `.globl`, `.byte`, `.word`, `.comm` directives
- Semicolon comments
- Hex and decimal literals

## File Formats

| Extension | Description | Tool |
|-----------|-------------|------|
| `.s` | Assembly source | Input to as24 |
| `.lst` | Assembly listing (addresses + hex + source) | as24 -l output |
| `.lgo` | Load-and-go hex file | longlgo output |
| `.mem` | Raw hex memory dump | Used for ROM initialization |
| `.c` | C source | Input to C compiler |

## Building the Tools (on Linux)

```bash
# From the COR24-TB archive
cd asld24
make            # builds as24, ld24, longlgo

# Or manually:
gcc -o as24 as24.c
gcc -o ld24 ld24.c
gcc -o longlgo longlgo.c
```

## Notes

- The assembler reads from stdin -- use `<` redirection or pipe.
- The MakerLisp C compiler is proprietary and not available for tc24r.
  That is why we are building tc24r as an open-source alternative.
- The cor24-rs emulator can run assembled programs without hardware,
  which is the primary testing path during development.
