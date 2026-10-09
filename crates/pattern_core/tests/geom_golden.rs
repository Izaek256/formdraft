// T-FR-026-03 [GOLDEN] Reference curves
// T-FR-026-06 [UNIT] Split and recombine, and reversal, preserve shape and length
// T-NFR-014-02 [PROP] Repeated runs give bit-identical output
//
// Oracles are written directly in each test using closed-form or dense-sampling
// methods that do not call the code under test (ADR-0013).
//
// Requirements: FR-026 (AC-FR-026-1, AC-FR-026-3), NFR-014 (AC-NFR-014-1)

#![allow(clippy::expect_used)]

use pattern_core::geom::{tolerance, CubicBez, Path, Point2, Segment};

// ── T-FR-026-03 case 1: collinear control points ──────────────────────────────

/// When all four control points are collinear the Bézier is a straight line.
/// Its arc length must equal the chord length exactly (within length accuracy
/// tolerance).
///
/// Oracle: chord = distance from P0 to P3 (Euclidean, closed-form).
#[test]
fn t_fr_026_03_collinear_control_points_length_equals_chord() {
    // P0=(0,0), P1=(33.3,0), P2=(66.6,0), P3=(100,0) — all on x-axis
    let seg = CubicBez::new(
        pt(0.0, 0.0),
        pt(33.333_333, 0.0),
        pt(66.666_667, 0.0),
        pt(100.0, 0.0),
    );
    let path = path_from_seg(seg);
    let length = path.arc_length_mm();

    // Oracle: chord = 100 mm exactly.
    let chord = 100.0_f64;
    assert!(
        (length - chord).abs() <= tolerance::LENGTH_ACCURACY_MM,
        "collinear length {length} mm, chord {chord} mm, diff {} > {}",
        (length - chord).abs(),
        tolerance::LENGTH_ACCURACY_MM
    );
}

// ── T-FR-026-03 case 2: symmetric arch ───────────────────────────────────────

/// A symmetric arch: P0=(0,0), P1=(0,h), P2=(w,h), P3=(w,0).
/// Oracle: dense-sampling integration in the test (not calling arc_length_mm).
///
/// The dense-sampling oracle evaluates the Bézier at N points using the
/// standard formula and sums chord lengths.  For N = 100,000 this gives an
/// error well under 0.001 mm on smooth curves.
#[test]
fn t_fr_026_03_symmetric_arch_length_within_tolerance() {
    let w = 200.0_f64;
    let h = 80.0_f64;
    let seg = CubicBez::new(pt(0.0, 0.0), pt(0.0, h), pt(w, h), pt(w, 0.0));

    let computed = path_from_seg(seg).arc_length_mm();
    let oracle = dense_sample_length(seg, tolerance::ORACLE_SAMPLES);

    assert!(
        (computed - oracle).abs() <= tolerance::LENGTH_ACCURACY_MM,
        "arch: computed={computed:.6} oracle={oracle:.6} diff={:.6} > {}",
        (computed - oracle).abs(),
        tolerance::LENGTH_ACCURACY_MM
    );
}

// ── T-FR-026-03 case 3: near-degenerate cusp-like curve ──────────────────────

/// P0=(0,0), P1=(50,100), P2=(50,-100), P3=(100,0) — a pronounced S-curve
/// that approaches a cusp as the control arms grow.  The implementation must
/// not panic and must return a length within the accuracy tolerance.
#[test]
fn t_fr_026_03_near_degenerate_s_curve_does_not_panic() {
    let seg = CubicBez::new(
        pt(0.0, 0.0),
        pt(50.0, 100.0),
        pt(50.0, -100.0),
        pt(100.0, 0.0),
    );
    let computed = path_from_seg(seg).arc_length_mm();
    let oracle = dense_sample_length(seg, tolerance::ORACLE_SAMPLES);

    assert!(
        (computed - oracle).abs() <= tolerance::LENGTH_ACCURACY_MM,
        "s-curve: computed={computed:.6} oracle={oracle:.6} diff={:.6} > {}",
        (computed - oracle).abs(),
        tolerance::LENGTH_ACCURACY_MM
    );
}

// ── T-FR-026-06: split and recombine preserve shape and length ────────────────

/// Splitting a Bézier at t=0.5 and summing the halves' lengths must equal the
/// original length within tolerance.
#[test]
fn t_fr_026_06_split_sum_equals_original_length() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(20.0, 80.0), pt(80.0, 80.0), pt(100.0, 0.0));
    let original_len = path_from_seg(seg).arc_length_mm();

    let (left, right) = seg.split(0.5);
    let left_len = path_from_seg(left).arc_length_mm();
    let right_len = path_from_seg(right).arc_length_mm();

    let combined = left_len + right_len;
    assert!(
        (combined - original_len).abs() <= tolerance::LENGTH_ACCURACY_MM * 2.0,
        "split sum {combined:.6} ≠ original {original_len:.6}, diff {:.6}",
        (combined - original_len).abs()
    );
}

/// After splitting, the left half's endpoint must equal the right half's
/// start point (shape preservation).
#[test]
fn t_fr_026_06_split_join_point_matches() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(20.0, 80.0), pt(80.0, 80.0), pt(100.0, 0.0));
    let (left, right) = seg.split(0.5);

    let left_end = left.p3();
    let right_start = right.p0();

    let dx = (left_end.x_mm() - right_start.x_mm()).abs();
    let dy = (left_end.y_mm() - right_start.y_mm()).abs();
    assert!(
        dx < 1e-9 && dy < 1e-9,
        "split join point mismatch: left_end=({},{}) right_start=({},{})",
        left_end.x_mm(),
        left_end.y_mm(),
        right_start.x_mm(),
        right_start.y_mm()
    );
}

/// Reversing a Bézier preserves length.
#[test]
fn t_fr_026_06_reversal_preserves_length() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(20.0, 80.0), pt(80.0, 80.0), pt(100.0, 0.0));
    let original_len = path_from_seg(seg).arc_length_mm();
    let reversed_len = path_from_seg(seg.reverse()).arc_length_mm();

    assert!(
        (original_len - reversed_len).abs() <= tolerance::LENGTH_ACCURACY_MM,
        "reversal changed length: {original_len:.6} vs {reversed_len:.6}"
    );
}

/// Reversing a Bézier swaps start and end points.
#[test]
fn t_fr_026_06_reversal_swaps_endpoints() {
    let seg = CubicBez::new(
        pt(10.0, 20.0),
        pt(30.0, 80.0),
        pt(70.0, 80.0),
        pt(90.0, 30.0),
    );
    let rev = seg.reverse();
    assert!((rev.p0().x_mm() - seg.p3().x_mm()).abs() < 1e-9);
    assert!((rev.p0().y_mm() - seg.p3().y_mm()).abs() < 1e-9);
    assert!((rev.p3().x_mm() - seg.p0().x_mm()).abs() < 1e-9);
    assert!((rev.p3().y_mm() - seg.p0().y_mm()).abs() < 1e-9);
}

// ── T-NFR-014-02: repeated runs give bit-identical output ─────────────────────

/// Running arc_length_mm on the same curve twice must return the same f64 bits.
/// This checks INV-01 at the geometry level.
#[test]
fn t_nfr_014_02_arc_length_bit_identical() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(25.0, 75.0), pt(75.0, 75.0), pt(100.0, 0.0));
    let path = path_from_seg(seg);
    let a = path.arc_length_mm();
    let b = path.arc_length_mm();
    assert_eq!(
        a.to_bits(),
        b.to_bits(),
        "arc_length_mm is not bit-identical across runs"
    );
}

/// Tessellation of the same curve with the same tolerance must return the same
/// point sequence on every call.
#[test]
fn t_nfr_014_02_tessellation_bit_identical() {
    let seg = CubicBez::new(pt(0.0, 0.0), pt(25.0, 75.0), pt(75.0, 75.0), pt(100.0, 0.0));
    let pts_a = seg.tessellate(0.01);
    let pts_b = seg.tessellate(0.01);
    assert_eq!(
        pts_a.len(),
        pts_b.len(),
        "tessellation length differs between runs"
    );
    for (i, (a, b)) in pts_a.iter().zip(pts_b.iter()).enumerate() {
        assert_eq!(
            a.x_mm().to_bits(),
            b.x_mm().to_bits(),
            "tessellation x differs at index {i}"
        );
        assert_eq!(
            a.y_mm().to_bits(),
            b.y_mm().to_bits(),
            "tessellation y differs at index {i}"
        );
    }
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn pt(x: f64, y: f64) -> Point2 {
    Point2::from_mm(x, y).expect("finite")
}

/// Wrap a single Bézier segment in a Path for arc-length queries.
fn path_from_seg(seg: CubicBez) -> Path {
    Path::new(seg.p0(), vec![Segment::Cubic(seg)]).expect("single-segment path")
}

/// Oracle: dense-sampling arc length using the standard cubic Bézier formula.
/// This function must NOT call any of the production flatten/length code.
///
/// B(t) = (1-t)^3 P0 + 3(1-t)^2 t P1 + 3(1-t) t^2 P2 + t^3 P3
fn dense_sample_length(seg: CubicBez, n: usize) -> f64 {
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

    let mut len = 0.0_f64;
    let mut prev = eval(0.0);
    for i in 1..=n {
        let t = i as f64 / n as f64;
        let curr = eval(t);
        let dx = curr.0 - prev.0;
        let dy = curr.1 - prev.1;
        len += (dx * dx + dy * dy).sqrt();
        prev = curr;
    }
    len
}
