//! Native Atlas Forge showcase built with Rust, Slint, and Atlas UI.

#![allow(missing_docs)]

use slint::{ComponentHandle, ModelRc, VecModel};

slint::include_modules!();

mod mock;

fn model<T: Clone + 'static>(items: Vec<T>) -> ModelRc<T> {
    ModelRc::new(VecModel::from(items))
}

fn state_page(state: &str) -> i32 {
    match state {
        "search" | "search-collapsed" => 1,
        "source-control" => 2,
        "dependencies" | "dependencies-selected-serde" => 3,
        "tasks" => 4,
        "extensions-installed" | "extensions-recommended" => 5,
        "settings" | "command-palette" => 6,
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

    app.set_tree_items(model(mock::tree_items()));
    app.set_code_lines(model(mock::code_lines()));
    app.set_terminal_lines(model(mock::terminal_lines()));
    app.set_dependencies(model(mock::dependencies()));
    app.set_tasks(model(mock::tasks()));
    app.set_extensions(model(mock::extensions()));
    app.set_search_results(model(mock::search_results()));
    app.set_commands(model(mock::commands()));
    app.set_changes(model(mock::changes()));

    let state = std::env::var("FORGE_STATE").unwrap_or_else(|_| "explorer".into());
    app.set_current_page(state_page(&state));
    app.set_capture_state(state.into());

    let weak = app.as_weak();
    app.on_navigate_requested(move |page| {
        if let Some(app) = weak.upgrade() {
            app.set_current_page(page.clamp(0, 6));
            app.set_palette_open(false);
        }
    });

    if let Some(path) = std::env::var_os("FORGE_CAPTURE") {
        let width = std::env::var("FORGE_WIDTH")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(1440.0);
        let height = std::env::var("FORGE_HEIGHT")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .unwrap_or(900.0);
        app.window()
            .set_size(slint::LogicalSize::new(width, height));
        let weak = app.as_weak();
        app.show()?;
        slint::Timer::single_shot(std::time::Duration::from_millis(320), move || {
            let app = weak.upgrade().expect("Forge renderer remains alive");
            let pixels = app.window().take_snapshot().expect("Forge snapshot");
            image::save_buffer(
                &path,
                pixels.as_bytes(),
                pixels.width(),
                pixels.height(),
                image::ColorType::Rgba8,
            )
            .expect("save Forge snapshot");
            slint::quit_event_loop().expect("quit Forge capture event loop");
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
        assert_eq!(state_page("explorer"), 0);
        assert_eq!(state_page("explorer-tabs-closed"), 0);
        assert_eq!(state_page("explorer-preview-file"), 0);
        assert_eq!(state_page("search"), 1);
        assert_eq!(state_page("source-control"), 2);
        assert_eq!(state_page("dependencies-selected-serde"), 3);
        assert_eq!(state_page("tasks"), 4);
        assert_eq!(state_page("extensions-recommended"), 5);
        assert_eq!(state_page("command-palette"), 6);
    }

    #[test]
    fn fixtures_cover_the_complete_forge_workbench() {
        assert_eq!(mock::tree_items().len(), 13);
        assert_eq!(mock::code_lines().len(), 28);
        assert_eq!(mock::dependencies().len(), 5);
        assert_eq!(mock::tasks().len(), 6);
        assert_eq!(mock::extensions().len(), 6);
        assert_eq!(mock::commands().len(), 6);
    }
}
