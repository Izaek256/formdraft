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
pub fn apply(ctx: &egui::Context, kind: ThemeKind) {
    let t = tokens(kind);
    let base = match kind {
        ThemeKind::Light => egui::Visuals::light(),
        ThemeKind::Dark | ThemeKind::HighContrast => egui::Visuals::dark(),
    };
    let visuals = egui::Visuals {
        panel_fill: t.colors.panel,
        window_fill: t.colors.panel,
        extreme_bg_color: t.colors.canvas,
        faint_bg_color: t.colors.background,
        override_text_color: Some(t.colors.text),
        hyperlink_color: t.colors.accent,
        warn_fg_color: t.colors.warning,
        error_fg_color: t.colors.error,
        selection: egui::style::Selection {
            bg_fill: t.colors.selection,
            stroke: egui::Stroke::new(t.sizes.stroke_thin, t.colors.focus),
        },
        ..base
    };
    ctx.set_visuals(visuals);
}
