#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/slint/fleet/Cargo.toml"
binary="${CARGO_TARGET_DIR:-$repo_root/slint/fleet/target}/debug/atlas-fleet"
output_dir="${1:-$repo_root/ai/comparison/fleet/candidate}"

mkdir -p "$output_dir"
cargo build --manifest-path "$manifest" --bin atlas-fleet --locked

states=(
  overview overview-devices overview-list overview-filter-online
  overview-filter-warning overview-filter-offline overview-device-warning
  overview-telemetry-memory overview-telemetry-24h restart-confirmation
  overview-lower devices devices-filter-warning devices-selected-warning
  topology deployments metrics metrics-memory metrics-24h alerts
  alerts-selected-warning automation automation-run-history settings
  settings-telemetry logs add-device
)

for state in "${states[@]}"; do
  FLEET_STATE="$state" FLEET_CAPTURE="$output_dir/$state.png" \
  FLEET_WIDTH=1440 FLEET_HEIGHT=900 \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

for viewport in 1280x800 1600x1000 1920x1080; do
  width="${viewport%x*}"
  height="${viewport#*x}"
  FLEET_STATE=overview FLEET_CAPTURE="$output_dir/overview-$viewport.png" \
  FLEET_WIDTH="$width" FLEET_HEIGHT="$height" \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

echo "Captured 30 Atlas Fleet states in $output_dir"
