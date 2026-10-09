/// Two-dimensional point in canonical millimetre geometry (decision D1, D5).
///
/// # Coordinate system
/// x increases to the right, y increases downward (as in SVG and egui).
/// Units are millimetres.  PDF export flips y later.
///
/// # Storage
/// Fields are private.  The only crossings between [`Length`] (exact nm) and
/// `f64` mm are [`Point2::from_lengths`] and [`Point2::to_lengths`].
///
/// # Validity
/// A `Point2` always holds finite f64 mm values.  The constructors reject
/// NaN and infinity via the `Length` type.
use crate::units::{Length, LengthError};

/// Error returned when a `Point2` cannot be constructed.
#[derive(Debug, Clone, PartialEq)]
pub enum PointError {
    /// The x or y value is not a finite number.
    NotFinite(LengthError),
}

impl core::fmt::Display for PointError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PointError::NotFinite(e) => write!(f, "point coordinate is not finite: {e}"),
        }
    }
}

impl From<LengthError> for PointError {
    fn from(e: LengthError) -> Self {
        PointError::NotFinite(e)
    }
}

/// A point in 2-D millimetre space with private `f64` fields.
///
/// Constructed from [`Length`] values; coordinates are stored as `f64` mm
/// for fast geometry computation (ADR-0002).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2 {
    /// x in mm, increasing rightward.
    x: f64,
    /// y in mm, increasing downward.
    y: f64,
}

impl Point2 {
    /// Construct from two [`Length`] values (the canonical crossing point
    /// between exact nm storage and `f64` geometry).
    ///
    /// # Errors
    /// Returns [`PointError`] if either length is NaN, infinite, or overflows.
    pub fn from_lengths(x: Length, y: Length) -> Result<Self, PointError> {
        // Length stores i64 nm, conversion to mm is always finite.
        Ok(Self {
            x: x.to_mm(),
            y: y.to_mm(),
        })
    }

    /// Convert back to two [`Length`] values.
    ///
    /// The round-trip `nm → f64 mm → nm` is exact for all values that can be
    /// represented as an integer nm count up to ±1 km (ADR-0002, T-REN-01-03).
    pub fn to_lengths(self) -> (Length, Length) {
        (
            Length::from_mm(self.x).unwrap_or(Length::ZERO),
            Length::from_mm(self.y).unwrap_or(Length::ZERO),
        )
    }

    /// Construct directly from `f64` mm values (geometry-internal use).
    ///
    /// # Errors
    /// Returns [`PointError`] if either value is not finite.
    pub fn from_mm(x_mm: f64, y_mm: f64) -> Result<Self, PointError> {
        if !x_mm.is_finite() || !y_mm.is_finite() {
            return Err(PointError::NotFinite(crate::units::LengthError::NotANumber));
        }
        Ok(Self { x: x_mm, y: y_mm })
    }

    /// x coordinate in mm.
    #[inline]
    pub fn x_mm(self) -> f64 {
        self.x
    }

    /// y coordinate in mm.
    #[inline]
    pub fn y_mm(self) -> f64 {
        self.y
    }

    /// Euclidean distance to another point, in mm.
    pub fn distance_to(self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Interpolate linearly between `self` and `other` at parameter `t`.
    /// `t = 0` gives `self`, `t = 1` gives `other`.
    #[inline]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        Self {
            x: self.x + t * (other.x - self.x),
            y: self.y + t * (other.y - self.y),
        }
    }
}
