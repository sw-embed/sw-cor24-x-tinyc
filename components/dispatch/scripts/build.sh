#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
cargo fmt --check
cargo clippy --tests
cargo test
cargo build
