/// Canonical millimetre geometry (S02).
///
/// # Coordinate system (decision D1)
/// - x increases to the right
/// - y increases downward (as in SVG and egui)
/// - units are millimetres
/// - closed piece outlines wind clockwise as seen on screen
/// - PDF export flips y — this module does not handle that
///
/// # Design (decisions D1–D7)
/// - [`Point2`]      private f64 fields; from_lengths/to_lengths are the only
///   nm↔mm crossing points (ADR-0002)
/// - [`Segment`]     a line-to or cubic Bézier
/// - [`Path`]        ordered list of segments with a start point
/// - [`ClosedPath`]  a Path that passed validation
/// - [`CubicBez`]    evaluate, tangent, de-Casteljau split, reverse, tessellate
/// - [`validate`]    continuity inspection (position gap, tangent angle)
/// - [`tolerance`]   engineering tolerance constants
///
/// # Out of scope for S02
/// - Offsets and self-intersection (S08)
/// - Checksums (S06)
/// - Bounding boxes (deferred until an exporter needs them)
/// - Specific waistline/side-seam curves (OQ-11, deferred)
///
/// # Dependencies
/// None beyond `pattern_core::units`.  No HashMap in anything that feeds output.
/// No time, no randomness, no I/O.  Deterministic on every call (INV-01).
pub mod tolerance;
pub mod validate;

mod bezier;
mod path;
mod point;

pub use bezier::CubicBez;
pub use path::{ClosedPath, Path, PathError, Segment};
pub use point::{Point2, PointError};
