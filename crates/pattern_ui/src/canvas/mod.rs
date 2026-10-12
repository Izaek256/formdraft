//! The canvas region: viewport, rulers, grid, content drawing and selection.
//!
//! Pure logic lives in the submodules (viewport maths, ruler ticks, dash
//! segmentation, selection). This file is the thin view that ties them to egui.

pub mod grid;
pub mod navigation;
pub mod render_cpu;
pub mod roles;
pub mod rulers;
pub mod select;
pub mod viewport;

pub use select::{Bounds, Selection};
pub use viewport::{Viewport, NOMINAL_PPI};

use egui::{Align2, CornerRadius, FontId, Rect, Sense, Stroke, StrokeKind};
use pattern_core::units::Unit;

use crate::shell::{Action, Tool};
use crate::theme::Tokens;

/// Pixels of margin left when fitting content.
const FIT_MARGIN_PX: f64 = 24.0;
/// Distance the view pans per keyboard pan action, in points.
const PAN_STEP_PX: f64 = 48.0;
/// Minimum spacing between ruler labels, in points.
const RULER_MIN_LABEL_PX: f64 = 56.0;
/// Zoom step for the keyboard zoom actions.
const KEY_ZOOM_FACTOR: f64 = 1.2;

/// A millimetres-to-egui mapping for one frame. Holds a copy of the viewport,
/// so the canvas may be mutated while a map from before the change is still in
/// use (for example for a click hit-test).
#[derive(Debug, Clone, Copy)]
pub struct ScreenMap {
    /// The viewport snapshot.
    pub viewport: Viewport,
    /// The screen position of the canvas origin, in points.
    pub offset: [f64; 2],
}

impl ScreenMap {
    /// Screen point to canvas-local coordinates.
    pub fn to_local(&self, pos: egui::Pos2) -> [f64; 2] {
        [
            f64::from(pos.x) - self.offset[0],
            f64::from(pos.y) - self.offset[1],
        ]
    }

    /// Screen point to millimetres.
    pub fn to_world(&self, pos: egui::Pos2) -> [f64; 2] {
        let local = self.to_local(pos);
        self.viewport.screen_to_world(local[0], local[1])
    }

    /// Millimetres to a screen point.
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_pos(&self, x_mm: f64, y_mm: f64) -> egui::Pos2 {
        let p = self.viewport.world_to_screen(x_mm, y_mm);
        egui::pos2(
            (p[0] + self.offset[0]) as f32,
            (p[1] + self.offset[1]) as f32,
        )
    }

    /// The millimetre bounds visible in a screen rectangle.
    pub fn rect_world_bounds(&self, rect: Rect) -> ([f64; 2], [f64; 2]) {
        (self.to_world(rect.min), self.to_world(rect.max))
    }
}

/// The drawable content on the canvas.
#[derive(Debug, Clone, PartialEq)]
pub struct Content {
    /// The bounding box in millimetres.
    pub bounds: Bounds,
    /// The label shown for the piece.
    pub label: String,
    /// The role used to draw the outline.
    pub role: roles::Role,
    /// Whether the piece is currently visible.
    pub visible: bool,
}

/// The canvas state: viewport, view toggles, content and selection.
#[derive(Debug, Clone)]
pub struct Canvas {
    /// The screen-to-millimetre transform.
    pub viewport: Viewport,
    /// The display unit for rulers and grid.
    pub display_unit: Unit,
    /// Whether the rulers are shown.
    pub show_rulers: bool,
    /// Whether the grid is shown.
    pub show_grid: bool,
    /// The content, when a shape can be drawn.
    pub content: Option<Content>,
    /// The current selection, when a piece is selected.
    pub selection: Option<Selection>,
}

impl Canvas {
    /// A new canvas of the given size in points.
    pub fn new(size_px: [f64; 2]) -> Self {
        Self {
            viewport: Viewport::new(size_px),
            display_unit: Unit::Mm,
            show_rulers: true,
            show_grid: true,
            content: None,
            selection: None,
        }
    }

    /// Set the drawable content. Keeps an existing selection when the piece is
    /// unchanged, so a redraw does not drop the user's selection.
    pub fn set_content(&mut self, bounds: Bounds, label: String, role: roles::Role) {
        let unchanged = self
            .content
            .as_ref()
            .is_some_and(|c| c.bounds == bounds && c.label == label && c.role == role);
        let visible = self.content.as_ref().is_none_or(|c| c.visible);
        self.content = Some(Content {
            bounds,
            label,
            role,
            visible,
        });
        if !unchanged {
            self.selection = None;
        }
    }

    /// Select a piece when `point_mm` lies inside it, otherwise clear.
    pub fn select_at(&mut self, point_mm: [f64; 2]) {
        self.selection = match &self.content {
            Some(content) if content.visible && content.bounds.contains(point_mm) => {
                Some(Selection {
                    bounds: content.bounds,
                    label: content.label.clone(),
                })
            }
            _ => None,
        };
    }

    /// Fit the content (or the origin) into the viewport.
    pub fn fit(&mut self) {
        if let Some(content) = &self.content {
            self.viewport
                .fit(content.bounds.min_mm, content.bounds.max_mm, FIT_MARGIN_PX);
        }
    }

    fn centre_px(&self) -> [f64; 2] {
        let size = self.viewport.size_px();
        [size[0] / 2.0, size[1] / 2.0]
    }

    /// Apply a view action from the menu or a shortcut.
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::ZoomIn => self.viewport.zoom_about(KEY_ZOOM_FACTOR, self.centre_px()),
            Action::ZoomOut => self
                .viewport
                .zoom_about(1.0 / KEY_ZOOM_FACTOR, self.centre_px()),
            Action::FitToWindow => self.fit(),
            Action::ActualSize => self.viewport.set_actual_size(NOMINAL_PPI),
            Action::PanLeft => self.viewport.pan_by([PAN_STEP_PX, 0.0]),
            Action::PanRight => self.viewport.pan_by([-PAN_STEP_PX, 0.0]),
            Action::PanUp => self.viewport.pan_by([0.0, PAN_STEP_PX]),
            Action::PanDown => self.viewport.pan_by([0.0, -PAN_STEP_PX]),
            Action::SelectTool
            | Action::HandTool
            | Action::ZoomTool
            | Action::ToggleRulers
            | Action::ToggleGrid
            | Action::HidePanels
            | Action::ShortcutList => {}
        }
    }

    /// Draw the canvas and apply pointer input. Returns whether the canvas has
    /// keyboard focus, and the cursor position in millimetres if hovering.
    pub fn show(&mut self, ui: &mut egui::Ui, tokens: &Tokens, tool: Tool) -> CanvasResponse {
        let desired = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click_and_drag());
        if response.clicked() {
            response.request_focus();
        }
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, CornerRadius::ZERO, tokens.colors.canvas);

        let mut inner = rect;
        if self.show_rulers {
            inner.min.x += tokens.sizes.ruler_thickness;
            inner.min.y += tokens.sizes.ruler_thickness;
        }
        self.viewport
            .set_size([f64::from(inner.width()), f64::from(inner.height())]);

        let before = ScreenMap {
            viewport: self.viewport,
            offset: [f64::from(inner.min.x), f64::from(inner.min.y)],
        };
        navigation::interact(ui, &response, &before, self, tool);

        let map = ScreenMap {
            viewport: self.viewport,
            offset: [f64::from(inner.min.x), f64::from(inner.min.y)],
        };

        if self.show_grid {
            grid::draw(&painter, &map, inner, self.display_unit, tokens);
        }

        let cursor_world = if response.hovered() {
            ui.input(|i| i.pointer.hover_pos()).map(|p| map.to_world(p))
        } else {
            None
        };

        if self.show_rulers {
            draw_rulers(
                &painter,
                rect,
                inner,
                &map,
                self.display_unit,
                cursor_world,
                tokens,
            );
        }

        self.draw_content(&painter, &map, tokens);

        CanvasResponse {
            focused: response.has_focus(),
            cursor_world_mm: cursor_world,
        }
    }

    fn draw_content(&self, painter: &egui::Painter, map: &ScreenMap, tokens: &Tokens) {
        let Some(content) = &self.content else {
            return;
        };
        if !content.visible {
            return;
        }
        let b = content.bounds;
        let outline = [
            [b.min_mm[0], b.min_mm[1]],
            [b.max_mm[0], b.min_mm[1]],
            [b.max_mm[0], b.max_mm[1]],
            [b.min_mm[0], b.max_mm[1]],
        ];
        render_cpu::draw_polyline(painter, map, &outline, true, content.role, tokens);

        if let Some(selection) = &self.selection {
            let sel = selection.bounds;
            painter.rect_stroke(
                Rect::from_min_max(
                    map.to_pos(sel.min_mm[0], sel.min_mm[1]),
                    map.to_pos(sel.max_mm[0], sel.max_mm[1]),
                ),
                CornerRadius::ZERO,
                Stroke::new(tokens.sizes.stroke_thick, tokens.colors.selection),
                StrokeKind::Inside,
            );
        }
    }
}

/// What the canvas reported after a frame.
#[derive(Debug, Clone, Copy)]
pub struct CanvasResponse {
    /// Whether the canvas has keyboard focus.
    pub focused: bool,
    /// The cursor position in millimetres, when hovering.
    pub cursor_world_mm: Option<[f64; 2]>,
}

/// Draw the top and left rulers.
#[allow(clippy::too_many_arguments)]
fn draw_rulers(
    painter: &egui::Painter,
    full: Rect,
    inner: Rect,
    map: &ScreenMap,
    unit: Unit,
    cursor_world: Option<[f64; 2]>,
    tokens: &Tokens,
) {
    if inner.width() <= 0.0 || inner.height() <= 0.0 {
        return;
    }
    let top = Rect::from_min_max(
        egui::pos2(inner.min.x, full.min.y),
        egui::pos2(full.max.x, inner.min.y),
    );
    let left = Rect::from_min_max(
        egui::pos2(full.min.x, inner.min.y),
        egui::pos2(inner.min.x, full.max.y),
    );
    let corner = Rect::from_min_max(full.min, egui::pos2(inner.min.x, inner.min.y));
    painter.rect_filled(top, CornerRadius::ZERO, tokens.colors.ruler_bg);
    painter.rect_filled(left, CornerRadius::ZERO, tokens.colors.ruler_bg);
    painter.rect_filled(corner, CornerRadius::ZERO, tokens.colors.ruler_bg);

    let step = rulers::step_mm(unit, map.viewport.px_per_mm(), RULER_MIN_LABEL_PX);
    let tick = Stroke::new(tokens.sizes.stroke_thin, tokens.colors.border);
    let font = FontId::proportional(tokens.sizes.font_small);
    let (min_mm, max_mm) = map.rect_world_bounds(inner);
    let tick_len = f64::from(tokens.sizes.ruler_tick_len);
    let pad = f64::from(tokens.sizes.ruler_label_pad);

    for x_mm in rulers::ticks_mm(step, min_mm[0], max_mm[0]) {
        let sx = map.viewport.world_to_screen(x_mm, 0.0)[0] + map.offset[0];
        painter.line_segment(
            [
                egui::pos2(sx as f32, inner.min.y - tick_len as f32),
                egui::pos2(sx as f32, inner.min.y),
            ],
            tick,
        );
        painter.text(
            egui::pos2(sx as f32 + pad as f32, full.min.y),
            Align2::LEFT_TOP,
            rulers::tick_label(unit, x_mm),
            font.clone(),
            tokens.colors.text_muted,
        );
    }
    for y_mm in rulers::ticks_mm(step, min_mm[1], max_mm[1]) {
        let sy = map.viewport.world_to_screen(0.0, y_mm)[1] + map.offset[1];
        painter.line_segment(
            [
                egui::pos2(inner.min.x - tick_len as f32, sy as f32),
                egui::pos2(inner.min.x, sy as f32),
            ],
            tick,
        );
        painter.text(
            egui::pos2(full.min.x, sy as f32 + pad as f32),
            Align2::LEFT_TOP,
            rulers::tick_label(unit, y_mm),
            font.clone(),
            tokens.colors.text_muted,
        );
    }

    if let Some(world) = cursor_world {
        let marker = Stroke::new(tokens.sizes.stroke_thin, tokens.colors.focus);
        let sx = map.viewport.world_to_screen(world[0], 0.0)[0] + map.offset[0];
        let sy = map.viewport.world_to_screen(0.0, world[1])[1] + map.offset[1];
        painter.line_segment(
            [
                egui::pos2(sx as f32, full.min.y),
                egui::pos2(sx as f32, inner.min.y),
            ],
            marker,
        );
        painter.line_segment(
            [
                egui::pos2(full.min.x, sy as f32),
                egui::pos2(inner.min.x, sy as f32),
            ],
            marker,
        );
    }
}
