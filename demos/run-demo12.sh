#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
DEMO_C="$SCRIPT_DIR/demo12.c"
DEMO_S="$SCRIPT_DIR/demo12.s"

if [ ! -f "$CC24" ]; then
    cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
fi

echo "=== tc24r Demo 12: do...while ==="
echo ""
echo "Compiling demo12.c -> demo12.s"
"$CC24" "$DEMO_C" -o "$DEMO_S"

echo "Assembling and running on COR24 emulator..."
LGO=$(mktemp --suffix=.lgo)
cor24-asm "$DEMO_S" -o "$LGO"
OUTPUT=$(cor24-emu --lgo "$LGO" --dump --speed 0 --time 10 2>&1)
rm -f "$LGO"
echo "$OUTPUT"
echo ""

R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')
UART=$(echo "$OUTPUT" | grep "UART TX log:" | awk -F'UART TX log:' '{print $2}' | tr -d ' "')

echo "=== Validation ==="
PASS=true
if [ "$HALTED" = "true" ]; then echo "  [PASS] CPU halted cleanly"; else echo "  [FAIL] CPU did not halt"; PASS=false; fi
if [ "$R0" = "42" ]; then echo "  [PASS] r0 = 42"; else echo "  [FAIL] r0 = $R0 (expected 42)"; PASS=false; fi
if echo "$UART" | grep -q "D12OK"; then echo "  [PASS] UART: D12OK"; else echo "  [FAIL] UART: '$UART'"; PASS=false; fi
echo ""
if [ "$PASS" = true ]; then echo "Demo 12 PASSED"; else echo "Demo 12 FAILED"; exit 1; fi
