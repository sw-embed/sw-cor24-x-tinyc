#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
INCLUDE_DIR="$ROOT_DIR/include"
SRC="$SCRIPT_DIR/demo55.c"
cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT
"$CC24" "$SRC" -o "$TMPDIR/demo55.s" -I "$INCLUDE_DIR"
echo "=== Compiled demo55.c ==="
OUTPUT=$(cor24-run --run "$TMPDIR/demo55.s" --dump --speed 0 --time 10 2>&1)
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')
if [ "$HALTED" = "true" ] && [ "$R0" = "55" ]; then
    echo "Demo 55 PASSED (r0=55)"
else
    echo "Demo 55 FAILED (r0=$R0, halted=$HALTED)"
    exit 1
fi
