//! Pointer navigation: pan, zoom to cursor and click selection.
//!
//! Keyboard navigation lives in [`crate::canvas::Canvas::handle_action`]; this
//! module handles the mouse only, so the mapping stays easy to test.

use egui::{PointerButton, Response, Ui};

use crate::canvas::{Canvas, ScreenMap};
use crate::shell::Tool;

/// Wheel sensitivity: factor per unit of scroll.
const WHEEL_ZOOM_RATE: f64 = 0.0015;
/// Factor applied when the Zoom tool is clicked.
const CLICK_ZOOM_FACTOR: f64 = 1.25;

/// Apply pointer input to the canvas. `map` is a snapshot of the view before
/// this call.
pub fn interact(ui: &Ui, response: &Response, map: &ScreenMap, canvas: &mut Canvas, tool: Tool) {
    let space_down = ui.input(|i| i.key_down(egui::Key::Space));

    let hand_drag =
        response.dragged_by(PointerButton::Primary) && (space_down || tool == Tool::Hand);
    let middle_drag = response.dragged_by(PointerButton::Middle);
    if hand_drag || middle_drag {
        let delta = response.drag_delta();
        canvas
            .viewport
            .pan_by([f64::from(delta.x), f64::from(delta.y)]);
    }

    if response.hovered() {
        let scroll = ui.input(|i| i.smooth_scroll_delta.y);
        if scroll.abs() > 0.0 {
            if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                let factor = (f64::from(scroll) * WHEEL_ZOOM_RATE).exp();
                canvas.viewport.zoom_about(factor, map.to_local(pos));
            }
        }
    }

    if response.clicked() && !space_down {
        if let Some(pos) = response.interact_pointer_pos() {
            match tool {
                Tool::Select => canvas.select_at(map.to_world(pos)),
                Tool::Zoom => {
                    canvas
                        .viewport
                        .zoom_about(CLICK_ZOOM_FACTOR, map.to_local(pos));
                }
                Tool::Hand => {}
            }
        }
    }
}
