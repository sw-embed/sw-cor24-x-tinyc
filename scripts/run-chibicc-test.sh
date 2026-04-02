#!/usr/bin/env bash
# Run a single chibicc test through tc24r -> cor24-run pipeline.
#
# Usage: scripts/run-chibicc-test.sh <test-name>
#   e.g.: scripts/run-chibicc-test.sh arith
#
# Reads test from ~/github/softwarewrighter/chibicc/test/<name>.c
# Copies to temp, compiles with tc24r, runs with cor24-run.
# Exit 0 if r0=0 (all assertions pass), exit 1 otherwise.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
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

# Adapt the test file for tc24r freestanding:
# - Strip lines with printf/sprintf/exit/float/double/long/hosted decls
# - Strip binary and octal literals our lexer doesn't handle
# - Our include/test.h provides a freestanding ASSERT
# Using awk (portable across GNU/BSD) instead of sed.
awk '
/printf/   { next }
/sprintf/  { next }
/vsprintf/ { next }
/exit\(/   { next }
/^void assert/ { next }
/^int ext/  { next }
/^int \*ext/ { next }
/^int common_/ { next }
/^static int common_/ { next }
/^int false_fn/ { next }
/^int true_fn/ { next }
/^int char_fn/ { next }
/^int short_fn/ { next }
/^int uchar_fn/ { next }
/^int ushort_fn/ { next }
/^static int static_fn/ { next }
/^int ext_fn/ { next }
/ext_fn1/  { next }
/ext_fn2/  { next }
/extern.*ext/  { next }
/ext[0-9]/ { next }
/float/    { next }
/double/   { next }
/long/     { next }
/short/    { next }
/_Bool/    { next }
/0b[01]/   { next }
/0[0-7][0-7]/ { next }
/assert.*size/ { next }
/[0-9]\.[0-9]/  { next }
/[0-9]e[0-9]/   { next }
/&&[a-z]/       { next }
/goto \*/       { next }
/case.*\.\.\./  { next }
/,.*\)=[^=]/    { next }
{ print }
' "$SRC" > "$TMPDIR/$NAME.c"

# Copy our freestanding test.h into the temp dir so it takes precedence
# over chibicc's test.h (quoted includes check source_dir first)
cp "$INCLUDE_DIR/test.h" "$TMPDIR/test.h"

# Compile
if ! "$CC24" "$TMPDIR/$NAME.c" -o "$TMPDIR/$NAME.s" -I "$INCLUDE_DIR" -I "$CHIBICC_TEST" 2>"$TMPDIR/tc24r.err"; then
    echo "COMPILE_FAIL: $NAME -- $(head -1 "$TMPDIR/tc24r.err")"
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
