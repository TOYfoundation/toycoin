#!/usr/bin/env sh
# Local quality gate: the same checks as CI (CLAUDE.md, working rule 3).
set -eu
cd "$(dirname "$0")/.."
echo "==> cargo fmt --all -- --check"
cargo fmt --all -- --check
echo "==> cargo clippy --workspace --all-targets -- -D warnings"
cargo clippy --workspace --all-targets -- -D warnings
echo "==> cargo test --workspace"
cargo test --workspace
echo "OK: all checks passed"
