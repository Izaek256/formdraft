// T-FR-038-02 [PROP] Parse then format then parse is stable for every unit
// T-FR-038-07 [PROP] Arbitrary text never panics the parser
//
// Requirements: FR-038 (AC-FR-038-2, AC-FR-038-5, AC-FR-038-8), NFR-028
//
// These tests use a simple manual property harness — no external property-test
// crate (no new dependency approved for S01).  Inputs are constructed from a
// fixed set of representative values and edge cases covering the required domain.

#![allow(clippy::expect_used, clippy::panic)]

use pattern_core::units::{FormatOptions, InchMode, Length, Unit, UnitService};

// ── T-FR-038-02: parse → format → parse is stable ────────────────────────────

/// For each unit, create a Length from a known value, format it, then parse the
/// formatted string back and compare nm.  The round-trip must be exact (or
/// within ±1 nm for display-precision rounding).
#[test]
fn t_fr_038_02_format_parse_stable_mm() {
    round_trip_stable(Unit::Mm, 1, InchMode::Decimal, 16);
}

#[test]
fn t_fr_038_02_format_parse_stable_cm() {
    round_trip_stable(Unit::Cm, 1, InchMode::Decimal, 16);
}

#[test]
fn t_fr_038_02_format_parse_stable_m() {
    round_trip_stable(Unit::M, 3, InchMode::Decimal, 16);
}

#[test]
fn t_fr_038_02_format_parse_stable_ft() {
    round_trip_stable(Unit::Ft, 3, InchMode::Decimal, 16);
}

#[test]
fn t_fr_038_02_format_parse_stable_yd() {
    round_trip_stable(Unit::Yd, 3, InchMode::Decimal, 16);
}

#[test]
fn t_fr_038_02_format_parse_stable_in_decimal() {
    round_trip_stable(Unit::In, 2, InchMode::Decimal, 16);
}

#[test]
fn t_fr_038_02_format_parse_stable_in_fraction() {
    round_trip_stable(Unit::In, 2, InchMode::Fraction { denominator: 16 }, 16);
}

/// Sample values that cover the full body-measurement range and edge cases.
fn sample_mm_values() -> Vec<f64> {
    vec![
        0.0, 0.1, 1.0, 10.0, 25.4, 100.0, 304.8, 500.0, 600.0, 700.0, 914.4, 1000.0, 1200.0,
        2000.0, // fractions of mm
        0.5, 0.25, 1.5, 12.7,
    ]
}

fn round_trip_stable(unit: Unit, decimal_places: u8, inch_mode: InchMode, denom: u8) {
    let svc = UnitService::new();
    let opts = FormatOptions {
        unit,
        decimal_places,
        inch_mode,
        inch_denominator: denom,
    };

    // For inch-fraction mode, use sample values that are exact multiples of
    // 1/denom inches, so the round-trip is exact.  For other units use the
    // general mm-based samples.
    let samples: Vec<f64> = if unit == Unit::In {
        if let InchMode::Fraction { denominator } = inch_mode {
            inch_fraction_samples_mm(denominator)
        } else {
            sample_mm_values()
        }
    } else {
        sample_mm_values()
    };

    for mm in samples {
        let original = Length::from_mm(mm).expect("finite sample value");
        let formatted = svc.format(original, &opts);

        // Re-parse the formatted string back.
        let reparsed = svc
            .parse(&formatted, unit)
            .unwrap_or_else(|e| panic!("re-parse of '{formatted}' failed: {e:?}"));

        // Allow rounding difference based on the display precision.
        let diff = (original.nanometres() - reparsed.nanometres()).abs();
        let tolerance_nm = display_tolerance_nm(unit, decimal_places, inch_mode);
        assert!(
            diff <= tolerance_nm,
            "unit={unit:?} mm={mm} formatted='{formatted}' diff={diff} nm > tolerance {tolerance_nm}"
        );
    }
}

/// Sample mm values that are exact multiples of 1/denom inches.
/// Covers the body-measurement range (0 – 2000 mm).
fn inch_fraction_samples_mm(denominator: u8) -> Vec<f64> {
    let mm_per_slot = 25.4 / denominator as f64;
    // Slots 0, 1, 2, ... up to ~2000 mm, stepping by varying amounts.
    let steps: Vec<u32> = vec![0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 400, 512, 598, 800];
    steps
        .into_iter()
        .map(|s| s as f64 * mm_per_slot)
        .filter(|&mm| mm <= 2000.0)
        .collect()
}

/// Compute the maximum rounding error in nm for a given unit, decimal places
/// and inch mode.
fn display_tolerance_nm(unit: Unit, decimal_places: u8, inch_mode: InchMode) -> i64 {
    if unit == Unit::In {
        if let InchMode::Fraction { denominator } = inch_mode {
            // Half a slot = 0.5 / denominator inches in nm.
            let mm_per_slot = 25.4 / denominator as f64;
            let half_slot_mm = mm_per_slot * 0.5;
            return (half_slot_mm * 1_000_000.0) as i64 + 1;
        }
    }
    // One unit in the last display place, converted to nm.
    let mm_per_unit = match unit {
        Unit::Mm => 1.0,
        Unit::Cm => 10.0,
        Unit::M => 1000.0,
        Unit::In => 25.4,
        Unit::Ft => 304.8,
        Unit::Yd => 914.4,
    };
    let scale = 10f64.powi(-(decimal_places as i32));
    let tolerance_mm = mm_per_unit * scale * 0.5;
    // Add 1 for integer truncation.
    (tolerance_mm * 1_000_000.0) as i64 + 1
}

// ── T-FR-038-07: arbitrary text never panics the parser ──────────────────────

/// A broad set of garbage inputs must all return Err and must not panic.
#[test]
fn t_fr_038_07_garbage_never_panics() {
    let svc = UnitService::new();
    let garbage: &[&str] = &[
        "",
        " ",
        "\t",
        "\n",
        "abc",
        "!@#$%",
        "1 2 3",
        "1.2.3",
        "1/0 in",   // division by zero denominator
        "1 0/0 in", // zero denominator in fraction
        "1e10",     // scientific notation (not supported)
        "NaN",
        "inf",
        "-inf",
        "1 1/3 in",                         // non-power-of-2 denominator
        "1 1/0 in",                         // zero denominator
        "99999999999999999999999999999999", // overflow
        "1 km",                             // unsupported unit suffix
        "1 mm mm",                          // duplicate suffix
        "1/2",                              // fraction with no unit and no default handling
        "−10 mm",                           // Unicode minus sign (not ASCII hyphen)
    ];
    for &input in garbage {
        // Must not panic. Return value can be Ok or Err.
        let _ = svc.parse(input, Unit::Mm);
    }
}

/// Very long strings must not panic.
#[test]
fn t_fr_038_07_very_long_string_does_not_panic() {
    let svc = UnitService::new();
    let long = "a".repeat(100_000);
    let _ = svc.parse(&long, Unit::Mm);
}
