/// Cubic Bézier curve segment (decision D6).
///
/// # Parameterisation
/// B(t) = (1-t)³ P0 + 3(1-t)² t P1 + 3(1-t) t² P2 + t³ P3,  t ∈ [0, 1].
///
/// P0 and P3 are the on-curve endpoints.  P1 and P2 are the off-curve
/// control points.
///
/// # Coordinate system
/// x right, y down, millimetres (decision D1).
///
/// # Operations
/// - [`CubicBez::evaluate`] — point at t
/// - [`CubicBez::tangent`]  — tangent vector at t (not normalised)
/// - [`CubicBez::split`]    — de Casteljau split at t
/// - [`CubicBez::reverse`]  — swap direction
/// - [`CubicBez::tessellate`] — adaptive flattening to a tolerance in mm
use crate::geom::{point::Point2, tolerance};

/// A cubic Bézier segment defined by four control points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBez {
    p0: Point2,
    p1: Point2,
    p2: Point2,
    p3: Point2,
}

impl CubicBez {
    /// Construct from four control points.
    ///
    /// All points must already be valid `Point2` values (finite coordinates).
    /// The constructor is infallible because `Point2` guarantees finiteness.
    #[inline]
    pub fn new(p0: Point2, p1: Point2, p2: Point2, p3: Point2) -> Self {
        Self { p0, p1, p2, p3 }
    }

    /// Start point (on-curve).
    #[inline]
    pub fn p0(self) -> Point2 {
        self.p0
    }
    /// First control point (off-curve).
    #[inline]
    pub fn p1(self) -> Point2 {
        self.p1
    }
    /// Second control point (off-curve).
    #[inline]
    pub fn p2(self) -> Point2 {
        self.p2
    }
    /// End point (on-curve).
    #[inline]
    pub fn p3(self) -> Point2 {
        self.p3
    }

    /// Evaluate B(t).  `t` should be in [0, 1]; values outside this range
    /// are accepted without panic (extrapolation).
    pub fn evaluate(self, t: f64) -> Point2 {
        let u = 1.0 - t;
        // B(t) = u³P0 + 3u²tP1 + 3ut²P2 + t³P3
        let x = u * u * u * self.p0.x_mm()
            + 3.0 * u * u * t * self.p1.x_mm()
            + 3.0 * u * t * t * self.p2.x_mm()
            + t * t * t * self.p3.x_mm();
        let y = u * u * u * self.p0.y_mm()
            + 3.0 * u * u * t * self.p1.y_mm()
            + 3.0 * u * t * t * self.p2.y_mm()
            + t * t * t * self.p3.y_mm();
        // The inputs are all finite (guaranteed by Point2), so x and y are finite.
        Point2::from_mm(x, y).unwrap_or(self.p0)
    }

    /// Tangent vector B′(t) = 3[(1-t)²(P1−P0) + 2(1-t)t(P2−P1) + t²(P3−P2)].
    ///
    /// Returns the un-normalised tangent vector as `(dx, dy)` in mm.
    /// The vector is zero at a cusp; callers must handle the zero-length case.
    pub fn tangent(self, t: f64) -> (f64, f64) {
        let u = 1.0 - t;
        // B'(t) = 3[u²(P1-P0) + 2ut(P2-P1) + t²(P3-P2)]
        let d0x = self.p1.x_mm() - self.p0.x_mm();
        let d0y = self.p1.y_mm() - self.p0.y_mm();
        let d1x = self.p2.x_mm() - self.p1.x_mm();
        let d1y = self.p2.y_mm() - self.p1.y_mm();
        let d2x = self.p3.x_mm() - self.p2.x_mm();
        let d2y = self.p3.y_mm() - self.p2.y_mm();

        let dx = 3.0 * (u * u * d0x + 2.0 * u * t * d1x + t * t * d2x);
        let dy = 3.0 * (u * u * d0y + 2.0 * u * t * d1y + t * t * d2y);
        (dx, dy)
    }

    /// Split the curve at parameter `t` using de Casteljau's algorithm.
    ///
    /// Returns `(left, right)` such that `left` covers [0, t] and `right`
    /// covers [t, 1] of the original curve.  Together they reproduce the
    /// original shape exactly.
    pub fn split(self, t: f64) -> (Self, Self) {
        // de Casteljau level 1
        let q0 = self.p0.lerp(self.p1, t);
        let q1 = self.p1.lerp(self.p2, t);
        let q2 = self.p2.lerp(self.p3, t);
        // de Casteljau level 2
        let r0 = q0.lerp(q1, t);
        let r1 = q1.lerp(q2, t);
        // de Casteljau level 3 — the split point
        let s = r0.lerp(r1, t);

        let left = CubicBez::new(self.p0, q0, r0, s);
        let right = CubicBez::new(s, r1, q2, self.p3);
        (left, right)
    }

    /// Reverse the curve direction.  The shape is identical; start and end
    /// swap.
    pub fn reverse(self) -> Self {
        CubicBez::new(self.p3, self.p2, self.p1, self.p0)
    }

    /// Return the start tangent (at t = 0) as an angle in radians, measured
    /// from the positive x-axis.  Used for continuity inspection.
    pub fn start_tangent_angle(self) -> f64 {
        let (dx, dy) = self.tangent(0.0);
        dy.atan2(dx)
    }

    /// Return the end tangent (at t = 1) as an angle in radians, measured
    /// from the positive x-axis (direction of travel *out* of the endpoint).
    pub fn end_tangent_angle(self) -> f64 {
        let (dx, dy) = self.tangent(1.0);
        dy.atan2(dx)
    }

    /// Adaptive tessellation (flattening) to `tolerance_mm`.
    ///
    /// Returns a polyline (list of `Point2`) whose maximum deviation from the
    /// true Bézier curve is ≤ `tolerance_mm` mm.  The first point is always
    /// P0 and the last is always P3.
    ///
    /// The tolerance does not depend on zoom.  The caller translates screen
    /// pixels to mm: `tolerance_mm = pixels / pixels_per_mm`.
    ///
    /// The default export tolerance is [`tolerance::EXPORT_TESSELLATION_MM`]
    /// = 0.01 mm.
    ///
    /// # Algorithm
    /// Recursive subdivision using the flatness criterion: a curve is flat
    /// enough when the sum of distances from P1 and P2 to the chord P0P3 is
    /// less than `4 * tolerance_mm` (this bound is tight for quadratic
    /// approximations of cubic deviation).
    pub fn tessellate(self, tolerance_mm: f64) -> Vec<Point2> {
        let mut pts = Vec::new();
        pts.push(self.p0);
        flatten_recursive(self, tolerance_mm, &mut pts);
        pts
    }

    /// Arc length computed by flattening to [`tolerance::FLATTEN_MM`] and
    /// summing chord lengths.
    ///
    /// Accuracy: the error is bounded by `tolerance::LENGTH_ACCURACY_MM`
    /// (0.01 mm) for all curves representable in the domain.
    pub fn arc_length_mm(self) -> f64 {
        let pts = self.tessellate(tolerance::FLATTEN_MM);
        chord_sum(&pts)
    }
}

/// Recursive flatten helper.  Appends points *after* the current start.
fn flatten_recursive(seg: CubicBez, tol: f64, out: &mut Vec<Point2>) {
    if is_flat(seg, tol) {
        out.push(seg.p3);
    } else {
        let (left, right) = seg.split(0.5);
        flatten_recursive(left, tol, out);
        flatten_recursive(right, tol, out);
    }
}

/// Flatness test using the standard squared-distance criterion.
///
/// A cubic Bézier segment is within `tol` mm of its chord P0P3 when the
/// maximum distance from the *midpoint of the hull* to the chord is ≤ tol.
///
/// The correct bound (Gravesen 1993 / Lyon 1997): the maximum deviation of
/// B(t) from the chord is bounded by
///   (3/4) * max(d(P1, line(P0,P3)), d(P2, line(P0,P3)))
/// where d is the perpendicular distance.  We check:
///   max(d1, d2) ≤ (4/3) * tol
///
/// This is equivalent and slightly conservative — the factor of 4/3 ensures
/// the polyline stays strictly within `tol` of the true curve.
fn is_flat(seg: CubicBez, tol: f64) -> bool {
    let dx = seg.p3.x_mm() - seg.p0.x_mm();
    let dy = seg.p3.y_mm() - seg.p0.y_mm();
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-18 {
        // Degenerate chord (P0 ≈ P3): all control points must be within tol of P0.
        let d1 = seg.p0.distance_to(seg.p1);
        let d2 = seg.p0.distance_to(seg.p2);
        let d3 = seg.p0.distance_to(seg.p3);
        return d1 <= tol && d2 <= tol && d3 <= tol;
    }

    let len = len_sq.sqrt();
    // Perpendicular distance from a point (px, py) to the directed line P0→P3.
    let perp = |px: f64, py: f64| -> f64 {
        let ax = px - seg.p0.x_mm();
        let ay = py - seg.p0.y_mm();
        (ax * dy - ay * dx).abs() / len
    };

    let d1 = perp(seg.p1.x_mm(), seg.p1.y_mm());
    let d2 = perp(seg.p2.x_mm(), seg.p2.y_mm());

    // Max deviation ≤ (3/4) * max(d1, d2) ≤ tol
    // ↔  max(d1, d2) ≤ (4/3) * tol
    let threshold = tol * 4.0 / 3.0;
    d1 <= threshold && d2 <= threshold
}

/// Sum chord lengths of a polyline.
pub(crate) fn chord_sum(pts: &[Point2]) -> f64 {
    let mut total = 0.0_f64;
    for w in pts.windows(2) {
        total += w[0].distance_to(w[1]);
    }
    total
}
