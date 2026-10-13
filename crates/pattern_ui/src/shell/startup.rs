//! The branded startup splash: logo, product name, real version and genuine
//! staged progress.
//!
//! Progress is discrete steps over real work, never a fabricated percentage:
//! thresholds presence, demo-profile presence, and first canvas fit. Each step
//! runs on its own frame so the splash paints promptly, with a short minimum
//! display time so it reads as intentional rather than a flash.

use egui::{RichText, Ui, Vec2};

use crate::shell::titlebar::{draw_mark, APP_VERSION, PRODUCT_NAME, PRODUCT_TAGLINE};
use crate::theme::Tokens;

/// Splash steps, in order. Each names real work the app performs.
const STEPS: [&str; 3] = [
    "Loading measurement limits…",
    "Preparing the placeholder profile…",
    "Fitting the canvas…",
];

/// Minimum splash display time, in seconds.
const MIN_SPLASH_SEC: f64 = 0.7;

/// Startup progress before the workspace appears.
#[derive(Debug, Clone, Default)]
pub struct Startup {
    /// Steps finished so far.
    pub done: usize,
    /// First frame time in egui seconds, once known.
    pub started_sec: Option<f64>,
    /// Genuine problem found during startup, if any.
    pub problem: Option<String>,
}

impl Startup {
    /// Total steps.
    pub fn total() -> usize {
        STEPS.len()
    }

    /// Whether every step and the minimum display time have completed.
    pub fn is_finished(&self, now_sec: f64) -> bool {
        if self.done < Self::total() {
            return false;
        }
        self.started_sec
            .is_some_and(|start| now_sec - start >= MIN_SPLASH_SEC)
    }
}

/// Draw the splash screen. Returns the label of the step completed this frame,
/// if the caller should run it now.
pub fn draw(ui: &mut Ui, startup: &mut Startup, tokens: &Tokens) -> Option<&'static str> {
    let now_sec = ui.input(|i| i.time);
    if startup.started_sec.is_none() {
        startup.started_sec = Some(now_sec);
    }
    ui.ctx().request_repaint();

    egui::CentralPanel::default().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(tokens.sizes.spacing_l);
            let mark = Vec2::splat(tokens.sizes.icon_lg + tokens.sizes.spacing_l);
            let (rect, _) = ui.allocate_exact_size(mark, egui::Sense::hover());
            draw_mark(ui, rect, tokens);
            ui.label(
                RichText::new(PRODUCT_NAME)
                    .size(tokens.sizes.font_heading)
                    .strong(),
            );
            ui.label(
                RichText::new(PRODUCT_TAGLINE)
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.text_muted),
            );
            ui.label(
                RichText::new(format!("Version {APP_VERSION}"))
                    .size(tokens.sizes.font_small)
                    .color(tokens.colors.text_muted),
            );
            ui.add_space(tokens.sizes.spacing_m);
            for (index, step) in STEPS.iter().enumerate() {
                let (glyph, color) = if index < startup.done {
                    ("✓", tokens.colors.success)
                } else if index == startup.done {
                    ("…", tokens.colors.accent)
                } else {
                    ("·", tokens.colors.disabled)
                };
                ui.label(
                    RichText::new(format!("{glyph} {step}"))
                        .size(tokens.sizes.font_small)
                        .color(color),
                );
            }
            if let Some(problem) = &startup.problem {
                ui.label(
                    RichText::new(problem)
                        .size(tokens.sizes.font_small)
                        .color(tokens.colors.warning),
                );
            }
            // Indeterminate motion while work runs: a sweeping bar. Painted
            // only during startup, so idle repaints stay quiet.
            let bar_width = tokens.sizes.splash_width.min(ui.available_width());
            let bar = Vec2::new(bar_width, tokens.sizes.spacing_xs);
            let (rect, _) = ui.allocate_exact_size(bar, egui::Sense::hover());
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(tokens.sizes.corner_radius),
                tokens.colors.surface,
            );
            let span = rect.width() / 3.0;
            let phase = (now_sec * 1.5) % 2.0;
            let x = rect.min.x + (rect.width() - span) * triangle(phase);
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(x, rect.min.y),
                    egui::Vec2::new(span, rect.height()),
                ),
                egui::CornerRadius::same(tokens.sizes.corner_radius),
                tokens.colors.accent,
            );
        });
    });
    if startup.done < STEPS.len() {
        Some(STEPS[startup.done])
    } else {
        None
    }
}

/// Triangle wave in `0..=1` for the sweeping bar.
fn triangle(phase: f64) -> f32 {
    let p = if phase < 1.0 { phase } else { 2.0 - phase };
    // Safe: an animation fraction in 0..=1; f32 precision is plenty.
    p.clamp(0.0, 1.0) as f32
}
