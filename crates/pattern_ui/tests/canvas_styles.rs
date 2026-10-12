//! T-SK01-16: stitch and cut styles differ by dash pattern, and dash
//! segmentation matches the requested pattern.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::canvas::render_cpu::dash_segments;
use pattern_ui::canvas::roles::{self, Dash, Role};

#[test]
fn t_sk01_16_stitch_and_cut_differ_by_dash_pattern() {
    assert_ne!(
        roles::style(Role::Stitch).dash,
        roles::style(Role::Cut).dash
    );
    assert_eq!(roles::style(Role::Stitch).dash, Dash::Dashed);
    assert_eq!(roles::style(Role::Cut).dash, Dash::Solid);
    assert_eq!(roles::style(Role::NetBlock).dash, Dash::Solid);
}

#[test]
fn t_sk01_16_dash_segments_split_a_line() {
    let line = [[0.0, 0.0], [120.0, 0.0]];

    let solid = dash_segments(&line, 0.0, 0.0);
    assert_eq!(solid.len(), 1);

    let dashed = dash_segments(&line, 6.0, 4.0);
    assert!(
        dashed.len() > 5,
        "dashed produced {} segments",
        dashed.len()
    );
    for segment in &dashed {
        assert!((segment[0][1]).abs() < 1e-9);
        assert!((segment[1][1]).abs() < 1e-9);
        let len = segment[1][0] - segment[0][0];
        assert!(len > 0.0 && len <= 6.0 + 1e-9);
    }
}

#[test]
fn t_sk01_16_degenerate_input_is_safe() {
    assert!(dash_segments(&[], 6.0, 4.0).is_empty());
    assert!(dash_segments(&[[1.0, 1.0]], 6.0, 4.0).is_empty());
    let zero_length = dash_segments(&[[1.0, 1.0], [1.0, 1.0]], 6.0, 4.0);
    assert!(zero_length.is_empty());
}
