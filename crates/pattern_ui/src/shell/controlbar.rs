//! The control bar under the menu: tool, zoom and the always-available view controls.

use egui::{RichText, Ui};

use crate::theme::Tokens;

/// What the control bar displays.
#[derive(Debug, Clone, Copy)]
pub struct ControlBarState {
    /// Current zoom, as a percentage.
    pub zoom_percent: f32,
    /// Whether the view is at approximate actual size.
    pub actual_size: bool,
}

/// Something the control bar asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlBarEvent {
    /// Zoom in.
    ZoomIn,
    /// Zoom out.
    ZoomOut,
    /// Fit to window.
    Fit,
    /// Approximate actual size.
    ActualSize,
}

/// Draw the control bar and return the events the user triggered.
pub fn draw(ui: &mut Ui, state: &ControlBarState, tokens: &Tokens) -> Vec<ControlBarEvent> {
    let mut events = Vec::new();
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(format!("Zoom: {:.0}%", state.zoom_percent)).size(tokens.sizes.font_body),
        );
        if ui.button("Zoom in").clicked() {
            events.push(ControlBarEvent::ZoomIn);
        }
        if ui.button("Zoom out").clicked() {
            events.push(ControlBarEvent::ZoomOut);
        }
        if ui.button("Fit").clicked() {
            events.push(ControlBarEvent::Fit);
        }
        if ui.button("Actual size").clicked() {
            events.push(ControlBarEvent::ActualSize);
        }
        if state.actual_size {
            ui.label(
                RichText::new("approximate")
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.text_muted),
            );
        }
    });
    events
}
