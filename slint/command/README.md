# Atlas Command — Slint + Rust

Native Atlas Command template. The app consumes the sibling Atlas UI source
facade and keeps all Command-specific UI in this folder.

## Run

From `slint/command`:

```bash
cargo run --bin atlas-command
```

The supported runtime pages are Overview, Analytics, Services, Activity,
Alerts, Logs, Team, Integrations, and Settings. Sidebar navigation, the command
palette, notifications, report form, chart periods, service selection, log
pause/resume, and settings controls are interactive.

## Capture and compare

From the repository root:

```bash
./scripts/capture-command-native.sh
```

The script builds the native app and captures 16 deterministic 1440×900 states
under `ai/comparison/command/candidate/`. Optional local reference captures can
be stored under `ai/reference/command/golden/`.

Compare one pair and write a heat map with:

```bash
cargo run --manifest-path slint/command/Cargo.toml --bin visual_diff -- \
  ai/reference/command/golden/overview.png \
  ai/comparison/command/candidate/overview.png \
  ai/comparison/command/diff/overview.png
```

`COMMAND_PAGE=0..8`, `COMMAND_OVERLAY`, `COMMAND_CAPTURE`, `COMMAND_WIDTH`,
and `COMMAND_HEIGHT` are available for targeted deterministic captures.

## Atlas integration

Command consumes Atlas's stable `AtlasStatusIndicator` for standalone semantic
signals and the configurable preview `AtlasProgressBar` track for compact
rails. Domain-specific composition, fixtures, navigation, and behavior remain
owned by this application.
