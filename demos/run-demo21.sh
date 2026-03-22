#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/cc24"
DEMO_C="$SCRIPT_DIR/demo21.c"
DEMO_S="$SCRIPT_DIR/demo21.s"
if [ ! -f "$CC24" ]; then cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet; fi

echo "=== cc24 Demo 21: Compound Assignment ==="
"$CC24" "$DEMO_C" -o "$DEMO_S"
OUTPUT=$(cor24-run --run "$DEMO_S" --dump --speed 0 --time 10 2>&1)
echo "$OUTPUT"
echo ""
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')
UART=$(echo "$OUTPUT" | grep "UART TX log:" | sed 's/.*UART TX log:   //' | tr -d '"')
echo "=== Validation ==="
PASS=true
if [ "$HALTED" = "true" ]; then echo "  [PASS] CPU halted"; else echo "  [FAIL] no halt"; PASS=false; fi
if [ "$R0" = "42" ]; then echo "  [PASS] r0 = 42"; else echo "  [FAIL] r0 = $R0"; PASS=false; fi
if echo "$UART" | grep -q "D21OK"; then echo "  [PASS] UART: D21OK"; else echo "  [FAIL] UART: '$UART'"; PASS=false; fi
echo ""
if [ "$PASS" = true ]; then echo "Demo 21 PASSED"; else echo "Demo 21 FAILED"; exit 1; fi
