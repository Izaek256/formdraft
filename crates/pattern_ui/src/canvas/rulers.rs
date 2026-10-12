//! Ruler tick maths (OQ-39/D9, T-SK01-13).
//!
//! Tick steps are 1, 2 or 5 times a power of ten in the *display* unit, for
//! every unit including inches. Labels are produced only by the S01 unit
//! service, never formatted here (ADR-0007).

use pattern_core::units::{format_length, FormatOptions, Length, Unit};

/// Largest number of ticks we will enumerate in one range. A guard against a
/// pathological zoom producing an enormous vector.
const MAX_TICKS: i64 = 20_000;

/// The smallest "nice" step, in display units, whose spacing is at least
/// `min_px` on screen.
pub fn step_in_unit(unit: Unit, px_per_mm: f64, min_px: f64) -> f64 {
    let px_per_unit = (px_per_mm * unit.mm_per_unit()).max(1e-9);
    for exp in -6..=12 {
        let decade = 10f64.powi(exp);
        for mantissa in [1.0, 2.0, 5.0] {
            let value = mantissa * decade;
            if value * px_per_unit >= min_px {
                return value;
            }
        }
    }
    5.0 * 10f64.powi(12)
}

/// The same step expressed in millimetres.
pub fn step_mm(unit: Unit, px_per_mm: f64, min_px: f64) -> f64 {
    step_in_unit(unit, px_per_mm, min_px) * unit.mm_per_unit()
}

/// The minor tick spacing (five minors per major), in millimetres.
pub fn minor_step_mm(major_step_mm: f64) -> f64 {
    major_step_mm / 5.0
}

/// Every major tick position, in millimetres, within `[min_mm, max_mm]`.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn ticks_mm(step_mm: f64, min_mm: f64, max_mm: f64) -> Vec<f64> {
    if !step_mm.is_finite() || step_mm <= 0.0 || !min_mm.is_finite() || !max_mm.is_finite() {
        return Vec::new();
    }
    if max_mm < min_mm {
        return Vec::new();
    }
    let first = (min_mm / step_mm).ceil();
    let last = (max_mm / step_mm).floor();
    let count = last - first;
    if count < 0.0 || count > MAX_TICKS as f64 {
        return Vec::new();
    }
    #[allow(clippy::cast_possible_truncation)]
    let count = count as i64;
    (0..=count).map(|i| (first + i as f64) * step_mm).collect()
}

/// A ruler label for a millimetre value, in the display unit, through the unit
/// service.
pub fn tick_label(unit: Unit, mm: f64) -> String {
    match Length::from_mm(mm) {
        Ok(length) => format_length(length, &FormatOptions::default_for(unit)),
        Err(_) => String::from("--"),
    }
}
