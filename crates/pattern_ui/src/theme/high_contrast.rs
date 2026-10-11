//! High-contrast palette (FR-039).

use egui::Color32;

use super::tokens::Colors;

/// High-contrast theme colours.
pub const COLORS: Colors = Colors {
    background: Color32::from_rgb(0, 0, 0),
    panel: Color32::from_rgb(0, 0, 0),
    canvas: Color32::from_rgb(0, 0, 0),
    grid: Color32::from_rgb(90, 90, 90),
    ruler_bg: Color32::from_rgb(0, 0, 0),
    text: Color32::from_rgb(255, 255, 255),
    text_muted: Color32::from_rgb(220, 220, 220),
    accent: Color32::from_rgb(0, 220, 255),
    focus: Color32::from_rgb(255, 255, 0),
    border: Color32::from_rgb(255, 255, 255),
    selection: Color32::from_rgb(255, 255, 0),
    warning: Color32::from_rgb(255, 220, 0),
    error: Color32::from_rgb(255, 80, 80),
    stitch: Color32::from_rgb(255, 255, 255),
    cut: Color32::from_rgb(0, 255, 0),
    mark: Color32::from_rgb(0, 255, 255),
    label: Color32::from_rgb(255, 150, 255),
    placeholder: Color32::from_rgb(0, 220, 255),
};
