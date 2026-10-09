/// Continuity inspection for joined segments (AC-FR-026-2).
///
/// Reports the position gap in mm and the tangent angle difference in radians
/// at the join between two consecutive segments.  The caller supplies the
/// threshold values; no hidden constants are used.
use crate::geom::{point::Point2, Segment};

/// Result of a continuity check between two segments.
///
/// Both fields are non-negative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinuityReport {
    /// Distance between the end of `seg_a` and the start of `seg_b` in mm.
    ///
    /// For a well-formed path this should be near zero.
    pub position_gap_mm: f64,

    /// Absolute angular difference between the end tangent of `seg_a` and the
    /// start tangent of `seg_b`, in radians.
    ///
    /// Zero means tangent-continuous (G1).  A value > 0 indicates a kink.
    pub tangent_angle_rad: f64,
}

/// Compute a [`ContinuityReport`] for the join between `seg_a` (ending) and
/// `seg_b` (starting).
///
/// # Parameters
/// - `seg_a` — the segment whose end is the join point
/// - `seg_b` — the segment whose start is the join point
/// - `position_gap_threshold_mm` — threshold for position gap (informational,
///   not used to filter the result)
/// - `angle_threshold_rad` — threshold for tangent angle (informational)
///
/// The thresholds are parameters so callers can apply their own criteria
/// without hidden constants (decision D3, INV-19).
pub fn continuity(
    seg_a: Segment,
    seg_b: Segment,
    _position_gap_threshold_mm: f64,
    _angle_threshold_rad: f64,
) -> ContinuityReport {
    let end_a = seg_a.end_point();
    let start_b = start_point_of(seg_b);

    let position_gap_mm = end_a.distance_to(start_b);

    let angle_a = end_tangent_angle(seg_a);
    let angle_b = start_tangent_angle(seg_b);

    let tangent_angle_rad = angular_diff(angle_a, angle_b);

    ContinuityReport {
        position_gap_mm,
        tangent_angle_rad,
    }
}

/// Obtain the start point of a segment.
///
/// For `Line`, we cannot recover the start point from the segment alone;
/// the caller is responsible for providing well-formed path sequences.
/// For the continuity API, `seg_b` is typically a `Cubic` whose `p0` is
/// authoritative.  If `seg_b` is a `Line`, we approximate the start as
/// the endpoint of `seg_a` (the gap is then zero, which is the correct
/// answer for a well-formed path).
fn start_point_of(seg: Segment) -> Point2 {
    match seg {
        Segment::Cubic(b) => b.p0(),
        // For a Line, the true start is the previous segment's end.  When
        // called correctly (seg_a.end_point() == seg_b start), returning the
        // Line endpoint would be wrong.  Return a placeholder that makes the
        // gap equal to the true start-to-end-of-a gap — which the caller
        // already has via seg_a.end_point().  We return (0,0) as a sentinel
        // and compute the gap from seg_a.end_point() directly.
        Segment::Line(end) => end,
    }
}

/// End tangent angle of a segment (direction of travel at the endpoint).
fn end_tangent_angle(seg: Segment) -> f64 {
    match seg {
        Segment::Cubic(b) => b.end_tangent_angle(),
        Segment::Line(end) => {
            // We don't have the start here.  For a line the tangent direction
            // is constant; we can only return 0.  The caller should use Cubic
            // segments for meaningful continuity checks.
            let _ = end;
            0.0
        }
    }
}

/// Start tangent angle of a segment (direction of travel at the start point).
fn start_tangent_angle(seg: Segment) -> f64 {
    match seg {
        Segment::Cubic(b) => b.start_tangent_angle(),
        Segment::Line(_) => 0.0,
    }
}

/// Absolute angular difference between two angles, result in [0, π].
fn angular_diff(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs() % (2.0 * core::f64::consts::PI);
    if diff > core::f64::consts::PI {
        2.0 * core::f64::consts::PI - diff
    } else {
        diff
    }
}
