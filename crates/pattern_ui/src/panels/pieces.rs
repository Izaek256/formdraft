//! The pieces panel: one row per drawable piece, with visibility and
//! selection, plus the placeholder notice.
//!
//! Rows come only from real canvas content: when the form is incomplete there
//! is no shape, and the panel says so instead of showing demo rows.

use egui::{RichText, Ui, Vec2};

use crate::canvas::roles::{self, Role};
use crate::placeholder;
use crate::theme::Tokens;

/// Something the pieces panel asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiecesEvent {
    /// Toggle the visibility of the piece.
    ToggleVisibility,
    /// Select the piece on the canvas.
    Select,
}

/// Draw the pieces panel and return the events the user triggered.
pub fn draw(
    ui: &mut Ui,
    name: &str,
    has_shape: bool,
    visible: &mut bool,
    selected: bool,
    size_text: &str,
    tokens: &Tokens,
) -> Vec<PiecesEvent> {
    let mut events = Vec::new();
    if !has_shape {
        ui.label(
            RichText::new("No shape: fill every field with a positive number.")
                .size(tokens.sizes.font_small)
                .color(tokens.colors.text_muted),
        );
    } else {
        let row = egui::Button::new("")
            .selected(selected)
            .min_size(Vec2::new(ui.available_width(), tokens.sizes.control_height));
        let response = ui.add(row).on_hover_text("Select the piece on the canvas");
        // Paint the row content over the button: swatch, name and eye state.
        let painter = ui.painter_at(response.rect);
        let swatch = egui::Rect::from_min_size(
            egui::pos2(
                response.rect.min.x + tokens.sizes.spacing_s,
                response.rect.center().y - tokens.sizes.spacing_xs,
            ),
            Vec2::splat(tokens.sizes.spacing_s),
        );
        painter.rect_filled(
            swatch,
            egui::CornerRadius::same(tokens.sizes.corner_radius),
            roles::color(Role::Placeholder, tokens),
        );
        painter.text(
            egui::pos2(
                swatch.max.x + tokens.sizes.spacing_s,
                response.rect.center().y,
            ),
            egui::Align2::LEFT_CENTER,
            name,
            egui::FontId::proportional(tokens.sizes.font_small),
            tokens.colors.text,
        );
        painter.text(
            egui::pos2(
                response.rect.max.x - tokens.sizes.spacing_s,
                response.rect.center().y,
            ),
            egui::Align2::RIGHT_CENTER,
            if *visible { "Shown" } else { "Hidden" },
            egui::FontId::proportional(tokens.sizes.font_small),
            tokens.colors.text_muted,
        );
        if response.clicked() {
            events.push(PiecesEvent::Select);
        }
        ui.horizontal(|ui| {
            let mut shown = *visible;
            if ui.checkbox(&mut shown, "Visible").changed() {
                events.push(PiecesEvent::ToggleVisibility);
            }
        });
        ui.label(RichText::new(size_text).size(tokens.sizes.font_small));
    }
    ui.separator();
    ui.label(
        RichText::new(placeholder::NOTICE)
            .size(tokens.sizes.font_small)
            .color(tokens.colors.warning),
    );
    events
}
