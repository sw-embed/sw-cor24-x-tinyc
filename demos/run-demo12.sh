#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/cc24"
DEMO_C="$SCRIPT_DIR/demo12.c"
DEMO_S="$SCRIPT_DIR/demo12.s"

if [ ! -f "$CC24" ]; then
    cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
fi

echo "=== cc24 Demo 12: do...while ==="
echo ""
echo "Compiling demo12.c -> demo12.s"
"$CC24" "$DEMO_C" -o "$DEMO_S"

echo "Assembling and running on COR24 emulator..."
OUTPUT=$(cor24-run --run "$DEMO_S" --dump --speed 0 --time 10 2>&1)
echo "$OUTPUT"
echo ""

R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | sed 's/.*(\s*//' | sed 's/\s*)//')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | sed 's/.*Halted: //')
UART=$(echo "$OUTPUT" | grep "UART TX log:" | sed 's/.*UART TX log:   //' | tr -d '"')

echo "=== Validation ==="
PASS=true
if [ "$HALTED" = "true" ]; then echo "  [PASS] CPU halted cleanly"; else echo "  [FAIL] CPU did not halt"; PASS=false; fi
if [ "$R0" = "42" ]; then echo "  [PASS] r0 = 42"; else echo "  [FAIL] r0 = $R0 (expected 42)"; PASS=false; fi
if echo "$UART" | grep -q "D12OK"; then echo "  [PASS] UART: D12OK"; else echo "  [FAIL] UART: '$UART'"; PASS=false; fi
echo ""
if [ "$PASS" = true ]; then echo "Demo 12 PASSED"; else echo "Demo 12 FAILED"; exit 1; fi
