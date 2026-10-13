//! The keyboard shortcut scheme (OQ-39, docs/design/ui-spec.md section 6).
//!
//! Only shortcuts for features that exist are bound. `Tab` is reserved for
//! focus order and is never a shortcut. The table is pure data so a test can
//! prove that every documented shortcut maps to exactly one action.

use egui::{Key, KeyboardShortcut, Modifiers};

/// A command reachable from the keyboard or a menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Select tool.
    SelectTool,
    /// Hand (pan) tool.
    HandTool,
    /// Zoom tool.
    ZoomTool,
    /// Zoom in.
    ZoomIn,
    /// Zoom out.
    ZoomOut,
    /// Fit the drawing to the window.
    FitToWindow,
    /// Approximate actual size (96 pixels per inch).
    ActualSize,
    /// Show or hide the rulers.
    ToggleRulers,
    /// Show or hide the grid.
    ToggleGrid,
    /// Hide or show all panels.
    HidePanels,
    /// Open the shortcut list.
    ShortcutList,
    /// Pan left.
    PanLeft,
    /// Pan right.
    PanRight,
    /// Pan up.
    PanUp,
    /// Pan down.
    PanDown,
    /// Open the export readiness dialog.
    OpenExport,
}

/// When a shortcut is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// Active anywhere in the window.
    Global,
    /// Active only when the canvas has keyboard focus.
    CanvasFocus,
}

/// One documented shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shortcut {
    /// The action performed.
    pub action: Action,
    /// The key.
    pub key: Key,
    /// Control (or Command on macOS).
    pub ctrl: bool,
    /// Shift.
    pub shift: bool,
    /// Where the shortcut is active.
    pub scope: Scope,
}

impl Shortcut {
    const fn new(action: Action, key: Key, ctrl: bool, shift: bool, scope: Scope) -> Self {
        Self {
            action,
            key,
            ctrl,
            shift,
            scope,
        }
    }

    /// The egui keyboard shortcut for consuming input.
    pub fn egui(self) -> KeyboardShortcut {
        let mut modifiers = Modifiers::NONE;
        if self.ctrl {
            modifiers |= Modifiers::CTRL;
        }
        if self.shift {
            modifiers |= Modifiers::SHIFT;
        }
        KeyboardShortcut::new(modifiers, self.key)
    }
}

/// The documented shortcuts, in the order the spec lists them.
pub const SHORTCUTS: &[Shortcut] = &[
    Shortcut::new(Action::SelectTool, Key::V, false, false, Scope::Global),
    Shortcut::new(Action::HandTool, Key::H, false, false, Scope::Global),
    Shortcut::new(Action::ZoomTool, Key::Z, false, false, Scope::Global),
    Shortcut::new(Action::ZoomIn, Key::Plus, true, false, Scope::Global),
    Shortcut::new(Action::ZoomIn, Key::Plus, false, false, Scope::CanvasFocus),
    Shortcut::new(Action::ZoomOut, Key::Minus, true, false, Scope::Global),
    Shortcut::new(
        Action::ZoomOut,
        Key::Minus,
        false,
        false,
        Scope::CanvasFocus,
    ),
    Shortcut::new(Action::FitToWindow, Key::Num0, true, false, Scope::Global),
    Shortcut::new(Action::ActualSize, Key::Num1, true, false, Scope::Global),
    Shortcut::new(Action::OpenExport, Key::E, true, false, Scope::Global),
    Shortcut::new(Action::ToggleRulers, Key::R, true, false, Scope::Global),
    Shortcut::new(Action::ToggleGrid, Key::Quote, true, false, Scope::Global),
    Shortcut::new(Action::HidePanels, Key::H, true, true, Scope::Global),
    Shortcut::new(Action::ShortcutList, Key::F1, false, false, Scope::Global),
    Shortcut::new(
        Action::PanLeft,
        Key::ArrowLeft,
        false,
        false,
        Scope::CanvasFocus,
    ),
    Shortcut::new(
        Action::PanRight,
        Key::ArrowRight,
        false,
        false,
        Scope::CanvasFocus,
    ),
    Shortcut::new(
        Action::PanUp,
        Key::ArrowUp,
        false,
        false,
        Scope::CanvasFocus,
    ),
    Shortcut::new(
        Action::PanDown,
        Key::ArrowDown,
        false,
        false,
        Scope::CanvasFocus,
    ),
];

/// Resolve a key press to at most one action.
pub fn lookup(key: Key, ctrl: bool, shift: bool, canvas_focus: bool) -> Option<Action> {
    SHORTCUTS
        .iter()
        .find(|s| {
            s.key == key
                && s.ctrl == ctrl
                && s.shift == shift
                && (s.scope == Scope::Global || canvas_focus)
        })
        .map(|s| s.action)
}

/// A human-readable description of a shortcut, e.g. `Ctrl+Shift+H`.
pub fn describe(shortcut: Shortcut) -> String {
    let mut parts: Vec<&str> = Vec::new();
    if shortcut.ctrl {
        parts.push("Ctrl");
    }
    if shortcut.shift {
        parts.push("Shift");
    }
    let key = format!("{:?}", shortcut.key);
    parts.push(&key);
    parts.join("+")
}

/// Every documented shortcut as `(description, action)` pairs for the F1 window.
pub fn documented() -> Vec<(String, Action)> {
    SHORTCUTS.iter().map(|s| (describe(*s), s.action)).collect()
}

/// The display name of an action.
pub fn action_label(action: Action) -> &'static str {
    match action {
        Action::SelectTool => "Select tool",
        Action::HandTool => "Hand tool",
        Action::ZoomTool => "Zoom tool",
        Action::ZoomIn => "Zoom in",
        Action::ZoomOut => "Zoom out",
        Action::FitToWindow => "Fit to window",
        Action::ActualSize => "Approximate actual size",
        Action::ToggleRulers => "Toggle rulers",
        Action::ToggleGrid => "Toggle grid",
        Action::HidePanels => "Hide or show panels",
        Action::ShortcutList => "Shortcut list",
        Action::PanLeft => "Pan left",
        Action::PanRight => "Pan right",
        Action::PanUp => "Pan up",
        Action::PanDown => "Pan down",
        Action::OpenExport => "Export readiness",
    }
}
