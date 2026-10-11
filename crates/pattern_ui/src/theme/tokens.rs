//! Design tokens: the single source of every colour, size and stroke width.
//!
//! Views must read tokens from here. `scripts/check-ui-tokens.sh` fails when a
//! view outside this module contains a colour literal or a hard-coded size.
//! Contrast helpers implement WCAG 2.x relative luminance.

use egui::Color32;

/// Named colours for one palette. No view may bypass these.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colors {
    /// Window and menu-bar background.
    pub background: Color32,
    /// Panel background.
    pub panel: Color32,
    /// Canvas background.
    pub canvas: Color32,
    /// Grid line colour.
    pub grid: Color32,
    /// Ruler strip background.
    pub ruler_bg: Color32,
    /// Primary text.
    pub text: Color32,
    /// Secondary or disabled text.
    pub text_muted: Color32,
    /// Accent used for controls.
    pub accent: Color32,
    /// Focus ring colour.
    pub focus: Color32,
    /// Dividing lines and outlines.
    pub border: Color32,
    /// Selection highlight.
    pub selection: Color32,
    /// Warning severity.
    pub warning: Color32,
    /// Error severity.
    pub error: Color32,
    /// Stitch-path role colour.
    pub stitch: Color32,
    /// Cut-path role colour.
    pub cut: Color32,
    /// Construction-mark role colour.
    pub mark: Color32,
    /// Label role colour.
    pub label: Color32,
    /// Placeholder shape colour.
    pub placeholder: Color32,
}

/// Spacing, dimension and stroke tokens, shared by every palette.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sizes {
    /// Extra-small spacing.
    pub spacing_xs: f32,
    /// Small spacing.
    pub spacing_s: f32,
    /// Medium spacing.
    pub spacing_m: f32,
    /// Large spacing.
    pub spacing_l: f32,
    /// Width of the left tool strip.
    pub tool_strip_width: f32,
    /// Width of the right panel column.
    pub panel_width: f32,
    /// Height of the status bar.
    pub status_bar_height: f32,
    /// Thickness of a ruler strip.
    pub ruler_thickness: f32,
    /// Thin stroke width.
    pub stroke_thin: f32,
    /// Medium stroke width.
    pub stroke_medium: f32,
    /// Thick stroke width.
    pub stroke_thick: f32,
    /// Small font size.
    pub font_small: f32,
    /// Body font size.
    pub font_body: f32,
    /// Heading font size.
    pub font_heading: f32,
    /// Corner radius for framed widgets.
    pub corner_radius: u8,
    /// Icon square size in the tool strip.
    pub icon_size: f32,
}

/// Shared sizes for every palette.
pub const SIZES: Sizes = Sizes {
    spacing_xs: 4.0,
    spacing_s: 6.0,
    spacing_m: 8.0,
    spacing_l: 12.0,
    tool_strip_width: 40.0,
    panel_width: 280.0,
    status_bar_height: 24.0,
    ruler_thickness: 20.0,
    stroke_thin: 1.0,
    stroke_medium: 1.5,
    stroke_thick: 2.5,
    font_small: 11.0,
    font_body: 13.0,
    font_heading: 16.0,
    corner_radius: 3,
    icon_size: 20.0,
};

/// A complete set of tokens for one theme.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tokens {
    /// The palette colours.
    pub colors: Colors,
    /// The shared sizes.
    pub sizes: Sizes,
}

/// WCAG relative luminance of a colour, in `0.0..=1.0`.
pub fn relative_luminance(color: Color32) -> f64 {
    fn channel(value: u8) -> f64 {
        let s = f64::from(value) / 255.0;
        if s <= 0.039_28 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        }
    }
    let r = channel(color.r());
    let g = channel(color.g());
    let b = channel(color.b());
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// WCAG contrast ratio between two colours, from `1.0` to `21.0`.
pub fn contrast_ratio(a: Color32, b: Color32) -> f64 {
    let la = relative_luminance(a);
    let lb = relative_luminance(b);
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}
