# UI specification

Status: Proposed. It becomes binding when the lead developer accepts ADR-0014 and adds FR-044 (section 9) to the SRS. SK-01 builds the first version of this workspace around a placeholder shape.

## 1. Intent

The interface should feel like professional vector and design software: a canvas surrounded by tools and panels, with rulers, precise numbers, fast zoom and pan, and keyboard shortcuts. This is a pattern-drafting tool, so the numbers matter more than the drawing, and every length is shown through the unit service.

What "resemble" means here and what it does not:

- It means layout, canvas behaviour, panel roles and shortcut habits that are common to editors such as Illustrator, Inkscape, Affinity and Figma.
- It does not mean copying any product's icons, names, colour palette or exact look. The icons are drawn in this project. No Adobe assets or trade dress are used.
- It does not mean matching a mature product's feature set. Editing tools are R2 (FR-025).

## 2. Layout

```
+--------------------------------------------------------------------------------+
| File  Edit  View  Pattern  Export  Help             project name      [theme]   |  menu bar
+-----+--------------------------------------------------------+------------------+
| [V] | Control bar: display unit | zoom % | fit | tile preview  | PROPERTIES       |
| [H] +--+-----------------------------------------------------+  measurements    |
| [Z] |  | 0       10        20        30        40  (rulers)    |  source tags     |
|     |  +-----------------------------------------------------+  adaptation      |
|     |R |                                                      |  allowances      |
|     |u |             canvas                                   +------------------+
|     |l |       pieces: stitch dashed, cut solid, marks        | PIECES           |
|     |e |                                                      |  Back   [vis]    |
|     |r |                                                      |  Front  [vis]    |
|     |  |                                                      +------------------+
|     |  |                                                      | WARNINGS         |
|     |  |                                                      | PROVENANCE       |
+-----+--+------------------------------------------------------+------------------+
| zoom 100% (approx) | cursor x, y in display unit | selection | limits notice | mode |  status bar
+--------------------------------------------------------------------------------+
```

## 3. Regions

| Region | Content | Release | Slice |
|---|---|---|---|
| Menu bar | File (new, open, save, save as, recent, export), Edit (undo and redo if OQ-29 says R1), View (zoom, fit, rulers, grid, themes), Pattern (method, adaptation, regenerate), Help (shortcuts, about, method and edition notice) | R1 | SK-01, S14e |
| Tool strip | Select (inspect pieces and points), Hand, Zoom. R2 adds Direct select, Measure, Notch and mark tools | R1 (three tools) | SK-01 |
| Control bar | Display unit per quantity class, zoom percentage, fit, tile preview toggle, tool options | R1 | SK-01 |
| Canvas | Rulers, grid, pieces, selection, tile outlines when previewing the PDF | R1 | SK-01, S14d |
| Properties panel | Four measurements with unit suffix entry, source tags, adaptation presets, swing and flare, per-edge allowances, style number | R1 | SK-01, S14c |
| Pieces panel | Piece list, visibility per piece and per role (stitch, cut, marks, labels), numeric dimensions | R1 | SK-01 (placeholder only), S14d |
| Warnings panel | Errors and warnings with code, location, remediation text. Clicking one selects the cause | R1 | SK-01, S14c |
| Provenance panel | For the selected point: rule IDs, kind (B, I, X), status, page, measurements used | R1 | SK-01, S14c |
| Status bar | Zoom, cursor coordinates, selection, limits-not-configured notice, production or developer mode | R1 | SK-01 |
| Export dialog | Page size, units, overlap, tile preview, toile statement, blockers | R1 | S14e |

## 4. Canvas behaviour

- Rulers on top and left show tick labels through the unit service in the display unit. Tick spacing adapts to zoom. The cursor position is marked on both rulers and shown in the status bar.
- Zoom to cursor with the wheel. Space plus drag, the Hand tool and middle-button drag pan. Fit and zoom percentage are always available.
- Grid spacing follows the display unit (for example 1 cm, 1 in) and adapts with zoom.
- Stitch lines are dashed and cut lines are solid, so meaning never depends on colour alone. Marks and labels use their own stroke and text styles.
- Actual size on screen can only be approximate, because monitors differ. It is labelled approximate. The reliable check is the 100 mm calibration square in the printed PDF (OQ-37).
- The canvas shows geometry from the core. It never computes it. Tessellation tolerance is derived from pixels per millimetre (S02).
- Snapping, guides and manual point editing are R2.

## 5. Themes and accessibility

- Dark theme by default (OQ-38), light theme, and a high contrast theme (FR-039).
- Proposed contrast targets: 4.5 to 1 for text and 3 to 1 for controls and focus rings, checked by a unit test on each palette.
- Every colour, size, spacing value and icon comes from the theme module. No view hard-codes them.
- Tab is for keyboard focus order and is never used to hide panels. A different shortcut hides panels.
- All fields and tools have accessible labels. Status changes are announced. Critical dimensions also appear as text outside the canvas.

## 6. Shortcut scheme (proposed, OQ-39)

| Action | Shortcut |
|---|---|
| Select, Hand, Zoom tools | V, H, Z |
| Temporary pan | Hold Space and drag |
| Zoom in and out | Ctrl and plus, Ctrl and minus. Plus and minus when the canvas has focus |
| Fit to window | Ctrl 0 |
| Approximate actual size | Ctrl 1 |
| Pan | Arrow keys when the canvas has focus |
| New, open, save, save as | Ctrl N, Ctrl O, Ctrl S, Ctrl Shift S |
| Export | Ctrl E |
| Undo and redo | Ctrl Z, Ctrl Shift Z (only if undo is in R1, OQ-29) |
| Toggle rulers, grid | Ctrl R, Ctrl apostrophe |
| Hide or show panels | Ctrl Shift H |
| Shortcut list | F1 |

## 7. Implementation approach

- egui's own menu bar, side panels, top and bottom panels. No extra dependency for the workspace.
- A theme module with design tokens, three palettes and the project's own SVG icons, loaded with the SVG support already named in the SRS.
- A model and view split in every panel, as in SK-01 and every later panel: the model holds state and logic and is unit tested, the view only draws.
- Movable and dockable panels are not in R1. Spike SP-04 tests a docking crate against the pinned egui version first, because docking crates lag egui releases (egui_dock 0.18 documents egui 0.33, the SRS pins 0.36).
- Performance: regeneration within the NFR-017 budget, and never on the paint path.

## 8. What is deliberately later

| Item | Where |
|---|---|
| Direct selection, point editing, snapping, guides, measure tool | R2, FR-025 |
| Dockable and saved workspaces | After SP-04, OQ-36 |
| Texture and fabric preview panels | R2, FR-029 and FR-030 |
| Multiple documents in tabs | Not planned |
| Calibrated actual size | OQ-37 |

## 9. Proposed SRS addition

Add this to the SRS on a doc branch before building beyond SK-01, then add it to docs/requirements.

FR-044: Professional workspace (proposed)

Requirement: The application provides a workspace with a menu bar, tool strip, control bar, canvas with rulers, properties panel, pieces panel, warnings panel, provenance panel and status bar. Rulers, grid, cursor read-out and dimensions use the display unit through the unit service. Navigation includes zoom to cursor, hand and keyboard pan, fit and an approximate actual size. Three themes are provided. All commands have keyboard routes. Stitch and cut lines are distinguishable without colour. The interface never computes geometry.

Acceptance: A reviewer can complete WF-01 using only the workspace. Rulers and read-outs change units without changing geometry. Each theme meets the agreed contrast targets. No view contains a hard-coded colour or a direct length format.

Priority: Should | Target: R1

## 10. Open questions raised

OQ-36 movable panels. OQ-37 actual size on screen. OQ-38 contrast targets and default theme. OQ-39 shortcut scheme. OQ-40 fonts and icon licences.
