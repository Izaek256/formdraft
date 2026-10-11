//! T-SK01-11 / AC-FR-044-2: every theme meets its contrast targets.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::theme::{contrast_ratio, tokens, ThemeKind};

#[test]
fn t_sk01_11_body_text_meets_wcag_aa_on_every_surface() {
    for kind in ThemeKind::ALL {
        let t = tokens(kind);
        let pairs = [
            (t.colors.text, t.colors.background, "text/background"),
            (t.colors.text, t.colors.panel, "text/panel"),
            (t.colors.text, t.colors.canvas, "text/canvas"),
            (t.colors.text, t.colors.ruler_bg, "text/ruler"),
        ];
        for (fg, bg, label) in pairs {
            let ratio = contrast_ratio(fg, bg);
            assert!(
                ratio >= 4.5,
                "{}: {} contrast {:.2} is below 4.5",
                kind.label(),
                label,
                ratio
            );
        }
    }
}

#[test]
fn t_sk01_11_accent_and_focus_meet_non_text_contrast() {
    for kind in ThemeKind::ALL {
        let t = tokens(kind);
        let pairs = [
            (t.colors.accent, t.colors.background, "accent/background"),
            (t.colors.accent, t.colors.panel, "accent/panel"),
            (t.colors.focus, t.colors.background, "focus/background"),
            (t.colors.focus, t.colors.panel, "focus/panel"),
        ];
        for (fg, bg, label) in pairs {
            let ratio = contrast_ratio(fg, bg);
            assert!(
                ratio >= 3.0,
                "{}: {} contrast {:.2} is below 3.0",
                kind.label(),
                label,
                ratio
            );
        }
    }
}

#[test]
fn t_sk01_11_reference_white_on_black_is_twenty_one() {
    let ratio = contrast_ratio(egui::Color32::WHITE, egui::Color32::BLACK);
    assert!((ratio - 21.0).abs() < 0.01, "ratio {ratio}");
}
