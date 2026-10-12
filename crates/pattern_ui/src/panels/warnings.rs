//! The warnings panel: S03 validation errors and warnings, with rule IDs.

use egui::Ui;
use pattern_core::measure::ValidationReport;

use crate::theme::Tokens;

/// Draw the validation report. Shows the limits-not-configured notice and
/// never invents a value (T-SK01-07).
pub fn draw(ui: &mut Ui, report: &ValidationReport, tokens: &Tokens) {
    if !report.limits_complete {
        ui.label(
            egui::RichText::new("Limits are not configured: no range checks are applied.")
                .size(tokens.sizes.font_small)
                .color(tokens.colors.warning),
        );
    }

    if report.errors.is_empty() && report.warnings.is_empty() {
        ui.label("No problems found.");
        return;
    }

    for error in &report.errors {
        ui.label(
            egui::RichText::new(format!("[{}] {}", error.code, error.message))
                .size(tokens.sizes.font_small)
                .color(tokens.colors.error),
        );
    }
    for warning in &report.warnings {
        ui.label(
            egui::RichText::new(format!("[{}] {}", warning.code, warning.message))
                .size(tokens.sizes.font_small)
                .color(tokens.colors.warning),
        );
    }
}
