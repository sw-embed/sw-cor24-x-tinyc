#!/usr/bin/env bash
# Demo 62: parenthesised and mixed const-expr in array sizes
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
INCLUDE_DIR="$ROOT_DIR/include"
SRC="$SCRIPT_DIR/demo62.c"
cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT
"$CC24" "$SRC" -o "$TMPDIR/demo62.s" -I "$INCLUDE_DIR"
echo "=== Compiled demo62.c ==="
OUTPUT=$(cor24-run --run "$TMPDIR/demo62.s" --dump --speed 0 --time 10 2>&1)
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')
if [ "$HALTED" = "true" ] && [ "$R0" = "62" ]; then
    echo "Demo 62 PASSED (r0=62)"
else
    echo "Demo 62 FAILED (r0=$R0, halted=$HALTED)"
    exit 1
fi
