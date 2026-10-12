//! T-SK01-10: the keyboard view actions zoom, fit, pan and set actual size.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::canvas::roles::Role;
use pattern_ui::canvas::{Bounds, Canvas, NOMINAL_PPI};
use pattern_ui::shell::Action;

fn canvas_with_content() -> Canvas {
    let mut canvas = Canvas::new([800.0, 600.0]);
    canvas.set_content(
        Bounds::new([0.0, 0.0], [400.0, 600.0]),
        String::from("Placeholder rectangle"),
        Role::Placeholder,
    );
    canvas
}

#[test]
fn t_sk01_10_zoom_actions_change_and_restore_the_scale() {
    let mut canvas = canvas_with_content();
    let before = canvas.viewport.zoom_percent();
    canvas.handle_action(Action::ZoomIn);
    assert!(canvas.viewport.zoom_percent() > before);
    canvas.handle_action(Action::ZoomOut);
    assert!((canvas.viewport.zoom_percent() - before).abs() < 1e-6);
}

#[test]
fn t_sk01_10_fit_places_content_inside_the_viewport() {
    let mut canvas = canvas_with_content();
    canvas.handle_action(Action::FitToWindow);
    let min = canvas.viewport.world_to_screen(0.0, 0.0);
    let max = canvas.viewport.world_to_screen(400.0, 600.0);
    let size = canvas.viewport.size_px();
    assert!(min[0] >= -0.5 && min[1] >= -0.5);
    assert!(max[0] <= size[0] + 0.5 && max[1] <= size[1] + 0.5);
}

#[test]
fn t_sk01_10_actual_size_sets_ninety_six_ppi() {
    let mut canvas = canvas_with_content();
    canvas.handle_action(Action::ActualSize);
    let expected = NOMINAL_PPI / 25.4;
    assert!((canvas.viewport.px_per_mm() - expected).abs() < 1e-9);
    assert!((canvas.viewport.zoom_percent() - 100.0).abs() < 1e-6);
}

#[test]
fn t_sk01_10_pan_actions_move_and_restore_the_origin() {
    let mut canvas = canvas_with_content();
    let ox = canvas.viewport.origin_px()[0];
    canvas.handle_action(Action::PanRight);
    assert!(canvas.viewport.origin_px()[0] < ox);
    canvas.handle_action(Action::PanLeft);
    assert!((canvas.viewport.origin_px()[0] - ox).abs() < 1e-9);

    let oy = canvas.viewport.origin_px()[1];
    canvas.handle_action(Action::PanUp);
    assert!(canvas.viewport.origin_px()[1] > oy);
    canvas.handle_action(Action::PanDown);
    assert!((canvas.viewport.origin_px()[1] - oy).abs() < 1e-9);
}
