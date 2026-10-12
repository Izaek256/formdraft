//! The pieces panel: one entry, the placeholder rectangle, with a visibility
//! toggle and its size in the pattern unit and in millimetres.

use egui::Ui;

use crate::placeholder;
use crate::theme::Tokens;

/// Draw the pieces panel. Returns whether the visibility toggle changed.
pub fn draw(ui: &mut Ui, visible: &mut bool, size_text: &str, tokens: &Tokens) -> bool {
    let mut changed = ui.checkbox(visible, placeholder::PIECE_LABEL).changed();
    ui.label(egui::RichText::new(size_text).size(tokens.sizes.font_small));
    ui.label(
        egui::RichText::new(placeholder::NOTICE)
            .size(tokens.sizes.font_small)
            .color(tokens.colors.warning),
    );
    let _ = &mut changed;
    changed
}
