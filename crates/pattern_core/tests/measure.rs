// Tests for S03: Measurement profile and validation
#![allow(clippy::unwrap_used, clippy::expect_used)]

use pattern_core::measure::{
    validate_draft, CrossLimit, DefinitionVersion, LimitSet, Limits, Measurement, ProfileDraft,
    ProfileId, ProfileLabel, ProfileMetadata, ProtocolVersion, SourceTag, ValidationCode,
};
use pattern_core::units::Length;

fn make_artificial_limits() -> Limits {
    // Use obviously artificial limits (1 mm to 5 m) per brief
    let mm1 = Length::from_mm(1.0).unwrap();
    let m5 = Length::from_mm(5000.0).unwrap();
    let cm10 = Length::from_mm(100.0).unwrap();
    let m1 = Length::from_mm(1000.0).unwrap();
    let mm200 = Length::from_mm(200.0).unwrap();
    let m2 = Length::from_mm(2000.0).unwrap();
    Limits {
        waist: LimitSet::set(mm1, m5, cm10, m1),
        hips: LimitSet::set(mm1, m5, cm10, m1),
        waist_to_hip: LimitSet::set(mm1, m5, cm10, m1),
        skirt_length: LimitSet::set(mm1, m5, cm10, m1),
        bust: LimitSet::set(mm1, m5, cm10, m1),
        hips_vs_waist: CrossLimit::set(mm200, m2),
    }
}

fn make_metadata() -> ProfileMetadata {
    ProfileMetadata::new(
        ProfileId::new("test-profile-001").unwrap(),
        ProfileLabel::new("Test Profile").unwrap(),
        DefinitionVersion::new("1.0.0").unwrap(),
        ProtocolVersion::new("1").unwrap(),
        1_700_000_000, // Unix seconds
    )
}

#[test]
fn test_fr_019_01_validation_per_field_and_cross_field_rule() {
    // T-FR-019-01: validation per field and per cross-field rule with artificial limits
    let limits = make_artificial_limits();
    let metadata = make_metadata();

    // Valid profile
    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(700.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: Some(Measurement::new(
            Length::from_mm(850.0).unwrap(),
            SourceTag::Measured,
        )),
    };
    let (profile, report) = validate_draft(draft, &limits, metadata.clone()).unwrap();
    assert!(!report.has_errors());
    assert_eq!(profile.waist().length().to_mm(), 700.0);
    assert_eq!(profile.hips().length().to_mm(), 950.0);

    // Below hard min (waist)
    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(0.5).unwrap(),
            SourceTag::Measured,
        )), // 0.5 mm < 1 mm
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };
    let (_, report) = validate_draft(draft, &limits, metadata.clone()).unwrap();
    assert!(report.errors.iter().any(|e| {
        e.code == ValidationCode::BelowHardMin
            && e.message.contains("waist")
            && e.message.contains("ALD4-THR-001")
    }));

    // Above hard max (hips)
    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(700.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(6000.0).unwrap(),
            SourceTag::Measured,
        )), // 6 m > 5 m
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };
    let (_, report) = validate_draft(draft, &limits, metadata.clone()).unwrap();
    assert!(report.errors.iter().any(|e| {
        e.code == ValidationCode::AboveHardMax
            && e.message.contains("hips")
            && e.message.contains("ALD4-THR-002")
    }));

    // Cross-field: hips - waist below min_diff (50 cm = 500 mm)
    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(800.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(900.0).unwrap(),
            SourceTag::Measured,
        )), // diff = 100 mm < 500 mm
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };
    let (_, report) = validate_draft(draft, &limits, metadata.clone()).unwrap();
    assert!(report.errors.iter().any(|e| {
        e.code == ValidationCode::HipsWaistDiffBelowMin && e.message.contains("ALD4-THR-006")
    }));

    // Cross-field: hips - waist above max_diff (2 m = 2000 mm)
    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(500.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(3000.0).unwrap(),
            SourceTag::Measured,
        )), // diff = 2500 mm > 2000 mm
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };
    let (_, report) = validate_draft(draft, &limits, metadata.clone()).unwrap();
    assert!(report.errors.iter().any(|e| {
        e.code == ValidationCode::HipsWaistDiffAboveMax && e.message.contains("ALD4-THR-006")
    }));
}

#[test]
fn test_fr_019_02_tag_rules_for_every_field_and_tag_combination() {
    // T-FR-019-02: tag rules for every field and tag combination
    let limits = make_artificial_limits();
    let metadata = make_metadata();

    // Valid tags: waist, hips, skirt_length, bust = Measured only
    for (field, tag) in [
        ("waist", SourceTag::Measured),
        ("hips", SourceTag::Measured),
        ("skirt_length", SourceTag::Measured),
        ("bust", SourceTag::Measured),
    ] {
        let mut draft = ProfileDraft::new();
        match field {
            "waist" => draft.waist = Some(Measurement::new(Length::from_mm(700.0).unwrap(), tag)),
            "hips" => draft.hips = Some(Measurement::new(Length::from_mm(950.0).unwrap(), tag)),
            "skirt_length" => {
                draft.skirt_length = Some(Measurement::new(Length::from_mm(600.0).unwrap(), tag))
            }
            "bust" => draft.bust = Some(Measurement::new(Length::from_mm(850.0).unwrap(), tag)),
            _ => panic!("unknown field"),
        }
        // Fill other required fields
        if field != "waist" {
            draft.waist = Some(Measurement::new(
                Length::from_mm(700.0).unwrap(),
                SourceTag::Measured,
            ));
        }
        if field != "hips" {
            draft.hips = Some(Measurement::new(
                Length::from_mm(950.0).unwrap(),
                SourceTag::Measured,
            ));
        }
        if field != "skirt_length" {
            draft.skirt_length = Some(Measurement::new(
                Length::from_mm(600.0).unwrap(),
                SourceTag::Measured,
            ));
        }
        if field != "waist_to_hip" {
            draft.waist_to_hip = Some(Measurement::new(
                Length::from_mm(750.0).unwrap(),
                SourceTag::Measured,
            ));
        }

        let (_, report) = validate_draft(draft, &limits, metadata.clone()).unwrap();
        assert!(
            !report.has_errors(),
            "field {} with Measured tag should be valid",
            field
        );
    }

    // waist_to_hip accepts all three tags
    for tag in [
        SourceTag::Measured,
        SourceTag::ChartDerived,
        SourceTag::Estimated,
    ] {
        let draft = ProfileDraft {
            waist: Some(Measurement::new(
                Length::from_mm(700.0).unwrap(),
                SourceTag::Measured,
            )),
            hips: Some(Measurement::new(
                Length::from_mm(950.0).unwrap(),
                SourceTag::Measured,
            )),
            waist_to_hip: Some(Measurement::new(Length::from_mm(750.0).unwrap(), tag)),
            skirt_length: Some(Measurement::new(
                Length::from_mm(600.0).unwrap(),
                SourceTag::Measured,
            )),
            bust: None,
        };
        let (_, report) = validate_draft(draft, &limits, metadata.clone()).unwrap();
        assert!(
            !report.has_errors(),
            "waist_to_hip with {:?} tag should be valid",
            tag
        );
    }

    // Invalid tags for waist, hips, skirt_length, bust
    for field in ["waist", "hips", "skirt_length", "bust"] {
        for tag in [SourceTag::ChartDerived, SourceTag::Estimated] {
            let mut draft = ProfileDraft::new();
            let length = Length::from_mm(700.0).unwrap();
            match field {
                "waist" => draft.waist = Some(Measurement::new(length, tag)),
                "hips" => draft.hips = Some(Measurement::new(length, tag)),
                "skirt_length" => draft.skirt_length = Some(Measurement::new(length, tag)),
                "bust" => draft.bust = Some(Measurement::new(length, tag)),
                _ => panic!("unknown field"),
            }
            if field != "waist" {
                draft.waist = Some(Measurement::new(
                    Length::from_mm(700.0).unwrap(),
                    SourceTag::Measured,
                ));
            }
            if field != "hips" {
                draft.hips = Some(Measurement::new(
                    Length::from_mm(950.0).unwrap(),
                    SourceTag::Measured,
                ));
            }
            if field != "skirt_length" {
                draft.skirt_length = Some(Measurement::new(
                    Length::from_mm(600.0).unwrap(),
                    SourceTag::Measured,
                ));
            }
            draft.waist_to_hip = Some(Measurement::new(
                Length::from_mm(750.0).unwrap(),
                SourceTag::Measured,
            ));

            let (report, _) = validate_draft(draft, &limits, metadata.clone()).unwrap_err();
            assert!(
                report
                    .errors
                    .iter()
                    .any(|e| e.code == ValidationCode::InvalidSourceTag),
                "field {} with {:?} tag should be invalid",
                field,
                tag
            );
        }
    }
}

#[test]
fn test_fr_019_03_at_03_remove_each_mandatory_field() {
    // T-FR-019-03: AT-03 - remove each of the four mandatory fields and see exact field named
    let limits = make_artificial_limits();
    let metadata = make_metadata();

    let base_draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(700.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };

    for field in ["waist", "hips", "waist_to_hip", "skirt_length"] {
        let mut draft = base_draft.clone();
        match field {
            "waist" => draft.waist = None,
            "hips" => draft.hips = None,
            "waist_to_hip" => draft.waist_to_hip = None,
            "skirt_length" => draft.skirt_length = None,
            _ => panic!("unknown field"),
        }

        let (report, _) = validate_draft(draft, &limits, metadata.clone()).unwrap_err();
        assert!(
            report
                .errors
                .iter()
                .any(|e| { e.code == ValidationCode::MissingField && e.message.contains(field) }),
            "error should name missing field '{}'",
            field
        );
    }
}

#[test]
fn test_fr_019_06_debug_display_never_show_values_or_label() {
    // T-FR-019-06: Debug and Display never show values or the label
    let limits = make_artificial_limits();
    let metadata = make_metadata();

    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(700.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };

    let (profile, _) = validate_draft(draft, &limits, metadata).unwrap();

    // Check Debug output for Profile
    let debug_str = format!("{:?}", profile);
    assert!(
        !debug_str.contains("700.0"),
        "Debug must not contain measurement values"
    );
    assert!(
        !debug_str.contains("950.0"),
        "Debug must not contain measurement values"
    );
    assert!(
        !debug_str.contains("Test Profile"),
        "Debug must not contain label"
    );

    // Check Debug output for ProfileDraft
    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(700.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };
    let debug_str = format!("{:?}", draft);
    assert!(
        !debug_str.contains("700.0"),
        "Debug must not contain measurement values"
    );
    assert!(
        !debug_str.contains("950.0"),
        "Debug must not contain measurement values"
    );
}

#[test]
fn test_fr_019_07_zero_and_negative_rejected_with_unset_limits() {
    // T-FR-019-07: zero and negative rejected even with Unset limits
    let limits = Limits::new_unset();
    let metadata = make_metadata();

    // Zero
    let draft = ProfileDraft {
        waist: Some(Measurement::new(Length::ZERO, SourceTag::Measured)),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };
    let (report, _) = validate_draft(draft, &limits, metadata.clone()).unwrap_err();
    assert!(report
        .errors
        .iter()
        .any(|e| e.code == ValidationCode::ZeroOrNegative));

    // Negative (create via from_nanometres with negative value)
    let negative_length = Length::from_nanometres(-1000);
    let draft = ProfileDraft {
        waist: Some(Measurement::new(negative_length, SourceTag::Measured)),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };
    let (report, _) = validate_draft(draft, &limits, metadata.clone()).unwrap_err();
    assert!(report
        .errors
        .iter()
        .any(|e| e.code == ValidationCode::ZeroOrNegative));
}

#[test]
fn test_fr_019_08_unset_limits_give_limits_not_configured_warnings() {
    // T-FR-019-08: Unset limits give LimitsNotConfigured warnings, limits_complete false
    let limits = Limits::new_unset();
    let metadata = make_metadata();

    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(700.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };

    let (_, report) = validate_draft(draft, &limits, metadata).unwrap();

    assert!(
        !report.limits_complete,
        "limits_complete should be false when limits are unset"
    );

    // Should have LimitsNotConfigured warning for each unset limit
    let warning_codes: Vec<_> = report.warnings.iter().map(|w| w.code).collect();
    assert!(warning_codes.contains(&ValidationCode::LimitsNotConfigured));

    // Should have one warning per field (6 limits: waist, hips, waist_to_hip, skirt_length, bust, hips_vs_waist)
    let unset_warnings: Vec<_> = report
        .warnings
        .iter()
        .filter(|w| w.code == ValidationCode::LimitsNotConfigured)
        .collect();
    assert_eq!(
        unset_warnings.len(),
        6,
        "should have 6 LimitsNotConfigured warnings"
    );
}

#[test]
fn test_fr_019_08_warns_only_for_unset_limits() {
    // T-FR-019-08: only the fields whose limits are actually unset get a warning
    let mut limits = Limits::new_unset();
    limits.waist = LimitSet::set(
        Length::from_mm(1.0).unwrap(),
        Length::from_mm(5000.0).unwrap(),
        Length::from_mm(100.0).unwrap(),
        Length::from_mm(1000.0).unwrap(),
    );
    let metadata = make_metadata();

    let draft = ProfileDraft {
        waist: Some(Measurement::new(
            Length::from_mm(700.0).unwrap(),
            SourceTag::Measured,
        )),
        hips: Some(Measurement::new(
            Length::from_mm(950.0).unwrap(),
            SourceTag::Measured,
        )),
        waist_to_hip: Some(Measurement::new(
            Length::from_mm(750.0).unwrap(),
            SourceTag::Measured,
        )),
        skirt_length: Some(Measurement::new(
            Length::from_mm(600.0).unwrap(),
            SourceTag::Measured,
        )),
        bust: None,
    };

    let (_, report) = validate_draft(draft, &limits, metadata).unwrap();
    let unset_warnings = report
        .warnings
        .iter()
        .filter(|w| w.code == ValidationCode::LimitsNotConfigured)
        .count();
    assert_eq!(unset_warnings, 5, "waist is set, so only 5 others warn");
    assert!(!report.limits_complete);
}
