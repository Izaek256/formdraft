//! The left tool strip. Icons are drawn in code with egui's painter (OQ-40).

use egui::{Pos2, Rect, Sense, Shape, Ui, Vec2};

use crate::theme::Tokens;

/// The three tools available in R1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tool {
    /// Select and inspect pieces.
    #[default]
    Select,
    /// Pan the canvas.
    Hand,
    /// Zoom the canvas.
    Zoom,
}

impl Tool {
    /// All tools, top to bottom.
    pub const ALL: [Tool; 3] = [Tool::Select, Tool::Hand, Tool::Zoom];

    /// A human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            Tool::Select => "Select",
            Tool::Hand => "Hand",
            Tool::Zoom => "Zoom",
        }
    }
}

/// Draw the tool strip and update the current tool on a click.
pub fn draw(ui: &mut Ui, current: &mut Tool, tokens: &Tokens) {
    ui.vertical_centered(|ui| {
        for tool in Tool::ALL {
            let size = Vec2::splat(tokens.sizes.icon_size);
            let (rect, response) = ui.allocate_exact_size(size, Sense::click());
            let selected = *current == tool;
            let background = if selected {
                tokens.colors.accent
            } else if response.hovered() {
                tokens.colors.border
            } else {
                tokens.colors.panel
            };
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::same(tokens.sizes.corner_radius),
                background,
            );
            draw_icon(ui, tool, rect, tokens);
            if response.clicked() {
                *current = tool;
            }
            ui.add_space(tokens.sizes.spacing_xs);
        }
    });
}

fn draw_icon(ui: &Ui, tool: Tool, rect: Rect, tokens: &Tokens) {
    let painter = ui.painter();
    let color = tokens.colors.text;
    let stroke = egui::Stroke::new(tokens.sizes.stroke_medium, color);
    let center = rect.center();
    let s = rect.width();
    match tool {
        Tool::Select => {
            let points = vec![
                Pos2::new(center.x - s * 0.25, center.y - s * 0.30),
                Pos2::new(center.x - s * 0.25, center.y + s * 0.25),
                Pos2::new(center.x - s * 0.05, center.y + s * 0.08),
                Pos2::new(center.x + s * 0.20, center.y + s * 0.25),
                Pos2::new(center.x + s * 0.30, center.y + s * 0.10),
                Pos2::new(center.x + s * 0.05, center.y - s * 0.05),
                Pos2::new(center.x + s * 0.25, center.y - s * 0.30),
            ];
            painter.add(Shape::convex_polygon(points, color, egui::Stroke::NONE));
        }
        Tool::Hand => {
            painter.rect_stroke(
                Rect::from_center_size(center, Vec2::new(s * 0.45, s * 0.45)),
                egui::CornerRadius::same(tokens.sizes.corner_radius),
                stroke,
                egui::StrokeKind::Inside,
            );
            for i in 0..3 {
                let x = center.x - s * 0.15 + s * 0.15 * i as f32;
                painter.line_segment(
                    [
                        Pos2::new(x, center.y + s * 0.22),
                        Pos2::new(x, center.y - s * 0.22),
                    ],
                    stroke,
                );
            }
        }
        Tool::Zoom => {
            painter.circle_stroke(center, s * 0.28, stroke);
            painter.line_segment(
                [
                    Pos2::new(center.x + s * 0.18, center.y + s * 0.18),
                    Pos2::new(center.x + s * 0.38, center.y + s * 0.38),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x - s * 0.10, center.y),
                    Pos2::new(center.x + s * 0.10, center.y),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x, center.y - s * 0.10),
                    Pos2::new(center.x, center.y + s * 0.10),
                ],
                stroke,
            );
        }
    }
}
