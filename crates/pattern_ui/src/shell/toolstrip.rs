//! Tool selection: the canvas tools (R1) and the pattern-tool grid from the
//! reference layout.
//!
//! [`Tool`] drives canvas behaviour and is fully wired. [`PatternTool`] shows
//! the twelve drafting tools; only Select exists in R1, so choosing any other
//! tool records the choice, labels the mode honestly, and asks the app for a
//! toast through [`PatternToolEvent::Later`]. The canvas keeps Select
//! behaviour until the tool arrives.

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
    /// All tools, in toolbar order.
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

/// A drafting tool from the reference layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PatternTool {
    /// Select and inspect (R1).
    #[default]
    Select,
    /// Construction point (R2).
    Point,
    /// Straight line (R2).
    Line,
    /// Curve (R2).
    Curve,
    /// Arc (R2).
    Arc,
    /// Dart (R2).
    Dart,
    /// Notch (R2).
    Notch,
    /// Muted label text (R2).
    Text,
    /// Grainline (R2).
    Grainline,
    /// Rectangle (R2).
    Rectangle,
    /// Circle (R2).
    Circle,
    /// Tape measure (R2).
    Measure,
}

impl PatternTool {
    /// All tools, in grid order.
    pub const ALL: [PatternTool; 12] = [
        PatternTool::Select,
        PatternTool::Point,
        PatternTool::Line,
        PatternTool::Curve,
        PatternTool::Arc,
        PatternTool::Dart,
        PatternTool::Notch,
        PatternTool::Text,
        PatternTool::Grainline,
        PatternTool::Rectangle,
        PatternTool::Circle,
        PatternTool::Measure,
    ];

    /// A human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            PatternTool::Select => "Select",
            PatternTool::Point => "Point",
            PatternTool::Line => "Line",
            PatternTool::Curve => "Curve",
            PatternTool::Arc => "Arc",
            PatternTool::Dart => "Dart",
            PatternTool::Notch => "Notch",
            PatternTool::Text => "Text",
            PatternTool::Grainline => "Grainline",
            PatternTool::Rectangle => "Rectangle",
            PatternTool::Circle => "Circle",
            PatternTool::Measure => "Measure",
        }
    }

    /// Whether the tool works in this release.
    pub fn is_available(self) -> bool {
        self == PatternTool::Select
    }
}

/// Something the pattern-tool grid asked the application to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternToolEvent {
    /// A later-release tool was chosen; the string names it.
    Later(String),
}

/// Draw the pattern-tool grid and update the choice on click.
///
/// Choosing an unavailable tool still records it (so the mode line can name
/// it) and reports [`PatternToolEvent::Later`].
pub fn draw_pattern_tools(
    ui: &mut Ui,
    current: &mut PatternTool,
    tokens: &Tokens,
) -> Vec<PatternToolEvent> {
    let mut events = Vec::new();
    ui.label(
        egui::RichText::new("Pattern Tools")
            .size(tokens.sizes.font_body)
            .strong(),
    );
    ui.add_space(tokens.sizes.spacing_xs);
    egui::Grid::new("pattern_tool_grid")
        .num_columns(4)
        .spacing(Vec2::splat(tokens.sizes.spacing_xs))
        .show(ui, |ui| {
            for (index, tool) in PatternTool::ALL.iter().enumerate() {
                let selected = *current == *tool;
                let cell = Vec2::splat(tokens.sizes.icon_lg + tokens.sizes.spacing_l);
                let (rect, response) = ui.allocate_exact_size(cell, Sense::click());
                let background = if selected {
                    tokens.colors.selection
                } else if response.hovered() {
                    tokens.colors.hover
                } else {
                    tokens.colors.surface
                };
                ui.painter().rect_filled(
                    rect,
                    egui::CornerRadius::same(tokens.sizes.corner_radius),
                    background,
                );
                let icon_color = if tool.is_available() || selected {
                    tokens.colors.text
                } else {
                    tokens.colors.disabled
                };
                draw_pattern_icon(ui, *tool, rect, icon_color, tokens);
                if response.clicked() {
                    *current = *tool;
                    if !tool.is_available() {
                        events.push(PatternToolEvent::Later(tool.label().to_owned()));
                    }
                }
                response.on_hover_text(if tool.is_available() {
                    tool.label().to_owned()
                } else {
                    format!("{} (a later release)", tool.label())
                });
                if (index + 1) % 4 == 0 {
                    ui.end_row();
                }
            }
        });
    ui.add_space(tokens.sizes.spacing_xs);
    ui.label(
        egui::RichText::new(current.label())
            .size(tokens.sizes.font_small)
            .color(tokens.colors.text_muted),
    );
    events
}

/// Draw one canvas-tool button (Select, Hand, Zoom) for the center toolbar.
pub fn tool_button(ui: &mut Ui, tool: Tool, current: Tool, tokens: &Tokens) -> egui::Response {
    let selected = current == tool;
    let size = Vec2::splat(tokens.sizes.control_height);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    let background = if selected {
        tokens.colors.selection
    } else if response.hovered() {
        tokens.colors.hover
    } else {
        tokens.colors.panel
    };
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(tokens.sizes.corner_radius),
        background,
    );
    draw_tool_icon(ui, tool, rect, tokens.colors.text, tokens);
    response.on_hover_text(tool.label())
}

/// Draw the icon for a canvas tool.
fn draw_tool_icon(ui: &Ui, tool: Tool, rect: Rect, color: egui::Color32, tokens: &Tokens) {
    let painter = ui.painter();
    let stroke = egui::Stroke::new(tokens.sizes.stroke_medium, color);
    let center = rect.center();
    let side = rect.width();
    match tool {
        Tool::Select => {
            let points = vec![
                Pos2::new(center.x - side * 0.25, center.y - side * 0.30),
                Pos2::new(center.x - side * 0.25, center.y + side * 0.25),
                Pos2::new(center.x - side * 0.05, center.y + side * 0.08),
                Pos2::new(center.x + side * 0.20, center.y + side * 0.25),
                Pos2::new(center.x + side * 0.30, center.y + side * 0.10),
                Pos2::new(center.x + side * 0.05, center.y - side * 0.05),
                Pos2::new(center.x + side * 0.25, center.y - side * 0.30),
            ];
            painter.add(Shape::convex_polygon(points, color, egui::Stroke::NONE));
        }
        Tool::Hand => {
            painter.rect_stroke(
                Rect::from_center_size(center, Vec2::new(side * 0.45, side * 0.45)),
                egui::CornerRadius::same(tokens.sizes.corner_radius),
                stroke,
                egui::StrokeKind::Inside,
            );
            for offset in [-1.0_f32, 0.0, 1.0] {
                let x = center.x + offset * side * 0.15;
                painter.line_segment(
                    [
                        Pos2::new(x, center.y + side * 0.22),
                        Pos2::new(x, center.y - side * 0.22),
                    ],
                    stroke,
                );
            }
        }
        Tool::Zoom => {
            painter.circle_stroke(center, side * 0.28, stroke);
            painter.line_segment(
                [
                    Pos2::new(center.x + side * 0.18, center.y + side * 0.18),
                    Pos2::new(center.x + side * 0.38, center.y + side * 0.38),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x - side * 0.10, center.y),
                    Pos2::new(center.x + side * 0.10, center.y),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x, center.y - side * 0.10),
                    Pos2::new(center.x, center.y + side * 0.10),
                ],
                stroke,
            );
        }
    }
}

/// Draw the glyph for one pattern tool. All strokes come from the tokens.
fn draw_pattern_icon(
    ui: &Ui,
    tool: PatternTool,
    rect: Rect,
    color: egui::Color32,
    tokens: &Tokens,
) {
    let painter = ui.painter();
    let stroke = egui::Stroke::new(tokens.sizes.stroke_medium, color);
    let thin = egui::Stroke::new(tokens.sizes.stroke_thin, color);
    let center = rect.center();
    let side = rect.width().min(rect.height()) * 0.62;
    let half = side * 0.5;
    let top_left = Pos2::new(center.x - half, center.y - half);
    let top_right = Pos2::new(center.x + half, center.y - half);
    let bottom_left = Pos2::new(center.x - half, center.y + half);
    let bottom_right = Pos2::new(center.x + half, center.y + half);
    match tool {
        PatternTool::Select => {
            let points = vec![
                Pos2::new(center.x - half, center.y - half),
                Pos2::new(center.x - half, center.y + half * 0.6),
                Pos2::new(center.x - half * 0.2, center.y + half * 0.2),
                Pos2::new(center.x + half * 0.5, center.y + half * 0.7),
                Pos2::new(center.x + half * 0.8, center.y + half * 0.3),
                Pos2::new(center.x + half * 0.1, center.y - half * 0.1),
                Pos2::new(center.x + half * 0.7, center.y - half),
            ];
            painter.add(Shape::convex_polygon(points, color, egui::Stroke::NONE));
        }
        PatternTool::Point => {
            painter.circle_stroke(center, half * 0.35, stroke);
            painter.line_segment(
                [
                    Pos2::new(center.x - half, center.y),
                    Pos2::new(center.x + half, center.y),
                ],
                thin,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x, center.y - half),
                    Pos2::new(center.x, center.y + half),
                ],
                thin,
            );
        }
        PatternTool::Line => {
            painter.line_segment([bottom_left, top_right], stroke);
            painter.circle_stroke(bottom_left, half * 0.18, thin);
            painter.circle_stroke(top_right, half * 0.18, thin);
        }
        PatternTool::Curve => {
            painter.circle_stroke(center, half * 0.8, stroke);
        }
        PatternTool::Arc => {
            let mut arc = Vec::with_capacity(12);
            for i in 0..=12_u8 {
                let angle = std::f32::consts::PI * (0.9 + f32::from(i) / 12.0);
                arc.push(Pos2::new(
                    center.x + half * angle.cos(),
                    center.y + half * angle.sin(),
                ));
            }
            painter.add(Shape::line(arc, stroke));
        }
        PatternTool::Dart => {
            painter.add(Shape::line(
                vec![
                    Pos2::new(center.x, center.y - half),
                    Pos2::new(center.x - half * 0.7, center.y + half),
                    Pos2::new(center.x + half * 0.7, center.y + half),
                    Pos2::new(center.x, center.y - half),
                ],
                stroke,
            ));
            painter.line_segment([Pos2::new(center.x, center.y - half), center], thin);
        }
        PatternTool::Notch => {
            for offset in [-1.0_f32, 0.0, 1.0] {
                let x = center.x + offset * half * 0.7;
                painter.line_segment(
                    [Pos2::new(x, center.y - half), Pos2::new(x, center.y + half)],
                    thin,
                );
            }
        }
        PatternTool::Text => {
            painter.line_segment([top_left, top_right], stroke);
            painter.line_segment([center, Pos2::new(center.x, center.y + half)], stroke);
            painter.line_segment(
                [
                    Pos2::new(center.x - half * 0.5, center.y + half),
                    Pos2::new(center.x + half * 0.5, center.y + half),
                ],
                thin,
            );
        }
        PatternTool::Grainline => {
            painter.line_segment(
                [
                    Pos2::new(center.x, center.y - half),
                    Pos2::new(center.x, center.y + half),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x - half * 0.4, center.y - half * 0.5),
                    center,
                ],
                thin,
            );
            painter.line_segment(
                [
                    Pos2::new(center.x + half * 0.4, center.y - half * 0.5),
                    center,
                ],
                thin,
            );
        }
        PatternTool::Rectangle => {
            painter.rect_stroke(
                Rect::from_min_max(top_left, bottom_right),
                egui::CornerRadius::ZERO,
                stroke,
                egui::StrokeKind::Inside,
            );
        }
        PatternTool::Circle => {
            painter.circle_stroke(center, half * 0.85, stroke);
        }
        PatternTool::Measure => {
            painter.line_segment([bottom_left, top_right], stroke);
            for t in [0.2_f32, 0.5, 0.8] {
                let base = Pos2::new(
                    bottom_left.x + (top_right.x - bottom_left.x) * t,
                    bottom_left.y + (top_right.y - bottom_left.y) * t,
                );
                painter.line_segment([base, Pos2::new(base.x, base.y - half * 0.4)], thin);
            }
        }
    }
}
