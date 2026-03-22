#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

COMPONENTS=(core frontend backend cli)

for component in "${COMPONENTS[@]}"; do
    echo "=== $component ==="
    bash "$ROOT_DIR/components/$component/scripts/build.sh"
done

echo ""
echo "All components built successfully."
