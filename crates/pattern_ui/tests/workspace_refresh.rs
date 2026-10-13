//! T-SK01-20: the real input path. Field text goes through parsing, S03
//! validation and the placeholder draft into canvas content, the fit request
//! and the warnings report - the same `Workspace::refresh` the frame loop
//! calls, with no UI harness and no prefilled data.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::app::Workspace;
use pattern_ui::panels::properties::FieldId;

#[test]
fn t_sk01_20_empty_form_draws_nothing() {
    let mut workspace = Workspace::default();
    assert!(workspace.form.waist.is_empty(), "form must start empty");
    workspace.refresh();
    assert!(
        workspace.canvas.content.is_none(),
        "empty form drew a shape"
    );
    assert!(!workspace.canvas.fit_pending, "empty form requested a fit");
}

#[test]
fn t_sk01_20_field_text_reaches_the_canvas_and_requests_a_fit() {
    let mut workspace = Workspace::default();
    workspace.form.set_text(FieldId::Waist, String::from("70"));
    workspace.form.set_text(FieldId::Hips, String::from("94"));
    workspace
        .form
        .set_text(FieldId::WaistToHip, String::from("20"));
    workspace
        .form
        .set_text(FieldId::SkirtLength, String::from("60"));

    workspace.refresh();

    let content = workspace.canvas.content.as_ref().expect("drawable shape");
    assert_eq!(content.bounds.min_mm, [0.0, 0.0]);
    assert_eq!(content.bounds.size_mm(), [940.0, 600.0]);
    assert!(content.visible);
    assert!(
        workspace.canvas.fit_pending,
        "first shape did not request a fit"
    );
    assert!(
        workspace.report.errors.is_empty(),
        "unexpected errors: {:?}",
        workspace.report.errors
    );
}

#[test]
fn t_sk01_20_clearing_a_field_removes_the_shape() {
    let mut workspace = Workspace::default();
    workspace.form.set_text(FieldId::Waist, String::from("70"));
    workspace.form.set_text(FieldId::Hips, String::from("94"));
    workspace
        .form
        .set_text(FieldId::WaistToHip, String::from("20"));
    workspace
        .form
        .set_text(FieldId::SkirtLength, String::from("60"));
    workspace.refresh();
    workspace.canvas.select_at([100.0, 100.0]);
    assert!(workspace.canvas.selection.is_some());

    workspace.form.set_text(FieldId::Hips, String::new());
    workspace.refresh();

    assert!(
        workspace.canvas.content.is_none(),
        "cleared form still draws"
    );
    assert!(
        workspace.canvas.selection.is_none(),
        "selection survived clear"
    );
}
