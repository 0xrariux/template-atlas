# Atlas Fleet — native Slint port

This directory contains the native Rust + Slint implementation of the Atlas
Fleet template. It consumes the published Atlas UI crate; Fleet screens,
fixtures, and product compositions remain owned by `template-atlas`.

Run it from this directory with:

```sh
cargo run
```

Capture the deterministic visual matrix from the repository root with:

```sh
./scripts/capture-fleet-native.sh
./scripts/compare-fleet-screenshots.sh
```

`FLEET_STATE` selects any state listed by the
[`capture-fleet-native.sh`](../../scripts/capture-fleet-native.sh) harness.
`FLEET_CAPTURE`, `FLEET_WIDTH`, and `FLEET_HEIGHT` enable headless snapshot
output for the comparison harness.
