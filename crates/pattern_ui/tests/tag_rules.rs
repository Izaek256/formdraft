//! T-SK01-06: source tag rules are shown and enforced.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_core::measure::{validate_draft, Measurement, ProfileDraft, ProfileError, SourceTag};
use pattern_core::units::Length;
use pattern_ui::panels::properties::{FieldId, PropertiesForm};
use pattern_ui::placeholder::demo_profile;

fn length(mm: f64) -> Length {
    Length::from_mm(mm).unwrap()
}

fn draft(waist_tag: SourceTag, waist_to_hip_tag: SourceTag) -> ProfileDraft {
    ProfileDraft {
        waist: Some(Measurement::new(length(700.0), waist_tag)),
        hips: Some(Measurement::new(length(940.0), SourceTag::Measured)),
        waist_to_hip: Some(Measurement::new(length(200.0), waist_to_hip_tag)),
        skirt_length: Some(Measurement::new(length(600.0), SourceTag::Measured)),
        bust: None,
    }
}

#[test]
fn t_sk01_06_form_offers_the_right_tags_per_field() {
    let form = PropertiesForm::default();
    assert_eq!(form.allowed_tags(FieldId::Waist), &[SourceTag::Measured]);
    assert!(form
        .allowed_tags(FieldId::WaistToHip)
        .contains(&SourceTag::ChartDerived));
}

#[test]
fn t_sk01_06_chart_derived_rejected_on_waist() {
    let limits = pattern_templates::thresholds::load_thresholds().unwrap();
    let metadata = demo_profile::metadata().unwrap();
    let error = validate_draft(
        draft(SourceTag::ChartDerived, SourceTag::Measured),
        &limits,
        metadata,
    )
    .unwrap_err();
    assert_eq!(
        error.1,
        ProfileError::InvalidSourceTag {
            field: "waist",
            tag: String::from("chart-derived"),
        }
    );
}

#[test]
fn t_sk01_06_chart_derived_accepted_on_waist_to_hip() {
    let limits = pattern_templates::thresholds::load_thresholds().unwrap();
    let metadata = demo_profile::metadata().unwrap();
    let result = validate_draft(
        draft(SourceTag::Measured, SourceTag::ChartDerived),
        &limits,
        metadata,
    );
    assert!(result.is_ok());
}
