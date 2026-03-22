#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/cc24"
DEMO_C="$SCRIPT_DIR/demo.c"
DEMO_S="$SCRIPT_DIR/demo.s"

# Build cc24 if needed
if [ ! -f "$CC24" ]; then
    echo "Building cc24..."
    cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
fi

echo "=== cc24 Demo ==="
echo ""
echo "Source: demo.c"
echo "---"
cat "$DEMO_C"
echo "---"
echo ""

# Compile C to assembly
echo "Compiling demo.c -> demo.s"
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
OUTPUT=$(cor24-run --run "$DEMO_S" --dump --speed 0 --time 10 2>&1)
echo "$OUTPUT"
echo ""

# Validate result
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | sed 's/.*(\s*//' | sed 's/\s*)//')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | sed 's/.*Halted: //')

echo "=== Validation ==="
PASS=true

if [ "$HALTED" = "true" ]; then
    echo "  [PASS] CPU halted cleanly"
else
    echo "  [FAIL] CPU did not halt"
    PASS=false
fi

if [ "$R0" = "42" ]; then
    echo "  [PASS] r0 = 42 (all checks passed in C code)"
else
    echo "  [FAIL] r0 = $R0 (expected 42)"
    PASS=false
fi

echo ""
if [ "$PASS" = true ]; then
    echo "Demo PASSED"
else
    echo "Demo FAILED"
    exit 1
fi
