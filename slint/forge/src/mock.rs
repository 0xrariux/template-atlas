use crate::{
    ChangeRecord, CodeRecord, CommandRecord, DependencyRecord, ExtensionRecord, SearchRecord,
    TaskRecord, TerminalRecord, TreeRecord,
};

pub fn tree_items() -> Vec<TreeRecord> {
    [
        ("src", 0, 0, true, false, false),
        ("components", 1, 0, true, false, false),
        ("button.rs", 2, 1, false, false, false),
        ("card.rs", 2, 1, false, false, true),
        ("input.rs", 2, 1, false, false, false),
        ("ui", 1, 0, true, false, false),
        ("app.slint", 2, 2, false, true, false),
        ("theme.slint", 2, 2, false, false, false),
        ("main.rs", 1, 1, false, true, false),
        ("lib.rs", 1, 1, false, false, false),
        ("Cargo.toml", 0, 3, false, false, false),
        ("Cargo.lock", 0, 3, false, false, false),
        ("README.md", 0, 4, false, false, false),
    ]
    .into_iter()
    .map(
        |(name, depth, kind, expanded, modified, added)| TreeRecord {
            name: name.into(),
            depth,
            kind,
            expanded,
            modified,
            added,
        },
    )
    .collect()
}

pub fn code_lines() -> Vec<CodeRecord> {
    [
        "use slint::{ComponentHandle, SharedString};",
        "use atlas_ui::{AtlasTheme, PrimaryButton};",
        "",
        "slint::include_modules!();",
        "",
        "#[tokio::main]",
        "async fn main() -> anyhow::Result<()> {",
        "    tracing_subscriber::fmt()",
        "        .with_target(false)",
        "        .compact()",
        "        .init();",
        "",
        "    let app = AppWindow::new()?;",
        "    let weak = app.as_weak();",
        "",
        "    app.on_run_task(move |task_id| {",
        "        let handle = weak.clone();",
        "        tokio::spawn(async move {",
        "            let result = execute_task(task_id).await;",
        "            handle.upgrade_in_event_loop(move |ui| {",
        "                ui.set_status(SharedString::from(result));",
        "            });",
        "        });",
        "    });",
        "",
        "    app.run()?;",
        "    Ok(())",
        "}",
    ]
    .into_iter()
    .enumerate()
    .map(|(index, text)| CodeRecord {
        number: (index + 1).to_string().into(),
        text: text.into(),
        tone: if text.trim_start().starts_with("use") || text.contains("async fn") {
            1
        } else if text.contains("include_modules") || text.contains("tokio") {
            2
        } else {
            0
        },
    })
    .collect()
}

pub fn terminal_lines() -> Vec<TerminalRecord> {
    [
        ("$ cargo run", 1),
        ("   Compiling atlas-core v0.4.0", 0),
        ("   Compiling atlas-ui v0.4.0", 0),
        (
            "    Finished dev [unoptimized + debuginfo] target(s) in 1.84s",
            2,
        ),
        ("     Running `target/debug/atlas-ui`", 0),
        ("", 0),
        ("Application started successfully.", 2),
    ]
    .into_iter()
    .map(|(text, tone)| TerminalRecord {
        text: text.into(),
        tone,
    })
    .collect()
}

pub fn dependencies() -> Vec<DependencyRecord> {
    [
        (
            "slint",
            "1.13.1",
            "1.14.0",
            "Update available",
            "Declarative GUI toolkit for Rust",
            true,
        ),
        (
            "serde",
            "1.0.219",
            "1.0.219",
            "Latest",
            "Serialization framework",
            false,
        ),
        (
            "tokio",
            "1.47.1",
            "1.48.0",
            "Update available",
            "Asynchronous runtime",
            true,
        ),
        (
            "anyhow",
            "1.0.98",
            "1.0.99",
            "Update available",
            "Flexible error handling",
            true,
        ),
        (
            "tracing",
            "0.1.41",
            "0.1.41",
            "Latest",
            "Application-level diagnostics",
            false,
        ),
    ]
    .into_iter()
    .map(
        |(name, installed, latest, status, description, update)| DependencyRecord {
            name: name.into(),
            installed: installed.into(),
            latest: latest.into(),
            status: status.into(),
            description: description.into(),
            update,
        },
    )
    .collect()
}

pub fn tasks() -> Vec<TaskRecord> {
    [
        (
            "Build workspace",
            "cargo build --workspace",
            "Success",
            "0.48s",
            "2 min ago",
            0,
            0,
        ),
        (
            "Run application",
            "cargo run --package atlas-ui",
            "Success",
            "1.84s",
            "8 min ago",
            1,
            0,
        ),
        (
            "Test all crates",
            "cargo test --workspace",
            "Success",
            "3.12s",
            "14 min ago",
            2,
            0,
        ),
        (
            "Clippy analysis",
            "cargo clippy --all-targets",
            "Warning",
            "2.08s",
            "28 min ago",
            3,
            1,
        ),
        (
            "Format check",
            "cargo fmt --all -- --check",
            "Success",
            "0.16s",
            "1 hr ago",
            4,
            0,
        ),
        (
            "Bundle release",
            "cargo build --release",
            "Idle",
            "—",
            "Never",
            5,
            2,
        ),
    ]
    .into_iter()
    .map(
        |(name, command, status, duration, last, icon, tone)| TaskRecord {
            name: name.into(),
            command: command.into(),
            status: status.into(),
            duration: duration.into(),
            last: last.into(),
            icon,
            tone,
        },
    )
    .collect()
}

pub fn extensions() -> Vec<ExtensionRecord> {
    [
        (
            "Rust Analyzer",
            "Demo Publisher",
            "0.4.2182",
            "Language",
            "Enabled",
            "RA",
            "8.2M",
            0,
        ),
        (
            "Slint Tools",
            "Demo Publisher",
            "1.14.0",
            "Language",
            "Enabled",
            "SL",
            "182K",
            1,
        ),
        (
            "TOML Toolkit",
            "Demo Publisher",
            "0.21.2",
            "Language",
            "Enabled",
            "TM",
            "4.1M",
            2,
        ),
        (
            "Diagnostics Lens",
            "Demo Publisher",
            "3.20.0",
            "Diagnostics",
            "Enabled",
            "DL",
            "3.8M",
            3,
        ),
        (
            "Source Graph",
            "Demo Publisher",
            "1.30.0",
            "Source control",
            "Recommended",
            "SG",
            "12M",
            4,
        ),
        (
            "Native Debugger",
            "Demo Publisher",
            "1.11.1",
            "Debugging",
            "Recommended",
            "ND",
            "6.4M",
            2,
        ),
    ]
    .into_iter()
    .map(
        |(name, author, version, category, state, initials, downloads, tone)| ExtensionRecord {
            name: name.into(),
            author: author.into(),
            version: version.into(),
            category: category.into(),
            state: state.into(),
            initials: initials.into(),
            downloads: downloads.into(),
            tone,
        },
    )
    .collect()
}

pub fn search_results() -> Vec<SearchRecord> {
    [
        (
            "src/components/button.rs",
            "12",
            "pub struct ",
            "PrimaryButton",
            " {",
            true,
            "4",
        ),
        ("", "28", "impl ", "PrimaryButton", " {", false, ""),
        (
            "",
            "46",
            "let button = ",
            "PrimaryButton",
            "::new();",
            false,
            "",
        ),
        (
            "ui/app.slint",
            "18",
            "export component ",
            "PrimaryButton",
            " inherits Rectangle {",
            true,
            "3",
        ),
        ("", "42", "", "PrimaryButton", " {", false, ""),
        (
            "src/main.rs",
            "2",
            "use atlas_ui::{AtlasTheme, ",
            "PrimaryButton",
            "};",
            true,
            "2",
        ),
        (
            "",
            "37",
            "let action = ",
            "PrimaryButton",
            "::default();",
            false,
            "",
        ),
    ]
    .into_iter()
    .map(
        |(file, line, before, match_text, after, first, count)| SearchRecord {
            file: file.into(),
            line: line.into(),
            before: before.into(),
            match_text: match_text.into(),
            after: after.into(),
            first,
            count: count.into(),
        },
    )
    .collect()
}

pub fn commands() -> Vec<CommandRecord> {
    [
        ("Open File", "Go to a file by name", "⌘P", 0),
        ("Build Project", "Compile the current workspace", "⌘B", 1),
        ("Run Project", "Build and launch application", "⌘R", 2),
        ("Search Dependencies", "Browse crate dependencies", "", 3),
        ("Toggle Terminal", "Show or hide bottom panel", "⌘J", 4),
        ("Open Settings", "Configure Atlas Forge", "⌘,", 5),
    ]
    .into_iter()
    .map(|(name, detail, shortcut, icon)| CommandRecord {
        name: name.into(),
        detail: detail.into(),
        shortcut: shortcut.into(),
        icon,
    })
    .collect()
}

pub fn changes() -> Vec<ChangeRecord> {
    [
        ("src/main.rs", "M", "+8 −2"),
        ("ui/app.slint", "M", "+12 −4"),
        ("src/components/card.rs", "A", "+48 −0"),
    ]
    .into_iter()
    .map(|(file, state, delta)| ChangeRecord {
        file: file.into(),
        state: state.into(),
        delta: delta.into(),
    })
    .collect()
}
