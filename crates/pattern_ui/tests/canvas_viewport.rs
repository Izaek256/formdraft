//! T-SK01-08 / T-SK01-14: the viewport transform is stable and zoom keeps the
//! point under the cursor fixed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::canvas::Viewport;

#[test]
fn t_sk01_08_screen_world_round_trip_is_stable() {
    let zooms = [0.05, 0.2, 1.0, 3.0, 12.0, 60.0];
    let samples = [(0.0, 0.0), (123.4, -56.7), (-999.0, 888.0), (3.5, 2.25)];

    for zoom in zooms {
        let mut viewport = Viewport::new([900.0, 700.0]);
        let factor = zoom / viewport.px_per_mm();
        viewport.zoom_about(factor, [450.0, 350.0]);
        viewport.pan_by([13.7, -42.1]);

        for &(x, y) in &samples {
            let screen = viewport.world_to_screen(x, y);
            let world = viewport.screen_to_world(screen[0], screen[1]);
            assert!(
                (world[0] - x).abs() < 1e-6,
                "x {x} round-tripped to {world:?}"
            );
            assert!(
                (world[1] - y).abs() < 1e-6,
                "y {y} round-tripped to {world:?}"
            );
        }
    }
}

#[test]
fn t_sk01_08_positive_millimetres_move_down_and_right() {
    let viewport = Viewport::new([800.0, 600.0]);
    let origin = viewport.world_to_screen(0.0, 0.0);
    let right = viewport.world_to_screen(10.0, 0.0);
    let down = viewport.world_to_screen(0.0, 10.0);
    assert!(right[0] > origin[0]);
    assert!(down[1] > origin[1]);
}

#[test]
fn t_sk01_14_zoom_keeps_the_cursor_world_point_fixed() {
    let mut viewport = Viewport::new([800.0, 600.0]);
    for cursor in [[0.0, 0.0], [123.0, 45.0], [800.0, 600.0], [400.0, 300.0]] {
        for factor in [1.5, 0.7, 3.0, 1.0 / 3.0] {
            let before = viewport.screen_to_world(cursor[0], cursor[1]);
            viewport.zoom_about(factor, cursor);
            let after = viewport.screen_to_world(cursor[0], cursor[1]);
            assert!(
                (after[0] - before[0]).abs() < 1e-6,
                "cursor {cursor:?} factor {factor} moved x"
            );
            assert!(
                (after[1] - before[1]).abs() < 1e-6,
                "cursor {cursor:?} factor {factor} moved y"
            );
        }
    }
}
