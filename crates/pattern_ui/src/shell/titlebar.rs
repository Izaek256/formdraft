//! The branded title bar: logo mark, product name, project button and a
//! session status pill.
//!
//! The mark is drawn in code with the painter (no assets). The status pill
//! reflects genuine session state computed by the caller (validation
//! outcome), never a file-save state, because this release has no project
//! files yet.

use egui::{Pos2, Rect, Shape, Ui, Vec2};

use crate::theme::Tokens;

/// The product name shown in the title bar and the splash screen.
pub const PRODUCT_NAME: &str = "FormDraft";
/// The product descriptor shown beside the name.
pub const PRODUCT_TAGLINE: &str = "Precision Pattern Engineering";
/// The application version from the crate metadata.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Session status shown in the title-bar pill. Computed from real state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    /// The form is incomplete; no shape is drawn.
    NeedsInput,
    /// The shape draws but validation reports problems.
    NeedsAttention,
    /// The shape draws and validation is quiet.
    Ready,
}

impl SessionStatus {
    /// A human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            SessionStatus::NeedsInput => "Needs input",
            SessionStatus::NeedsAttention => "Needs attention",
            SessionStatus::Ready => "Ready",
        }
    }
}

/// Something the title bar asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TitleBarEvent {
    /// Open the project and session dialog.
    OpenProject,
}

/// Draw the logo mark: a hexagon nut with a needle slash, in theme text.
pub fn draw_mark(ui: &Ui, rect: Rect, tokens: &Tokens) {
    let painter = ui.painter_at(rect);
    let center = rect.center();
    let radius = rect.width().min(rect.height()) * 0.42;
    let stroke = egui::Stroke::new(tokens.sizes.stroke_medium, tokens.colors.text);
    let mut corners = Vec::with_capacity(6);
    for i in 0..6 {
        let angle = std::f32::consts::TAU * (i as f32) / 6.0 - std::f32::consts::FRAC_PI_2;
        corners.push(Pos2::new(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        ));
    }
    painter.add(Shape::closed_line(corners, stroke));
    painter.circle_stroke(center, radius * 0.45, stroke);
    painter.line_segment(
        [
            Pos2::new(center.x - radius * 0.9, center.y + radius * 0.9),
            Pos2::new(center.x - radius * 0.3, center.y + radius * 0.3),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(center.x + radius * 0.9, center.y + radius * 0.9),
            Pos2::new(center.x + radius * 0.3, center.y + radius * 0.3),
        ],
        stroke,
    );
}

/// Draw the title bar and return the events the user triggered.
pub fn draw(ui: &mut Ui, status: SessionStatus, tokens: &Tokens) -> Vec<TitleBarEvent> {
    let mut events = Vec::new();
    ui.horizontal(|ui| {
        let mark_size = Vec2::splat(tokens.sizes.icon_lg);
        let (mark_rect, _) = ui.allocate_exact_size(mark_size, egui::Sense::hover());
        draw_mark(ui, mark_rect, tokens);
        ui.label(
            egui::RichText::new(PRODUCT_NAME)
                .size(tokens.sizes.font_heading)
                .strong(),
        );
        ui.label(
            egui::RichText::new(PRODUCT_TAGLINE)
                .size(tokens.sizes.font_small)
                .color(tokens.colors.text_muted),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let dot = match status {
                SessionStatus::Ready => tokens.colors.success,
                SessionStatus::NeedsAttention => tokens.colors.warning,
                SessionStatus::NeedsInput => tokens.colors.disabled,
            };
            let pill = egui::RichText::new(format!("● {}", status.label()))
                .size(tokens.sizes.font_small)
                .color(dot);
            ui.label(pill);
            ui.separator();
            if ui
                .button(
                    egui::RichText::new("Project: Skirt Block Session")
                        .size(tokens.sizes.font_small),
                )
                .on_hover_text("Show session details")
                .clicked()
            {
                events.push(TitleBarEvent::OpenProject);
            }
        });
    });
    events
}
