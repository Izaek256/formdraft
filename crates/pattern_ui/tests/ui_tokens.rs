//! T-SK01-12 / AC-FR-044-3: no colour literal or hard-coded size outside the
//! theme module. The second test plants a violation and proves the check fails.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn shell() -> Option<&'static str> {
    ["bash", "sh"].into_iter().find(|name| {
        Command::new(name)
            .arg("-c")
            .arg("echo ok")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    })
}

#[test]
fn t_sk01_12_checker_flags_a_planted_colour_literal() {
    let Some(shell) = shell() else {
        eprintln!("skipping: no bash or sh on PATH");
        return;
    };
    let relative = format!("target/sk01_tokens_{}", std::process::id());
    let dir = repo_root().join(&relative);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("violation.rs"),
        "fn bad() -> egui::Color32 { egui::Color32::from_rgb(1, 2, 3) }\n",
    )
    .unwrap();

    let output = Command::new(shell)
        .current_dir(repo_root())
        .arg("scripts/check-ui-tokens.sh")
        .arg(&relative)
        .output()
        .unwrap();
    let failed = !output.status.success();
    std::fs::remove_dir_all(&dir).ok();

    assert!(
        failed,
        "checker accepted a colour literal:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn t_sk01_12_committed_views_have_no_token_literals() {
    let Some(shell) = shell() else {
        eprintln!("skipping: no bash or sh on PATH");
        return;
    };
    let output = Command::new(shell)
        .current_dir(repo_root())
        .arg("scripts/check-ui-tokens.sh")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "check-ui-tokens failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
