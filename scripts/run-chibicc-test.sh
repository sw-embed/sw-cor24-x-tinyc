#!/usr/bin/env bash
# Run a single chibicc test through cc24 -> cor24-run pipeline.
#
# Usage: scripts/run-chibicc-test.sh <test-name>
#   e.g.: scripts/run-chibicc-test.sh arith
#
# Reads test from ~/github/softwarewrighter/chibicc/test/<name>.c
# Copies to temp, compiles with cc24, runs with cor24-run.
# Exit 0 if r0=0 (all assertions pass), exit 1 otherwise.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/cc24"
INCLUDE_DIR="$ROOT_DIR/include"
CHIBICC_TEST="${HOME}/github/softwarewrighter/chibicc/test"

if [ $# -lt 1 ]; then
    echo "usage: $0 <test-name>"
    exit 1
fi

NAME="$1"
SRC="$CHIBICC_TEST/$NAME.c"

if [ ! -f "$SRC" ]; then
    echo "SKIP: $SRC not found"
    exit 2
fi

# Always rebuild to avoid stale binary issues (cargo is incremental)
cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Adapt the test file:
# - Use our test.h (via -I), no change needed for #include "test.h"
# - Strip printf/sprintf/exit calls (hosted C, not available)
# - Replace final return 0 with return _test_fail
sed \
    -e '/printf/d' \
    -e '/sprintf/d' \
    -e '/exit(/d' \
    -e 's/return 0;$/return _test_fail;/' \
    "$SRC" > "$TMPDIR/$NAME.c"

# Compile
if ! "$CC24" "$TMPDIR/$NAME.c" -o "$TMPDIR/$NAME.s" -I "$INCLUDE_DIR" -I "$CHIBICC_TEST" 2>"$TMPDIR/cc24.err"; then
    echo "COMPILE_FAIL: $NAME -- $(head -1 "$TMPDIR/cc24.err")"
    exit 1
fi

# Assemble and run
OUTPUT=$(cor24-run --run "$TMPDIR/$NAME.s" --dump --speed 0 --time 10 2>&1)
R0=$(echo "$OUTPUT" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
HALTED=$(echo "$OUTPUT" | grep "Halted:" | head -1 | awk '{print $2}')

if [ "$HALTED" != "true" ]; then
    echo "TIMEOUT: $NAME (did not halt)"
    exit 1
fi

if [ "$R0" = "0" ]; then
    echo "PASS: $NAME (r0=0)"
    exit 0
else
    echo "FAIL: $NAME (r0=$R0)"
    exit 1
fi
