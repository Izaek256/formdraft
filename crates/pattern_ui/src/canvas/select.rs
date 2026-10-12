//! Selection model for the canvas (T-SK01-18). Pure: no egui.

/// An axis-aligned box in millimetres, y down.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    /// Minimum corner (top-left).
    pub min_mm: [f64; 2],
    /// Maximum corner (bottom-right).
    pub max_mm: [f64; 2],
}

impl Bounds {
    /// Build a box from two opposite corners.
    pub fn new(a: [f64; 2], b: [f64; 2]) -> Self {
        Self {
            min_mm: [a[0].min(b[0]), a[1].min(b[1])],
            max_mm: [a[0].max(b[0]), a[1].max(b[1])],
        }
    }

    /// Width and height in millimetres.
    pub fn size_mm(&self) -> [f64; 2] {
        [
            self.max_mm[0] - self.min_mm[0],
            self.max_mm[1] - self.min_mm[1],
        ]
    }

    /// The centre of the box.
    pub fn center_mm(&self) -> [f64; 2] {
        [
            (self.min_mm[0] + self.max_mm[0]) / 2.0,
            (self.min_mm[1] + self.max_mm[1]) / 2.0,
        ]
    }

    /// Whether a millimetre point lies inside the box (edges included).
    pub fn contains(&self, point_mm: [f64; 2]) -> bool {
        point_mm[0] >= self.min_mm[0]
            && point_mm[0] <= self.max_mm[0]
            && point_mm[1] >= self.min_mm[1]
            && point_mm[1] <= self.max_mm[1]
    }
}

/// A selected piece and the label shown with it.
#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    /// The selected box.
    pub bounds: Bounds,
    /// The piece label.
    pub label: String,
}
