//! The right-hand panel column: visibility model for the working panels.
//!
//! The model is pure. `Ctrl+Shift+H` toggles the whole column; individual
//! toggles are added later. Hiding a panel never touches any other model.

/// Which right-hand panels are visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanelsModel {
    /// The properties panel.
    pub properties: bool,
    /// The pieces panel.
    pub pieces: bool,
    /// The warnings panel.
    pub warnings: bool,
}

impl Default for PanelsModel {
    fn default() -> Self {
        Self {
            properties: true,
            pieces: true,
            warnings: true,
        }
    }
}

impl PanelsModel {
    /// True when every panel in the column is visible.
    pub fn all_visible(&self) -> bool {
        self.properties && self.pieces && self.warnings
    }

    /// True when at least one panel is visible.
    pub fn any_visible(&self) -> bool {
        self.properties || self.pieces || self.warnings
    }

    /// Hide every visible panel, or show every panel when all are hidden.
    pub fn toggle_all(&mut self) {
        let show = !self.all_visible();
        self.properties = show;
        self.pieces = show;
        self.warnings = show;
    }
}
