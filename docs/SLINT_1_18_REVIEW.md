# Slint 1.18 and Atlas source review

Reviewed on 2026-09-17, before publishing the Atlas migration.

## Dependency state

Command, Forge, Fleet, and Ledger now pin `slint` and `slint-build` to 1.18.0
and use the sibling Atlas source checkout for runtime and build dependencies.
Atlas reports version 0.2.0 in this checkout; the published 0.1.1 crate
is the earlier Slint 1.17.1 release. All four lockfiles resolve the local
Atlas crates, Slint 1.18.0, and `const-field-offset` 0.2.1. Ledger's native
`Flickable` bindings use Slint 1.18 `content-*` properties; Atlas's public
`viewport-*` properties keep their distinct API names.

## Build and visual evidence

- The full `scripts/quality-gate.sh` passes on macOS arm64 with Rust 1.97.1:
  formatting, all-target checks, strict Clippy, and tests for all four apps.
- Atlas's `scripts/template-consumer-gate.sh --capture` passes against the
  current template worktree with locked dependencies and confirms all 97
  capture files.
- All 97 declared native states render with the software backend at scale
  factor 1: 16 Command, 26 Forge, 30 Fleet, and 25 Ledger. The normal state
  uses 1440×900; Forge, Fleet, and Ledger also include 1280×800, 1600×1000,
  and 1920×1080 states.
- The four tracked 1280×800 README previews were regenerated and inspected.

The earlier native captures in the ignored `ai/comparison/*/candidate/`
directories provide broad state coverage. Some predate the committed template
source, including its current brand marks, so their differences do not isolate
Slint changes. `changed_gt_12` is the percentage of pixels where at least one
RGB channel differs by more than 12.

| Template | States compared | Mean changed pixels | Largest state difference |
|---|---:|---:|---:|
| Command | 16 | 1.77% | Notifications, 2.34% |
| Forge | 26 | 1.65% | Explorer at 1280×800, 2.70% |
| Fleet | 30 | 2.03% | Overview at 1280×800, 3.50% |
| Ledger | 25 | 1.74% | Security, 2.55% |

The four tracked README previews are a more reproducible comparison against
the committed 1.17.1 template source. Their changed-pixel ratios are 2.38%
(Command), 2.70% (Forge), 3.42% (Fleet), and 1.97% (Ledger). Their heat maps
concentrate on text and icon edges without a large geometry shift.

The inspected high-difference scenes, Ledger's scrolled assets table,
Command's notification overlay, Fleet's restart and add-device dialogs, and
Ledger's transaction drawer retain their expected geometry and content. Most
changed pixels follow text and icon edges. The older `ai/reference/*/golden/`
images also include original design references; their larger differences are
not a valid measure of this Slint migration. No reference set was promoted.

## Release evidence and limits

The four applications consume the tagged Atlas v0.2.0 source. Rust 1.92 CI
passed on Linux, Windows, and macOS for the migration. Native keyboard and
pointer interaction checks, input-method behavior, and assistive technology
remain outside this macOS capture pass. The local `ai/` screenshots and diff
images are ignored working evidence, not portable release assets.
