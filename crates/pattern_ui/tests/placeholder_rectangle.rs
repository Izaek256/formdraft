//! T-SK01-01, T-SK01-02, T-SK01-03: the placeholder rectangle comes only from
//! hips and skirt length, is unit independent, and disappears when a field is
//! missing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_core::units::{format_length, FormatOptions, Length, Unit};
use pattern_ui::panels::properties::{FieldId, PropertiesForm};
use pattern_ui::placeholder;

fn form(
    body_unit: Unit,
    waist: &str,
    hips: &str,
    waist_to_hip: &str,
    skirt: &str,
) -> PropertiesForm {
    let mut form = PropertiesForm {
        body_unit,
        ..PropertiesForm::default()
    };
    form.set_text(FieldId::Waist, waist.to_owned());
    form.set_text(FieldId::Hips, hips.to_owned());
    form.set_text(FieldId::WaistToHip, waist_to_hip.to_owned());
    form.set_text(FieldId::SkirtLength, skirt.to_owned());
    form
}

#[test]
fn t_sk01_01_rectangle_is_hips_by_skirt_length_across_units() {
    let a = form(Unit::Cm, "70", "94", "20", "60").parse();
    let b = form(Unit::Mm, "700", "940", "200", "600").parse();
    let c = form(Unit::Cm, "70", "940 mm", "20", "600 mm").parse();

    let ra = placeholder::from_draft(&a.draft).expect("a complete");
    let rb = placeholder::from_draft(&b.draft).expect("b complete");
    let rc = placeholder::from_draft(&c.draft).expect("c complete");

    assert_eq!(ra, rb);
    assert_eq!(ra, rc);
    assert_eq!(ra.min_mm, [0.0, 0.0]);
    assert_eq!(ra.size_mm(), [940.0, 600.0]);
}

#[test]
fn t_sk01_02_pattern_unit_changes_strings_not_the_rectangle() {
    let mut form = form(Unit::Cm, "70", "94", "20", "60");
    let before = placeholder::from_draft(&form.parse().draft).expect("complete");

    let hips = Length::from_mm(940.0).expect("940 mm");
    let mut strings = std::collections::BTreeSet::new();
    for unit in pattern_ui::panels::properties::UNITS {
        form.pattern_unit = unit;
        let after = placeholder::from_draft(&form.parse().draft).expect("complete");
        assert_eq!(before, after, "pattern unit {unit:?} changed the rectangle");
        strings.insert(format_length(hips, &FormatOptions::default_for(unit)));
    }
    assert!(
        strings.len() > 1,
        "switching units produced no different strings: {strings:?}"
    );
}

#[test]
fn t_sk01_03_clearing_each_field_disables_drawing_and_names_it() {
    let cases = [
        (FieldId::Waist, "Waist"),
        (FieldId::Hips, "Hips"),
        (FieldId::WaistToHip, "Waist to hip"),
        (FieldId::SkirtLength, "Skirt length"),
    ];
    for (field, label) in cases {
        let mut form = form(Unit::Cm, "70", "94", "20", "60");
        form.set_text(field, String::new());
        let parsed = form.parse();
        assert!(
            !parsed.is_drawable(),
            "{field:?} cleared but still drawable"
        );
        let message = parsed.error_for(field).expect("field error");
        assert!(
            message.starts_with(label),
            "message {message:?} does not name {label:?}"
        );
    }
}
