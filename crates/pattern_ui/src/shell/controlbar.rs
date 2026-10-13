//! The center toolbar: canvas tools, zoom, grid readout and view toggles.
//!
//! Every control reflects real state. The grid pill names the live grid step
//! (precomputed by the caller through the unit service); the snap checkbox
//! stores a remembered preference — point snapping itself arrives in R2, and
//! the tooltip says so.

use egui::{RichText, Ui};

use crate::shell::toolstrip::{self, Tool};
use crate::theme::Tokens;

/// What the toolbar displays. All text is preformatted by the caller.
#[derive(Debug, Clone)]
pub struct ToolBarState {
    /// The active canvas tool.
    pub tool: Tool,
    /// Zoom read-out, e.g. `100%`.
    pub zoom: String,
    /// Grid read-out, e.g. `Grid: 10 mm`.
    pub grid: String,
    /// Whether the grid is shown.
    pub show_grid: bool,
    /// Whether the rulers are shown.
    pub show_rulers: bool,
    /// Whether snap is preferred.
    pub snap: bool,
}

/// Something the toolbar asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolBarEvent {
    /// Activate a canvas tool.
    UseTool(Tool),
    /// Zoom in.
    ZoomIn,
    /// Zoom out.
    ZoomOut,
    /// Fit to window.
    Fit,
    /// Approximate actual size.
    ActualSize,
    /// Toggle the grid.
    ToggleGrid,
    /// Toggle the rulers.
    ToggleRulers,
    /// Toggle the snap preference.
    ToggleSnap,
}

/// Draw the toolbar and return the events the user triggered.
pub fn draw(ui: &mut Ui, state: &ToolBarState, tokens: &Tokens) -> Vec<ToolBarEvent> {
    let mut events = Vec::new();
    ui.horizontal(|ui| {
        for tool in Tool::ALL {
            if toolstrip::tool_button(ui, tool, state.tool, tokens).clicked() {
                events.push(ToolBarEvent::UseTool(tool));
            }
        }
        ui.separator();
        if ui
            .button(RichText::new("−").size(tokens.sizes.font_body))
            .on_hover_text("Zoom out (Ctrl+-)")
            .clicked()
        {
            events.push(ToolBarEvent::ZoomOut);
        }
        ui.label(RichText::new(&state.zoom).size(tokens.sizes.font_body));
        if ui
            .button(RichText::new("+").size(tokens.sizes.font_body))
            .on_hover_text("Zoom in (Ctrl++)")
            .clicked()
        {
            events.push(ToolBarEvent::ZoomIn);
        }
        if ui
            .button("Fit")
            .on_hover_text("Fit to window (Ctrl+0)")
            .clicked()
        {
            events.push(ToolBarEvent::Fit);
        }
        if ui
            .button("1:1")
            .on_hover_text("Approximate actual size (Ctrl+1)")
            .clicked()
        {
            events.push(ToolBarEvent::ActualSize);
        }
        ui.separator();
        if ui
            .button(RichText::new(&state.grid).size(tokens.sizes.font_small))
            .on_hover_text("Toggle the grid")
            .clicked()
        {
            events.push(ToolBarEvent::ToggleGrid);
        }
        let mut snap = state.snap;
        if ui
            .checkbox(&mut snap, "Snap")
            .on_hover_text("Snap preference (point snapping arrives in R2)")
            .changed()
        {
            events.push(ToolBarEvent::ToggleSnap);
        }
        let mut grid = state.show_grid;
        if ui.checkbox(&mut grid, "Show Grid").changed() {
            events.push(ToolBarEvent::ToggleGrid);
        }
        let mut rulers = state.show_rulers;
        if ui.checkbox(&mut rulers, "Show Rulers").changed() {
            events.push(ToolBarEvent::ToggleRulers);
        }
    });
    events
}
