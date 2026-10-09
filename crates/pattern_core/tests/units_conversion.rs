// T-FR-038-01 [UNIT] Exact conversion for every unit pair
// T-FR-038-03 [UNIT] Suffix override and fractional inch parsing
// T-FR-038-08 [UNIT] Preference resolution order across scope and quantity class
// T-FR-038-09 [UNIT] Display rounding is half away from zero
//
// Requirements: FR-038 (AC-FR-038-1 to AC-FR-038-8), NFR-028

#![allow(clippy::expect_used)]

use pattern_core::units::{
    FormatOptions, InchMode, Length, ParseError, QuantityClass, Scope, Unit, UnitPreference,
    UnitService,
};

// ── T-FR-038-01: exact conversion for every unit pair ────────────────────────

/// 1 mm is 1,000,000 nm exactly.
#[test]
fn t_fr_038_01_mm_to_nm() {
    let l = Length::from_mm(1.0).expect("finite");
    assert_eq!(l.nanometres(), 1_000_000);
}

/// 1 cm = 10 mm.  Factor from AC-FR-038-1.
#[test]
fn t_fr_038_01_cm_to_mm() {
    let l = Length::from_unit(1.0, Unit::Cm).expect("finite");
    assert_eq!(l.nanometres(), 10_000_000);
}

/// 1 m = 1000 mm.  Factor from AC-FR-038-1.
#[test]
fn t_fr_038_01_m_to_mm() {
    let l = Length::from_unit(1.0, Unit::M).expect("finite");
    assert_eq!(l.nanometres(), 1_000_000_000);
}

/// 1 in = 25.4 mm exactly.  ADR-0002 guarantees exactness.
/// 25.4 mm = 25,400,000 nm.
#[test]
fn t_fr_038_01_in_to_mm() {
    let l = Length::from_unit(1.0, Unit::In).expect("finite");
    assert_eq!(l.nanometres(), 25_400_000);
}

/// 1 ft = 304.8 mm exactly.  304.8 mm = 304,800,000 nm.
#[test]
fn t_fr_038_01_ft_to_mm() {
    let l = Length::from_unit(1.0, Unit::Ft).expect("finite");
    assert_eq!(l.nanometres(), 304_800_000);
}

/// 1 yd = 914.4 mm exactly.  914.4 mm = 914,400,000 nm.
#[test]
fn t_fr_038_01_yd_to_mm() {
    let l = Length::from_unit(1.0, Unit::Yd).expect("finite");
    assert_eq!(l.nanometres(), 914_400_000);
}

/// Round-trip: store as nm then convert back to mm must be exact for a range of
/// whole-mm values and the critical 25.4 mm inch.
#[test]
fn t_fr_038_01_round_trip_mm() {
    for mm in [0.0_f64, 1.0, 10.0, 100.0, 25.4, 304.8, 914.4, 1000.0] {
        let l = Length::from_mm(mm).expect("finite");
        let back = l.to_mm();
        assert!((back - mm).abs() < 1e-9, "mm {mm} round-tripped to {back}");
    }
}

/// 1/64 inch = 396,875 nm exactly (from ADR-0002).
#[test]
fn t_fr_038_01_one_sixty_fourth_inch() {
    // 25.4 mm/in / 64 = 0.396875 mm = 396,875 nm
    let l = Length::from_unit(1.0 / 64.0, Unit::In).expect("finite");
    assert_eq!(l.nanometres(), 396_875);
}

// ── T-FR-038-03: suffix override and fractional inch parsing ─────────────────

/// "94 cm" parses to 940 mm regardless of field default.
#[test]
fn t_fr_038_03_suffix_cm() {
    let svc = UnitService::new();
    let l = svc.parse("94 cm", Unit::Mm).expect("valid");
    assert_eq!(l.nanometres(), 940_000_000);
}

/// "37 3/8 in" parses correctly.  37 3/8 in = 37.375 in = 949.325 mm.
/// 949.325 mm = 949,325,000 nm.
#[test]
fn t_fr_038_03_fractional_inch() {
    let svc = UnitService::new();
    let l = svc.parse("37 3/8 in", Unit::Mm).expect("valid");
    assert_eq!(l.nanometres(), 949_325_000);
}

/// Plain "37.375 in" also parses.
#[test]
fn t_fr_038_03_decimal_inch_with_suffix() {
    let svc = UnitService::new();
    let l = svc.parse("37.375 in", Unit::Mm).expect("valid");
    assert_eq!(l.nanometres(), 949_325_000);
}

/// "10" with default mm parses as 10 mm.
#[test]
fn t_fr_038_03_no_suffix_uses_default() {
    let svc = UnitService::new();
    let l = svc.parse("10", Unit::Mm).expect("valid");
    assert_eq!(l.nanometres(), 10_000_000);
}

/// "10" with default cm parses as 100 mm.
#[test]
fn t_fr_038_03_no_suffix_uses_cm_default() {
    let svc = UnitService::new();
    let l = svc.parse("10", Unit::Cm).expect("valid");
    assert_eq!(l.nanometres(), 100_000_000);
}

/// Suffix "IN" (uppercase) is accepted (case-insensitive per OQ-33 decision).
#[test]
fn t_fr_038_03_case_insensitive_suffix() {
    let svc = UnitService::new();
    let l = svc.parse("1 IN", Unit::Mm).expect("valid");
    assert_eq!(l.nanometres(), 25_400_000);
}

/// Malformed input returns an error, not a panic or a silently wrong value.
#[test]
fn t_fr_038_03_malformed_returns_error() {
    let svc = UnitService::new();
    assert!(svc.parse("abc", Unit::Mm).is_err());
    assert!(svc.parse("", Unit::Mm).is_err());
    assert!(svc.parse("1 2 3", Unit::Mm).is_err());
}

/// Unsupported fraction denominator (e.g. 1/3) returns an error.
#[test]
fn t_fr_038_03_bad_fraction_denominator() {
    let svc = UnitService::new();
    // 1/3 inch: denominator 3 is not a power-of-2 ≤ 64.
    assert!(matches!(
        svc.parse("1 1/3 in", Unit::Mm),
        Err(ParseError::FractionDenominator(_))
    ));
}

// ── T-FR-038-08: preference resolution order ─────────────────────────────────
// Most specific wins: project+class > global+class > project default > global default.

#[test]
fn t_fr_038_08_global_default_is_fallback() {
    let mut svc = UnitService::new();
    svc.set_preference(UnitPreference {
        scope: Scope::Global,
        class: None,
        unit: Unit::Cm,
    });
    let resolved = svc.resolve(QuantityClass::PatternDimension, Scope::Global);
    assert_eq!(resolved, Unit::Cm);
}

#[test]
fn t_fr_038_08_project_default_beats_global_default() {
    let mut svc = UnitService::new();
    svc.set_preference(UnitPreference {
        scope: Scope::Global,
        class: None,
        unit: Unit::Cm,
    });
    svc.set_preference(UnitPreference {
        scope: Scope::Project,
        class: None,
        unit: Unit::Mm,
    });
    let resolved = svc.resolve(QuantityClass::PatternDimension, Scope::Project);
    assert_eq!(resolved, Unit::Mm);
}

#[test]
fn t_fr_038_08_global_class_beats_project_default() {
    let mut svc = UnitService::new();
    svc.set_preference(UnitPreference {
        scope: Scope::Project,
        class: None,
        unit: Unit::Cm,
    });
    svc.set_preference(UnitPreference {
        scope: Scope::Global,
        class: Some(QuantityClass::PatternDimension),
        unit: Unit::M,
    });
    let resolved = svc.resolve(QuantityClass::PatternDimension, Scope::Project);
    assert_eq!(resolved, Unit::M);
}

#[test]
fn t_fr_038_08_project_class_beats_global_class() {
    let mut svc = UnitService::new();
    svc.set_preference(UnitPreference {
        scope: Scope::Global,
        class: Some(QuantityClass::PatternDimension),
        unit: Unit::M,
    });
    svc.set_preference(UnitPreference {
        scope: Scope::Project,
        class: Some(QuantityClass::PatternDimension),
        unit: Unit::Ft,
    });
    let resolved = svc.resolve(QuantityClass::PatternDimension, Scope::Project);
    assert_eq!(resolved, Unit::Ft);
}

// ── T-FR-038-09: display rounding is half away from zero ─────────────────────

/// 25.45 mm formatted in mm with 1 decimal place rounds to "25.5 mm".
#[test]
fn t_fr_038_09_round_half_away_from_zero_positive() {
    let svc = UnitService::new();
    let l = Length::from_mm(25.45).expect("finite");
    let opts = FormatOptions {
        unit: Unit::Mm,
        decimal_places: 1,
        inch_mode: InchMode::Decimal,
        inch_denominator: 16,
    };
    let s = svc.format(l, &opts);
    assert_eq!(s, "25.5 mm");
}

/// −25.45 mm rounds to "−25.5 mm" (half away from zero, i.e. towards −∞ for
/// negative values).
#[test]
fn t_fr_038_09_round_half_away_from_zero_negative() {
    let svc = UnitService::new();
    let l = Length::from_mm(-25.45).expect("finite");
    let opts = FormatOptions {
        unit: Unit::Mm,
        decimal_places: 1,
        inch_mode: InchMode::Decimal,
        inch_denominator: 16,
    };
    let s = svc.format(l, &opts);
    assert_eq!(s, "-25.5 mm");
}

/// 1 inch formatted as a fraction with denominator 16 is "1 in" (whole number,
/// no fraction part needed).
#[test]
fn t_fr_038_09_inch_fraction_whole() {
    let svc = UnitService::new();
    let l = Length::from_unit(1.0, Unit::In).expect("finite");
    let opts = FormatOptions {
        unit: Unit::In,
        decimal_places: 2,
        inch_mode: InchMode::Fraction { denominator: 16 },
        inch_denominator: 16,
    };
    let s = svc.format(l, &opts);
    assert_eq!(s, "1 in");
}

/// 1.5 inches formatted as a fraction with denominator 16 → "1 8/16 in".
/// (Simplified fractions are a display nicety but not required; the test uses
/// a non-simplified denominator to confirm correctness.)
#[test]
fn t_fr_038_09_inch_fraction_half() {
    let svc = UnitService::new();
    let l = Length::from_unit(1.5, Unit::In).expect("finite");
    let opts = FormatOptions {
        unit: Unit::In,
        decimal_places: 2,
        inch_mode: InchMode::Fraction { denominator: 16 },
        inch_denominator: 16,
    };
    let s = svc.format(l, &opts);
    // 0.5 in = 8/16 in
    assert!(s == "1 8/16 in" || s == "1 1/2 in", "unexpected: {s}");
}

// ── T-FR-038-05 / AT-20: same length in five units stores one value ──────────
// (integration-level but exercised here as a unit test for the library)

/// Parsing "940 mm", "94 cm", "0.94 m", and checking they all store the same nm.
#[test]
fn t_fr_038_05_same_length_different_units() {
    let svc = UnitService::new();
    let mm = svc.parse("940 mm", Unit::Mm).expect("mm");
    let cm = svc.parse("94 cm", Unit::Mm).expect("cm");
    let m = svc.parse("0.94 m", Unit::Mm).expect("m");
    assert_eq!(mm.nanometres(), cm.nanometres());
    assert_eq!(mm.nanometres(), m.nanometres());
}
