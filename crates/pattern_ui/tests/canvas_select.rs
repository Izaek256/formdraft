//! T-SK01-18: the select tool selects a piece clicked inside it and clears on a
//! click outside.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::canvas::roles::Role;
use pattern_ui::canvas::{Bounds, Canvas};

fn canvas_with_content() -> Canvas {
    let mut canvas = Canvas::new([800.0, 600.0]);
    canvas.set_content(
        Bounds::new([0.0, 0.0], [100.0, 200.0]),
        String::from("Placeholder rectangle"),
        Role::Placeholder,
    );
    canvas
}

#[test]
fn t_sk01_18_click_inside_selects_the_piece() {
    let mut canvas = canvas_with_content();
    canvas.select_at([50.0, 50.0]);
    let selection = canvas.selection.clone().expect("inside should select");
    assert_eq!(selection.label, "Placeholder rectangle");
    assert_eq!(selection.bounds.size_mm(), [100.0, 200.0]);
}

#[test]
fn t_sk01_18_click_outside_clears_the_selection() {
    let mut canvas = canvas_with_content();
    canvas.select_at([50.0, 50.0]);
    assert!(canvas.selection.is_some());
    canvas.select_at([500.0, 500.0]);
    assert!(canvas.selection.is_none());
}

#[test]
fn t_sk01_18_bounds_edges_count_as_inside() {
    let bounds = Bounds::new([0.0, 0.0], [100.0, 200.0]);
    assert!(bounds.contains([0.0, 0.0]));
    assert!(bounds.contains([100.0, 200.0]));
    assert!(!bounds.contains([100.001, 200.0]));
}
