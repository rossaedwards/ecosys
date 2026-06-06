#!/usr/bin/env bash
set -euo pipefail

echo '🧪 Running full AuraFS test suite...'
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
