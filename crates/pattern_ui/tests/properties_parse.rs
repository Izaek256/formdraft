//! T-SK01-04, T-SK01-05, T-SK01-09: parsing rules and robustness.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_possible_truncation
)]

use pattern_core::units::{parse_length, Unit};
use pattern_ui::panels::properties::{FieldId, PropertiesForm};

#[test]
fn t_sk01_04_suffix_overrides_and_a_bare_number_uses_the_default() {
    let mut form = PropertiesForm {
        body_unit: Unit::Cm,
        ..PropertiesForm::default()
    };

    form.set_text(FieldId::Hips, String::from("94"));
    let bare = form.parse();
    assert_eq!(bare.draft.hips.unwrap().length().to_mm(), 940.0);

    form.set_text(FieldId::Hips, String::from("940 mm"));
    let suffixed = form.parse();
    assert_eq!(suffixed.draft.hips.unwrap().length().to_mm(), 940.0);

    form.set_text(FieldId::Hips, String::from("37 in"));
    let inches = form.parse();
    assert!((inches.draft.hips.unwrap().length().to_mm() - 37.0 * 25.4).abs() < 1e-6);
}

#[test]
fn t_sk01_05_invalid_text_reports_only_that_field() {
    let mut form = PropertiesForm::default();
    form.set_text(FieldId::Waist, String::from("70"));
    form.set_text(FieldId::Hips, String::from("banana"));
    form.set_text(FieldId::WaistToHip, String::from("20"));
    form.set_text(FieldId::SkirtLength, String::from("60"));

    let parsed = form.parse();
    assert_eq!(parsed.errors.len(), 1);
    let error = &parsed.errors[0];
    assert_eq!(error.field, FieldId::Hips);

    let unit_message = parse_length("banana", Unit::Cm).unwrap_err().to_string();
    assert!(
        error.message.ends_with(&unit_message),
        "message {:?} should carry the unit service message {:?}",
        error.message,
        unit_message
    );
    assert!(parsed.error_for(FieldId::Waist).is_none());
}

#[test]
fn t_sk01_09_arbitrary_text_never_panics() {
    let alphabet: Vec<char> = "0123456789 .,-+/eEabcdfghiklmntwy\u{2212}\u{00b0}"
        .chars()
        .collect();

    let mut state: u64 = 0x1234_5678_9abc_def0;
    for _ in 0..5_000 {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let length = (state >> 33) as usize % 12;
        let mut text = String::new();
        let mut cursor = state;
        for _ in 0..length {
            cursor = cursor
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            text.push(alphabet[(cursor >> 33) as usize % alphabet.len()]);
        }

        let _ = parse_length(&text, Unit::Cm);

        let mut form = PropertiesForm::default();
        form.set_text(FieldId::Hips, text.clone());
        form.set_text(FieldId::Waist, text.clone());
        form.set_text(FieldId::WaistToHip, text.clone());
        form.set_text(FieldId::SkirtLength, text);
        let _ = form.parse();
    }
}
