#!/usr/bin/env sh
set -eu

repo_root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
template_target_dir="${CARGO_TARGET_DIR:-$repo_root/target/quality-gate}"
export CARGO_TARGET_DIR="$template_target_dir"

for product in command forge fleet ledger; do
  manifest="$repo_root/slint/$product/Cargo.toml"
  cargo fmt --manifest-path "$manifest" -- --check
  cargo check --manifest-path "$manifest" --all-targets
  cargo clippy --manifest-path "$manifest" --all-targets -- -D warnings
  cargo test --manifest-path "$manifest" --all-targets
done

echo "Template quality gate passed: Command, Forge, Fleet, and Ledger."
