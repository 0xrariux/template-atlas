# Atlas Template Suite

[![CI](https://github.com/0xrariux/template-atlas/actions/workflows/ci.yml/badge.svg)](https://github.com/0xrariux/template-atlas/actions/workflows/ci.yml)
[![Atlas UI 0.1.1](https://img.shields.io/badge/Atlas_UI-0.1.1-2379F4.svg)](https://crates.io/crates/atlas-ui/0.1.1)

Four native desktop interface templates built with Slint and Rust:

- **Atlas Command** — operations and analytics
- **Atlas Forge** — engineering workspace
- **Atlas Fleet** — infrastructure control
- **Atlas Ledger** — institutional treasury

The templates consume the published
[`atlas-ui` 0.1.1 crate](https://crates.io/crates/atlas-ui/0.1.1). Its source,
component catalog, and integration documentation live in the companion
[`Atlas UI`](https://github.com/0xrariux/Atlas-UI) repository.

## Preview

<table>
  <tr>
    <td width="50%">
      <a href="assets/previews/command.png"><img src="assets/previews/command.png" alt="Atlas Command operations overview"></a><br>
      <strong>Atlas Command</strong> — operations, services, activity, alerts, and administration.
    </td>
    <td width="50%">
      <a href="assets/previews/forge.png"><img src="assets/previews/forge.png" alt="Atlas Forge engineering workspace"></a><br>
      <strong>Atlas Forge</strong> — a code-oriented engineering workspace with explorer and inspector surfaces.
    </td>
  </tr>
  <tr>
    <td width="50%">
      <a href="assets/previews/fleet.png"><img src="assets/previews/fleet.png" alt="Atlas Fleet infrastructure overview"></a><br>
      <strong>Atlas Fleet</strong> — infrastructure, telemetry, deployments, and incident control.
    </td>
    <td width="50%">
      <a href="assets/previews/ledger.png"><img src="assets/previews/ledger.png" alt="Atlas Ledger treasury portfolio"></a><br>
      <strong>Atlas Ledger</strong> — portfolio, assets, transactions, markets, and treasury settings.
    </td>
  </tr>
</table>

These are complete demonstration applications rather than components copied
into Atlas UI. They show how Atlas primitives can be composed while keeping
product identity, domain models, and navigation in the consuming application.

## Requirements

- Rust 1.92 with Cargo
- Internet access for the first dependency download

## Clone the templates

```bash
git clone https://github.com/0xrariux/template-atlas.git template-atlas
cd template-atlas
```

No Atlas source checkout is required for normal use. Each application pins the
published Atlas crate and configures the `@atlas-ui` Slint library path through
`atlas_ui::slint_library_paths()` in its build script.

## Validate

```bash
cargo check --manifest-path slint/command/Cargo.toml --all-targets
cargo check --manifest-path slint/forge/Cargo.toml --all-targets
cargo check --manifest-path slint/fleet/Cargo.toml --all-targets
cargo check --manifest-path slint/ledger/Cargo.toml --all-targets
```

Run the complete formatting, compilation, Clippy, and test gate with:

```bash
./scripts/quality-gate.sh
```

## Run a template

```bash
cargo run --manifest-path slint/command/Cargo.toml --bin atlas-command
cargo run --manifest-path slint/forge/Cargo.toml --bin atlas-forge
cargo run --manifest-path slint/fleet/Cargo.toml --bin atlas-fleet
cargo run --manifest-path slint/ledger/Cargo.toml --bin atlas-ledger
```

Each template also has its own documentation:

- [`slint/command/README.md`](slint/command/README.md)
- [`slint/forge/README.md`](slint/forge/README.md)
- [`slint/fleet/README.md`](slint/fleet/README.md)
- [`slint/ledger/README.md`](slint/ledger/README.md)

Native screenshot capture and comparison helpers are available under
[`scripts/`](scripts/). Their generated images and AI-assisted working material
live under the ignored `ai/` directory.

Run `./scripts/capture-readme-previews.sh` to regenerate the four images above.
See [Preview capture](docs/PREVIEWS.md) for deterministic capture details and
guidance on when an animated GIF is useful.

## Validate a local Atlas change

Atlas maintainers can place both repositories next to each other and run the
external consumer gate from the Atlas checkout:

```text
workspace/
├── Atlas/
└── template-atlas/
```

```bash
cd ../Atlas
sh scripts/template-consumer-gate.sh --template-root ../template-atlas
```

The gate injects a temporary Cargo patch so all four applications compile
against the local Atlas checkout without changing these published dependency
declarations.

## Demo data

All identities, balances, devices, incidents, addresses, services, and events
shown by these templates are fictional demonstration data. Example endpoints
use non-functional `example.invalid` domains and documentation-only IP ranges.

## License

The original source code in this repository is distributed under the
[MIT License](LICENSE).

The applications depend on Slint, which is distributed under its own choice of
GPLv3, Royalty-Free, or commercial licensing terms. In particular, Slint's
Royalty-Free license can cover proprietary desktop, mobile, and web
applications at no charge when its attribution requirements are met; it does
not cover embedded systems. Proprietary embedded use requires a commercial
license, while GPLv3 remains available for compatible open-source use.

Review the official [Slint licensing overview](https://github.com/slint-ui/slint/blob/master/LICENSE.md),
[licensing FAQ](https://slint.dev/faqs), and
[pricing page](https://slint.dev/pricing) before distributing an application.
The MIT license of these templates does not replace Slint's terms.
