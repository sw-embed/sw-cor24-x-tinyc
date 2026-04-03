#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
DEMO_C="$SCRIPT_DIR/demo57.c"
DEMO_S="$SCRIPT_DIR/demo57.s"
cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet

echo "=== tc24r Demo 57: address-of struct members passed to functions ==="
"$CC24" "$DEMO_C" -o "$DEMO_S" -I "$ROOT_DIR/include"
OUTPUT=$(cor24-run --run "$DEMO_S" --dump --speed 0 --time 60 2>&1)
echo "$OUTPUT"
echo ""
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')
UART=$(echo "$OUTPUT" | grep "UART TX log:" | awk -F'UART TX log:' '{print $2}' | tr -d ' "')
echo "=== Validation ==="
PASS=true
if [ "$HALTED" = "true" ]; then echo "  [PASS] CPU halted"; else echo "  [FAIL] no halt"; PASS=false; fi
if [ "$R0" = "57" ]; then echo "  [PASS] r0 = 57"; else echo "  [FAIL] r0 = $R0"; PASS=false; fi
if echo "$UART" | grep -q "D57OK"; then echo "  [PASS] UART: D57OK"; else echo "  [FAIL] UART: '$UART'"; PASS=false; fi
echo ""
if [ "$PASS" = true ]; then echo "Demo 57 PASSED"; else echo "Demo 57 FAILED"; exit 1; fi
