//! T-SK01-19: the first shape requests a fit so the user sees it, and later
//! edits do not fight the user's pan and zoom.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::canvas::roles::Role;
use pattern_ui::canvas::{Bounds, Canvas};

fn bounds(width_mm: f64, height_mm: f64) -> Bounds {
    Bounds::new([0.0, 0.0], [width_mm, height_mm])
}

#[test]
fn t_sk01_19_first_shape_requests_a_fit_and_updates_do_not() {
    let mut canvas = Canvas::new([800.0, 600.0]);
    assert!(!canvas.fit_pending);

    canvas.set_content(
        bounds(940.0, 600.0),
        String::from("Piece"),
        Role::Placeholder,
    );
    assert!(canvas.fit_pending, "the first shape did not request a fit");

    // A redraw with a different size must not steal the view again.
    canvas.fit_pending = false;
    canvas.set_content(
        bounds(950.0, 610.0),
        String::from("Piece"),
        Role::Placeholder,
    );
    assert!(!canvas.fit_pending, "an edit requested a refit");

    // Clearing the form and drafting again is a new first shape.
    canvas.content = None;
    canvas.set_content(
        bounds(940.0, 600.0),
        String::from("Piece"),
        Role::Placeholder,
    );
    assert!(
        canvas.fit_pending,
        "re-drafting after a clear did not refit"
    );
}
