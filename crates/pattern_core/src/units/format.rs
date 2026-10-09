/// Length formatter (AC-FR-038-2, AC-FR-038-8, OQ-33).
///
/// Rules (OQ-33 decision, 2026-10-09):
/// - Decimal places per unit: mm 1, cm 1, m 3, in 2 (decimal mode), ft 3, yd 3.
/// - Inch fraction mode: nearest 1/N where N ∈ {2,4,8,16,32,64}, default 16.
/// - Rounding: half away from zero (display only, never changes stored values).
/// - Suffix: canonical lowercase with a space ("25.5 mm", "1 in", "1 8/16 in").
use crate::units::{Length, Unit};

/// How to display inch values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InchMode {
    /// Format as a decimal number.
    Decimal,
    /// Format as a whole number plus a fraction with the given denominator.
    Fraction {
        /// Must be a power of two in [1, 64].
        denominator: u8,
    },
}

/// Options controlling how a [`Length`] is formatted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatOptions {
    /// The unit to display in.
    pub unit: Unit,
    /// Number of decimal places for decimal display.
    pub decimal_places: u8,
    /// How to display inch values (ignored for non-inch units).
    pub inch_mode: InchMode,
    /// Default inch denominator (used when `inch_mode` is `Decimal` but
    /// the caller still needs to carry a denominator preference).
    pub inch_denominator: u8,
}

impl FormatOptions {
    /// Default options for a given unit, using the OQ-33 precision table.
    pub fn default_for(unit: Unit) -> Self {
        let (decimal_places, inch_mode) = match unit {
            Unit::Mm => (1, InchMode::Decimal),
            Unit::Cm => (1, InchMode::Decimal),
            Unit::M => (3, InchMode::Decimal),
            Unit::In => (2, InchMode::Fraction { denominator: 16 }),
            Unit::Ft => (3, InchMode::Decimal),
            Unit::Yd => (3, InchMode::Decimal),
        };
        Self {
            unit,
            decimal_places,
            inch_mode,
            inch_denominator: 16,
        }
    }
}

/// Format `length` according to `opts`.
///
/// The output string ends with a space and the unit suffix.  Rounding is half
/// away from zero (AC-FR-038-8).
pub fn format_length(length: Length, opts: &FormatOptions) -> String {
    let value = length.to_unit(opts.unit);

    if opts.unit == Unit::In {
        match opts.inch_mode {
            InchMode::Fraction { denominator } => {
                return format_inch_fraction(value, denominator);
            }
            InchMode::Decimal => {}
        }
    }

    // Decimal formatting with half-away-from-zero rounding.
    let scale = 10f64.powi(opts.decimal_places as i32);
    // Multiply, round half-away-from-zero, then divide back.
    let rounded = if value >= 0.0 {
        (value * scale + 0.5).floor() / scale
    } else {
        (value * scale - 0.5).ceil() / scale
    };

    // Format to exactly `decimal_places` digits.
    format!(
        "{:.prec$} {}",
        rounded,
        opts.unit.suffix(),
        prec = opts.decimal_places as usize
    )
}

/// Format an inch value as whole + fraction, e.g. "1 8/16 in".
/// Rounding is to the nearest 1/denominator inch, half away from zero.
fn format_inch_fraction(value_in: f64, denominator: u8) -> String {
    let denom = denominator as f64;
    // Total number of 1/denom slots, rounded half away from zero.
    let sign = if value_in < 0.0 { -1.0 } else { 1.0 };
    let abs = value_in.abs();
    let total_slots = (abs * denom + 0.5).floor() as i64;
    let sign_i = if value_in < 0.0 { -1i64 } else { 1 };

    let whole = total_slots / denominator as i64;
    let num = total_slots % denominator as i64;

    // Suppress fraction when it rounds to zero.
    let _ = sign; // used implicitly via sign_i
    let whole_val = sign_i * whole;
    if num == 0 {
        format!("{whole_val} in")
    } else {
        if whole == 0 {
            format!("{}/{} in", sign_i * num, denominator)
        } else {
            format!("{whole_val} {num}/{denominator} in")
        }
    }
}
