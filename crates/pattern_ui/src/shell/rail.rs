//! The left navigation rail: workspace sections in the reference layout.
//!
//! Only Drafting exists in R1. The other sections are shown so the workspace
//! matches the product structure, but choosing one keeps the user in Drafting
//! and says so through a toast (via [`RailEvent::Later`]) instead of opening
//! a fake workspace.

use egui::{Ui, Vec2};

use crate::theme::Tokens;

/// A workspace section from the reference layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NavSection {
    /// Product start (later release).
    Home,
    /// The pattern workspace (this release).
    #[default]
    Drafting,
    /// Measurement library (later release).
    Measurements,
    /// Pattern catalogue (later release).
    Patterns,
    /// Size grading (later release).
    Grading,
    /// Fabric preview (later release).
    MaterialPreview,
    /// File exports (later release).
    Exports,
    /// Project settings (later release).
    Project,
}

impl NavSection {
    /// All sections, top to bottom.
    pub const ALL: [NavSection; 8] = [
        NavSection::Home,
        NavSection::Drafting,
        NavSection::Measurements,
        NavSection::Patterns,
        NavSection::Grading,
        NavSection::MaterialPreview,
        NavSection::Exports,
        NavSection::Project,
    ];

    /// A human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            NavSection::Home => "Home",
            NavSection::Drafting => "Drafting",
            NavSection::Measurements => "Measurements",
            NavSection::Patterns => "Patterns",
            NavSection::Grading => "Grading",
            NavSection::MaterialPreview => "Material Preview",
            NavSection::Exports => "Exports",
            NavSection::Project => "Project",
        }
    }

    /// Whether the section is part of this release.
    pub fn is_available(self) -> bool {
        self == NavSection::Drafting
    }
}

/// Something the rail asked the application to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RailEvent {
    /// A later-release section was chosen; the string names it.
    Later(String),
}

/// Draw the rail. The current section only changes when an available section
/// is chosen; anything else reports [`RailEvent::Later`].
pub fn draw(ui: &mut Ui, current: &mut NavSection, tokens: &Tokens) -> Vec<RailEvent> {
    let mut events = Vec::new();
    ui.add_space(tokens.sizes.spacing_xs);
    for section in NavSection::ALL {
        let selected = *current == section;
        let label = egui::RichText::new(section.label())
            .size(tokens.sizes.font_small)
            .color(if selected {
                tokens.colors.text
            } else {
                tokens.colors.text_muted
            });
        let button = egui::Button::new(label)
            .selected(selected)
            .min_size(Vec2::new(ui.available_width(), tokens.sizes.control_height));
        let response = ui.add(button).on_hover_text(if section.is_available() {
            section.label().to_owned()
        } else {
            format!("{} (a later release)", section.label())
        });
        if response.clicked() {
            if section.is_available() {
                *current = section;
            } else {
                events.push(RailEvent::Later(section.label().to_owned()));
            }
        }
    }
    events
}
