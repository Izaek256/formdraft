//! The background grid. Spacing follows the same 1-2-5 ladder as the rulers.

use pattern_core::units::Unit;

use crate::canvas::rulers;
use crate::canvas::ScreenMap;
use crate::theme::Tokens;

/// Minimum spacing between grid lines, in points.
pub const GRID_MIN_PX: f64 = 10.0;

/// The grid step, in millimetres, for the current zoom and display unit.
pub fn grid_step_mm(unit: Unit, px_per_mm: f64) -> f64 {
    rulers::step_mm(unit, px_per_mm, GRID_MIN_PX)
}

/// Draw the grid across `rect`.
pub fn draw(
    painter: &egui::Painter,
    map: &ScreenMap,
    rect: egui::Rect,
    unit: Unit,
    tokens: &Tokens,
) {
    let step = grid_step_mm(unit, map.viewport.px_per_mm());
    if !step.is_finite() || step <= 0.0 {
        return;
    }
    let (min_mm, max_mm) = map.rect_world_bounds(rect);
    let stroke = egui::Stroke::new(tokens.sizes.stroke_thin, tokens.colors.grid);

    for x_mm in rulers::ticks_mm(step, min_mm[0], max_mm[0]) {
        painter.line_segment(
            [map.to_pos(x_mm, min_mm[1]), map.to_pos(x_mm, max_mm[1])],
            stroke,
        );
    }
    for y_mm in rulers::ticks_mm(step, min_mm[1], max_mm[1]) {
        painter.line_segment(
            [map.to_pos(min_mm[0], y_mm), map.to_pos(max_mm[0], y_mm)],
            stroke,
        );
    }
}
