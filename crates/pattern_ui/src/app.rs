//! Top-level application state and the eframe entry point.
//!
//! All logic lives in the pure models of the region modules; this file only
//! composes the regions and routes events between them.

use egui::Ui;
use pattern_core::measure::{validate_draft, Limits, ProfileMetadata, ValidationReport};
use pattern_core::units::{format_length, FormatOptions, Length, Unit};

use crate::canvas::{grid, roles::Role, Bounds, Canvas};
use crate::panels::inspector::{self, InspectorTab, PieceFacts, PropertiesEvent};
use crate::panels::properties::PropertiesForm;
use crate::panels::{layers, pieces, warnings};
use crate::placeholder;
use crate::shell::controlbar::{self, ToolBarEvent, ToolBarState};
use crate::shell::dialogs::{self, Dialog, SessionFacts};
use crate::shell::menu::{self, MenuEvent, MenuState};
use crate::shell::notify::{self, Toasts};
use crate::shell::rail::{self, NavSection, RailEvent};
use crate::shell::shortcuts::{self, Action, Scope};
use crate::shell::startup::{self, Startup};
use crate::shell::statusbar::{self, StatusBarEvent, StatusBarState};
use crate::shell::titlebar::{self, SessionStatus};
use crate::shell::toolstrip::{self, PatternTool, PatternToolEvent, Tool};
use crate::shell::PanelsModel;
use crate::theme::{self, ThemeKind, Tokens};

/// The method and edition notice shown in the status bar and the about window.
pub const METHOD_NOTICE: &str =
    "Aldrich, Metric Pattern Cutting, 4th ed. (2004) - block for toile verification";

/// Display units offered by the status-bar pill, in cycle order.
const UNIT_CYCLE: [Unit; 3] = [Unit::Mm, Unit::Cm, Unit::In];

/// Everything the workspace remembers between frames.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// The active theme.
    pub theme: ThemeKind,
    /// Which panels are visible.
    pub panels: PanelsModel,
    /// The active canvas tool.
    pub tool: Tool,
    /// The chosen drafting tool (only Select works in R1).
    pub pattern_tool: PatternTool,
    /// The navigation section (only Drafting works in R1).
    pub nav: NavSection,
    /// The inspector tab.
    pub inspector_tab: InspectorTab,
    /// The piece name, shown on the canvas.
    pub piece_name: String,
    /// Whether the rulers are shown.
    pub show_rulers: bool,
    /// Whether the grid is shown.
    pub show_grid: bool,
    /// Whether snap is preferred (point snapping arrives in R2).
    pub snap: bool,
    /// Whether the shortcut list window is open.
    pub show_shortcuts: bool,
    /// Which dialog is open, if any.
    pub dialog: Dialog,
    /// Pending toast notifications.
    pub toasts: Toasts,
    /// The canvas.
    pub canvas: Canvas,
    /// The properties form.
    pub form: PropertiesForm,
    /// The thresholds, when they load.
    pub limits: Option<Limits>,
    /// The demo metadata, when it builds.
    pub metadata: Option<ProfileMetadata>,
    /// Whether the placeholder piece is visible.
    pub piece_visible: bool,
    /// The latest validation report, for the warnings panel.
    pub report: ValidationReport,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            theme: ThemeKind::default(),
            panels: PanelsModel::default(),
            tool: Tool::default(),
            pattern_tool: PatternTool::default(),
            nav: NavSection::default(),
            inspector_tab: InspectorTab::default(),
            piece_name: placeholder::PIECE_LABEL.to_owned(),
            show_rulers: true,
            show_grid: true,
            snap: true,
            show_shortcuts: false,
            dialog: Dialog::Closed,
            toasts: Toasts::default(),
            canvas: Canvas::new([800.0, 600.0]),
            form: PropertiesForm::default(),
            limits: pattern_templates::thresholds::load_thresholds().ok(),
            metadata: placeholder::demo_profile::metadata().ok(),
            piece_visible: true,
            report: ValidationReport::new(),
        }
    }
}

impl Workspace {
    /// Apply a shortcut or menu action to the model.
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::SelectTool => {
                self.tool = Tool::Select;
                self.pattern_tool = PatternTool::Select;
            }
            Action::HandTool => self.tool = Tool::Hand,
            Action::ZoomTool => self.tool = Tool::Zoom,
            Action::ToggleRulers => self.show_rulers = !self.show_rulers,
            Action::ToggleGrid => self.show_grid = !self.show_grid,
            Action::HidePanels => self.panels.toggle_all(),
            Action::ShortcutList => self.show_shortcuts = !self.show_shortcuts,
            Action::OpenExport => {
                self.dialog = Dialog::Export {
                    format: dialogs::ExportFormat::Pdf,
                };
            }
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

    /// Queue a toast for the current frame.
    pub fn notify(&mut self, now_sec: f64, text: String) {
        self.toasts.push(now_sec, text);
    }

    /// Re-parse the form, run S03 validation and refresh the canvas content,
    /// the pattern display unit and the warnings report. This is the real
    /// input path: field text in, drawable bounds and report out. Tests call
    /// it directly so the flow is covered without a UI harness.
    pub fn refresh(&mut self) {
        self.canvas.display_unit = self.form.pattern_unit;

        let parsed = self.form.parse();
        let limits = self.limits.clone();
        let metadata = self.metadata.clone();

        let mut content: Option<Bounds> = None;
        if parsed.errors.is_empty() {
            if let (Some(limits), Some(metadata)) = (limits.as_ref(), metadata.as_ref()) {
                match validate_draft(parsed.draft.clone(), limits, metadata.clone()) {
                    Ok((_profile, report)) => {
                        self.report = report;
                        content = placeholder::from_draft(&parsed.draft);
                    }
                    Err((report, _error)) => self.report = report,
                }
            } else {
                self.report = ValidationReport::new();
            }
        } else {
            self.report = ValidationReport::new();
        }

        let label = self.piece_name.clone();
        match content {
            Some(bounds) => self.canvas.set_content(bounds, label, Role::Placeholder),
            None => {
                self.canvas.content = None;
                self.canvas.selection = None;
            }
        }
        if let Some(content) = self.canvas.content.as_mut() {
            content.visible = self.piece_visible;
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
    startup: Startup,
    workspace: Workspace,
    canvas_focused: bool,
    cursor_mm: Option<[f64; 2]>,
    centered: bool,
}

impl WorkspaceApp {
    /// The current workspace, for tests and embedding.
    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    /// Centre the window on the first frame, using the real outer rect.
    ///
    /// eframe's `NativeOptions::centered` centres by inner size, so the title
    /// bar and bottom edge land under the taskbar on short screens. Here the
    /// title bar is included (outer rect) and the y position is clamped so the
    /// window stays fully visible.
    fn center_window(&mut self, ctx: &egui::Context) {
        self.centered = true;
        let placement = ctx.input(|i| {
            let outer = i.viewport().outer_rect?;
            let monitor = i.viewport().monitor_size?;
            let x = (monitor.x - outer.size().x) / 2.0;
            let y = (monitor.y - outer.size().y) / 2.0;
            // ponytail: assumes the taskbar is at most 40 px and on a screen
            // edge; query the work area (winit/FFI) if that ever matters.
            let max_y = (monitor.y - outer.size().y) - 40.0;
            let y = if max_y < 8.0 {
                8.0
            } else {
                y.clamp(8.0, max_y)
            };
            Some([x, y])
        });
        if let Some([x, y]) = placement {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(x, y)));
        }
    }
}

impl eframe::App for WorkspaceApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        if !self.centered {
            self.center_window(ui.ctx());
        }
        theme::apply(ui.ctx(), self.workspace.theme);
        self.workspace.refresh();

        let now_sec = ui.input(|i| i.time);
        if !self.startup.is_finished(now_sec) {
            self.run_startup(ui);
            return;
        }

        self.run_workspace(ui, now_sec);
    }
}

impl WorkspaceApp {
    /// One startup frame: paint the splash and run the due step.
    fn run_startup(&mut self, ui: &mut Ui) {
        let tokens = theme::tokens(self.workspace.theme);
        let step = startup::draw(ui, &mut self.startup, &tokens);
        if let Some(label) = step {
            self.run_startup_step(label);
            self.startup.done += 1;
        }
    }

    /// Perform one genuine startup step named by the splash screen.
    fn run_startup_step(&mut self, label: &str) {
        match label {
            "Loading measurement limits…" => {
                if self.workspace.limits.is_none() {
                    self.startup.problem = Some(String::from(
                        "Measurement limits are unavailable: no range checks are applied.",
                    ));
                }
            }
            "Preparing the placeholder profile…" => {
                if self.workspace.metadata.is_none() {
                    self.startup.problem = Some(String::from(
                        "The placeholder profile failed to build: no shape will draw.",
                    ));
                }
            }
            "Fitting the canvas…" => {
                self.workspace.canvas.fit();
            }
            _ => {}
        }
    }

    /// Select the content bounds, when a shape exists.
    fn select_content(&mut self) {
        if let Some(content) = &self.workspace.canvas.content {
            let centre = content.bounds.center_mm();
            self.workspace.canvas.select_at(centre);
        }
    }

    /// One workspace frame: the full application shell.
    fn run_workspace(&mut self, ui: &mut Ui, now_sec: f64) {
        let tokens = theme::tokens(self.workspace.theme);
        self.workspace.canvas.show_rulers = self.workspace.show_rulers;
        self.workspace.canvas.show_grid = self.workspace.show_grid;

        // Title bar.
        let session = self.session_status();
        let mut title_events = Vec::new();
        egui::Panel::top("title_bar")
            .exact_size(tokens.sizes.titlebar_height)
            .show(ui, |ui| {
                title_events = titlebar::draw(ui, session, &tokens);
            });
        for event in title_events {
            match event {
                titlebar::TitleBarEvent::OpenProject => {
                    self.workspace.dialog = Dialog::Project;
                }
            }
        }

        // Menu bar.
        let menu_state = MenuState {
            theme: self.workspace.theme,
            show_rulers: self.workspace.show_rulers,
            show_grid: self.workspace.show_grid,
        };
        let mut menu_events: Vec<MenuEvent> = Vec::new();
        egui::Panel::top("menu_bar").show(ui, |ui| {
            menu_events = menu::draw(ui, &menu_state);
        });

        // Status bar.
        let zoom_percent = self.workspace.canvas.viewport.zoom_percent();
        let status_state = self.status_state(zoom_percent);
        let mut status_events: Vec<StatusBarEvent> = Vec::new();
        egui::Panel::bottom("status_bar")
            .exact_size(tokens.sizes.status_bar_height)
            .show(ui, |ui| {
                status_events = statusbar::draw(ui, &status_state, &tokens);
            });

        // Bottom view strip: full height for a card, one status line when empty.
        let has_shape = self.workspace.canvas.content.is_some();
        let strip_height = if has_shape {
            tokens.sizes.strip_height
        } else {
            tokens.sizes.status_bar_height
        };
        egui::Panel::bottom("view_strip")
            .exact_size(strip_height)
            .show(ui, |ui| {
                self.draw_view_strip(ui, &tokens);
            });

        // Left column: navigation rail and contextual tools.
        if self.workspace.panels.left {
            let mut rail_events: Vec<RailEvent> = Vec::new();
            egui::Panel::left("nav_rail")
                .exact_size(tokens.sizes.rail_width)
                .show(ui, |ui| {
                    rail_events = rail::draw(ui, &mut self.workspace.nav, &tokens);
                });
            for event in rail_events {
                let RailEvent::Later(name) = event;
                self.workspace.notify(
                    now_sec,
                    format!("{name} arrives in a later release — staying in Drafting."),
                );
            }

            let mut tool_events: Vec<PatternToolEvent> = Vec::new();
            let mut piece_events = Vec::new();
            let mut layer_events = Vec::new();
            let piece_name = self.workspace.piece_name.clone();
            let piece_selected = self.workspace.canvas.selection.is_some();
            let size_text = self.piece_size_text();
            let show_grid = self.workspace.show_grid;
            let show_rulers = self.workspace.show_rulers;
            egui::Panel::left("tools_panel")
                .exact_size(tokens.sizes.tools_width)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        tool_events = toolstrip::draw_pattern_tools(
                            ui,
                            &mut self.workspace.pattern_tool,
                            &tokens,
                        );
                        ui.separator();
                        if self.workspace.panels.pieces {
                            ui.label(
                                egui::RichText::new("Pattern Pieces")
                                    .size(tokens.sizes.font_body)
                                    .strong(),
                            );
                            piece_events = pieces::draw(
                                ui,
                                &piece_name,
                                has_shape,
                                &mut self.workspace.piece_visible,
                                piece_selected,
                                &size_text,
                                &tokens,
                            );
                            ui.separator();
                            ui.label(
                                egui::RichText::new("Layers")
                                    .size(tokens.sizes.font_body)
                                    .strong(),
                            );
                            layer_events = layers::draw(
                                ui,
                                self.workspace.piece_visible,
                                has_shape,
                                show_grid,
                                show_rulers,
                                &tokens,
                            );
                        }
                    });
                });
            for event in tool_events {
                let PatternToolEvent::Later(name) = event;
                self.workspace.notify(
                    now_sec,
                    format!("{name} tool arrives in R2 — canvas stays in Select."),
                );
            }
            self.apply_piece_events(piece_events, now_sec);
            self.apply_layer_events(layer_events);
        }

        // Right inspector.
        if self.workspace.panels.properties || self.workspace.panels.warnings {
            let parsed = self.workspace.form.parse();
            let facts = self.piece_facts();
            let show_properties = self.workspace.panels.properties;
            let show_warnings = self.workspace.panels.warnings;
            let report = self.workspace.report.clone();
            egui::Panel::right("inspector")
                .exact_size(tokens.sizes.inspector_width)
                .show(ui, |ui| {
                    inspector::draw_tabs(ui, &mut self.workspace.inspector_tab, &tokens);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        if show_properties {
                            match self.workspace.inspector_tab {
                                InspectorTab::Properties => {
                                    let events = inspector::draw_properties(
                                        ui,
                                        &mut self.workspace.piece_name,
                                        &facts,
                                        &tokens,
                                    );
                                    for event in events {
                                        match event {
                                            PropertiesEvent::Renamed => {}
                                            PropertiesEvent::OpenMeasurements => {
                                                self.workspace.inspector_tab =
                                                    InspectorTab::Measurements;
                                            }
                                        }
                                    }
                                }
                                InspectorTab::Measurements => {
                                    inspector::draw_measurements(
                                        ui,
                                        &mut self.workspace.form,
                                        &parsed,
                                        &tokens,
                                    );
                                }
                                InspectorTab::Style => inspector::draw_style(ui, &tokens),
                                InspectorTab::Instructions => {
                                    inspector::draw_instructions(ui, &tokens);
                                }
                            }
                        }
                        if show_warnings {
                            ui.separator();
                            ui.label(
                                egui::RichText::new("Warnings")
                                    .size(tokens.sizes.font_body)
                                    .strong(),
                            );
                            warnings::draw(ui, &report, &tokens);
                        }
                    });
                });
        }

        // Center: document tabs, toolbar, canvas.
        let mut bar_events = Vec::new();
        let bar_state = self.toolbar_state(zoom_percent);
        egui::CentralPanel::default().show(ui, |ui| {
            self.draw_doc_tabs(ui, &tokens, now_sec);
            ui.separator();
            bar_events = controlbar::draw(ui, &bar_state, &tokens);
            ui.separator();
            let tool = self.workspace.tool;
            let response = self.workspace.canvas.show(ui, &tokens, tool);
            self.canvas_focused = response.focused;
            self.cursor_mm = response.cursor_world_mm;
            if response.open_measurements {
                self.workspace.inspector_tab = InspectorTab::Measurements;
            }
        });

        self.apply_menu_events(menu_events, now_sec);
        self.apply_toolbar_events(bar_events, now_sec);
        self.apply_status_events(status_events, now_sec);

        self.handle_shortcuts(ui);

        // Shortcut list window.
        let mut show_shortcuts = self.workspace.show_shortcuts;
        egui::Window::new("Keyboard shortcuts")
            .open(&mut show_shortcuts)
            .collapsible(false)
            .show(ui.ctx(), |ui| shortcut_list(ui, &tokens));
        self.workspace.show_shortcuts = show_shortcuts;

        // Modal dialogs.
        if self.workspace.dialog.is_open() {
            let facts = self.session_facts();
            let blockers = self.export_blockers();
            let events = dialogs::draw(ui, &mut self.workspace.dialog, &facts, &blockers, &tokens);
            for event in events {
                match event {
                    dialogs::DialogEvent::Close => {
                        self.workspace.dialog = Dialog::Closed;
                    }
                    dialogs::DialogEvent::SetFormat(format) => {
                        self.workspace.dialog = Dialog::Export { format };
                    }
                }
            }
        }

        // Toasts last, so they paint above everything.
        if notify::draw(ui, &mut self.workspace.toasts, &tokens) {
            ui.ctx().request_repaint();
        }
    }

    /// Apply piece-panel events.
    fn apply_piece_events(&mut self, events: Vec<pieces::PiecesEvent>, now_sec: f64) {
        for event in events {
            match event {
                pieces::PiecesEvent::ToggleVisibility => {
                    self.workspace.piece_visible = !self.workspace.piece_visible;
                    self.workspace.notify(
                        now_sec,
                        if self.workspace.piece_visible {
                            String::from("Piece shown.")
                        } else {
                            String::from("Piece hidden.")
                        },
                    );
                }
                pieces::PiecesEvent::Select => self.select_content(),
                pieces::PiecesEvent::OpenMeasurements => {
                    self.workspace.inspector_tab = InspectorTab::Measurements;
                }
            }
        }
    }

    /// Apply layers-panel events.
    fn apply_layer_events(&mut self, events: Vec<layers::LayersEvent>) {
        for event in events {
            match event {
                layers::LayersEvent::ToggleOutline => {
                    self.workspace.piece_visible = !self.workspace.piece_visible;
                }
                layers::LayersEvent::ToggleGrid => {
                    self.workspace.apply_action(Action::ToggleGrid);
                }
                layers::LayersEvent::ToggleRulers => {
                    self.workspace.apply_action(Action::ToggleRulers);
                }
            }
        }
    }

    /// Apply menu events.
    fn apply_menu_events(&mut self, events: Vec<MenuEvent>, now_sec: f64) {
        for event in events {
            match event {
                MenuEvent::Action(action) => self.workspace.apply_action(action),
                MenuEvent::SetTheme(theme) => {
                    self.workspace.theme = theme;
                    self.workspace
                        .notify(now_sec, format!("Theme: {}.", theme.label()));
                }
                MenuEvent::ShowAbout => self.workspace.dialog = Dialog::About,
                MenuEvent::ShowMethod => self.workspace.dialog = Dialog::Method,
                MenuEvent::OpenExport => self.workspace.apply_action(Action::OpenExport),
                MenuEvent::OpenProject => self.workspace.dialog = Dialog::Project,
                MenuEvent::ValidateNow => {
                    self.workspace.inspector_tab = InspectorTab::Measurements;
                    let parsed = self.workspace.form.parse();
                    let errors = parsed.errors.len() + self.workspace.report.errors.len();
                    let warnings = self.workspace.report.warnings.len();
                    self.workspace.notify(
                        now_sec,
                        format!("Validation: {errors} errors, {warnings} warnings."),
                    );
                }
                MenuEvent::Quit => {
                    // No context handle here; quitting stays on the native
                    // window control so it cannot fire by accident.
                }
            }
        }
    }

    /// Apply toolbar events.
    fn apply_toolbar_events(&mut self, events: Vec<ToolBarEvent>, now_sec: f64) {
        for event in events {
            match event {
                ToolBarEvent::UseTool(tool) => {
                    self.workspace.tool = tool;
                    if tool == Tool::Select {
                        self.workspace.pattern_tool = PatternTool::Select;
                    }
                }
                ToolBarEvent::ZoomIn => self.workspace.apply_action(Action::ZoomIn),
                ToolBarEvent::ZoomOut => self.workspace.apply_action(Action::ZoomOut),
                ToolBarEvent::Fit => {
                    self.workspace.apply_action(Action::FitToWindow);
                    self.workspace
                        .notify(now_sec, String::from("Canvas fitted to view."));
                }
                ToolBarEvent::ActualSize => {
                    self.workspace.apply_action(Action::ActualSize);
                    self.workspace.notify(
                        now_sec,
                        String::from("Approximate actual size (monitors differ)."),
                    );
                }
                ToolBarEvent::ToggleGrid => self.workspace.apply_action(Action::ToggleGrid),
                ToolBarEvent::ToggleRulers => {
                    self.workspace.apply_action(Action::ToggleRulers);
                }
                ToolBarEvent::ToggleSnap => {
                    self.workspace.snap = !self.workspace.snap;
                    self.workspace.notify(
                        now_sec,
                        if self.workspace.snap {
                            String::from("Snap preference on (point snapping arrives in R2).")
                        } else {
                            String::from("Snap preference off.")
                        },
                    );
                }
            }
        }
    }

    /// Apply status-bar events.
    fn apply_status_events(&mut self, events: Vec<StatusBarEvent>, now_sec: f64) {
        for event in events {
            match event {
                StatusBarEvent::CycleUnit => {
                    let position = UNIT_CYCLE
                        .iter()
                        .position(|unit| *unit == self.workspace.form.pattern_unit);
                    let next = position
                        .and_then(|index| UNIT_CYCLE.get(index + 1))
                        .or(UNIT_CYCLE.first());
                    if let Some(unit) = next {
                        self.workspace.form.pattern_unit = *unit;
                        self.workspace
                            .notify(now_sec, format!("Display unit: {}.", unit.suffix()));
                    }
                }
                StatusBarEvent::SetZoom(percent) => {
                    // Widening a UI percentage back to the viewport scale.
                    self.workspace.canvas.set_zoom_percent(f64::from(percent));
                }
            }
        }
    }

    /// Document tabs: one real tab per drawable piece.
    fn draw_doc_tabs(&mut self, ui: &mut Ui, tokens: &Tokens, now_sec: f64) {
        ui.horizontal(|ui| {
            ui.set_min_height(tokens.sizes.tab_height);
            match &self.workspace.canvas.content {
                Some(content) => {
                    let label = content.label.clone();
                    let selected = self.workspace.canvas.selection.is_some();
                    if ui
                        .selectable_label(selected, &label)
                        .on_hover_text("Select the piece")
                        .clicked()
                    {
                        self.select_content();
                    }
                    if ui
                        .button("×")
                        .on_hover_text("Hide the piece (restore it from Pattern Pieces)")
                        .clicked()
                    {
                        self.workspace.piece_visible = false;
                        self.workspace.notify(
                            now_sec,
                            String::from("Piece hidden — toggle Visible to restore it."),
                        );
                    }
                }
                None => {
                    ui.label(
                        egui::RichText::new("No piece — complete the measurements.")
                            .size(tokens.sizes.font_small)
                            .color(tokens.colors.text_muted),
                    );
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_enabled(false, egui::Button::new("+"))
                    .on_hover_text("More pieces arrive in a later release");
            });
        });
    }

    /// Bottom view strip: one card per drawable piece plus a disabled add card.
    fn draw_view_strip(&mut self, ui: &mut Ui, tokens: &Tokens) {
        ui.horizontal(|ui| match &self.workspace.canvas.content {
            Some(content) => {
                let selected = self.workspace.canvas.selection.is_some();
                let label = content.label.clone();
                let bounds = content.bounds;
                let (rect, response) = ui.allocate_exact_size(
                    egui::Vec2::new(tokens.sizes.thumb_size, tokens.sizes.thumb_size),
                    egui::Sense::click(),
                );
                let background = if selected {
                    tokens.colors.selection
                } else if response.hovered() {
                    tokens.colors.hover
                } else {
                    tokens.colors.surface
                };
                ui.painter().rect_filled(
                    rect,
                    egui::CornerRadius::same(tokens.sizes.corner_radius),
                    background,
                );
                draw_miniature(ui, rect, bounds, tokens);
                if response.on_hover_text("Select the piece").clicked() {
                    self.select_content();
                }
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(&label).size(tokens.sizes.font_small));
                    ui.label(
                        egui::RichText::new(self.piece_size_text())
                            .size(tokens.sizes.font_small)
                            .color(tokens.colors.text_muted),
                    );
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_enabled(false, egui::Button::new("+ Add View"))
                        .on_hover_text("More views arrive in a later release");
                });
            }
            None => {
                ui.label(
                    egui::RichText::new("No views yet — enter the measurements to draft a piece.")
                        .size(tokens.sizes.font_small)
                        .color(tokens.colors.text_muted),
                );
            }
        });
    }

    /// The placeholder size in the pattern unit, with millimetres only when
    /// the pattern unit differs from millimetres.
    fn piece_size_text(&self) -> String {
        let unit = self.workspace.form.pattern_unit;
        match &self.workspace.canvas.content {
            Some(content) => {
                let size = content.bounds.size_mm();
                let primary = format!(
                    "{} x {}",
                    format_mm(unit, size[0]),
                    format_mm(unit, size[1])
                );
                if unit == Unit::Mm {
                    primary
                } else {
                    format!(
                        "{primary}  ({}, {})",
                        format_mm(Unit::Mm, size[0]),
                        format_mm(Unit::Mm, size[1])
                    )
                }
            }
            None => String::from("No shape: fill every field with a positive number."),
        }
    }

    /// Live dimensions for the inspector, from the selection when one exists.
    fn piece_facts(&self) -> PieceFacts {
        let unit = self.workspace.canvas.display_unit;
        let bounds = self
            .workspace
            .canvas
            .selection
            .as_ref()
            .map(|selection| (selection.bounds, true))
            .or_else(|| {
                self.workspace
                    .canvas
                    .content
                    .as_ref()
                    .map(|content| (content.bounds, false))
            });
        match bounds {
            Some((box_mm, from_selection)) => {
                let origin = box_mm.min_mm;
                let size = box_mm.size_mm();
                PieceFacts {
                    kind: String::from("Placeholder rectangle (not a pattern)"),
                    origin: format!(
                        "{}, {}",
                        format_mm(unit, origin[0]),
                        format_mm(unit, origin[1])
                    ),
                    size: format_size(unit, size[0], size[1]),
                    from_selection,
                    exists: true,
                }
            }
            None => PieceFacts {
                kind: String::from("No piece yet"),
                origin: String::from("-"),
                size: String::from("-"),
                from_selection: false,
                exists: false,
            },
        }
    }

    /// The toolbar state for this frame.
    fn toolbar_state(&self, zoom_percent: f64) -> ToolBarState {
        let unit = self.workspace.canvas.display_unit;
        let step_mm = grid::grid_step_mm(unit, self.workspace.canvas.viewport.px_per_mm());
        let zoom = zoom_percent.clamp(0.0, 999.0);
        ToolBarState {
            tool: self.workspace.tool,
            zoom: format!("{zoom:.0}%"),
            grid: format!("Grid: {}", format_mm(unit, step_mm)),
            show_grid: self.workspace.show_grid,
            show_rulers: self.workspace.show_rulers,
            snap: self.workspace.snap,
        }
    }

    /// The status-bar state for this frame.
    fn status_state(&self, zoom_percent: f64) -> StatusBarState {
        let unit = self.workspace.canvas.display_unit;
        let cursor = match self.cursor_mm {
            Some([x, y]) => format!("{}, {}", format_mm(unit, x), format_mm(unit, y)),
            None => String::from("-"),
        };
        let selection = match &self.workspace.canvas.selection {
            Some(selection) => format!("1 selected: {}", selection.label),
            None => String::from("Nothing selected"),
        };
        let mode = match self.workspace.pattern_tool {
            PatternTool::Select => self.workspace.tool.label().to_owned(),
            other => format!("{} (R2 preview)", other.label()),
        };
        // Safe: the slider spans 25..=400; wider viewport states are clamped.
        let zoom = (zoom_percent as f32).clamp(25.0, 400.0);
        StatusBarState {
            cursor,
            mode,
            selection,
            unit: format!("Unit: {}", unit.suffix()),
            grid: self.toolbar_state(zoom_percent).grid,
            snap: self.workspace.snap,
            zoom_percent: zoom,
            limits_notice: !self.workspace.report.limits_complete,
        }
    }

    /// Session status for the title-bar pill.
    fn session_status(&self) -> SessionStatus {
        if self.workspace.canvas.content.is_none() {
            SessionStatus::NeedsInput
        } else if !self.workspace.report.errors.is_empty()
            || !self.workspace.report.warnings.is_empty()
        {
            SessionStatus::NeedsAttention
        } else {
            SessionStatus::Ready
        }
    }

    /// Session facts for the project dialog.
    fn session_facts(&self) -> SessionFacts {
        SessionFacts {
            piece: self
                .workspace
                .canvas
                .content
                .as_ref()
                .map(|content| content.label.clone()),
            size: self.piece_size_text(),
            errors: self.workspace.form.parse().errors.len() + self.workspace.report.errors.len(),
            warnings: self.workspace.report.warnings.len(),
            limits_complete: self.workspace.report.limits_complete,
        }
    }

    /// Real export blockers: form errors plus validation errors.
    fn export_blockers(&self) -> Vec<String> {
        let parsed = self.workspace.form.parse();
        let mut blockers: Vec<String> = parsed
            .errors
            .iter()
            .map(|error| error.message.clone())
            .collect();
        blockers.extend(
            self.workspace
                .report
                .errors
                .iter()
                .map(|error| format!("[{}] {}", error.code, error.message)),
        );
        blockers
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

/// Draw the shortcut list window.
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

/// A miniature of the piece bounds, keeping the true aspect ratio.
fn draw_miniature(ui: &Ui, rect: egui::Rect, bounds: Bounds, tokens: &Tokens) {
    let size = bounds.size_mm();
    if size[0] <= 0.0 || size[1] <= 0.0 {
        return;
    }
    let pad = rect.width().min(rect.height()) * 0.18;
    let inner = rect.shrink(pad);
    let scale = (inner.width() / size[0] as f32).min(inner.height() / size[1] as f32);
    // Safe: thumbnail pixels; f32 precision is plenty here.
    let width = size[0] as f32 * scale;
    let height = size[1] as f32 * scale;
    let origin = egui::pos2(
        inner.center().x - width / 2.0,
        inner.center().y - height / 2.0,
    );
    ui.painter().rect_stroke(
        egui::Rect::from_min_size(origin, egui::Vec2::new(width, height)),
        egui::CornerRadius::ZERO,
        egui::Stroke::new(tokens.sizes.stroke_thin, tokens.colors.placeholder),
        egui::StrokeKind::Inside,
    );
}
