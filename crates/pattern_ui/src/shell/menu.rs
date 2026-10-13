//! The menu bar. Only items whose feature works are enabled; later-release
//! items appear disabled with an R2 note (D8).

use egui::Ui;

use crate::shell::shortcuts::Action;
use crate::theme::ThemeKind;

/// Something the menu asked the application to do.
#[derive(Debug, Clone, PartialEq)]
pub enum MenuEvent {
    /// A shortcut action was chosen.
    Action(Action),
    /// Switch to a theme.
    SetTheme(ThemeKind),
    /// Open the about dialog.
    ShowAbout,
    /// Open the method and edition dialog.
    ShowMethod,
    /// Open the export readiness dialog.
    OpenExport,
    /// Open the project and session dialog.
    OpenProject,
    /// Validate now: focus the measurements and report the counts.
    ValidateNow,
    /// Quit the application.
    Quit,
}

/// State the menu reflects.
#[derive(Debug, Clone, Copy)]
pub struct MenuState {
    /// The active theme.
    pub theme: ThemeKind,
    /// Whether the rulers are shown.
    pub show_rulers: bool,
    /// Whether the grid is shown.
    pub show_grid: bool,
}

/// A disabled row with an R2 note.
fn later(ui: &mut Ui, label: &str) {
    ui.add_enabled(false, egui::Button::new(format!("{label}  ·  R2")));
}

/// Draw the menu bar and return the events the user triggered.
pub fn draw(ui: &mut Ui, state: &MenuState) -> Vec<MenuEvent> {
    let mut events = Vec::new();
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            later(ui, "New project");
            later(ui, "Open project");
            later(ui, "Save project");
            later(ui, "Save as");
            later(ui, "Import measurements");
            if ui.button("Export pattern…   Ctrl+E").clicked() {
                events.push(MenuEvent::OpenExport);
                ui.close();
            }
            ui.separator();
            if ui.button("Quit").clicked() {
                events.push(MenuEvent::Quit);
                ui.close();
            }
        });
        ui.menu_button("Edit", |ui| {
            later(ui, "Undo");
            later(ui, "Redo");
            ui.separator();
            later(ui, "Cut");
            later(ui, "Copy");
            later(ui, "Paste");
        });
        ui.menu_button("View", |ui| {
            if ui.button("Zoom in   Ctrl++").clicked() {
                events.push(MenuEvent::Action(Action::ZoomIn));
                ui.close();
            }
            if ui.button("Zoom out   Ctrl+-").clicked() {
                events.push(MenuEvent::Action(Action::ZoomOut));
                ui.close();
            }
            if ui.button("Fit to window   Ctrl+0").clicked() {
                events.push(MenuEvent::Action(Action::FitToWindow));
                ui.close();
            }
            if ui.button("Approximate actual size   Ctrl+1").clicked() {
                events.push(MenuEvent::Action(Action::ActualSize));
                ui.close();
            }
            ui.separator();
            if ui
                .selectable_label(state.show_rulers, "Rulers   Ctrl+R")
                .clicked()
            {
                events.push(MenuEvent::Action(Action::ToggleRulers));
                ui.close();
            }
            if ui
                .selectable_label(state.show_grid, "Grid   Ctrl+'")
                .clicked()
            {
                events.push(MenuEvent::Action(Action::ToggleGrid));
                ui.close();
            }
            ui.separator();
            ui.menu_button("Theme", |ui| {
                for theme in ThemeKind::ALL {
                    if ui
                        .selectable_label(state.theme == theme, theme.label())
                        .clicked()
                    {
                        events.push(MenuEvent::SetTheme(theme));
                        ui.close();
                    }
                }
            });
            ui.separator();
            if ui.button("Hide or show panels   Ctrl+Shift+H").clicked() {
                events.push(MenuEvent::Action(Action::HidePanels));
                ui.close();
            }
        });
        ui.menu_button("Pattern", |ui| {
            if ui.button("Method and edition…").clicked() {
                events.push(MenuEvent::ShowMethod);
                ui.close();
            }
            later(ui, "New pattern piece");
            later(ui, "Duplicate piece");
            later(ui, "Seam allowance");
        });
        ui.menu_button("Measurements", |ui| {
            if ui.button("Validate measurements").clicked() {
                events.push(MenuEvent::ValidateNow);
                ui.close();
            }
            later(ui, "Measurement library");
            later(ui, "Add measurement");
        });
        ui.menu_button("Tools", |ui| {
            if ui.button("Select tool   V").clicked() {
                events.push(MenuEvent::Action(Action::SelectTool));
                ui.close();
            }
            if ui.button("Hand tool   H").clicked() {
                events.push(MenuEvent::Action(Action::HandTool));
                ui.close();
            }
            if ui.button("Zoom tool   Z").clicked() {
                events.push(MenuEvent::Action(Action::ZoomTool));
                ui.close();
            }
            ui.separator();
            later(ui, "Point tool");
            later(ui, "Line tool");
            later(ui, "Curve tool");
            later(ui, "Measure tool");
        });
        ui.menu_button("Grading", |ui| {
            later(ui, "Create size grade");
            later(ui, "Grade rules");
            later(ui, "Preview sizes");
        });
        ui.menu_button("Help", |ui| {
            if ui.button("Shortcuts   F1").clicked() {
                events.push(MenuEvent::Action(Action::ShortcutList));
                ui.close();
            }
            if ui.button("About").clicked() {
                events.push(MenuEvent::ShowAbout);
                ui.close();
            }
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(egui::Button::new(egui::RichText::new("Export ↓").strong()))
                .on_hover_text("Check export readiness")
                .clicked()
            {
                events.push(MenuEvent::OpenExport);
            }
        });
    });
    events
}
