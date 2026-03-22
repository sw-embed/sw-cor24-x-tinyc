#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/cc24"
DEMO_C="$SCRIPT_DIR/demo9.c"
DEMO_S="$SCRIPT_DIR/demo9.s"

if [ ! -f "$CC24" ]; then
    echo "Building cc24..."
    cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
fi

echo "=== cc24 Demo 9: UART RX Interrupt ==="
echo ""
echo "Source: demo9.c"
echo "---"
cat "$DEMO_C"
echo "---"
echo ""

echo "Compiling demo9.c -> demo9.s"
"$CC24" "$DEMO_C" -o "$DEMO_S"
echo ""

echo "Assembling and running on COR24 emulator (--uart-input \"A\")..."
echo ""
OUTPUT=$(cor24-run --run "$DEMO_S" --dump --speed 0 --time 5 --uart-input "A" 2>&1)
echo "$OUTPUT"
echo ""

R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | sed 's/.*(\s*//' | sed 's/\s*)//')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | sed 's/.*Halted: //')
INT_EN=$(echo "$OUTPUT" | grep "IntEn:" | head -1 | sed 's/.*IntEn:  //' | awk '{print $1}')

echo "=== Validation ==="
PASS=true

if [ "$HALTED" = "true" ]; then
    echo "  [PASS] CPU halted cleanly"
else
    echo "  [FAIL] CPU did not halt"
    PASS=false
fi

if [ "$R0" = "65" ]; then
    echo "  [PASS] r0 = 65 ('A' received via UART RX interrupt)"
else
    echo "  [FAIL] r0 = $R0 (expected 65 = 'A')"
    PASS=false
fi

if [ "$INT_EN" = "0x01" ]; then
    echo "  [PASS] UART RX interrupt enabled (IntEn = 0x01)"
else
    echo "  [FAIL] IntEn = $INT_EN (expected 0x01)"
    PASS=false
fi

echo ""
if [ "$PASS" = true ]; then
    echo "Demo 9 PASSED"
else
    echo "Demo 9 FAILED"
    exit 1
fi
