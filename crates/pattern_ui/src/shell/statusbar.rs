//! The bottom status bar: zoom, cursor, selection, the limits notice and mode.

use egui::{RichText, Ui};

use crate::theme::Tokens;

/// What the status bar displays. Text is already formatted by the caller
/// through the unit service.
#[derive(Debug, Clone)]
pub struct StatusBarState {
    /// Zoom read-out, e.g. `100% (approx)`.
    pub zoom: String,
    /// Cursor position in the display unit, or `-`.
    pub cursor: String,
    /// Selection summary, or `-`.
    pub selection: String,
    /// The method and edition notice.
    pub method: String,
    /// Whether the limits-not-configured notice should show.
    pub limits_notice: bool,
}

/// Draw the status bar.
pub fn draw(ui: &mut Ui, state: &StatusBarState, tokens: &Tokens) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(&state.zoom).size(tokens.sizes.font_small));
        ui.separator();
        ui.label(RichText::new(format!("cursor {}", state.cursor)).size(tokens.sizes.font_small));
        ui.separator();
        ui.label(
            RichText::new(format!("selection {}", state.selection)).size(tokens.sizes.font_small),
        );
        if state.limits_notice {
            ui.separator();
            ui.label(
                RichText::new("limits not configured")
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.warning),
            );
        }
        ui.separator();
        ui.label(
            RichText::new(&state.method)
                .size(tokens.sizes.font_small)
                .color(tokens.colors.text_muted),
        );
    });
}
