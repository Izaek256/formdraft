//! Toast notifications: short confirmations for real events.
//!
//! The model is pure: [`Toasts`] holds pending messages with their creation
//! time in egui seconds. Views push on events only (tool change, toggle,
//! validation summary) and prune every frame. Nothing here fabricates
//! progress; a toast always answers something the user just did.

use egui::{Align2, Ui};

use crate::theme::Tokens;

/// How long a toast stays visible, in seconds.
const TOAST_TTL_SEC: f64 = 2.4;

/// One pending notification.
#[derive(Debug, Clone)]
pub struct Toast {
    /// The message text.
    pub text: String,
    /// Creation time in egui seconds.
    pub created_sec: f64,
}

/// Pending toasts, oldest first.
#[derive(Debug, Clone, Default)]
pub struct Toasts {
    items: Vec<Toast>,
}

impl Toasts {
    /// Queue a message. Consecutive duplicates refresh the timer instead of
    /// stacking, so repeated shortcut presses do not flood the corner.
    pub fn push(&mut self, now_sec: f64, text: String) {
        self.prune(now_sec);
        if self.items.last().is_some_and(|t| t.text == text) {
            if let Some(last) = self.items.last_mut() {
                last.created_sec = now_sec;
            }
            return;
        }
        self.items.push(Toast {
            text,
            created_sec: now_sec,
        });
        while self.items.len() > 3 {
            self.items.remove(0);
        }
    }

    /// Drop expired messages.
    pub fn prune(&mut self, now_sec: f64) {
        self.items
            .retain(|t| now_sec - t.created_sec < TOAST_TTL_SEC);
    }

    /// Whether any toast is visible.
    pub fn any_visible(&self) -> bool {
        !self.items.is_empty()
    }

    /// The visible messages, oldest first.
    pub fn messages(&self) -> Vec<String> {
        self.items.iter().map(|t| t.text.clone()).collect()
    }
}

/// Draw the toast stack in the bottom-right corner with a short fade-in.
///
/// The caller must request a repaint while toasts are visible; [`Toasts::draw`]
/// reports whether one is needed through its return value.
pub fn draw(ui: &mut Ui, toasts: &mut Toasts, tokens: &Tokens) -> bool {
    let now_sec = ui.input(|i| i.time);
    toasts.prune(now_sec);
    if !toasts.any_visible() {
        return false;
    }
    let area = egui::Area::new(egui::Id::new("toasts"))
        .anchor(
            Align2::RIGHT_BOTTOM,
            egui::Vec2::splat(tokens.sizes.spacing_l),
        )
        .order(egui::Order::Foreground);
    let messages = toasts.messages();
    area.show(ui.ctx(), |ui| {
        ui.vertical(|ui| {
            for message in &messages {
                egui::Frame::new()
                    .fill(tokens.colors.surface)
                    .stroke(egui::Stroke::new(
                        tokens.sizes.stroke_thin,
                        tokens.colors.border,
                    ))
                    .corner_radius(egui::CornerRadius::same(tokens.sizes.corner_lg))
                    .inner_margin(tokens.sizes.spacing_m)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(message).size(tokens.sizes.font_small));
                    });
            }
        });
    });
    true
}
