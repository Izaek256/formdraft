//! The panel columns: visibility models for the working panels.
//!
//! The model is pure. `Ctrl+Shift+H` toggles every column; hiding a panel
//! never touches any other model.

/// Which panels are visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanelsModel {
    /// The properties panel.
    pub properties: bool,
    /// The pieces panel.
    pub pieces: bool,
    /// The warnings panel.
    pub warnings: bool,
    /// The left column (rail and pattern tools).
    pub left: bool,
}

impl Default for PanelsModel {
    fn default() -> Self {
        Self {
            properties: true,
            pieces: true,
            warnings: true,
            left: true,
        }
    }
}

impl PanelsModel {
    /// True when every panel is visible.
    pub fn all_visible(&self) -> bool {
        self.properties && self.pieces && self.warnings && self.left
    }

    /// True when at least one panel is visible.
    pub fn any_visible(&self) -> bool {
        self.properties || self.pieces || self.warnings || self.left
    }

    /// True when at least one right-hand inspector panel is visible.
    pub fn any_inspector_visible(&self) -> bool {
        self.properties || self.pieces || self.warnings
    }

    /// Hide every visible panel, or show every panel when all are hidden.
    pub fn toggle_all(&mut self) {
        let show = !self.all_visible();
        self.properties = show;
        self.pieces = show;
        self.warnings = show;
        self.left = show;
    }
}
