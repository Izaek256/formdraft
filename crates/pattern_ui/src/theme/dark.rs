//! Dark palette (the default theme, OQ-38).
//!
//! Charcoal workspace inspired by professional creative software. The canvas
//! stays dark so light-on-dark text keeps its contrast guarantee (T-SK01-11);
//! the paper-light canvas of the visual reference is honoured by the light
//! theme instead.

use egui::Color32;

use super::tokens::Colors;

/// Dark theme colours.
pub const COLORS: Colors = Colors {
    background: Color32::from_rgb(17, 21, 27),
    panel: Color32::from_rgb(23, 29, 37),
    surface: Color32::from_rgb(29, 37, 48),
    hover: Color32::from_rgb(34, 43, 55),
    canvas: Color32::from_rgb(12, 16, 22),
    grid: Color32::from_rgb(55, 68, 85),
    ruler_bg: Color32::from_rgb(27, 34, 44),
    text: Color32::from_rgb(240, 243, 247),
    text_muted: Color32::from_rgb(166, 176, 190),
    disabled: Color32::from_rgb(91, 102, 117),
    accent: Color32::from_rgb(40, 104, 240),
    focus: Color32::from_rgb(122, 165, 255),
    border: Color32::from_rgb(48, 58, 71),
    selection: Color32::from_rgb(36, 86, 200),
    warning: Color32::from_rgb(232, 163, 61),
    error: Color32::from_rgb(229, 96, 94),
    success: Color32::from_rgb(67, 196, 136),
    scrim: Color32::from_rgba_unmultiplied_const(3, 7, 12, 160),
    stitch: Color32::from_rgb(201, 210, 222),
    cut: Color32::from_rgb(240, 243, 247),
    mark: Color32::from_rgb(110, 200, 150),
    label: Color32::from_rgb(190, 170, 240),
    placeholder: Color32::from_rgb(110, 165, 255),
};
