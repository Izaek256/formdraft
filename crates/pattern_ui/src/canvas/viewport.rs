//! Screen-to-millimetre transform for the canvas.
//!
//! World coordinates are millimetres with y increasing downward, matching the
//! pattern coordinates from `pattern_core::geom`. Screen coordinates are egui
//! points, also y down, so the transform is a uniform scale plus a translation.
//!
//! The type is pure: no egui, no clock, no randomness. It is unit tested
//! (T-SK01-08, T-SK01-14).

/// Pixels per inch assumed for the approximate actual size (OQ-37).
pub const NOMINAL_PPI: f64 = 96.0;

/// Millimetres in an inch.
const MM_PER_INCH: f64 = 25.4;

/// Smallest allowed scale.
const MIN_PX_PER_MM: f64 = 0.02;
/// Largest allowed scale.
const MAX_PX_PER_MM: f64 = 500.0;

/// The default scale when a viewport is created: a comfortable overview.
const DEFAULT_PX_PER_MM: f64 = 2.0;

/// A uniform scale and translation mapping millimetres to screen points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    px_per_mm: f64,
    origin_px: [f64; 2],
    size_px: [f64; 2],
}

impl Viewport {
    /// A viewport of the given size, centred on the world origin.
    pub fn new(size_px: [f64; 2]) -> Self {
        let size = [size_px[0].max(1.0), size_px[1].max(1.0)];
        Self {
            px_per_mm: DEFAULT_PX_PER_MM,
            origin_px: [size[0] / 2.0, size[1] / 2.0],
            size_px: size,
        }
    }

    /// The current scale in pixels per millimetre.
    pub fn px_per_mm(self) -> f64 {
        self.px_per_mm
    }

    /// The screen position of the world origin, in points.
    pub fn origin_px(self) -> [f64; 2] {
        self.origin_px
    }

    /// The viewport size in points.
    pub fn size_px(self) -> [f64; 2] {
        self.size_px
    }

    /// Update the viewport size, keeping the scale and origin.
    pub fn set_size(&mut self, size_px: [f64; 2]) {
        self.size_px = [size_px[0].max(1.0), size_px[1].max(1.0)];
    }

    /// The scale that makes the drawing approximate actual size.
    pub fn actual_size_scale(ppi: f64) -> f64 {
        ppi / MM_PER_INCH
    }

    /// Map millimetres to screen points (y down).
    pub fn world_to_screen(self, x_mm: f64, y_mm: f64) -> [f64; 2] {
        [
            self.origin_px[0] + x_mm * self.px_per_mm,
            self.origin_px[1] + y_mm * self.px_per_mm,
        ]
    }

    /// Map screen points back to millimetres (inverse of [`world_to_screen`]).
    pub fn screen_to_world(self, x_px: f64, y_px: f64) -> [f64; 2] {
        [
            (x_px - self.origin_px[0]) / self.px_per_mm,
            (y_px - self.origin_px[1]) / self.px_per_mm,
        ]
    }

    /// Zoom by `factor` about a screen point, keeping the world point under
    /// that cursor fixed (T-SK01-14).
    pub fn zoom_about(&mut self, factor: f64, cursor_px: [f64; 2]) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let world = self.screen_to_world(cursor_px[0], cursor_px[1]);
        let new_scale = (self.px_per_mm * factor).clamp(MIN_PX_PER_MM, MAX_PX_PER_MM);
        self.px_per_mm = new_scale;
        self.origin_px = [
            cursor_px[0] - world[0] * new_scale,
            cursor_px[1] - world[1] * new_scale,
        ];
    }

    /// Pan the view by a screen delta, in points.
    pub fn pan_by(&mut self, delta_px: [f64; 2]) {
        self.origin_px[0] += delta_px[0];
        self.origin_px[1] += delta_px[1];
    }

    /// Set the scale to approximate actual size, about the viewport centre.
    pub fn set_actual_size(&mut self, ppi: f64) {
        let centre = [self.size_px[0] / 2.0, self.size_px[1] / 2.0];
        let target = Self::actual_size_scale(ppi);
        let factor = target / self.px_per_mm;
        self.zoom_about(factor, centre);
    }

    /// Fit an axis-aligned millimetre box inside the viewport with a margin.
    pub fn fit(&mut self, min_mm: [f64; 2], max_mm: [f64; 2], margin_px: f64) {
        let width_mm = (max_mm[0] - min_mm[0]).abs().max(f64::EPSILON);
        let height_mm = (max_mm[1] - min_mm[1]).abs().max(f64::EPSILON);
        let avail_w = (self.size_px[0] - 2.0 * margin_px).max(1.0);
        let avail_h = (self.size_px[1] - 2.0 * margin_px).max(1.0);
        let scale = (avail_w / width_mm)
            .min(avail_h / height_mm)
            .clamp(MIN_PX_PER_MM, MAX_PX_PER_MM);
        self.px_per_mm = scale;
        let centre_x = (min_mm[0] + max_mm[0]) / 2.0;
        let centre_y = (min_mm[1] + max_mm[1]) / 2.0;
        self.origin_px = [
            self.size_px[0] / 2.0 - centre_x * scale,
            self.size_px[1] / 2.0 - centre_y * scale,
        ];
    }

    /// The zoom as a percentage of approximate actual size (100% = 96 ppi).
    pub fn zoom_percent(self) -> f64 {
        self.px_per_mm / Self::actual_size_scale(NOMINAL_PPI) * 100.0
    }
}
