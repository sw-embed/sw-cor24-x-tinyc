#!/usr/bin/env bash
# Run all chibicc-subset tests through tc24r -> cor24-run pipeline.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CC24="$ROOT_DIR/components/cli/target/release/tc24r"
INCLUDE_DIR="$ROOT_DIR/include"
SUBSET_DIR="$ROOT_DIR/tests/chibicc-subset"

cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet

pass=0
fail=0
compile_fail=0
total=0

for f in "$SUBSET_DIR"/*.c; do
    name=$(basename "$f" .c)
    total=$((total + 1))
    TMPDIR=$(mktemp -d)

    if "$CC24" "$f" -o "$TMPDIR/$name.s" -I "$INCLUDE_DIR" 2>"$TMPDIR/err"; then
        cor24-asm "$TMPDIR/$name.s" -o "$TMPDIR/$name.lgo" 2>"$TMPDIR/asmerr" || true
        result=$(cor24-emu --lgo "$TMPDIR/$name.lgo" --dump --speed 0 --time 10 2>&1)
        r0=$(echo "$result" | grep "r0:" | head -1 | awk -F'[()]' '{print $2}' | tr -d ' ')
        halted=$(echo "$result" | grep "Halted:" | head -1 | awk '{print $2}')
        if [ "$halted" = "true" ] && [ "$r0" = "0" ]; then
            echo "  PASS: $name"
            pass=$((pass + 1))
        elif [ "$halted" = "true" ]; then
            echo "  FAIL: $name (r0=$r0)"
            fail=$((fail + 1))
        else
            echo "  TIMEOUT: $name"
            fail=$((fail + 1))
        fi
    else
        echo "  COMPILE_FAIL: $name -- $(head -1 "$TMPDIR/err")"
        compile_fail=$((compile_fail + 1))
    fi
    rm -rf "$TMPDIR"
done

echo ""
echo "=== chibicc-subset test results ==="
echo "  Pass:         $pass / $total"
echo "  Fail:         $fail / $total"
echo "  Compile fail: $compile_fail / $total"
pct=$((pass * 100 / total))
echo "  Coverage: ${pct}%"
