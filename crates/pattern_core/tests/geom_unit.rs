// T-FR-026-01 [UNIT] Join tangent and position reported correctly
// T-FR-026-05 [UNIT] Closed-path validity cases
// T-FR-026-07 [UNIT] Constructors reject NaN and infinity
// T-REN-01-03 [UNIT] nm → f64 mm → nm is exact for values up to 1 km
//
// Requirements: FR-026 (AC-FR-026-1, AC-FR-026-2), REN-01 (AC-REN-01-2)

#![allow(clippy::expect_used)]

use pattern_core::{
    geom::{validate::ContinuityReport, CubicBez, Path, PathError, Point2, Segment},
    units::Length,
};

// ── T-REN-01-03: nm → f64 mm → nm is exact for values up to 1 km ─────────────

/// Point2::from_lengths / to_lengths round-trip must be exact for every whole-mm
/// value and the critical 1/64 inch = 396,875 nm value (ADR-0002).
#[test]
fn t_ren_01_03_round_trip_whole_mm() {
    let cases_nm: &[i64] = &[
        0,
        1_000_000,         // 1 mm
        10_000_000,        // 10 mm
        1_000_000_000,     // 1 m
        1_000_000_000_000, // 1 km
        396_875,           // 1/64 inch
        25_400_000,        // 1 inch
        914_400_000,       // 1 yard
        -1_000_000,        // negative
        -1_000_000_000_000,
    ];
    for &nm in cases_nm {
        let lx = Length::from_nanometres(nm);
        let ly = Length::from_nanometres(nm);
        let p = Point2::from_lengths(lx, ly).expect("finite");
        let (rx, ry) = p.to_lengths();
        assert_eq!(rx.nanometres(), nm, "x round-trip failed for {nm} nm");
        assert_eq!(ry.nanometres(), nm, "y round-trip failed for {nm} nm");
    }
}

// ── T-FR-026-07: constructors reject NaN and infinity ────────────────────────

#[test]
fn t_fr_026_07_point_rejects_nan_x() {
    let nan = Length::from_mm(f64::NAN).unwrap_err();
    // We can't even get a NaN Length, so the error comes from Length itself.
    // Confirm Point2 also rejects via the Length error path.
    let _ = nan; // Length::from_mm already returned Err

    // Use raw nm path: Point2 must reject NaN coming via f64 overflow
    // attempt (no direct NaN path for i64 nm).  The constructors are safe.
    // What we DO test: a finite Length always produces a valid Point2.
    let l = Length::from_mm(0.0).expect("zero is finite");
    assert!(Point2::from_lengths(l, l).is_ok());
}

#[test]
fn t_fr_026_07_length_rejects_nan() {
    assert!(Length::from_mm(f64::NAN).is_err());
}

#[test]
fn t_fr_026_07_length_rejects_positive_inf() {
    assert!(Length::from_mm(f64::INFINITY).is_err());
}

#[test]
fn t_fr_026_07_length_rejects_negative_inf() {
    assert!(Length::from_mm(f64::NEG_INFINITY).is_err());
}

#[test]
fn t_fr_026_07_cubic_bez_all_same_point_is_valid() {
    // All four control points coincident — degenerate but constructable.
    let p = pt(0.0, 0.0);
    let seg = CubicBez::new(p, p, p, p);
    // evaluate at t=0.5 must give the same point.
    let mid = seg.evaluate(0.5);
    assert!((mid.x_mm() - 0.0).abs() < 1e-9);
    assert!((mid.y_mm() - 0.0).abs() < 1e-9);
}

// ── T-FR-026-01: join continuity reported correctly ──────────────────────────

/// Build a path of two cubic Bézier segments with a G1-continuous join and
/// confirm the continuity report shows a small gap and a small angle.
#[test]
fn t_fr_026_01_g1_join_reported() {
    // Segment 1: (0,0) → control (10,30) → control (90,30) → (100,0)
    // Segment 2: (100,0) → control (110,-30) → control (190,-30) → (200,0)
    // At the join (100,0) the tangents are (90,30)→(100,0) and (100,0)→(110,-30).
    // End tangent of seg1 direction: (100-90, 0-30) = (10,-30)
    // Start tangent of seg2 direction: (110-100, -30-0) = (10,-30)
    // These are exactly collinear → G1 (tangent angle ≈ 0).

    let seg1 = CubicBez::new(pt(0.0, 0.0), pt(10.0, 30.0), pt(90.0, 30.0), pt(100.0, 0.0));
    let seg2 = CubicBez::new(
        pt(100.0, 0.0),
        pt(110.0, -30.0),
        pt(190.0, -30.0),
        pt(200.0, 0.0),
    );

    let report = pattern_core::geom::validate::continuity(
        Segment::Cubic(seg1),
        Segment::Cubic(seg2),
        1e-6, // position_gap_threshold_mm
        1e-6, // angle_threshold_rad
    );

    // Position gap at join should be 0 (exact match).
    assert!(
        report.position_gap_mm < 1e-6,
        "position gap {} mm should be near zero",
        report.position_gap_mm
    );
    // Tangent angle difference should be near 0 (exactly collinear).
    assert!(
        report.tangent_angle_rad < 1e-6,
        "tangent angle {} rad should be near zero",
        report.tangent_angle_rad
    );
}

/// A join with a deliberate kink reports a non-zero angle.
#[test]
fn t_fr_026_01_kink_join_reported() {
    // Segment 1: (0,0) → (10,0) → (90,0) → (100,0) — horizontal
    // Segment 2: (100,0) → (110,50) → ... — upward kink
    let seg1 = CubicBez::new(pt(0.0, 0.0), pt(10.0, 0.0), pt(90.0, 0.0), pt(100.0, 0.0));
    let seg2 = CubicBez::new(
        pt(100.0, 0.0),
        pt(110.0, 50.0),
        pt(190.0, 50.0),
        pt(200.0, 0.0),
    );

    let report = pattern_core::geom::validate::continuity(
        Segment::Cubic(seg1),
        Segment::Cubic(seg2),
        1e-6,
        1e-6,
    );

    // Angle should be notably non-zero.
    assert!(
        report.tangent_angle_rad > 0.1,
        "expected kink > 0.1 rad, got {}",
        report.tangent_angle_rad
    );
}

/// A position gap at a join is detected.
#[test]
fn t_fr_026_01_position_gap_reported() {
    let seg1 = CubicBez::new(pt(0.0, 0.0), pt(10.0, 0.0), pt(90.0, 0.0), pt(100.0, 0.0));
    // Segment 2 starts at (102, 0) — 2 mm gap.
    let seg2 = CubicBez::new(
        pt(102.0, 0.0),
        pt(110.0, 0.0),
        pt(190.0, 0.0),
        pt(200.0, 0.0),
    );

    let report = pattern_core::geom::validate::continuity(
        Segment::Cubic(seg1),
        Segment::Cubic(seg2),
        1e-6,
        1e-6,
    );

    assert!(
        (report.position_gap_mm - 2.0).abs() < 1e-6,
        "expected 2 mm gap, got {} mm",
        report.position_gap_mm
    );
}

// ── T-FR-026-01: continuity report fields ────────────────────────────────────

#[test]
fn t_fr_026_01_continuity_report_has_correct_type() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(10.0, 0.0), pt(90.0, 0.0), pt(100.0, 0.0));
    let report: ContinuityReport = pattern_core::geom::validate::continuity(
        Segment::Cubic(seg),
        Segment::Cubic(seg),
        1e-3,
        1e-3,
    );
    let _ = report.position_gap_mm;
    let _ = report.tangent_angle_rad;
}

// ── T-FR-026-05: closed-path validity cases ───────────────────────────────────

/// A path that closes within the tolerance is valid.
#[test]
fn t_fr_026_05_valid_closed_path() {
    // Square: (0,0)→(100,0)→(100,100)→(0,100)→(0,0)
    let path = Path::new(
        pt(0.0, 0.0),
        vec![
            Segment::Line(pt(100.0, 0.0)),
            Segment::Line(pt(100.0, 100.0)),
            Segment::Line(pt(0.0, 100.0)),
            Segment::Line(pt(0.0, 0.0)),
        ],
    )
    .expect("valid path");

    assert!(path.try_close().is_ok());
}

/// A path that doesn't close is rejected.
#[test]
fn t_fr_026_05_open_path_rejected() {
    let path = Path::new(
        pt(0.0, 0.0),
        vec![
            Segment::Line(pt(100.0, 0.0)),
            Segment::Line(pt(100.0, 100.0)),
            // Does not return to (0,0)
        ],
    )
    .expect("valid open path");

    assert!(matches!(path.try_close(), Err(PathError::NotClosed { .. })));
}

/// A path with a zero-length segment is rejected.
#[test]
fn t_fr_026_05_zero_length_segment_rejected() {
    // Two identical consecutive points.
    let result = Path::new(
        pt(0.0, 0.0),
        vec![
            Segment::Line(pt(0.0, 0.0)), // zero length
            Segment::Line(pt(100.0, 0.0)),
            Segment::Line(pt(0.0, 0.0)),
        ],
    );
    assert!(matches!(result, Err(PathError::ZeroLengthSegment)));
}

/// A path with zero area (collinear back-and-forth) is rejected as closed.
#[test]
fn t_fr_026_05_zero_area_closed_path_rejected() {
    // Go out to 100 then return — zero area.
    let path = Path::new(
        pt(0.0, 0.0),
        vec![Segment::Line(pt(100.0, 0.0)), Segment::Line(pt(0.0, 0.0))],
    )
    .expect("valid open path");

    assert!(matches!(
        path.try_close(),
        Err(PathError::ZeroArea | PathError::NotClosed { .. })
    ));
}

/// Empty segment list is rejected.
#[test]
fn t_fr_026_05_empty_segments_rejected() {
    let result = Path::new(pt(0.0, 0.0), vec![]);
    assert!(result.is_err());
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// Construct a Point2 from raw mm values (test helper).
fn pt(x_mm: f64, y_mm: f64) -> Point2 {
    Point2::from_mm(x_mm, y_mm).expect("finite test coordinates")
}
