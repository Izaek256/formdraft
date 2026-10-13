//! The bottom status bar: cursor, mode, selection, unit, grid, snap and zoom.
//!
//! All text is preformatted by the caller through the unit service. The unit
//! pill cycles the canvas display unit; the zoom slider sets an exact zoom.

use egui::{RichText, Ui};

use crate::theme::Tokens;

/// What the status bar displays. Text is already formatted by the caller
/// through the unit service.
#[derive(Debug, Clone)]
pub struct StatusBarState {
    /// Cursor position in the display unit, or `-`.
    pub cursor: String,
    /// Current mode, e.g. `Select` or `Hand`.
    pub mode: String,
    /// Selection summary, e.g. `1 selected` or `Nothing selected`.
    pub selection: String,
    /// Display unit pill, e.g. `Unit: mm`.
    pub unit: String,
    /// Grid pill, e.g. `Grid: 10 mm`.
    pub grid: String,
    /// Whether snap is preferred.
    pub snap: bool,
    /// Zoom as a percentage of approximate actual size.
    pub zoom_percent: f32,
    /// Whether the limits-not-configured notice should show.
    pub limits_notice: bool,
}

/// Something the status bar asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusBarEvent {
    /// Cycle the display unit.
    CycleUnit,
    /// Set an exact zoom percentage.
    SetZoom(f32),
}

/// Draw the status bar and return the events the user triggered.
pub fn draw(ui: &mut Ui, state: &StatusBarState, tokens: &Tokens) -> Vec<StatusBarEvent> {
    let mut events = Vec::new();
    ui.horizontal(|ui| {
        ui.label(RichText::new(&state.cursor).size(tokens.sizes.font_small));
        ui.separator();
        ui.label(RichText::new(&state.mode).size(tokens.sizes.font_small));
        ui.separator();
        ui.label(RichText::new(&state.selection).size(tokens.sizes.font_small));
        if state.limits_notice {
            ui.separator();
            ui.label(
                RichText::new("limits not configured")
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.warning),
            );
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{:.0}%", state.zoom_percent)).size(tokens.sizes.font_small),
            );
            let mut zoom = state.zoom_percent;
            let slider = egui::Slider::new(&mut zoom, 25.0..=400.0)
                .show_value(false)
                .trailing_fill(true);
            if ui
                .add(slider)
                .on_hover_text("Canvas zoom (25–400% of approximate actual size)")
                .changed()
            {
                events.push(StatusBarEvent::SetZoom(zoom));
            }
            ui.separator();
            ui.label(
                RichText::new(if state.snap { "Snap: On" } else { "Snap: Off" })
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.text_muted),
            );
            ui.separator();
            ui.label(
                RichText::new(&state.grid)
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.text_muted),
            );
            ui.separator();
            if ui
                .button(RichText::new(&state.unit).size(tokens.sizes.font_small))
                .on_hover_text("Cycle the display unit (mm, cm, in)")
                .clicked()
            {
                events.push(StatusBarEvent::CycleUnit);
            }
        });
    });
    events
}
