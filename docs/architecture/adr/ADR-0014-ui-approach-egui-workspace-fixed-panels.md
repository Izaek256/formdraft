# ADR-0014: UI approach: egui workspace with fixed panels and design tokens

- Status: Accepted
- Date: 2026-10-11
- Source: FR-044, FR-039; docs/design/ui-spec.md
- Requirements: FR-044; FR-039; FR-038; FR-036; NFR-017

## Context

The interface must feel like professional vector and design software: a canvas surrounded by tools and panels, with rulers, precise numbers, fast zoom and pan, and keyboard routes (docs/design/ui-spec.md). The SRS already names eframe/egui 0.36 as the UI stack. Two questions were open: whether to add a docking crate and how to keep colours, sizes and icons consistent and testable.

Docking crates lag the pinned egui release (egui_dock 0.18 documents egui 0.33 while the SRS pins 0.36), so adopting one now risks a version conflict and an accessibility gap.

## Decision

1. Build the workspace with egui's own menu bar and side, top and bottom panels. No docking or workspace crate in R1. Panels are fixed and hidden or shown as a group (OQ-36 decided 2026-10-11).
2. Put eframe and egui in pattern_ui only, at the SRS-pinned 0.36 family, default features. No egui_extras, no image or font crate, for R1. Icons are drawn in code with egui's painter.
3. Hold every colour, size, spacing, stroke width and icon in a theme module with three palettes: dark (default), light and high contrast. No view hard-codes a token (OQ-38 decided 2026-10-11). scripts/check-ui-tokens.sh enforces this.
4. Use a model and view split in every region: a pure, unit-tested model holds state and logic; the view only draws. No pattern dimension is computed in a view (ADR-0005).
5. Fonts are egui's defaults for R1 (OQ-40 decided 2026-10-11). Shortcuts follow the ui-spec scheme; only shortcuts for features that exist are bound (OQ-39 decided 2026-10-11).

## Consequences

The workspace builds from the pinned egui family with no extra dependency. Reconsidering movable panels needs a spike (SP-04) against the pinned version first. Contrast targets are testable on the palettes. Actual size on screen is approximate at 96 pixels per inch and is always labelled approximate (OQ-37 decided 2026-10-11).

## Alternatives considered

- A docking crate now: rejected, version and accessibility risk before SP-04.
- egui_extras SVG icons: rejected for R1; painter-drawn icons avoid a dependency and any third-party asset licence. Revisit when richer icons are needed.
- Scattered styling constants: rejected, untestable and impossible to theme consistently.

## Revisit when

SP-04 reports on a docking crate, or the project needs external SVG icons or non-default fonts.
