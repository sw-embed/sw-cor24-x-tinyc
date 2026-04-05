#!/usr/bin/env bash
# Demo 61: Inline assembly on COR24
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
INCLUDE_DIR="$ROOT_DIR/include"
SRC="$SCRIPT_DIR/demo61.c"
cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT
"$CC24" "$SRC" -o "$TMPDIR/demo61.s" -I "$INCLUDE_DIR"
echo "=== Compiled demo61.c ==="
OUTPUT=$(cor24-run --run "$TMPDIR/demo61.s" --dump --speed 0 --time 10 2>&1)
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
echo "r0 = $R0 (0 = all tests passed)"
