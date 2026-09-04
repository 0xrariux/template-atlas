//! Native Atlas Fleet showcase built with Rust, Slint, and Atlas UI.

#![allow(missing_docs)]

use slint::{ComponentHandle, ModelRc, VecModel};

slint::include_modules!();

mod mock;

fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
    ModelRc::new(VecModel::from(items))
}

fn state_page(state: &str) -> i32 {
    match state {
        "devices" | "devices-selected-warning" | "devices-filter-warning" => 1,
        "topology" => 2,
        "deployments" => 3,
        "metrics" | "metrics-memory" | "metrics-24h" => 4,
        "alerts" | "alerts-selected-warning" => 5,
        "logs" | "add-device" => 6,
        "automation" | "automation-run-history" => 7,
        "settings" | "settings-telemetry" => 8,
        _ => 0,
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    app.global::<AtlasSettings>()
        .set_theme_mode(ThemeMode::Dark);
    app.global::<AtlasSettings>().set_density(Density::Compact);
    app.global::<AtlasSettings>()
        .set_typography_scale(TypographyScale::Compact);

    app.set_devices(model(mock::devices()));
    app.set_deployments(model(mock::deployments()));
    app.set_logs(model(mock::logs()));
    app.set_incidents(model(mock::incidents()));
    app.set_workflows(model(mock::workflows()));

    let state = std::env::var("FLEET_STATE").unwrap_or_else(|_| "overview".into());
    app.set_current_page(state_page(&state));
    app.set_capture_state(state.into());

    if let Some(path) = std::env::var_os("FLEET_CAPTURE") {
        let width = std::env::var("FLEET_WIDTH")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(1440.0);
        let height = std::env::var("FLEET_HEIGHT")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(900.0);
        app.window()
            .set_size(slint::LogicalSize::new(width, height));
        let weak = app.as_weak();
        app.show()?;
        slint::Timer::single_shot(std::time::Duration::from_millis(320), move || {
            let app = weak.upgrade().expect("Fleet renderer remains alive");
            let pixels = app.window().take_snapshot().expect("Fleet snapshot");
            image::save_buffer(
                &path,
                pixels.as_bytes(),
                pixels.width(),
                pixels.height(),
                image::ColorType::Rgba8,
            )
            .expect("save Fleet snapshot");
            slint::quit_event_loop().expect("quit Fleet capture event loop");
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
        assert_eq!(state_page("overview"), 0);
        assert_eq!(state_page("devices-selected-warning"), 1);
        assert_eq!(state_page("topology"), 2);
        assert_eq!(state_page("metrics-memory"), 4);
        assert_eq!(state_page("alerts-selected-warning"), 5);
        assert_eq!(state_page("add-device"), 6);
        assert_eq!(state_page("settings-telemetry"), 8);
    }

    #[test]
    fn fixtures_cover_the_fleet_workbench() {
        assert_eq!(mock::devices().len(), 6);
        assert_eq!(mock::deployments().len(), 6);
        assert_eq!(mock::logs().len(), 5);
        assert_eq!(mock::incidents().len(), 4);
        assert_eq!(mock::workflows().len(), 5);
    }
}
