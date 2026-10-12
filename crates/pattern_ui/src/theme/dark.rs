//! Dark palette (the default theme, OQ-38).

use egui::Color32;

use super::tokens::Colors;

/// Dark theme colours.
pub const COLORS: Colors = Colors {
    background: Color32::from_rgb(30, 30, 30),
    panel: Color32::from_rgb(37, 37, 38),
    canvas: Color32::from_rgb(24, 24, 24),
    grid: Color32::from_rgb(60, 60, 60),
    ruler_bg: Color32::from_rgb(45, 45, 48),
    text: Color32::from_rgb(230, 230, 230),
    text_muted: Color32::from_rgb(170, 170, 170),
    accent: Color32::from_rgb(90, 170, 255),
    focus: Color32::from_rgb(120, 190, 255),
    border: Color32::from_rgb(70, 70, 72),
    selection: Color32::from_rgb(255, 180, 60),
    warning: Color32::from_rgb(240, 190, 80),
    error: Color32::from_rgb(240, 110, 110),
    stitch: Color32::from_rgb(200, 200, 200),
    cut: Color32::from_rgb(245, 245, 245),
    mark: Color32::from_rgb(150, 210, 150),
    label: Color32::from_rgb(200, 180, 240),
    placeholder: Color32::from_rgb(120, 180, 240),
};
