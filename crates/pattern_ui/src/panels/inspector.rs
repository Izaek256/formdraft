//! The right inspector: tabbed piece metadata, measurements, style and
//! instructions.
//!
//! The Properties tab edits the piece name (wired to the canvas label) and
//! shows read-only method facts plus live dimensions from real selection or
//! content bounds. The Measurements tab hosts the body-measurement form. Style
//! and Instructions are honest placeholders: style options arrive with the
//! S14 slices, while instructions carry the real toile-verification guidance.

use egui::{RichText, Ui};

use crate::app::METHOD_NOTICE;
use crate::panels::properties::{draw as draw_form, ParsedForm, PropertiesForm};
use crate::placeholder;
use crate::theme::Tokens;

/// An inspector tab from the reference layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InspectorTab {
    /// Piece name, method facts and dimensions.
    #[default]
    Properties,
    /// The body-measurement form.
    Measurements,
    /// Style options (a later release).
    Style,
    /// Toile-verification guidance.
    Instructions,
}

impl InspectorTab {
    /// All tabs, in order.
    pub const ALL: [InspectorTab; 4] = [
        InspectorTab::Properties,
        InspectorTab::Measurements,
        InspectorTab::Style,
        InspectorTab::Instructions,
    ];

    /// A human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            InspectorTab::Properties => "Properties",
            InspectorTab::Measurements => "Measurements",
            InspectorTab::Style => "Style",
            InspectorTab::Instructions => "Instructions",
        }
    }
}

/// Draw the tab strip. Returns the newly chosen tab, if it changed.
pub fn draw_tabs(ui: &mut Ui, current: &mut InspectorTab, tokens: &Tokens) -> bool {
    let mut changed = false;
    ui.add_space(tokens.sizes.spacing_xs);
    ui.horizontal(|ui| {
        for tab in InspectorTab::ALL {
            let selected = *current == tab;
            if ui
                .selectable_label(selected, tab.label())
                .on_hover_text(if tab == InspectorTab::Style {
                    "Style options arrive with the S14 slices"
                } else {
                    tab.label()
                })
                .clicked()
            {
                *current = tab;
                changed = true;
            }
        }
    });
    ui.separator();
    changed
}

/// Read-only fact plus live dimensions for the Properties tab.
#[derive(Debug, Clone)]
pub struct PieceFacts {
    /// The piece kind. The placeholder is not a pattern piece.
    pub kind: String,
    /// Origin text, in the display unit.
    pub origin: String,
    /// Size text, in the display unit.
    pub size: String,
    /// Whether a selection supplies the dimensions.
    pub from_selection: bool,
}

/// Draw the Properties tab: rename field, method facts and dimensions.
pub fn draw_properties(
    ui: &mut Ui,
    piece_name: &mut String,
    facts: &PieceFacts,
    tokens: &Tokens,
) -> bool {
    let mut changed = false;
    section(ui, "Pattern Piece", tokens);
    ui.horizontal(|ui| {
        ui.label(RichText::new("Name").size(tokens.sizes.font_small));
        if ui
            .text_edit_singleline(piece_name)
            .on_hover_text("Rename the piece on the canvas")
            .changed()
        {
            changed = true;
        }
    });
    fact(ui, "Kind", &facts.kind, tokens);
    fact(ui, "Method", METHOD_NOTICE, tokens);
    section(ui, "Dimensions", tokens);
    ui.label(
        RichText::new(if facts.from_selection {
            "Selection"
        } else {
            "Piece"
        })
        .size(tokens.sizes.font_small)
        .color(tokens.colors.text_muted),
    );
    fact(ui, "Origin", &facts.origin, tokens);
    fact(ui, "Size", &facts.size, tokens);
    changed
}

/// Draw the Measurements tab: the body-measurement form.
pub fn draw_measurements(
    ui: &mut Ui,
    form: &mut PropertiesForm,
    parsed: &ParsedForm,
    tokens: &Tokens,
) -> bool {
    draw_form(ui, form, parsed, tokens)
}

/// Draw the Style tab: an honest empty state, never fake controls.
pub fn draw_style(ui: &mut Ui, tokens: &Tokens) {
    ui.label(
        RichText::new("Style options are not part of this release.").size(tokens.sizes.font_small),
    );
    ui.label(
        RichText::new(
            "Adaptation presets, ease, dart style and seam allowance arrive \
             with the S14 slices. Nothing here affects the shape.",
        )
        .size(tokens.sizes.font_small)
        .color(tokens.colors.text_muted),
    );
}

/// Draw the Instructions tab: the real toile-verification guidance.
pub fn draw_instructions(ui: &mut Ui, tokens: &Tokens) {
    ui.label(
        RichText::new(placeholder::NOTICE)
            .size(tokens.sizes.font_small)
            .color(tokens.colors.warning),
    );
    ui.label(
        RichText::new(
            "This build draws a block for toile verification. Confirm the fit \
             on a calico toile before cutting final fabric.",
        )
        .size(tokens.sizes.font_small),
    );
    ui.separator();
    ui.label(
        RichText::new(METHOD_NOTICE)
            .size(tokens.sizes.font_small)
            .color(tokens.colors.text_muted),
    );
}

/// A section heading.
fn section(ui: &mut Ui, title: &str, tokens: &Tokens) {
    ui.add_space(tokens.sizes.spacing_xs);
    ui.label(RichText::new(title).size(tokens.sizes.font_body).strong());
}

/// One read-only fact row.
fn fact(ui: &mut Ui, name: &str, value: &str, tokens: &Tokens) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(name)
                .size(tokens.sizes.font_small)
                .color(tokens.colors.text_muted),
        );
        ui.label(RichText::new(value).size(tokens.sizes.font_small));
    });
}
