/// The six supported length units (AC-FR-038-1).
///
/// Conversion factors to millimetres:
/// - `Mm`:  1 mm  = 1 mm
/// - `Cm`:  1 cm  = 10 mm
/// - `M`:   1 m   = 1000 mm
/// - `In`:  1 in  = 25.4 mm  (exact)
/// - `Ft`:  1 ft  = 304.8 mm (exact)
/// - `Yd`:  1 yd  = 914.4 mm (exact)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// Millimetres
    Mm,
    /// Centimetres
    Cm,
    /// Metres
    M,
    /// Inches
    In,
    /// Feet
    Ft,
    /// Yards
    Yd,
}

impl Unit {
    /// Millimetres per one unit of `self`, as an exact rational expressed as
    /// `(numerator_nm, denominator)` where `value_nm = value_in_unit * numerator_nm`.
    ///
    /// For units where the mm value is an integer multiple of mm, denominator is 1.
    /// For inches/feet/yards the factor is held as an exact i64 nm count:
    /// - 1 in = 25,400,000 nm
    /// - 1 ft = 304,800,000 nm
    /// - 1 yd = 914,400,000 nm
    ///
    /// The caller multiplies these by a rational quantity to get nm.
    pub(crate) fn nm_per_unit_exact(self) -> i64 {
        match self {
            Unit::Mm => 1_000_000,
            Unit::Cm => 10_000_000,
            Unit::M => 1_000_000_000,
            Unit::In => 25_400_000,
            Unit::Ft => 304_800_000,
            Unit::Yd => 914_400_000,
        }
    }

    /// Millimetres per one unit, as f64 for display/geometry use.
    ///
    /// This is the floating-point companion to `nm_per_unit_exact`.  It is used
    /// only when converting *from* nm to a display value or when the caller has
    /// already left the exact integer domain.
    pub fn mm_per_unit(self) -> f64 {
        match self {
            Unit::Mm => 1.0,
            Unit::Cm => 10.0,
            Unit::M => 1000.0,
            Unit::In => 25.4,
            Unit::Ft => 304.8,
            Unit::Yd => 914.4,
        }
    }

    /// The canonical lowercase suffix string for this unit (OQ-33 decision).
    pub fn suffix(self) -> &'static str {
        match self {
            Unit::Mm => "mm",
            Unit::Cm => "cm",
            Unit::M => "m",
            Unit::In => "in",
            Unit::Ft => "ft",
            Unit::Yd => "yd",
        }
    }

    /// Parse a suffix string (case-insensitive).  Returns `None` if the string
    /// is not a recognised suffix.
    pub fn from_suffix(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "mm" => Some(Unit::Mm),
            "cm" => Some(Unit::Cm),
            "m" => Some(Unit::M),
            "in" => Some(Unit::In),
            "ft" => Some(Unit::Ft),
            "yd" => Some(Unit::Yd),
            _ => None,
        }
    }
}

/// A quantity class names the kind of measurement.  Preferences can be set per
/// class (AC-FR-038-4, OQ-16, OQ-35).
///
/// Default display units per class (PROVISIONAL, OQ-16 decision 2026-10-09):
/// - `BodyMeasurement` → cm
/// - `PatternDimension` → mm
/// - `AllowanceOrTolerance` → mm
/// - `FabricWidth` → cm
/// - `FabricLength` → m
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantityClass {
    BodyMeasurement,
    PatternDimension,
    AllowanceOrTolerance,
    FabricWidth,
    FabricLength,
}

impl QuantityClass {
    /// PROVISIONAL default display unit per class (OQ-16 decision, 2026-10-09).
    pub fn provisional_default_unit(self) -> Unit {
        match self {
            QuantityClass::BodyMeasurement => Unit::Cm,
            QuantityClass::PatternDimension => Unit::Mm,
            QuantityClass::AllowanceOrTolerance => Unit::Mm,
            QuantityClass::FabricWidth => Unit::Cm,
            QuantityClass::FabricLength => Unit::M,
        }
    }
}

/// The scope at which a unit preference is set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scope {
    Global,
    Project,
}
