# Requirements: R2: editing, grading, fabric

Derived from SRS 1.2. Wording is condensed and made testable. Numeric values the SRS does not give are marked TBD with an open question (OQ-xx). Nothing here invents an answer.

Status values in the traceability file: Not started, In progress, Implemented, Verified.

## FR-025 Manual drafting tools

- Release and priority: R2 / Should
- SRS reference: SRS 7 FR-025
- Implementing slice: S22 (see ../plan/)
- Business rules: BR-05, BR-08
- Open questions: OQ-29
- Acceptance tests: none named in the SRS

**Statement.** Point selection, constrained moves, rulers, snapping, offset, mirror and curve-handle edits, stored as overrides distinct from formula geometry.

**Acceptance criteria**

- AC-FR-025-1: Direct edits are stored as overrides separate from formula geometry.
- AC-FR-025-2: Undo restores the exact preceding geometry.
- AC-FR-025-3: Constraints stay inspectable.
- AC-FR-025-4: Constraint violations warn.
- AC-FR-025-5: Manual overrides are labelled as kind X in provenance.

**Planned tests**

- T-FR-025-01 [UNIT] Undo restores exact geometry
- T-FR-025-02 [UNIT] Override isolation from formula geometry
- T-FR-025-03 [E2E] Edit, save, reload keeps overrides

## FR-027 Size grading

- Release and priority: R2 / Should
- SRS reference: SRS 7 FR-027 (rev 1.2)
- Implementing slice: S20 (see ../plan/)
- Business rules: BR-18
- Open questions: OQ-03, OQ-17
- Acceptance tests: AT-18

**Statement.** Per-landmark grade rules, never uniform scaling, using the method's increment tables.

**Acceptance criteria**

- AC-FR-027-1: A grade rule is a horizontal and vertical increment per landmark per size step from the transcribed tables.
- AC-FR-027-2: Uniform scaling is rejected as a substitute.
- AC-FR-027-3: Grading outside the chart range warns.
- AC-FR-027-4: Graded landmark positions match the approved tables within tolerance (AT-18).

**Planned tests**

- T-FR-027-01 [GOLDEN] Graded landmarks against the approved tables
- T-FR-027-02 [UNIT] Uniform scaling rejected
- T-FR-027-03 [UNIT] Out-of-range warning

**Note.** Blocked on DOM-02: figure-based mapping of numbered points to landmarks.

## FR-028 Pattern comparison

- Release and priority: R2 / Should
- SRS reference: SRS 7 FR-028
- Implementing slice: S23 (see ../plan/)
- Business rules: none specific
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** Overlay two versions or profiles with dimension deltas and highlighted landmark movement.

**Acceptance criteria**

- AC-FR-028-1: Two versions or two profiles can be overlaid.
- AC-FR-028-2: Dimension deltas are shown.
- AC-FR-028-3: The revision summary lists changed input, method and affected pieces.

**Planned tests**

- T-FR-028-01 [UNIT] Delta computation against known pairs
- T-FR-028-02 [E2E] Overlay and summary shown

## FR-029 Material library

- Release and priority: R2 / Should
- SRS reference: SRS 7 FR-029
- Implementing slice: S24 (see ../plan/)
- Business rules: BR-17
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** Fabric metadata stored as informational data, used only where an algorithm supports it.

**Acceptance criteria**

- AC-FR-029-1: Fields: label, width, weave or knit class, stretch direction and percentage when known, shrinkage allowance, nap flag, optional swatch image.
- AC-FR-029-2: Fabric choice never changes body measurements.
- AC-FR-029-3: The user can see which assumptions a fabric implies.

**Planned tests**

- T-FR-029-01 [UNIT] Selecting a fabric leaves measurements unchanged
- T-FR-029-02 [E2E] Assumptions visible in the UI

## FR-030 Material visualization

- Release and priority: R2 / Should
- SRS reference: SRS 7 FR-030
- Implementing slice: S24 (see ../plan/)
- Business rules: BR-17, BR-13
- Open questions: none
- Acceptance tests: AT-05

**Statement.** A 2D swatch or fill preview with scale, orientation and opacity controls. A visual approximation, not drape.

**Acceptance criteria**

- AC-FR-030-1: Scale, orientation and opacity controls exist.
- AC-FR-030-2: PNG, JPEG and SVG load through permitted decoders with bounds checks.
- AC-FR-030-3: The preview is labelled a visual approximation.
- AC-FR-030-4: A printed dimension is unchanged when texture or sampling mode changes (AT-05).

**Planned tests**

- T-FR-030-01 [INTEG] Export bytes unchanged across texture settings
- T-FR-030-02 [SECURITY] Malformed image rejected without panic
- T-FR-030-03 [MANUAL] Label wording review

## FR-031 Fabric layout and consumption

- Release and priority: R2 / Should
- SRS reference: SRS 7 FR-031
- Implementing slice: S25 (see ../plan/)
- Business rules: BR-17
- Open questions: none
- Acceptance tests: AT-12

**Statement.** Arrange pieces within a fabric width respecting grain, fold, quantity, mirrored pairs, nap and rotation. Report length and waste as an estimate.

**Acceptance criteria**

- AC-FR-031-1: All placed pieces stay within width.
- AC-FR-031-2: No overlaps, except explicitly permitted shared folds.
- AC-FR-031-3: Grainline, nap and rotation constraints are respected.
- AC-FR-031-4: Length and waste ratio are reported and labelled as estimates.
- AC-FR-031-5: A narrow fabric gives an explicit failure or a constrained alternative with no overlap (AT-12).

**Planned tests**

- T-FR-031-01 [PROP] No overlap and within width for generated cases
- T-FR-031-02 [UNIT] AT-12 narrow fabric
- T-FR-031-03 [UNIT] Nap and rotation constraints
