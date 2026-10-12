//! Top-level application state and the eframe entry point.
//!
//! All logic lives in the pure models of the region modules; this file only
//! composes the regions and routes events between them.

use egui::Ui;
use pattern_core::units::{format_length, FormatOptions, Length, Unit};

use crate::canvas::Canvas;
use crate::shell::menu::{self, MenuEvent, MenuState};
use crate::shell::shortcuts::{self, Action, Scope};
use crate::shell::statusbar::{self, StatusBarState};
use crate::shell::{controlbar, toolstrip, PanelsModel, Tool};
use crate::theme::{self, ThemeKind, Tokens};

/// The method and edition notice shown in the status bar and the about window.
pub const METHOD_NOTICE: &str =
    "Aldrich, Metric Pattern Cutting, 4th ed. (2004) - block for toile verification";

/// Everything the workspace remembers between frames.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// The active theme.
    pub theme: ThemeKind,
    /// Which right-hand panels are visible.
    pub panels: PanelsModel,
    /// The active tool.
    pub tool: Tool,
    /// Whether the rulers are shown.
    pub show_rulers: bool,
    /// Whether the grid is shown.
    pub show_grid: bool,
    /// Whether the shortcut list window is open.
    pub show_shortcuts: bool,
    /// Whether the about window is open.
    pub show_about: bool,
    /// The canvas.
    pub canvas: Canvas,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            theme: ThemeKind::default(),
            panels: PanelsModel::default(),
            tool: Tool::default(),
            show_rulers: true,
            show_grid: true,
            show_shortcuts: false,
            show_about: false,
            canvas: Canvas::new([800.0, 600.0]),
        }
    }
}

impl Workspace {
    /// Apply a shortcut or menu action to the model.
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::SelectTool => self.tool = Tool::Select,
            Action::HandTool => self.tool = Tool::Hand,
            Action::ZoomTool => self.tool = Tool::Zoom,
            Action::ToggleRulers => self.show_rulers = !self.show_rulers,
            Action::ToggleGrid => self.show_grid = !self.show_grid,
            Action::HidePanels => self.panels.toggle_all(),
            Action::ShortcutList => self.show_shortcuts = !self.show_shortcuts,
            Action::ZoomIn
            | Action::ZoomOut
            | Action::FitToWindow
            | Action::ActualSize
            | Action::PanLeft
            | Action::PanRight
            | Action::PanUp
            | Action::PanDown => self.canvas.handle_action(action),
        }
    }
}

/// Format a millimetre value through the unit service (ADR-0007).
fn format_mm(unit: Unit, mm: f64) -> String {
    match Length::from_mm(mm) {
        Ok(length) => format_length(length, &FormatOptions::default_for(unit)),
        Err(_) => String::from("--"),
    }
}

/// Format a millimetre size as `width x height` in the display unit.
fn format_size(unit: Unit, width_mm: f64, height_mm: f64) -> String {
    format!(
        "{} x {}",
        format_mm(unit, width_mm),
        format_mm(unit, height_mm)
    )
}

/// The eframe application.
#[derive(Default)]
pub struct WorkspaceApp {
    workspace: Workspace,
    canvas_focused: bool,
    cursor_mm: Option<[f64; 2]>,
}

impl WorkspaceApp {
    /// The current workspace, for tests and embedding.
    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }
}

impl eframe::App for WorkspaceApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        theme::apply(ui.ctx(), self.workspace.theme);
        let tokens = theme::tokens(self.workspace.theme);
        self.workspace.canvas.show_rulers = self.workspace.show_rulers;
        self.workspace.canvas.show_grid = self.workspace.show_grid;

        let menu_state = MenuState {
            theme: self.workspace.theme,
            show_rulers: self.workspace.show_rulers,
            show_grid: self.workspace.show_grid,
        };
        let mut menu_events: Vec<MenuEvent> = Vec::new();
        egui::Panel::top("menu_bar").show(ui, |ui| {
            menu_events = menu::draw(ui, &menu_state);
        });

        let zoom_percent = self.workspace.canvas.viewport.zoom_percent();
        let bar_state = controlbar::ControlBarState {
            zoom_percent: zoom_percent as f32,
            actual_size: (zoom_percent - 100.0).abs() < 0.5,
        };
        let mut bar_events = Vec::new();
        egui::Panel::top("control_bar").show(ui, |ui| {
            bar_events = controlbar::draw(ui, &bar_state, &tokens);
        });

        let status_state = self.status_state(zoom_percent);
        egui::Panel::bottom("status_bar")
            .exact_size(tokens.sizes.status_bar_height)
            .show(ui, |ui| {
                statusbar::draw(ui, &status_state, &tokens);
            });

        egui::Panel::left("tool_strip")
            .exact_size(tokens.sizes.tool_strip_width)
            .show(ui, |ui| {
                toolstrip::draw(ui, &mut self.workspace.tool, &tokens);
            });

        if self.workspace.panels.any_visible() {
            let panels = self.workspace.panels;
            egui::Panel::right("panel_column")
                .exact_size(tokens.sizes.panel_width)
                .show(ui, |ui| {
                    if panels.properties {
                        egui::CollapsingHeader::new("Properties")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.label("Stage C fills this panel.");
                            });
                    }
                    if panels.pieces {
                        egui::CollapsingHeader::new("Pieces")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.label("Stage C fills this panel.");
                            });
                    }
                    if panels.warnings {
                        egui::CollapsingHeader::new("Warnings")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.label("Stage C fills this panel.");
                            });
                    }
                });
        }

        let tool = self.workspace.tool;
        let response = egui::CentralPanel::default()
            .show(ui, |ui| self.workspace.canvas.show(ui, &tokens, tool));
        let inner = response.inner;
        self.canvas_focused = inner.focused;
        self.cursor_mm = inner.cursor_world_mm;

        for event in menu_events {
            match event {
                MenuEvent::Action(action) => self.workspace.apply_action(action),
                MenuEvent::SetTheme(theme) => self.workspace.theme = theme,
                MenuEvent::ShowAbout => self.workspace.show_about = true,
                MenuEvent::Quit => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
            }
        }
        for event in bar_events {
            use controlbar::ControlBarEvent;
            let action = match event {
                ControlBarEvent::ZoomIn => Some(Action::ZoomIn),
                ControlBarEvent::ZoomOut => Some(Action::ZoomOut),
                ControlBarEvent::Fit => Some(Action::FitToWindow),
                ControlBarEvent::ActualSize => Some(Action::ActualSize),
            };
            if let Some(action) = action {
                self.workspace.apply_action(action);
            }
        }

        self.handle_shortcuts(ui);

        let mut show_shortcuts = self.workspace.show_shortcuts;
        egui::Window::new("Shortcuts")
            .open(&mut show_shortcuts)
            .show(ui.ctx(), |ui| shortcut_list(ui, &tokens));
        self.workspace.show_shortcuts = show_shortcuts;

        let mut show_about = self.workspace.show_about;
        egui::Window::new("About")
            .open(&mut show_about)
            .show(ui.ctx(), |ui| about_text(ui, &tokens));
        self.workspace.show_about = show_about;
    }
}

impl WorkspaceApp {
    fn status_state(&self, zoom_percent: f64) -> StatusBarState {
        let unit = self.workspace.canvas.display_unit;
        let cursor = match self.cursor_mm {
            Some([x, y]) => format!("{} , {}", format_mm(unit, x), format_mm(unit, y)),
            None => String::from("-"),
        };
        let selection = match &self.workspace.canvas.selection {
            Some(selection) => {
                let size = selection.bounds.size_mm();
                format_size(unit, size[0], size[1])
            }
            None => String::from("-"),
        };
        StatusBarState {
            zoom: format!("{zoom_percent:.0}%"),
            cursor,
            selection,
            method: METHOD_NOTICE.to_owned(),
            limits_notice: true,
        }
    }

    fn handle_shortcuts(&mut self, ui: &mut Ui) {
        let typing = ui.ctx().egui_wants_keyboard_input();
        for shortcut in shortcuts::SHORTCUTS {
            if shortcut.scope == Scope::CanvasFocus && !self.canvas_focused {
                continue;
            }
            if !shortcut.ctrl && typing {
                continue;
            }
            if ui.input_mut(|i| i.consume_shortcut(&shortcut.egui())) {
                self.workspace.apply_action(shortcut.action);
            }
        }
    }
}

fn shortcut_list(ui: &mut Ui, tokens: &Tokens) {
    for (description, action) in shortcuts::documented() {
        ui.label(
            egui::RichText::new(format!(
                "{description}   {}",
                shortcuts::action_label(action)
            ))
            .size(tokens.sizes.font_small),
        );
    }
}

fn about_text(ui: &mut Ui, tokens: &Tokens) {
    ui.label(egui::RichText::new("formdraft").size(tokens.sizes.font_heading));
    ui.label(METHOD_NOTICE);
    ui.label(
        "PLACEHOLDER SHAPE, NOT A PATTERN. This build is a block for toile verification; \
         fit must be confirmed on a calico toile before final fabric is cut.",
    );
}
