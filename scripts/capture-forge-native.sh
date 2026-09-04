#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/slint/forge/Cargo.toml"
binary="$repo_root/slint/forge/target/debug/atlas-forge"
output_dir="${1:-$repo_root/ai/comparison/forge/candidate}"

mkdir -p "$output_dir"
cargo build --manifest-path "$manifest" --bin atlas-forge

states=(
  explorer explorer-inspector-outline explorer-problems explorer-output
  explorer-logs explorer-selected-tree-item explorer-terminal-closed
  explorer-minimized explorer-tree-collapsed explorer-outline-expanded
  explorer-timeline-expanded explorer-tabs-closed explorer-preview-file
  search search-collapsed
  source-control dependencies dependencies-selected-serde tasks
  extensions-installed extensions-recommended settings command-palette
)

for state in "${states[@]}"; do
  FORGE_STATE="$state" FORGE_CAPTURE="$output_dir/$state.png" \
  FORGE_WIDTH=1440 FORGE_HEIGHT=900 \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

for viewport in 1280x800 1600x1000 1920x1080; do
  width="${viewport%x*}"
  height="${viewport#*x}"
  FORGE_STATE=explorer FORGE_CAPTURE="$output_dir/explorer-$viewport.png" \
  FORGE_WIDTH="$width" FORGE_HEIGHT="$height" \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

echo "Captured 26 Atlas Forge states in $output_dir"
