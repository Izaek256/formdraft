#![forbid(unsafe_code)]
//! `pattern_ui` — the eframe/egui workspace for formdraft.
//!
//! The crate keeps every region split into a pure model (unit tested) and a
//! thin view. It holds no drafting logic: dimensions come from `pattern_core`
//! and rule data from `pattern_templates` (ADR-0005, ADR-0007).

pub mod app;
pub mod canvas;
pub mod panels;
pub mod placeholder;
pub mod shell;
pub mod theme;

/// Start the native window. Returns an error instead of panicking.
pub fn run() -> eframe::Result {
    // Window placement is handled once on the first frame in `WorkspaceApp`
    // (`center_window`): eframe's `centered` centres by inner size, which
    // pushes the title bar and bottom edge under the taskbar on short screens.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("FormDraft — Precision Pattern Engineering")
            .with_app_id("formdraft")
            .with_inner_size([1340.0, 680.0])
            .with_min_inner_size([1100.0, 680.0]),
        ..Default::default()
    };
    eframe::run_native(
        "formdraft",
        options,
        Box::new(|_cc| Ok(Box::<app::WorkspaceApp>::default())),
    )
}
