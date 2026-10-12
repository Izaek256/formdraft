//! CPU rendering of a polyline with a role style.
//!
//! Dash segmentation is pure and unit tested (T-SK01-16). The draw function is
//! the only part that touches egui. Tessellation of curved segments is done by
//! `pattern_core::geom` before the points reach here; this module only draws
//! straight screen segments.

use crate::canvas::roles::{self, Role};
use crate::canvas::ScreenMap;
use crate::theme::Tokens;

/// Split a screen-space polyline into dash segments.
///
/// Dash and gap are in points. A non-positive `dash` returns the whole
/// polyline as one segment per edge (solid). Coordinates are in the same space
/// as `points` (screen points before the draw offset is applied).
pub fn dash_segments(points: &[[f64; 2]], dash: f64, gap: f64) -> Vec<[[f64; 2]; 2]> {
    let mut out = Vec::new();
    if points.len() < 2 {
        return out;
    }
    if !dash.is_finite() || dash <= 0.0 || !gap.is_finite() || gap < 0.0 {
        for window in points.windows(2) {
            out.push([window[0], window[1]]);
        }
        return out;
    }

    let mut on = true;
    let mut remaining = dash;
    for window in points.windows(2) {
        let a = window[0];
        let b = window[1];
        let dx = b[0] - a[0];
        let dy = b[1] - a[1];
        let len = (dx * dx + dy * dy).sqrt();
        if len <= 0.0 {
            continue;
        }
        let ux = dx / len;
        let uy = dy / len;
        let mut travelled = 0.0;
        while travelled < len {
            let step = remaining.min(len - travelled);
            if on {
                out.push([
                    [a[0] + ux * travelled, a[1] + uy * travelled],
                    [
                        a[0] + ux * (travelled + step),
                        a[1] + uy * (travelled + step),
                    ],
                ]);
            }
            travelled += step;
            remaining -= step;
            if remaining <= 1e-9 {
                on = !on;
                remaining = if on { dash } else { gap };
            }
        }
    }
    out
}

/// Draw a millimetre polyline with a role's dash, weight and colour.
#[allow(clippy::cast_possible_truncation)]
pub fn draw_polyline(
    painter: &egui::Painter,
    map: &ScreenMap,
    points_mm: &[[f64; 2]],
    closed: bool,
    role: Role,
    tokens: &Tokens,
) {
    if points_mm.len() < 2 {
        return;
    }
    let style = roles::style(role);
    let stroke = egui::Stroke::new(
        roles::weight_px(style.weight, tokens),
        roles::color(role, tokens),
    );
    let (dash, gap) = roles::dash_px(style.dash, tokens);

    let mut screen: Vec<[f64; 2]> = points_mm
        .iter()
        .map(|p| map.viewport.world_to_screen(p[0], p[1]))
        .collect();
    if closed {
        screen.push(screen[0]);
    }

    for segment in dash_segments(&screen, f64::from(dash), f64::from(gap)) {
        painter.line_segment(
            [
                egui::pos2(
                    (segment[0][0] + map.offset[0]) as f32,
                    (segment[0][1] + map.offset[1]) as f32,
                ),
                egui::pos2(
                    (segment[1][0] + map.offset[0]) as f32,
                    (segment[1][1] + map.offset[1]) as f32,
                ),
            ],
            stroke,
        );
    }
}
