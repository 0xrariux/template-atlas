#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
reference_dir="${1:-$repo_root/ai/reference/forge/golden}"
candidate_dir="${2:-$repo_root/ai/comparison/forge/candidate}"
output_dir="${3:-$repo_root/ai/comparison/forge/diff}"
comparator="$repo_root/slint/command/target/debug/visual_diff"

mkdir -p "$output_dir"
if [[ ! -x "$comparator" ]]; then
  cargo build --manifest-path "$repo_root/slint/command/Cargo.toml" --bin visual_diff
fi

metrics_file="$output_dir/metrics.tsv"
printf 'state\tmae\trmse\tchanged_gt_12\n' > "$metrics_file"

for reference in "$reference_dir"/*.png; do
  state="$(basename "$reference" .png)"
  candidate="$candidate_dir/$state.png"
  if [[ ! -f "$candidate" ]]; then
    echo "Missing candidate: $candidate" >&2
    exit 1
  fi
  metrics="$($comparator "$reference" "$candidate" "$output_dir/$state.png")"
  values="$(printf '%s' "$metrics" | sed -E 's/mae=([^ ]+) rmse=([^ ]+) changed_gt_12=([^%]+)%/\1\t\2\t\3/')"
  printf '%s\t%s\n' "$state" "$values" >> "$metrics_file"
done

cat "$metrics_file"
