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
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "formdraft",
        options,
        Box::new(|_cc| Ok(Box::<app::WorkspaceApp>::default())),
    )
}
