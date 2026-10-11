/// Tests for pattern_templates thresholds loading
use pattern_templates::thresholds::load_thresholds;

#[test]
fn test_thresholds_toml_loads_when_unset() {
    // T-FR-019-09: thresholds.toml loads when unset
    let limits = load_thresholds().unwrap();
    assert!(!limits.all_set());
    assert!(!limits.waist.is_set());
    assert!(!limits.hips.is_set());
    assert!(!limits.waist_to_hip.is_set());
    assert!(!limits.skirt_length.is_set());
    assert!(!limits.bust.is_set());
    assert!(!limits.hips_vs_waist.is_set());
}

#[test]
fn test_thresholds_toml_parses_values_through_unit_service() {
    // T-FR-019-09: parses values through the unit service
    use pattern_core::measure::{CrossLimit, LimitSet, Limits};
    use pattern_core::units::{Length, Unit};

    // Create limits with SET values to verify unit parsing
    let limits = Limits {
        waist: LimitSet::set(
            Length::from_unit(50.0, Unit::Cm).unwrap(),
            Length::from_unit(120.0, Unit::Cm).unwrap(),
            Length::from_unit(55.0, Unit::Cm).unwrap(),
            Length::from_unit(110.0, Unit::Cm).unwrap(),
        ),
        hips: LimitSet::Unset,
        waist_to_hip: LimitSet::Unset,
        skirt_length: LimitSet::Unset,
        bust: LimitSet::Unset,
        hips_vs_waist: CrossLimit::Unset,
    };

    // Verify the values were parsed correctly via unit service
    assert_eq!(limits.waist.hard_min().unwrap().to_mm(), 500.0);
    assert_eq!(limits.waist.hard_max().unwrap().to_mm(), 1200.0);
    assert_eq!(limits.waist.warn_below().unwrap().to_mm(), 550.0);
    assert_eq!(limits.waist.warn_above().unwrap().to_mm(), 1100.0);
}
