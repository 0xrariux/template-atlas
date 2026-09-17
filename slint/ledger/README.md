# Atlas Ledger — Slint + Rust

Native Atlas Ledger template built with Slint and the reusable Atlas UI library
from the sibling source checkout.

The showcase is deterministic and local-only: it contains fictional balances,
identifiers, providers, and transfers, and never contacts a financial service.

## Run and validate

```sh
cargo run --manifest-path slint/ledger/Cargo.toml
cargo fmt --manifest-path slint/ledger/Cargo.toml -- --check
cargo test --manifest-path slint/ledger/Cargo.toml --all-targets
cargo clippy --manifest-path slint/ledger/Cargo.toml --all-targets -- -D warnings
```

The interactive build supports all sidebar destinations, portfolio modes and
periods, sortable holdings columns, row/tab/section selections, notifications,
copy feedback, transaction details, preference switches, and the two-step mock
transfer flow. Escape dismisses open overlays.

## Deterministic visual capture

Deterministic visual states are selected with `LEDGER_STATE`. A PNG can be
written with `LEDGER_CAPTURE` and optional `LEDGER_WIDTH` / `LEDGER_HEIGHT`:

```sh
LEDGER_STATE=portfolio LEDGER_CAPTURE=/tmp/ledger.png \
  cargo run --manifest-path slint/ledger/Cargo.toml
```

Capture the complete 25-state matrix and compare it with optional local
reference captures using:

```sh
./scripts/capture-ledger-native.sh
./scripts/compare-ledger-screenshots.sh
```

Goldens live in `ai/reference/ledger/golden`, final native candidates in
`ai/comparison/ledger/candidate`, and heat maps plus objective metrics in
`ai/comparison/ledger/diff`.
