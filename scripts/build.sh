#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo "=== Building sw-cor24-tinyc ==="

COMPONENTS=(core frontend backend cli)

for component in "${COMPONENTS[@]}"; do
    echo "--- $component ---"
    cargo build --manifest-path "$ROOT_DIR/components/$component/Cargo.toml" --release
done

echo ""
echo "=== Running tests ==="

for component in "${COMPONENTS[@]}"; do
    echo "--- $component tests ---"
    cargo test --manifest-path "$ROOT_DIR/components/$component/Cargo.toml"
done

echo ""
echo "Build complete. Binary: components/cli/target/release/tc24r"
