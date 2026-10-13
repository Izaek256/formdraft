//! Light palette.
//!
//! The light theme carries the paper canvas of the visual reference: a pale
//! working sheet on a neutral grey frame.

use egui::Color32;

use super::tokens::Colors;

/// Light theme colours.
pub const COLORS: Colors = Colors {
    background: Color32::from_rgb(232, 234, 238),
    panel: Color32::from_rgb(240, 241, 243),
    surface: Color32::from_rgb(255, 255, 255),
    hover: Color32::from_rgb(228, 232, 238),
    canvas: Color32::from_rgb(245, 247, 250),
    grid: Color32::from_rgb(208, 214, 222),
    ruler_bg: Color32::from_rgb(229, 233, 238),
    text: Color32::from_rgb(25, 25, 25),
    text_muted: Color32::from_rgb(90, 90, 90),
    disabled: Color32::from_rgb(150, 157, 166),
    accent: Color32::from_rgb(10, 90, 190),
    focus: Color32::from_rgb(0, 70, 160),
    border: Color32::from_rgb(200, 200, 205),
    selection: Color32::from_rgb(190, 110, 0),
    warning: Color32::from_rgb(140, 90, 0),
    error: Color32::from_rgb(180, 30, 30),
    success: Color32::from_rgb(30, 126, 70),
    scrim: Color32::from_rgba_unmultiplied_const(20, 30, 45, 90),
    stitch: Color32::from_rgb(30, 30, 30),
    cut: Color32::from_rgb(10, 10, 10),
    mark: Color32::from_rgb(20, 110, 20),
    label: Color32::from_rgb(110, 40, 160),
    placeholder: Color32::from_rgb(30, 90, 200),
};
