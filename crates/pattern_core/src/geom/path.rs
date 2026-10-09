/// Path types: ordered segment lists and closed-path validation (decision D1).
///
/// # Coordinate system
/// x right, y down, millimetres.  Closed outlines wind clockwise as seen on
/// screen (decision D1).
///
/// # Types
/// - [`Segment`]    — a line-to or cubic Bézier segment
/// - [`Path`]       — an ordered list of segments with a start point
/// - [`ClosedPath`] — a `Path` that has passed [`Path::try_close`] validation
///
/// # Validation rules for [`ClosedPath`]
/// 1. End meets start within [`tolerance::CLOSURE_GAP_MM`].
/// 2. All coordinate values are finite.
/// 3. No zero-length segment.
/// 4. Non-zero signed area (not a degenerate back-and-forth path).
use crate::geom::{point::Point2, tolerance, CubicBez};

/// A single segment — either a straight line to a point or a cubic Bézier.
///
/// The segment's *start* is the previous segment's end (or the path's start
/// point for the first segment).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Segment {
    /// Straight line to the given endpoint.
    Line(Point2),
    /// Cubic Bézier; the `.p0()` field carries the segment's start point for
    /// consistency but the path builder uses only `.p3()` as the new end.
    Cubic(CubicBez),
}

impl Segment {
    /// The endpoint of this segment.
    pub fn end_point(self) -> Point2 {
        match self {
            Segment::Line(p) => p,
            Segment::Cubic(b) => b.p3(),
        }
    }

    /// The start point of this segment (P0 for Bézier, previous end for Line).
    /// For `Line`, the start must be tracked by the caller.
    pub fn start_point_cubic(self) -> Option<Point2> {
        match self {
            Segment::Line(_) => None,
            Segment::Cubic(b) => Some(b.p0()),
        }
    }

    /// Arc length of this segment in mm.
    pub fn arc_length_mm(self, start: Point2) -> f64 {
        match self {
            Segment::Line(end) => start.distance_to(end),
            Segment::Cubic(b) => b.arc_length_mm(),
        }
    }

    /// Is this segment zero-length from `start`?
    pub fn is_zero_length(self, start: Point2) -> bool {
        let end = self.end_point();
        start.distance_to(end) < tolerance::CLOSURE_GAP_MM
    }
}

/// Errors returned by [`Path::new`] or [`Path::try_close`].
#[derive(Debug, Clone, PartialEq)]
pub enum PathError {
    /// The segment list is empty — a path needs at least one segment.
    EmptySegments,
    /// A segment has zero length from its start point.
    ZeroLengthSegment,
    /// The path end does not meet the start within [`tolerance::CLOSURE_GAP_MM`].
    NotClosed {
        /// Measured gap in mm.
        gap_mm: f64,
    },
    /// The signed area of the closed path is zero or negative (degenerate
    /// back-and-forth or counter-clockwise winding).
    ZeroArea,
    /// A coordinate value is not finite.
    NotFinite,
}

impl core::fmt::Display for PathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PathError::EmptySegments => write!(f, "path has no segments"),
            PathError::ZeroLengthSegment => write!(f, "path contains a zero-length segment"),
            PathError::NotClosed { gap_mm } => {
                write!(
                    f,
                    "path is not closed: end-to-start gap is {gap_mm:.6} mm (tolerance {:.6} mm)",
                    tolerance::CLOSURE_GAP_MM
                )
            }
            PathError::ZeroArea => write!(f, "closed path has zero or negative area"),
            PathError::NotFinite => write!(f, "path contains a non-finite coordinate"),
        }
    }
}

/// An open or closed ordered list of segments with a start point.
#[derive(Debug, Clone)]
pub struct Path {
    start: Point2,
    segments: Vec<Segment>,
}

impl Path {
    /// Construct a path from a start point and segment list.
    ///
    /// # Errors
    /// - [`PathError::EmptySegments`] if `segments` is empty.
    /// - [`PathError::ZeroLengthSegment`] if any segment is zero-length.
    /// - [`PathError::NotFinite`] if any coordinate is not finite.
    pub fn new(start: Point2, segments: Vec<Segment>) -> Result<Self, PathError> {
        if segments.is_empty() {
            return Err(PathError::EmptySegments);
        }

        // Check all coordinates are finite and no segment is zero-length.
        let mut current = start;
        for seg in &segments {
            if !current.x_mm().is_finite() || !current.y_mm().is_finite() {
                return Err(PathError::NotFinite);
            }
            let end = seg.end_point();
            if !end.x_mm().is_finite() || !end.y_mm().is_finite() {
                return Err(PathError::NotFinite);
            }
            if seg.is_zero_length(current) {
                return Err(PathError::ZeroLengthSegment);
            }
            current = end;
        }

        Ok(Self { start, segments })
    }

    /// Try to validate this path as a [`ClosedPath`].
    ///
    /// Checks:
    /// 1. End meets start within [`tolerance::CLOSURE_GAP_MM`].
    /// 2. Non-zero signed area (clockwise winding has positive area in y-down
    ///    coordinates).
    ///
    /// # Errors
    /// Returns [`PathError::NotClosed`] or [`PathError::ZeroArea`].
    pub fn try_close(self) -> Result<ClosedPath, PathError> {
        let end = self
            .segments
            .last()
            .map(|s| s.end_point())
            .unwrap_or(self.start);

        let gap = self.start.distance_to(end);
        if gap > tolerance::CLOSURE_GAP_MM {
            return Err(PathError::NotClosed { gap_mm: gap });
        }

        // Compute signed area using the shoelace formula over the tessellated
        // polyline.  In y-down coordinates, clockwise winding gives a positive
        // shoelace result.
        let area = signed_area(&self);
        if area.abs() < 1e-6 {
            return Err(PathError::ZeroArea);
        }

        Ok(ClosedPath { path: self })
    }

    /// Arc length of this path in mm.
    pub fn arc_length_mm(&self) -> f64 {
        let mut current = self.start;
        let mut total = 0.0_f64;
        for seg in &self.segments {
            total += seg.arc_length_mm(current);
            current = seg.end_point();
        }
        total
    }

    /// The start point of the path.
    pub fn start(&self) -> Point2 {
        self.start
    }

    /// The segments of the path.
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

/// A path that has passed closure and area validation.
///
/// Obtained via [`Path::try_close`].
#[derive(Debug, Clone)]
pub struct ClosedPath {
    path: Path,
}

impl ClosedPath {
    /// Access the underlying path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Arc length of the closed path in mm.
    pub fn arc_length_mm(&self) -> f64 {
        self.path.arc_length_mm()
    }
}

/// Compute the signed area of a path using the shoelace formula on tessellated
/// points.
///
/// In y-down screen coordinates, a clockwise polygon has positive signed area.
fn signed_area(path: &Path) -> f64 {
    // Collect tessellated points.
    let mut pts: Vec<Point2> = Vec::new();
    pts.push(path.start);

    for seg in &path.segments {
        match seg {
            Segment::Line(end) => pts.push(*end),
            Segment::Cubic(b) => {
                let tess = b.tessellate(tolerance::EXPORT_TESSELLATION_MM);
                for p in tess.iter().skip(1) {
                    pts.push(*p);
                }
            }
        }
    }

    // Shoelace formula: Σ (xᵢ · yᵢ₊₁ − xᵢ₊₁ · yᵢ) / 2
    let n = pts.len();
    let mut area = 0.0_f64;
    for i in 0..n {
        let j = (i + 1) % n;
        area += pts[i].x_mm() * pts[j].y_mm();
        area -= pts[j].x_mm() * pts[i].y_mm();
    }
    area / 2.0
}
