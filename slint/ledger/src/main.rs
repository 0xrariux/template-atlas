//! Native Atlas Ledger showcase built with Rust, Slint, and Atlas UI.

#![allow(missing_docs)]

use std::cell::Cell;

use slint::{ComponentHandle, ModelRc, VecModel};

slint::include_modules!();

mod formatting;
mod mock;

fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
    ModelRc::new(VecModel::from(items))
}

fn state_page(state: &str) -> i32 {
    match state {
        "assets" | "assets-btc" => 1,
        "transactions" | "transactions-drawer" | "transactions-drawer-copied" => 2,
        "markets" | "markets-btc" => 3,
        "accounts" => 4,
        "analytics" => 5,
        "connections" | "connections-degraded" => 6,
        "security" | "security-approvals" => 7,
        "settings" | "settings-currency" => 8,
        _ => 0,
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    app.global::<AtlasSettings>()
        .set_theme_mode(ThemeMode::Light);
    app.global::<AtlasSettings>().set_density(Density::Compact);
    app.global::<AtlasSettings>()
        .set_typography_scale(TypographyScale::Compact);

    app.set_assets(model(mock::assets()));
    app.set_transactions(model(mock::transactions()));
    app.set_markets(model(mock::markets()));
    app.set_accounts(model(mock::accounts()));
    app.set_connections(model(mock::connections()));
    app.set_security_events(model(mock::security_events()));
    app.set_signers(model(mock::signers()));

    let active_sort = Cell::new(1);
    let ascending = Cell::new(false);
    let weak = app.as_weak();
    app.on_sort_assets(move |sort_key| {
        let next_ascending = if active_sort.get() == sort_key {
            !ascending.get()
        } else {
            true
        };
        active_sort.set(sort_key);
        ascending.set(next_ascending);
        if let Some(app) = weak.upgrade() {
            app.set_assets(model(mock::sorted_assets(sort_key, next_ascending)));
        }
    });

    let state = std::env::var("LEDGER_STATE").unwrap_or_else(|_| "portfolio".into());
    app.set_current_page(state_page(&state));
    app.set_capture_state(state.into());

    if let Some(path) = std::env::var_os("LEDGER_CAPTURE") {
        let width = std::env::var("LEDGER_WIDTH")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(1440.0);
        let height = std::env::var("LEDGER_HEIGHT")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(900.0);
        app.window()
            .set_size(slint::LogicalSize::new(width, height));
        let weak = app.as_weak();
        app.show()?;
        slint::Timer::single_shot(std::time::Duration::from_millis(320), move || {
            let app = weak.upgrade().expect("Ledger renderer remains alive");
            let pixels = app.window().take_snapshot().expect("Ledger snapshot");
            image::save_buffer(
                &path,
                pixels.as_bytes(),
                pixels.width(),
                pixels.height(),
                image::ColorType::Rgba8,
            )
            .expect("save Ledger snapshot");
            slint::quit_event_loop().expect("quit Ledger capture event loop");
        });
        slint::run_event_loop()
    } else {
        app.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_states_map_to_expected_pages() {
        assert_eq!(state_page("portfolio"), 0);
        assert_eq!(state_page("assets-btc"), 1);
        assert_eq!(state_page("transactions-drawer"), 2);
        assert_eq!(state_page("markets-btc"), 3);
        assert_eq!(state_page("transfer-review"), 0);
        assert_eq!(state_page("analytics"), 5);
        assert_eq!(state_page("connections-degraded"), 6);
        assert_eq!(state_page("security-approvals"), 7);
        assert_eq!(state_page("settings-currency"), 8);
    }

    #[test]
    fn fixtures_cover_the_ledger_workbench() {
        assert_eq!(mock::assets().len(), 5);
        assert_eq!(mock::transactions().len(), 5);
        assert_eq!(mock::markets().len(), 4);
        assert_eq!(mock::accounts().len(), 4);
        assert_eq!(mock::connections().len(), 5);
        assert_eq!(mock::security_events().len(), 4);
        assert_eq!(mock::signers().len(), 4);
    }

    #[test]
    fn holdings_sort_matches_the_reference_controls() {
        let symbols = |records: Vec<AssetRecord>| {
            records
                .into_iter()
                .map(|record| record.symbol.to_string())
                .collect::<Vec<_>>()
        };
        assert_eq!(symbols(mock::sorted_assets(0, true))[0], "BTC");
        assert_eq!(symbols(mock::sorted_assets(1, false))[0], "ETH");
        assert_eq!(symbols(mock::sorted_assets(2, true))[0], "USDC");
    }
}
