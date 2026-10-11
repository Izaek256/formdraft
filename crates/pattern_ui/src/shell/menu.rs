//! The menu bar. Only items whose feature works are shown (D8).

use egui::Ui;

use crate::shell::shortcuts::Action;
use crate::theme::ThemeKind;

/// Something the menu asked the application to do.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuEvent {
    /// A shortcut action was chosen.
    Action(Action),
    /// Switch to a theme.
    SetTheme(ThemeKind),
    /// Open the about window.
    ShowAbout,
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

/// Draw the menu bar and return the events the user triggered.
pub fn draw(ui: &mut Ui, state: &MenuState) -> Vec<MenuEvent> {
    let mut events = Vec::new();
    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
                events.push(MenuEvent::Quit);
                ui.close();
            }
        });
        ui.menu_button("View", |ui| {
            if ui.button("Zoom in").clicked() {
                events.push(MenuEvent::Action(Action::ZoomIn));
                ui.close();
            }
            if ui.button("Zoom out").clicked() {
                events.push(MenuEvent::Action(Action::ZoomOut));
                ui.close();
            }
            if ui.button("Fit to window").clicked() {
                events.push(MenuEvent::Action(Action::FitToWindow));
                ui.close();
            }
            if ui.button("Approximate actual size").clicked() {
                events.push(MenuEvent::Action(Action::ActualSize));
                ui.close();
            }
            ui.separator();
            if ui.selectable_label(state.show_rulers, "Rulers").clicked() {
                events.push(MenuEvent::Action(Action::ToggleRulers));
                ui.close();
            }
            if ui.selectable_label(state.show_grid, "Grid").clicked() {
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
            if ui.button("Hide or show panels").clicked() {
                events.push(MenuEvent::Action(Action::HidePanels));
                ui.close();
            }
        });
        ui.menu_button("Help", |ui| {
            if ui.button("Shortcuts").clicked() {
                events.push(MenuEvent::Action(Action::ShortcutList));
                ui.close();
            }
            if ui.button("About").clicked() {
                events.push(MenuEvent::ShowAbout);
                ui.close();
            }
        });
    });
    events
}
