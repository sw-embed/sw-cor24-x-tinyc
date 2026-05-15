#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
DEMO_C="$SCRIPT_DIR/demo2.c"
DEMO_S="$SCRIPT_DIR/demo2.s"

# Build tc24r if needed
if [ ! -f "$CC24" ]; then
    echo "Building tc24r..."
    cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
fi

echo "=== tc24r Demo 2: Pointers, Char, MMIO ==="
echo ""
echo "Source: demo2.c"
echo "---"
cat "$DEMO_C"
echo "---"
echo ""

# Compile C to assembly
echo "Compiling demo2.c -> demo2.s"
"$CC24" "$DEMO_C" -o "$DEMO_S"
echo ""

echo "Generated assembly:"
echo "---"
cat "$DEMO_S"
echo "---"
echo ""

# Assemble and run on COR24
echo "Assembling and running on COR24 emulator..."
echo ""
LGO=$(mktemp --suffix=.lgo)
cor24-asm "$DEMO_S" -o "$LGO"
OUTPUT=$(cor24-emu --lgo "$LGO" --dump --speed 0 --time 10 2>&1 | awk '!/^Loaded [0-9]+ bytes from /')
rm -f "$LGO"
echo "$OUTPUT"
echo ""

# Extract values for validation
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')
LED=$(echo "$OUTPUT" | grep "LED D2:" | head -1 | awk '{print $3}')
UART=$(echo "$OUTPUT" | grep "UART TX log:" | awk -F'UART TX log:' '{print $2}' | tr -d ' "')

echo "=== Validation ==="
PASS=true

if [ "$HALTED" = "true" ]; then
    echo "  [PASS] CPU halted cleanly"
else
    echo "  [FAIL] CPU did not halt"
    PASS=false
fi

if [ "$R0" = "42" ]; then
    echo "  [PASS] r0 = 42 (all C checks passed)"
else
    echo "  [FAIL] r0 = $R0 (expected 42)"
    PASS=false
fi

if [ "$LED" = "0x00" ]; then
    echo "  [PASS] LED D2 on (0x00, active-low)"
else
    echo "  [FAIL] LED = $LED (expected 0x00 for D2 on)"
    PASS=false
fi

if echo "$UART" | grep -q "OK"; then
    echo "  [PASS] UART output contains 'OK'"
else
    echo "  [FAIL] UART output: '$UART' (expected 'OK')"
    PASS=false
fi

echo ""
if [ "$PASS" = true ]; then
    echo "Demo 2 PASSED"
else
    echo "Demo 2 FAILED"
    exit 1
fi
