//! T-SK01-15 / AC-FR-044-5: each documented key maps to exactly one action,
//! Tab is never bound, and no two shortcuts with the same key and modifiers
//! disagree.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use egui::Key;
use pattern_ui::shell::shortcuts::{lookup, Scope, SHORTCUTS};

#[test]
fn t_sk01_15_tab_is_never_a_shortcut() {
    for shortcut in SHORTCUTS {
        assert_ne!(shortcut.key, Key::Tab, "Tab must stay free for focus order");
    }
    for ctrl in [false, true] {
        for shift in [false, true] {
            for canvas in [false, true] {
                assert_eq!(lookup(Key::Tab, ctrl, shift, canvas), None);
            }
        }
    }
}

#[test]
fn t_sk01_15_each_shortcut_resolves_to_its_own_action() {
    for shortcut in SHORTCUTS {
        let canvas = shortcut.scope == Scope::CanvasFocus;
        let got = lookup(shortcut.key, shortcut.ctrl, shortcut.shift, canvas);
        assert_eq!(
            got,
            Some(shortcut.action),
            "shortcut {:?}+{}{:?} did not resolve",
            shortcut.key,
            if shortcut.ctrl { "Ctrl+" } else { "" },
            shortcut.shift
        );
    }
}

#[test]
fn t_sk01_15_no_conflicting_bindings() {
    for (i, a) in SHORTCUTS.iter().enumerate() {
        for b in &SHORTCUTS[i + 1..] {
            if a.key == b.key && a.ctrl == b.ctrl && a.shift == b.shift {
                assert_eq!(
                    a.action, b.action,
                    "key {:?} is bound to two different actions",
                    a.key
                );
            }
        }
    }
}

#[test]
fn t_sk01_15_canvas_shortcuts_need_focus() {
    for shortcut in SHORTCUTS {
        if shortcut.scope == Scope::CanvasFocus {
            let global_twin = SHORTCUTS.iter().any(|s| {
                s.scope == Scope::Global
                    && s.key == shortcut.key
                    && s.ctrl == shortcut.ctrl
                    && s.shift == shortcut.shift
            });
            if !global_twin {
                assert_eq!(
                    lookup(shortcut.key, shortcut.ctrl, shortcut.shift, false),
                    None,
                    "{:?} must not fire without canvas focus",
                    shortcut.key
                );
            }
        }
    }
}
