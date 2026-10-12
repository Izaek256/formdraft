//! T-SK01-13: ruler ticks are multiples of a 1-2-5 step in every unit, and
//! labels come from the unit service.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_core::units::Unit;
use pattern_ui::canvas::rulers;

#[test]
fn t_sk01_13_step_is_one_two_or_five_times_a_power_of_ten() {
    let units = [Unit::Mm, Unit::Cm, Unit::M, Unit::In, Unit::Ft, Unit::Yd];
    let zooms = [0.1, 0.5, 1.0, 2.5, 8.0, 40.0];

    for unit in units {
        for zoom in zooms {
            let step_mm = rulers::step_mm(unit, zoom, 56.0);
            let step_display = step_mm / unit.mm_per_unit();
            let exponent = step_display.log10().floor();
            let mantissa = step_display / 10f64.powf(exponent);
            assert!(
                [1.0, 2.0, 5.0].iter().any(|m| (mantissa - m).abs() < 1e-6),
                "{unit:?} at {zoom} px/mm gave mantissa {mantissa}"
            );
            assert!(
                step_mm * zoom >= 56.0 * 0.999,
                "{unit:?} at {zoom} px/mm step too tight"
            );
        }
    }
}

#[test]
fn t_sk01_13_ticks_are_multiples_and_labels_use_the_unit_service() {
    let units = [Unit::Mm, Unit::Cm, Unit::M, Unit::In, Unit::Ft, Unit::Yd];
    for unit in units {
        let step_mm = rulers::step_mm(unit, 2.0, 56.0);
        let ticks = rulers::ticks_mm(step_mm, -50.0, 250.0);
        assert!(!ticks.is_empty(), "{unit:?} produced no ticks");
        for tick in ticks {
            let multiple = tick / step_mm;
            assert!(
                (multiple - multiple.round()).abs() < 1e-6,
                "{unit:?}: tick {tick} is not a multiple of {step_mm}"
            );
            let label = rulers::tick_label(unit, tick);
            assert!(
                label.ends_with(unit.suffix()),
                "{unit:?}: label {label:?} does not end with the unit suffix"
            );
        }
    }
}

#[test]
fn t_sk01_13_minor_step_is_a_fifth_of_the_major() {
    assert!((rulers::minor_step_mm(50.0) - 10.0).abs() < 1e-9);
}
