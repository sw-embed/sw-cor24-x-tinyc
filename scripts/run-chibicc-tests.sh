#!/usr/bin/env bash
# Run all chibicc tests and report results.
#
# Usage: scripts/run-chibicc-tests.sh
#
# Output: summary of pass/fail/compile_fail/skip counts.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CHIBICC_TEST="${HOME}/github/softwarewrighter/chibicc/test"

# Build once upfront to avoid stale binary and repeated rebuilds
cargo build --manifest-path "$ROOT_DIR/components/cli/Cargo.toml" --release --quiet

pass=0
fail=0
compile_fail=0
skip=0
total=0

for f in "$CHIBICC_TEST"/*.c; do
    name=$(basename "$f" .c)
    total=$((total + 1))
    result=$("$SCRIPT_DIR/run-chibicc-test.sh" "$name" 2>&1) || true
    status=$(echo "$result" | head -1 | cut -d: -f1)
    case "$status" in
        PASS) pass=$((pass + 1)); echo "  PASS: $name" ;;
        FAIL) fail=$((fail + 1)); echo "  FAIL: $name" ;;
        COMPILE_FAIL) compile_fail=$((compile_fail + 1)) ;;
        TIMEOUT) fail=$((fail + 1)); echo "  TIMEOUT: $name" ;;
        SKIP) skip=$((skip + 1)) ;;
        *) compile_fail=$((compile_fail + 1)) ;;
    esac
done

echo ""
echo "=== chibicc test results ==="
echo "  Pass:         $pass / $total"
echo "  Fail:         $fail / $total"
echo "  Compile fail: $compile_fail / $total"
echo "  Skip:         $skip / $total"
echo ""
pct=$((pass * 100 / total))
echo "  Coverage: ${pct}%"
