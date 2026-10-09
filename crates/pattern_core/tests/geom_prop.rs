// T-FR-026-02 [PROP] Tessellation error within tolerance at low and high zoom
// T-FR-026-08 [PROP] Arbitrary finite input never panics
//
// No external property-test crate — we use a fixed set of deterministic
// representative curves (seed-equivalent approach without an RNG dependency).
//
// Requirements: FR-026 (AC-FR-026-3), INV-15

#![allow(clippy::expect_used)]

use pattern_core::geom::{tolerance, CubicBez, Point2};

// ── T-FR-026-02: tessellation error within tolerance ─────────────────────────

/// For each combination of (curve, tolerance), the maximum deviation of the
/// tessellation polyline from the true curve must be ≤ the requested tolerance.
///
/// Oracle: for each polyline chord (A, B) find the maximum distance from any
/// point on the true Bézier arc between the corresponding t-values to the
/// chord.  We sample 1000 points per chord — sufficient for the 0.001 mm
/// level.
///
/// This oracle is independent: it evaluates the Bézier directly via the
/// standard formula and measures point-to-segment distance.
#[test]
fn t_fr_026_02_tessellation_error_within_tolerance_gentle_curve() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(30.0, 80.0), pt(70.0, 80.0), pt(100.0, 0.0));
    for &tol in &[1.0_f64, 0.1, 0.01, 0.001] {
        check_tessellation_tolerance(seg, tol);
    }
}

#[test]
fn t_fr_026_02_tessellation_error_within_tolerance_tight_curve() {
    // High-curvature: control points far from chord.
    let seg = CubicBez::new(
        pt(0.0, 0.0),
        pt(0.0, 150.0),
        pt(100.0, 150.0),
        pt(100.0, 0.0),
    );
    for &tol in &[1.0_f64, 0.1, 0.01, 0.001] {
        check_tessellation_tolerance(seg, tol);
    }
}

#[test]
fn t_fr_026_02_tessellation_error_within_tolerance_s_curve() {
    let seg = CubicBez::new(
        pt(0.0, 0.0),
        pt(50.0, 100.0),
        pt(50.0, -100.0),
        pt(100.0, 0.0),
    );
    for &tol in &[1.0_f64, 0.1, 0.01, 0.001] {
        check_tessellation_tolerance(seg, tol);
    }
}

#[test]
fn t_fr_026_02_tessellation_error_within_tolerance_straight_line() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(33.0, 0.0), pt(67.0, 0.0), pt(100.0, 0.0));
    // A straight line should tessellate to just the endpoints.
    for &tol in &[1.0_f64, 0.1, 0.01, 0.001] {
        check_tessellation_tolerance(seg, tol);
    }
}

// ── T-FR-026-08: arbitrary finite input never panics ─────────────────────────

/// A large set of diverse finite inputs must not cause any panic.
/// Covers: zero-length curves, very large coordinates, very small coordinates,
/// negative coordinates, asymmetric control points.
#[test]
fn t_fr_026_08_diverse_finite_inputs_no_panic() {
    type Quad = (f64, f64);
    let cases: &[(Quad, Quad, Quad, Quad)] = &[
        // degenerate: all same point
        ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 0.0)),
        // tiny segment
        (
            (0.0, 0.0),
            (0.000_001, 0.0),
            (0.000_002, 0.0),
            (0.000_003, 0.0),
        ),
        // large coordinates (near 1 m)
        ((0.0, 0.0), (333.0, 0.0), (667.0, 0.0), (1000.0, 0.0)),
        // negative coordinates
        (
            (-500.0, -500.0),
            (-300.0, 200.0),
            (300.0, 200.0),
            (500.0, -500.0),
        ),
        // near-cusp: P1 and P2 very close together, far from P0 and P3
        ((0.0, 0.0), (50.0, 200.0), (50.001, 200.0), (100.0, 0.0)),
        // asymmetric
        ((0.0, 0.0), (5.0, 90.0), (95.0, 10.0), (100.0, 100.0)),
        // point at origin, control arms large
        ((0.0, 0.0), (0.0, 1000.0), (1000.0, 1000.0), (1000.0, 0.0)),
        // zero x movement
        ((50.0, 0.0), (50.0, 30.0), (50.0, 70.0), (50.0, 100.0)),
        // single axis
        ((0.0, 0.0), (100.0, 0.0), (200.0, 0.0), (300.0, 0.0)),
    ];

    for &(p0, p1, p2, p3) in cases {
        let seg = CubicBez::new(
            pt(p0.0, p0.1),
            pt(p1.0, p1.1),
            pt(p2.0, p2.1),
            pt(p3.0, p3.1),
        );

        // Must not panic.
        let _ = seg.evaluate(0.0);
        let _ = seg.evaluate(0.5);
        let _ = seg.evaluate(1.0);
        let _ = seg.tangent(0.0);
        let _ = seg.tangent(0.5);
        let _ = seg.tangent(1.0);
        let (l, r) = seg.split(0.5);
        let _ = l;
        let _ = r;
        let _ = seg.reverse();
        let _ = seg.tessellate(0.1);
        let _ = seg.tessellate(0.01);
    }
}

/// t values outside [0,1] must not panic (clamped or extrapolated, but no panic).
#[test]
fn t_fr_026_08_out_of_range_t_no_panic() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(30.0, 80.0), pt(70.0, 80.0), pt(100.0, 0.0));
    let _ = seg.evaluate(-1.0);
    let _ = seg.evaluate(2.0);
    let _ = seg.tangent(-0.5);
    let _ = seg.tangent(1.5);
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn pt(x: f64, y: f64) -> Point2 {
    Point2::from_mm(x, y).expect("finite")
}

/// Verify that the maximum deviation of the tessellation from the true curve
/// is within `tolerance_mm`.
///
/// The oracle works by:
/// 1. Calling `seg.tessellate(tolerance_mm)` to get the polyline.
/// 2. Sampling the whole Bézier at 100,000 points and finding the maximum
///    distance from any sample to the nearest chord segment of the polyline.
///    This is independent of the production flatten code.
fn check_tessellation_tolerance(seg: CubicBez, tolerance_mm: f64) {
    let polyline = seg.tessellate(tolerance_mm);
    assert!(
        polyline.len() >= 2,
        "tessellation must have at least 2 points for tol={tolerance_mm}"
    );

    let p0 = (seg.p0().x_mm(), seg.p0().y_mm());
    let p1 = (seg.p1().x_mm(), seg.p1().y_mm());
    let p2 = (seg.p2().x_mm(), seg.p2().y_mm());
    let p3 = (seg.p3().x_mm(), seg.p3().y_mm());

    let eval = |t: f64| -> (f64, f64) {
        let u = 1.0 - t;
        let x =
            u * u * u * p0.0 + 3.0 * u * u * t * p1.0 + 3.0 * u * t * t * p2.0 + t * t * t * p3.0;
        let y =
            u * u * u * p0.1 + 3.0 * u * u * t * p1.1 + 3.0 * u * t * t * p2.1 + t * t * t * p3.1;
        (x, y)
    };

    let n = tolerance::ORACLE_SAMPLES;
    let mut max_dev = 0.0_f64;

    for i in 0..=n {
        let t = i as f64 / n as f64;
        let curve_pt = eval(t);

        // Find the nearest point on the polyline to curve_pt.
        let mut min_dist = f64::MAX;
        for w in polyline.windows(2) {
            let a = (w[0].x_mm(), w[0].y_mm());
            let b = (w[1].x_mm(), w[1].y_mm());
            let d = point_to_segment_dist(curve_pt, a, b);
            if d < min_dist {
                min_dist = d;
            }
        }
        if min_dist > max_dev {
            max_dev = min_dist;
        }
    }

    assert!(
        max_dev <= tolerance_mm,
        "seg tol={tolerance_mm:.4} mm: max deviation {max_dev:.6} mm exceeds tolerance"
    );
}

/// Distance from point P to the segment AB.
fn point_to_segment_dist(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let ab = (b.0 - a.0, b.1 - a.1);
    let ap = (p.0 - a.0, p.1 - a.1);
    let ab_len_sq = ab.0 * ab.0 + ab.1 * ab.1;
    if ab_len_sq < 1e-18 {
        // Degenerate segment: a == b
        return (ap.0 * ap.0 + ap.1 * ap.1).sqrt();
    }
    let t = (ap.0 * ab.0 + ap.1 * ab.1) / ab_len_sq;
    let t = t.clamp(0.0, 1.0);
    let closest = (a.0 + t * ab.0, a.1 + t * ab.1);
    let dx = p.0 - closest.0;
    let dy = p.1 - closest.1;
    (dx * dx + dy * dy).sqrt()
}
