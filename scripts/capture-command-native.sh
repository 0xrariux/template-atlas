#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/slint/command/Cargo.toml"
binary="$repo_root/slint/command/target/debug/atlas-command"
output_dir="${1:-$repo_root/ai/comparison/command/candidate}"

mkdir -p "$output_dir"
cargo build --manifest-path "$manifest" --bin atlas-command

pages=(overview analytics services activity alerts logs team integrations settings)
for index in {0..8}; do
  COMMAND_PAGE="$index" \
  COMMAND_CAPTURE="$output_dir/${pages[$index]}.png" \
  COMMAND_WIDTH=1440 COMMAND_HEIGHT=900 \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

names=(
  overview-notifications overview-command-palette overview-create-report
  overview-period-7d overview-lower overview-services-table
  overview-services-selected-menu
)
overlays=(notifications palette report period-7d lower services-table row-menu)
for index in {0..6}; do
  COMMAND_PAGE=0 COMMAND_OVERLAY="${overlays[$index]}" \
  COMMAND_CAPTURE="$output_dir/${names[$index]}.png" \
  COMMAND_WIDTH=1440 COMMAND_HEIGHT=900 \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

echo "Captured 16 Atlas Command states in $output_dir"
