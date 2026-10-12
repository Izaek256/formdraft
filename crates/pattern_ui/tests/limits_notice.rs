//! T-SK01-07: with thresholds unset the app warns and invents no limits.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_core::measure::{validate_draft, Measurement, ProfileDraft, SourceTag, ValidationCode};
use pattern_core::units::Length;
use pattern_ui::placeholder::demo_profile;

fn length(mm: f64) -> Length {
    Length::from_mm(mm).unwrap()
}

fn complete_draft() -> ProfileDraft {
    ProfileDraft {
        waist: Some(Measurement::new(length(700.0), SourceTag::Measured)),
        hips: Some(Measurement::new(length(940.0), SourceTag::Measured)),
        waist_to_hip: Some(Measurement::new(length(200.0), SourceTag::Measured)),
        skirt_length: Some(Measurement::new(length(600.0), SourceTag::Measured)),
        bust: None,
    }
}

#[test]
fn t_sk01_07_unset_limits_warn_and_invent_nothing() {
    let limits = pattern_templates::thresholds::load_thresholds().unwrap();
    assert!(!limits.all_set(), "test assumes thresholds are still unset");

    let metadata = demo_profile::metadata().unwrap();
    let result = validate_draft(complete_draft(), &limits, metadata);
    let (_profile, report) = result.unwrap();

    assert!(!report.limits_complete);
    assert!(
        report.errors.is_empty(),
        "unexpected errors: {:?}",
        report.errors
    );
    assert!(!report.warnings.is_empty(), "no limits notice was raised");
    assert!(report
        .warnings
        .iter()
        .all(|warning| warning.code == ValidationCode::LimitsNotConfigured));
}
