//! Theme module: the one place that holds colours, sizes and stroke widths.
//!
//! Three palettes exist (dark default, light, high contrast). Every view reads
//! its colours and sizes from here. `scripts/check-ui-tokens.sh` enforces that
//! no view outside this module contains a colour literal or a hard-coded size.

pub mod tokens;

mod dark;
mod high_contrast;
mod light;

pub use tokens::{contrast_ratio, relative_luminance, Colors, Sizes, Tokens, SIZES};

/// The three selectable themes (FR-039, OQ-38).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ThemeKind {
    /// Dark theme, the default.
    #[default]
    Dark,
    /// Light theme.
    Light,
    /// High-contrast theme.
    HighContrast,
}

impl ThemeKind {
    /// All themes, in menu order.
    pub const ALL: [ThemeKind; 3] = [ThemeKind::Dark, ThemeKind::Light, ThemeKind::HighContrast];

    /// A human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            ThemeKind::Dark => "Dark",
            ThemeKind::Light => "Light",
            ThemeKind::HighContrast => "High contrast",
        }
    }

    /// The colours for this theme.
    pub fn colors(self) -> Colors {
        match self {
            ThemeKind::Dark => dark::COLORS,
            ThemeKind::Light => light::COLORS,
            ThemeKind::HighContrast => high_contrast::COLORS,
        }
    }
}

/// The full token set for a theme.
pub fn tokens(kind: ThemeKind) -> Tokens {
    Tokens {
        colors: kind.colors(),
        sizes: SIZES,
    }
}

/// Apply a theme to the egui context so every widget agrees with the tokens.
///
/// Hover, pressed and selected states come from the palette (no per-view
/// colours), corners and strokes come from the shared sizes, and the
/// separator adopts the border colour for a crisp panel look.
pub fn apply(ctx: &egui::Context, kind: ThemeKind) {
    let t = tokens(kind);
    let c = &t.colors;
    let base = match kind {
        ThemeKind::Light => egui::Visuals::light(),
        ThemeKind::Dark | ThemeKind::HighContrast => egui::Visuals::dark(),
    };
    let rounding = egui::CornerRadius::same(t.sizes.corner_radius);
    let widgets = |fill: egui::Color32, stroke_color: egui::Color32| egui::style::WidgetVisuals {
        bg_fill: fill,
        weak_bg_fill: fill,
        bg_stroke: egui::Stroke::new(t.sizes.stroke_thin, stroke_color),
        corner_radius: rounding,
        fg_stroke: egui::Stroke::new(t.sizes.stroke_medium, c.text),
        expansion: 0.0,
    };
    let visuals = egui::Visuals {
        panel_fill: c.panel,
        window_fill: c.surface,
        extreme_bg_color: c.canvas,
        faint_bg_color: c.background,
        code_bg_color: c.surface,
        override_text_color: Some(c.text),
        hyperlink_color: c.accent,
        warn_fg_color: c.warning,
        error_fg_color: c.error,
        window_corner_radius: egui::CornerRadius::same(t.sizes.corner_lg),
        window_shadow: egui::Shadow {
            offset: [0, 14],
            blur: 38,
            spread: 0,
            color: c.scrim,
        },
        popup_shadow: egui::Shadow {
            offset: [0, 8],
            blur: 24,
            spread: 0,
            color: c.scrim,
        },
        window_stroke: egui::Stroke::new(t.sizes.stroke_thin, c.border),
        widgets: egui::style::Widgets {
            noninteractive: widgets(c.panel, c.border),
            inactive: widgets(c.surface, c.border),
            hovered: widgets(c.hover, c.focus),
            active: widgets(c.selection, c.focus),
            open: widgets(c.hover, c.border),
        },
        selection: egui::style::Selection {
            bg_fill: c.selection,
            stroke: egui::Stroke::new(t.sizes.stroke_thin, c.focus),
        },
        ..base
    };
    ctx.set_visuals(visuals);
}
