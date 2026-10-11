# Brief SK-01: Workspace skeleton (professional layout, placeholder shape)

This replaces the earlier SK-01 brief. It merges the walking skeleton with the workspace design in docs/design/ui-spec.md, so the first thing you see is the real application layout, with a clearly labelled placeholder where the pattern will go.

Part 1 is for you and must be done before the agent starts. Part 2 is the text to paste into OpenCode. Part 3 is how you check it, and it includes running the window yourself.

Assumes S01, S02 and S03 are merged into develop and develop is green.

## What this slice is, and what it costs

You get: a dark theme workspace with menu bar, tool strip, control bar, rulers, grid, zoom and pan, form, validation and warnings panels, and a status bar. The canvas draws a labelled placeholder rectangle. It contains no drafting knowledge and satisfies no requirement by itself.

It is much larger than the original SK-01. To keep it safe with a free model, the agent works in three stages inside one branch, commits after each stage, and runs the full verification after each. If it runs out of room, you still have the finished stages. Everything is built in its final location. Only the placeholder module is throwaway.

It absorbs the old slices S14a (theme and shell) and S14b (canvas and rulers), and the form and validation list part of S14c.

## Part 1. Before you send it (you)

Do these on a branch `doc/sk01-decisions` from develop, merge it, then start SK-01 from the updated develop.

1. Accept ADR-0014 (set its status to Accepted).
2. Add FR-044 to the SRS (text is in docs/design/ui-spec.md section 9) and then to docs/requirements. The register rule is SRS first. If you choose to build ahead of the SRS edit, write that decision down.
3. Record the decisions below in docs/requirements/open-questions.md and record the decision on OQ-36 to OQ-40.
4. Add the new slice text from the update zip (docs/plan/slices-r1.md) and the new design file.

| # | Decision | Proposal (accept or edit) |
|---|---|---|
| D1 | Dependencies | eframe and egui in pattern_ui only, at the version S00 pinned. Default features. No egui_extras, no docking crate, no image or font crates. Icons are drawn in code with egui's painter. Fonts are egui's defaults (OQ-40 decided: defaults for now) |
| D2 | OQ-36 panels | Fixed panels for R1. Docking waits for spike SP-04 |
| D3 | OQ-37 actual size | Approximate, labelled so, assuming 96 pixels per inch. Never presented as exact |
| D4 | OQ-38 contrast and default | Text 4.5 to 1, controls and focus 3 to 1. Dark theme is the default |
| D5 | OQ-39 shortcuts | The scheme in the spec. Only shortcuts for features that exist now are implemented |
| D6 | Structure | Final locations: theme/, shell/, canvas/, panels/. Throwaway only: placeholder/. Each region has a pure model with unit tests and a thin view |
| D7 | Staging | Stage A theme and shell. Stage B canvas and navigation. Stage C properties, validation and placeholder. One commit per stage. Full verification after each |
| D8 | Menu contents | Only working items: File (Quit), View (zoom, fit, actual size, rulers, grid, themes, hide panels), Help (shortcut list, about with method and edition notice). Other menus appear when their features exist |
| D9 | Ruler ticks | Steps of 1, 2 or 5 times a power of ten in the display unit, for every unit including inches. Fractional inch ticks come later |
| D10 | Demo data | Fixed demo profile: id sk01-demo, label Demo profile, versions sk01, updated_at 0. Placeholder rectangle has its top-left corner at world origin |
| D11 | New check | scripts/check-ui-tokens.sh fails when a colour literal or hard-coded size appears in a view outside the theme module |

## Part 2. Brief to paste

```
Read AGENTS.md first, including Branching and merging. Start from origin/develop, never from another branch.
Slice: SK-01 Workspace skeleton (docs/plan/slices-r1.md). Design: docs/design/ui-spec.md. ADR-0014.
Branch: create feat/sk01-workspace-skeleton from origin/develop. Open a PR into develop. Do not merge.

Read also: docs/requirements/05-units.md (FR-038), FR-039 in docs/requirements/04-safety-privacy-access.md,
docs/architecture/01-modules-and-boundaries.md (pattern_ui row and dependency table), ADR-0005, ADR-0007,
ADR-0014, and the S01, S02 and S03 code you will call: pattern_core::units, ::geom, ::measure, and the
thresholds loader in pattern_templates.

Goal: a native egui application in pattern_ui with the professional workspace layout from the ui-spec, driving the
real units, validation and geometry code, drawing one clearly labelled placeholder rectangle.

Work in three stages. Commit after each stage with the message prefix "feat(SK-01) stage A:", "stage B:", "stage C:".
After each stage run the full verification commands and fix failures before starting the next. If you are running
out of room, stop at a stage boundary, report what is done, and do not start a half stage.

STAGE A: theme and shell.
- Theme module with design tokens (colours, sizes, spacing, stroke widths), three palettes (dark default, light, high
  contrast), switchable live from the View menu. Every colour, size and icon used by any view comes from tokens.
- Regions: menu bar, left tool strip with Select, Hand and Zoom (icons drawn with egui's painter), control bar,
  right panel column (Properties, Pieces, Warnings, stacked), bottom status bar, central canvas area.
- Menu items only for things that work now (see decisions). Ctrl+Shift+H hides and shows the panels. F1 opens a
  shortcut list window. Tab is never a shortcut.
- scripts/check-ui-tokens.sh, with proof it fails on a planted violation.

STAGE B: canvas and navigation.
- Rulers on top and left show ticks and labels through the unit service in the display unit. Tick steps are 1, 2 or 5
  times a power of ten in that unit. Spacing adapts to zoom. The cursor position is marked on both rulers and shown in
  the status bar in the display unit.
- Grid with spacing following the same steps. Toggles for rulers and grid.
- Zoom to cursor with the wheel. Space plus drag, the Hand tool and middle-button drag pan. Fit (Ctrl+0), zoom percentage,
  approximate actual size (Ctrl+1, 96 pixels per inch, labelled approximate), Ctrl plus and Ctrl minus, plus and minus
  and arrow keys when the canvas has focus. V, H and Z choose tools.
- Role styles function in canvas/roles.rs: stitch is dashed, cut is solid, marks and labels have their own styles, and
  placeholder has its own. Differences are by dash pattern and weight, not colour alone. Use the S02 tessellation with
  tolerance derived from pixels per millimetre.
- Select tool: clicking inside the placeholder selects it, highlights it and shows its size in the status bar.

STAGE C: properties, validation and placeholder.
- Properties panel: four text fields (waist, hips, waist-to-hip, skirt length), a source tag selector for waist-to-hip
  only, a display unit selector for body measurements and one for pattern dimensions (all six units), an inch format
  toggle with denominator selector (2 to 64). Parsing and display use only the S01 unit service. A bare number uses the
  class default unit. A typed suffix overrides it. Invalid text shows the unit service message beside that field.
- The ProfileDraft goes to S03 validation with limits from thresholds.toml (all Unset). Warnings panel lists S03 errors
  and warnings with rule IDs. Show the limits-not-configured notice. Metadata: id sk01-demo, label Demo profile,
  versions sk01, updated_at 0.
- Pieces panel lists one entry, Placeholder rectangle, with a visibility toggle.
- Placeholder module: one rectangle, hips wide and skirt length high, in mm, y down, top-left at world origin, scaled to
  fit. Straight lines only. If any field is missing or invalid, draw nothing and name the field. A text panel shows the
  rectangle's width and height in the pattern dimension unit and in millimetres.
- Always show: "PLACEHOLDER SHAPE, NOT A PATTERN. Dimensions are your inputs only."

Decisions already made: eframe and egui in pattern_ui only, at S00's pinned versions, default features, nothing else.
Native only. Do not use std::time::SystemTime. Fixed panels, no docking. Fonts are egui's defaults. Actual size is
approximate. Contrast targets: text 4.5 to 1, controls and focus 3 to 1. No drafting numbers anywhere: no size, ease,
dart, allowance or chart value from the book or from memory. Nothing copied from any other product, including icons.
main.rs starts the window and returns or prints an error without unwrap, expect or panic.

Write tests first (IDs are local to this slice, T-SK01-nn):
01 rectangle in mm equals hips by skirt length for the same input in different units; 02 switching display units
changes strings and never the rectangle; 03 clearing each of the four fields disables drawing and names that field;
04 typed suffix overrides the default unit, bare number uses the class default; 05 invalid text shows the unit service
message for that field only; 06 tag rules shown and enforced (chart-derived rejected on waist, accepted on
waist-to-hip); 07 Unset limits show the notice and invent nothing; 08 transform screen to mm and back is stable at many
zooms and pans, y down; 09 arbitrary text input never panics the model (seeded property test); 10 keyboard actions
zoom, fit and pan as described; 11 each of the three palettes meets the contrast targets; 12 check-ui-tokens.sh fails
on a planted violation and passes on the clean tree; 13 ruler ticks are multiples of the step in every unit at several
zooms, labels come from the unit service; 14 zoom to cursor keeps the point under the cursor fixed; 15 every documented
shortcut maps to exactly one action and Tab is unassigned; 16 stitch and cut styles differ by dash pattern; 17 hiding
and showing panels does not change any model; 18 select tool: click inside selects and shows size, click outside clears.

Protect: INV-02, INV-15. No unwrap, expect or panic in pattern_ui. No unsafe. No length formatted outside the unit
service (run scripts/check-no-raw-length-format.sh and report). No colour or size literals outside the theme module
(run scripts/check-ui-tokens.sh and report). Public items documented. Pure models carry the logic. Views only draw.

Stop and ask if: a decision above conflicts with a document, eframe or egui will not build at the pinned version, you
need another dependency, you need logic that belongs in a core crate, you find a bug in S01 to S03, a stage cannot be
finished, or a test can only pass by inventing a value.

You cannot see the window. Say so. In the PR body include the completion report, how to run it on Windows and on
Linux, what you could not verify visually, the dependency approval line for eframe and egui (purpose, licence,
maintenance, size effect), the three stage commits, the planted-violation proofs for both scripts, and the output of
the verification commands in docs/plan/slices-r1.md (SK-01) and AGENTS.md.
```

## Part 3. How to review (you run it)

Read the diff, then run the window yourself.

1. `cargo test -p pattern_ui` passes. All eighteen SK01 tests exist. Check the "test result" counts add up.
2. Three stage commits exist and each one built and passed on its own: `git log --oneline` and, if you like, check out each.
3. `git diff origin/develop --stat`: changes only under `crates/pattern_ui`, `scripts`, and manifests. Core crates untouched, or a reported bug.
4. `cargo tree -p pattern_core` shows no eframe or egui. `cargo check -p pattern_core --target wasm32-unknown-unknown` still passes.
5. Run `scripts/check-ui-tokens.sh` and `scripts/check-no-raw-length-format.sh` yourself.
6. Run `cargo run -p pattern_ui` (Windows PowerShell if WSL cannot open the window).

Manual checks:

| # | Try | Expect |
|---|---|---|
| 1 | Look at the whole window | Menu bar, tool strip, control bar, rulers, canvas, three right panels, status bar. Dark theme |
| 2 | Switch the three themes in View | Everything changes together, text stays readable in each |
| 3 | Type hips 94, waist 70, waist-to-hip 20, length 60 (cm default) | A rectangle appears, labelled placeholder. The text panel shows width and height |
| 4 | Change the pattern dimension unit through all six units | Rulers, labels and text change. The rectangle does not |
| 5 | Wheel zoom with the cursor over a corner of the rectangle | That corner stays under the cursor. Rulers re-tick sensibly |
| 6 | Space and drag, then H tool and drag | Both pan |
| 7 | Ctrl 0, then Ctrl 1 | Fit, then approximate actual size labelled approximate |
| 8 | Click the rectangle, then empty canvas | Selected with size in the status bar, then cleared |
| 9 | Type 37 3/8 in, then 1/3 in, then 94,5 cm, then 0 | Clear, plain messages next to the field. Zero is an error |
| 10 | Clear the skirt length | Drawing stops. The message names skirt length |
| 11 | Press Tab through the window | Focus visits every field in reading order and never traps |
| 12 | Ctrl Shift H, F1 | Panels hide and show. Shortcut list opens |
| 13 | Stare at the status bar and notices | Limits-not-configured notice and the placeholder label always visible |

Then squash merge into develop. Do checkpoint CP2: fill the "CP2 after SK-01" column of review-log.md for FR-038, FR-039, FR-019, FR-036 and NFR-017. Also write down in plain words what does and does not feel like the professional software you have in mind. That list is the input for tuning the spec before S14c.
