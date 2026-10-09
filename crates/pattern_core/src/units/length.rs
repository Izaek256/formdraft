/// Exact length value stored as an integer count of nanometres (ADR-0002).
///
/// # Storage
/// One nanometre = 1e-6 mm.  One millimetre = 1,000,000 nm (stored in an `i64`).
/// This range covers −9,223,372 m to +9,223,372 m, well beyond any sewing
/// measurement.
///
/// # Canonical unit
/// The canonical unit is the millimetre.  All conversions go through nm so that
/// no floating-point drift accumulates at the storage boundary.
///
/// # Fields
/// Private.  Construct with [`Length::from_mm`], [`Length::from_unit`], or
/// [`Length::from_nanometres`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Length {
    /// Exact nanometre count.  One mm = 1,000,000 nm.
    nm: i64,
}

/// Error returned when a floating-point value cannot be represented as a `Length`.
#[derive(Debug, Clone, PartialEq)]
pub enum LengthError {
    /// The value is NaN.
    NotANumber,
    /// The value is infinite.
    Infinite,
    /// The value overflows an i64 nanometre count.
    Overflow,
}

impl core::fmt::Display for LengthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LengthError::NotANumber => write!(f, "length value is NaN"),
            LengthError::Infinite => write!(f, "length value is infinite"),
            LengthError::Overflow => write!(f, "length value overflows the nanometre range"),
        }
    }
}

impl Length {
    /// Construct from a count of nanometres.  This is exact and infallible for
    /// any `i64`.
    #[inline]
    pub const fn from_nanometres(nm: i64) -> Self {
        Self { nm }
    }

    /// Construct from a millimetre value.
    ///
    /// # Errors
    /// Returns [`LengthError::NotANumber`] if `mm` is NaN, [`LengthError::Infinite`]
    /// if it is infinite, or [`LengthError::Overflow`] if the nm count overflows
    /// `i64`.
    pub fn from_mm(mm: f64) -> Result<Self, LengthError> {
        if mm.is_nan() {
            return Err(LengthError::NotANumber);
        }
        if mm.is_infinite() {
            return Err(LengthError::Infinite);
        }
        // 1 mm = 1,000,000 nm.  Use round() so that the nearest representable nm
        // is chosen rather than truncation.
        let nm_f = mm * 1_000_000.0;
        if nm_f > i64::MAX as f64 || nm_f < i64::MIN as f64 {
            return Err(LengthError::Overflow);
        }
        Ok(Self {
            nm: nm_f.round() as i64,
        })
    }

    /// Construct from a value in any supported unit.
    ///
    /// For units where the nm count is integer-exact (mm, cm, m, in, ft, yd for
    /// whole-unit values), the conversion is exact provided `value` is an integer
    /// or a power-of-two fraction that fits in an f64 mantissa without loss.
    ///
    /// # Errors
    /// Same as [`Length::from_mm`].
    pub fn from_unit(value: f64, unit: crate::units::Unit) -> Result<Self, LengthError> {
        if value.is_nan() {
            return Err(LengthError::NotANumber);
        }
        if value.is_infinite() {
            return Err(LengthError::Infinite);
        }
        let nm_f = value * unit.nm_per_unit_exact() as f64;
        if nm_f > i64::MAX as f64 || nm_f < i64::MIN as f64 {
            return Err(LengthError::Overflow);
        }
        Ok(Self {
            nm: nm_f.round() as i64,
        })
    }

    /// Return the exact nanometre count.
    #[inline]
    pub const fn nanometres(self) -> i64 {
        self.nm
    }

    /// Return the value in millimetres as `f64`.
    ///
    /// This conversion is lossy for values that are not multiples of 1e-6 mm, but
    /// for the domain of sewing measurements the precision is adequate.
    #[inline]
    pub fn to_mm(self) -> f64 {
        self.nm as f64 / 1_000_000.0
    }

    /// Return the value in the given unit as `f64`.
    pub fn to_unit(self, unit: crate::units::Unit) -> f64 {
        self.to_mm() / unit.mm_per_unit()
    }

    /// The zero length.
    pub const ZERO: Self = Self { nm: 0 };
}

impl core::fmt::Display for Length {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} mm", self.to_mm())
    }
}
