//! Modal dialogs: about, method, project session and export readiness.
//!
//! Every dialog reports genuine state. Destructive or file-writing actions do
//! not exist in this release, so none is offered: the export dialog is a
//! readiness checklist (real blockers from validation) with the file-writing
//! button honestly disabled until slice S14e.

use egui::{RichText, Ui};

use crate::app::METHOD_NOTICE;
use crate::placeholder;
use crate::shell::titlebar::{APP_VERSION, PRODUCT_NAME};
use crate::theme::Tokens;

/// The export file format remembered by the readiness dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExportFormat {
    /// Print-ready PDF (default).
    #[default]
    Pdf,
    /// Vector SVG.
    Svg,
    /// CAD interchange DXF.
    Dxf,
}

impl ExportFormat {
    /// All formats, in dialog order.
    pub const ALL: [ExportFormat; 3] = [ExportFormat::Pdf, ExportFormat::Svg, ExportFormat::Dxf];

    /// A human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            ExportFormat::Pdf => "PDF (print-ready, tiled)",
            ExportFormat::Svg => "SVG (vector)",
            ExportFormat::Dxf => "DXF (CAD interchange)",
        }
    }
}

/// Which dialog is open, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Dialog {
    /// No dialog.
    #[default]
    Closed,
    /// Product and method information.
    About,
    /// Drafting method and edition.
    Method,
    /// Session facts (piece, validation, units).
    Project,
    /// Export readiness checklist.
    Export {
        /// The remembered file format.
        format: ExportFormat,
    },
}

impl Dialog {
    /// Whether a dialog is open.
    pub fn is_open(self) -> bool {
        self != Dialog::Closed
    }
}

/// Session facts shown by the project dialog. All values are real.
#[derive(Debug, Clone)]
pub struct SessionFacts {
    /// The piece label, or `None` when no shape draws.
    pub piece: Option<String>,
    /// The piece size in the pattern unit, or a why-not message.
    pub size: String,
    /// Validation error count.
    pub errors: usize,
    /// Validation warning count.
    pub warnings: usize,
    /// Whether limits are configured.
    pub limits_complete: bool,
}

/// Something a dialog asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogEvent {
    /// Close the dialog.
    Close,
    /// Remember another export format.
    SetFormat(ExportFormat),
}

/// Draw the open dialog, if any. Returns triggered events.
pub fn draw(
    ui: &mut Ui,
    dialog: &mut Dialog,
    facts: &SessionFacts,
    blockers: &[String],
    tokens: &Tokens,
) -> Vec<DialogEvent> {
    let mut events = Vec::new();
    if *dialog == Dialog::Closed {
        return events;
    }
    let title = match dialog {
        Dialog::Closed => return events,
        Dialog::About => "About FormDraft",
        Dialog::Method => "Method and edition",
        Dialog::Project => "Project session",
        Dialog::Export { .. } => "Export pattern",
    };
    let mut open = true;
    egui::Window::new(title)
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ui.ctx(), |ui| {
            ui.set_min_width(tokens.sizes.splash_width.min(ui.available_width()));
            match dialog {
                Dialog::Closed => {}
                Dialog::About => {
                    ui.label(
                        RichText::new(format!("{PRODUCT_NAME} {APP_VERSION}"))
                            .size(tokens.sizes.font_heading)
                            .strong(),
                    );
                    ui.label(RichText::new(METHOD_NOTICE).size(tokens.sizes.font_small));
                    ui.separator();
                    ui.label(
                        RichText::new(placeholder::NOTICE)
                            .size(tokens.sizes.font_small)
                            .color(tokens.colors.warning),
                    );
                    ui.label(
                        RichText::new(
                            "This build is a block for toile verification; fit must be \
                             confirmed on a calico toile before final fabric is cut.",
                        )
                        .size(tokens.sizes.font_small),
                    );
                }
                Dialog::Method => {
                    ui.label(RichText::new(METHOD_NOTICE).size(tokens.sizes.font_body));
                    ui.label(
                        RichText::new(
                            "Rule data comes from verified rule records only. \
                             Interpretations and extensions are labelled kind I and kind X.",
                        )
                        .size(tokens.sizes.font_small)
                        .color(tokens.colors.text_muted),
                    );
                }
                Dialog::Project => {
                    project_row(
                        ui,
                        "Piece",
                        &facts.piece.clone().unwrap_or_else(|| String::from("—")),
                        tokens,
                    );
                    project_row(ui, "Size", &facts.size, tokens);
                    project_row(
                        ui,
                        "Validation",
                        &format!("{} errors, {} warnings", facts.errors, facts.warnings),
                        tokens,
                    );
                    project_row(
                        ui,
                        "Limits",
                        if facts.limits_complete {
                            "configured"
                        } else {
                            "not configured: no range checks are applied"
                        },
                        tokens,
                    );
                    ui.separator();
                    ui.label(
                        RichText::new(
                            "Project files arrive in a later release; this session \
                             lives in memory only.",
                        )
                        .size(tokens.sizes.font_small)
                        .color(tokens.colors.text_muted),
                    );
                }
                Dialog::Export { format } => {
                    ui.label(RichText::new("Format").size(tokens.sizes.font_small));
                    let mut choice = *format;
                    for option in ExportFormat::ALL {
                        if ui
                            .radio_value(&mut choice, option, option.label())
                            .changed()
                        {
                            events.push(DialogEvent::SetFormat(choice));
                        }
                    }
                    ui.separator();
                    if blockers.is_empty() {
                        ui.label(
                            RichText::new("Ready: the shape draws and validation is quiet.")
                                .size(tokens.sizes.font_small)
                                .color(tokens.colors.success),
                        );
                    } else {
                        ui.label(RichText::new("Blocked:").size(tokens.sizes.font_small));
                        for blocker in blockers {
                            ui.label(
                                RichText::new(format!("• {blocker}"))
                                    .size(tokens.sizes.font_small)
                                    .color(tokens.colors.error),
                            );
                        }
                    }
                    ui.separator();
                    ui.add_enabled(false, egui::Button::new("Write file (arrives with S14e)"));
                    ui.label(
                        RichText::new("Exports always carry the toile-verification statement.")
                            .size(tokens.sizes.font_small)
                            .color(tokens.colors.text_muted),
                    );
                }
            }
            ui.separator();
            if ui.button("Close").clicked() {
                events.push(DialogEvent::Close);
            }
        });
    if !open {
        events.push(DialogEvent::Close);
    }
    // Escape dismisses without acting.
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        events.push(DialogEvent::Close);
    }
    events
}

/// One read-only fact row.
fn project_row(ui: &mut Ui, name: &str, value: &str, tokens: &Tokens) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(name)
                .size(tokens.sizes.font_small)
                .color(tokens.colors.text_muted),
        );
        ui.label(RichText::new(value).size(tokens.sizes.font_small));
    });
}
