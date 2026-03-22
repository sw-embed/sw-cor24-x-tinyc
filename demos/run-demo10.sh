#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/cc24"
INCLUDE_DIR="$ROOT_DIR/include"
DEMO_C="$SCRIPT_DIR/demo10.c"
DEMO_S="$SCRIPT_DIR/demo10.s"

if [ ! -f "$CC24" ]; then
    echo "Building cc24..."
    cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
fi

echo "=== cc24 Demo 10: #include, #pragma once, System Headers ==="
echo ""
echo "Header: demo10_io.h"
echo "---"
cat "$SCRIPT_DIR/demo10_io.h"
echo "---"
echo ""
echo "Source: demo10.c"
echo "---"
cat "$DEMO_C"
echo "---"
echo ""

echo "Compiling demo10.c -> demo10.s (with -I $INCLUDE_DIR)"
"$CC24" "$DEMO_C" -o "$DEMO_S" -I "$INCLUDE_DIR"
echo ""

echo "Generated assembly:"
echo "---"
cat "$DEMO_S"
echo "---"
echo ""

echo "Assembling and running on COR24 emulator..."
echo ""
OUTPUT=$(cor24-run --run "$DEMO_S" --dump --speed 0 --time 10 2>&1)
echo "$OUTPUT"
echo ""

R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | sed 's/.*(\s*//' | sed 's/\s*)//')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | sed 's/.*Halted: //')
LED=$(echo "$OUTPUT" | grep "LED:" | head -1 | sed 's/.*LED:  //' | awk '{print $1}')
UART=$(echo "$OUTPUT" | grep "UART TX log:" | sed 's/.*UART TX log:   //' | tr -d '"')

echo "=== Validation ==="
PASS=true

if [ "$HALTED" = "true" ]; then
    echo "  [PASS] CPU halted cleanly"
else
    echo "  [FAIL] CPU did not halt"
    PASS=false
fi

if [ "$R0" = "42" ]; then
    echo "  [PASS] r0 = 42 (all #include checks passed)"
else
    echo "  [FAIL] r0 = $R0 (expected 42)"
    PASS=false
fi

if [ "$LED" = "0x00" ]; then
    echo "  [PASS] LED D2 on (from included header function)"
else
    echo "  [FAIL] LED = $LED (expected 0x00)"
    PASS=false
fi

if echo "$UART" | grep -q "D10OK"; then
    echo "  [PASS] UART output contains 'D10OK'"
else
    echo "  [FAIL] UART output: '$UART' (expected 'D10OK')"
    PASS=false
fi

echo ""
if [ "$PASS" = true ]; then
    echo "Demo 10 PASSED"
else
    echo "Demo 10 FAILED"
    exit 1
fi
