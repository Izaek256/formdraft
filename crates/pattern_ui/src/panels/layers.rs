//! The layers panel: visibility toggles for what the canvas really draws.
//!
//! Rows exist only for content this build renders (the placeholder outline,
//! the grid and the rulers). Future roles appear here when they draw.

use egui::{RichText, Ui};

use crate::theme::Tokens;

/// Something the layers panel asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayersEvent {
    /// Toggle the piece outline.
    ToggleOutline,
    /// Toggle the grid.
    ToggleGrid,
    /// Toggle the rulers.
    ToggleRulers,
}

/// Draw the layers panel and return the events the user triggered.
///
/// `outline_enabled` is false while there is no shape, which greys the
/// outline row instead of pretending a layer exists.
pub fn draw(
    ui: &mut Ui,
    outline_visible: bool,
    outline_enabled: bool,
    show_grid: bool,
    show_rulers: bool,
    tokens: &Tokens,
) -> Vec<LayersEvent> {
    let mut events = Vec::new();
    let mut shown = outline_visible;
    ui.add_enabled_ui(outline_enabled, |ui| {
        if ui.checkbox(&mut shown, "Outline").changed() {
            events.push(LayersEvent::ToggleOutline);
        }
    });
    let mut grid = show_grid;
    if ui.checkbox(&mut grid, "Grid").changed() {
        events.push(LayersEvent::ToggleGrid);
    }
    let mut rulers = show_rulers;
    if ui.checkbox(&mut rulers, "Rulers").changed() {
        events.push(LayersEvent::ToggleRulers);
    }
    ui.label(
        RichText::new("Stitch path, cut path and balance marks arrive with the drafting slices.")
            .size(tokens.sizes.font_small)
            .color(tokens.colors.text_muted),
    );
    events
}
