//! Native Atlas Command showcase built with Rust, Slint, and Atlas UI.

#![allow(missing_docs)]

use slint::{ComponentHandle, ModelRc, VecModel};

slint::include_modules!();

mod mock;

fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
    ModelRc::new(VecModel::from(items))
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    app.global::<AtlasSettings>()
        .set_theme_mode(ThemeMode::Light);
    app.global::<AtlasSettings>().set_density(Density::Compact);
    app.global::<AtlasSettings>()
        .set_typography_scale(TypographyScale::Compact);

    app.set_metrics(model(mock::metrics()));
    app.set_services(model(mock::services()));
    app.set_health_services(model(mock::health_services()));
    app.set_activities(model(mock::activities()));
    app.set_alerts(model(mock::alerts()));
    app.set_logs(model(mock::logs()));
    app.set_team(model(mock::team()));
    app.set_integrations(model(mock::integrations()));
    app.set_regions(model(mock::regions()));

    let weak = app.as_weak();
    app.on_navigate_requested(move |page| {
        if let Some(app) = weak.upgrade() {
            app.set_current_page(page.clamp(0, 8));
            app.invoke_close_overlays();
        }
    });

    if let Ok(page) = std::env::var("COMMAND_PAGE") {
        app.set_current_page(page.parse().unwrap_or(0));
    }
    if let Ok(overlay) = std::env::var("COMMAND_OVERLAY") {
        app.set_capture_overlay(overlay.into());
    }

    if let Some(path) = std::env::var_os("COMMAND_CAPTURE") {
        let width = std::env::var("COMMAND_WIDTH")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(1440.0);
        let height = std::env::var("COMMAND_HEIGHT")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(900.0);
        app.window()
            .set_size(slint::LogicalSize::new(width, height));
        let weak = app.as_weak();
        app.show()?;
        slint::Timer::single_shot(std::time::Duration::from_millis(300), move || {
            let app = weak.upgrade().expect("Command renderer remains alive");
            let pixels = app.window().take_snapshot().expect("Command snapshot");
            image::save_buffer(
                &path,
                pixels.as_bytes(),
                pixels.width(),
                pixels.height(),
                image::ColorType::Rgba8,
            )
            .expect("save Command snapshot");
            slint::quit_event_loop().expect("quit capture event loop");
        });
        slint::run_event_loop()
    } else {
        app.run()
    }
}
