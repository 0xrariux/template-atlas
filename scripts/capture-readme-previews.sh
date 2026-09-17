#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="$repo_root/assets/previews"
preview_target_dir="${CARGO_TARGET_DIR:-$repo_root/target/readme-previews}"

mkdir -p "$output_dir"

for product in command forge fleet ledger; do
  CARGO_TARGET_DIR="$preview_target_dir" \
    cargo build --manifest-path "$repo_root/slint/$product/Cargo.toml" \
    --bin "atlas-$product" --locked
done

COMMAND_PAGE=0 \
COMMAND_CAPTURE="$output_dir/command.png" \
COMMAND_WIDTH=1280 COMMAND_HEIGHT=800 \
SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$preview_target_dir/debug/atlas-command"

FORGE_STATE=explorer \
FORGE_CAPTURE="$output_dir/forge.png" \
FORGE_WIDTH=1280 FORGE_HEIGHT=800 \
SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$preview_target_dir/debug/atlas-forge"

FLEET_STATE=overview \
FLEET_CAPTURE="$output_dir/fleet.png" \
FLEET_WIDTH=1280 FLEET_HEIGHT=800 \
SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$preview_target_dir/debug/atlas-fleet"

LEDGER_STATE=portfolio \
LEDGER_CAPTURE="$output_dir/ledger.png" \
LEDGER_WIDTH=1280 LEDGER_HEIGHT=800 \
SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$preview_target_dir/debug/atlas-ledger"

echo "README previews written to $output_dir"
