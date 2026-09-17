#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/slint/ledger/Cargo.toml"
binary="${CARGO_TARGET_DIR:-$repo_root/slint/ledger/target}/debug/atlas-ledger"
output_dir="${1:-$repo_root/ai/comparison/ledger/candidate}"

mkdir -p "$output_dir"
cargo build --manifest-path "$manifest" --bin atlas-ledger --locked

states=(
  portfolio portfolio-1y portfolio-performance portfolio-notifications
  portfolio-assets-table transactions transactions-drawer
  transactions-drawer-copied markets markets-btc accounts transfer
  transfer-review assets assets-btc analytics connections
  connections-degraded security security-approvals settings settings-currency
)

for state in "${states[@]}"; do
  LEDGER_STATE="$state" LEDGER_CAPTURE="$output_dir/$state.png" \
  LEDGER_WIDTH=1440 LEDGER_HEIGHT=900 \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

for viewport in 1280x800 1600x1000 1920x1080; do
  width="${viewport%x*}"
  height="${viewport#*x}"
  LEDGER_STATE=portfolio LEDGER_CAPTURE="$output_dir/portfolio-$viewport.png" \
  LEDGER_WIDTH="$width" LEDGER_HEIGHT="$height" \
  SLINT_BACKEND=winit-software SLINT_SCALE_FACTOR=1 \
  "$binary"
done

echo "Captured 25 Atlas Ledger states in $output_dir"
