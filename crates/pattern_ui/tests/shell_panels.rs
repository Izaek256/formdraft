//! T-SK01-17 / AC-FR-044-6: hiding and showing panels changes nothing else.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pattern_ui::app::Workspace;
use pattern_ui::shell::{Action, PanelsModel, Tool};
use pattern_ui::theme::ThemeKind;

#[test]
fn t_sk01_17_panels_model_round_trips() {
    let mut panels = PanelsModel::default();
    let before = panels;
    assert!(panels.all_visible());

    panels.toggle_all();
    assert!(!panels.any_visible());
    assert!(!panels.all_visible());

    panels.toggle_all();
    assert_eq!(panels, before);
}

#[test]
fn t_sk01_17_hiding_panels_leaves_other_state_untouched() {
    let mut workspace = Workspace {
        theme: ThemeKind::Light,
        tool: Tool::Zoom,
        show_rulers: false,
        show_grid: true,
        ..Default::default()
    };
    let snapshot = workspace.clone();

    workspace.apply_action(Action::HidePanels);
    assert!(!workspace.panels.any_visible());
    assert_eq!(workspace.theme, snapshot.theme);
    assert_eq!(workspace.tool, snapshot.tool);
    assert_eq!(workspace.show_rulers, snapshot.show_rulers);
    assert_eq!(workspace.show_grid, snapshot.show_grid);

    workspace.apply_action(Action::HidePanels);
    assert_eq!(workspace.panels, snapshot.panels);
}
