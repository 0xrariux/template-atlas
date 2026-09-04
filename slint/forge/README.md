# Atlas Forge — native Slint port

Atlas Forge is a native Rust + Slint engineering-workspace template. It
consumes the published Atlas UI crate and keeps all Forge-specific workbench
composition in this crate.

```bash
cargo run --manifest-path slint/forge/Cargo.toml
```

Deterministic screenshot states are selected with `FORGE_STATE`; captures use
`FORGE_CAPTURE`, `FORGE_WIDTH`, and `FORGE_HEIGHT`. The repository capture
scripts produce the full 1440 × 900 state matrix and responsive Explorer
checks.
