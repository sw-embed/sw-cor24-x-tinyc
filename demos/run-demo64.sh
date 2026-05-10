#!/usr/bin/env bash
# Demo 64: no-std minimal — proves DCE strips everything when nothing
# is included. The .s should contain only _start, _halt, _main, the
# buf data symbol, and codegen-required helpers (none for this).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
SRC="$SCRIPT_DIR/demo64.c"
cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT
"$CC24" "$SRC" -o "$TMPDIR/demo64.s"
echo "=== Compiled demo64.c ==="

# Sanity check: confirm DCE actually stripped the obvious unused symbols.
# (None of these should appear because nothing is #included.)
for sym in _malloc _free _calloc _realloc _printf _abs _atoi _strcmp _strcpy _putchar; do
    if grep -q "^${sym}:" "$TMPDIR/demo64.s"; then
        echo "  [FAIL] symbol ${sym} present in .s but should be DCE-stripped"
        exit 1
    fi
done
echo "  [PASS] no stdlib/stdio/string symbols leaked into .s"
cor24-asm "$TMPDIR/demo64.s" -o "$TMPDIR/demo64.lgo"

OUTPUT=$(cor24-emu --lgo "$TMPDIR/demo64.lgo" --dump --speed 0 --time 10 2>&1)
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')
if [ "$HALTED" = "true" ] && [ "$R0" = "5" ]; then
    echo "Demo 64 PASSED (r0=5, no-std minimal)"
else
    echo "Demo 64 FAILED (r0=$R0, halted=$HALTED)"
    exit 1
fi
