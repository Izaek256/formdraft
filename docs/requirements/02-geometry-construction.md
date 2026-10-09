# Requirements: Geometry and construction

Derived from SRS 1.2. Wording is condensed and made testable. Numeric values the SRS does not give are marked TBD with an open question (OQ-xx). Nothing here invents an answer.

Status values in the traceability file: Not started, In progress, Implemented, Verified.

## FR-022 Stitch versus cut geometry

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-022 (rev 1.2)
- Implementing slice: S08 (see ../plan/)
- Business rules: BR-06, BR-07
- Open questions: OQ-14, OQ-19, OQ-20
- Acceptance tests: AT-04, AT-09

**Statement.** Net stitch boundary stored separately from the cutting boundary. Per-edge allowance, corner policy, concave and convex curves, and self-intersection handling.

**Acceptance criteria**

- AC-FR-022-1: Each piece holds a net stitch path and a cut path as separate objects.
- AC-FR-022-2: Allowance is per edge. Proposed defaults are side seams 15 mm and hem 30 mm (OQ-14). No allowance is added on a fold line.
- AC-FR-022-3: Changing an allowance changes the cut path only. The stitch path is identical before and after (AT-04).
- AC-FR-022-4: A documented corner policy covers convex corners, concave corners and curves (OQ-19).
- AC-FR-022-5: Offset self-intersections are detected. Severe cases block export (AT-09). The meaning of severe is TBD (OQ-20).
- AC-FR-022-6: Derived from NFR-015: the cut path lies at the stated allowance from the stitch path within 0.1 mm for digital geometry.

**Planned tests**

- T-FR-022-01 [PROP] Offset distance holds on generated convex and concave polygons
- T-FR-022-02 [UNIT] Per-edge allowance and fold-line exemption
- T-FR-022-03 [GOLDEN] Reference offset cases: concave notch, rounded corner, tight curve
- T-FR-022-04 [UNIT] Self-intersection detection
- T-FR-022-05 [INTEG] Severe self-intersection blocks export and writes nothing

## FR-023 Dart operations

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-023 (rev 1.2)
- Implementing slice: S06 (see ../plan/)
- Business rules: BR-10
- Open questions: OQ-13, OQ-27
- Acceptance tests: AT-14

**Statement.** Darts are first-class objects with legs, apex, intake and fold direction. They recompute when inputs change. A conditional small-waist rule widens them.

**Acceptance criteria**

- AC-FR-023-1: Each dart exposes legs, apex, intake and fold direction.
- AC-FR-023-2: The back has two darts and the front has one.
- AC-FR-023-3: Darts recompute when a measurement or style setting changes.
- AC-FR-023-4: The small-waist rule applies when its trigger is true and shows a visible notice (AT-14). The trigger is TBD (OQ-13).
- AC-FR-023-5: Dart labels stay valid after every recompute.

**Planned tests**

- T-FR-023-01 [GOLDEN] Dart geometry against reference drafts
- T-FR-023-02 [UNIT] AT-14 trigger boundary: just inside and just beyond
- T-FR-023-03 [UNIT] Labels valid after recompute
- T-FR-023-04 [PROP] Recompute is stable and repeatable

## FR-024 Construction marks

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-024 (rev 1.2)
- Implementing slice: S09 (see ../plan/)
- Business rules: BR-20
- Open questions: OQ-12, OQ-27
- Acceptance tests: AT-01

**Statement.** Marks the method requires on a pattern, at correct dimensions on every exported piece.

**Acceptance criteria**

- AC-FR-024-1: Each piece carries piece name, centre back or centre front line, cut quantity, fold indicator, balance marks, seam allowance marking, dart construction lines, grainline, size and style number.
- AC-FR-024-2: Method ID, rulebook version, edition pin and orientation are also present. Drill points appear where appropriate.
- AC-FR-024-3: Notch and grainline positions come from Verified interpretation rules (OQ-12).
- AC-FR-024-4: Grainlines are placed before a pattern is divided into sections.
- AC-FR-024-5: Dimension labels use the project display unit. Millimetre values are in metadata.

**Planned tests**

- T-FR-024-01 [UNIT] Every required mark present per piece
- T-FR-024-02 [GOLDEN] Mark positions against the verified reference
- T-FR-024-03 [INTEG] Exported SVG parsed: all marks present at correct coordinates

## FR-026 Curve construction

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-026 (rev 1.2)
- Implementing slice: S02 (see ../plan/)
- Business rules: BR-20
- Open questions: OQ-11
- Acceptance tests: none named in the SRS

**Statement.** Lines and cubic Béziers with continuity inspection and adaptive tessellation independent of zoom. Qualitative curves need numeric interpretations.

**Acceptance criteria**

- AC-FR-026-1: Geometry is lines and cubic Béziers. A curve is never held as raster pixels.
- AC-FR-026-2: An API reports continuity at joins (position and tangent).
- AC-FR-026-3: Adaptive tessellation meets a documented maximum deviation independent of zoom level.
- AC-FR-026-4: Exported paths deviate from the core curve by no more than 0.1 mm (NFR-015).
- AC-FR-026-5: Each qualitative curve in the block has an interpretation rule record with a verifier (OQ-11).

**Planned tests**

- T-FR-026-01 [UNIT] Join tangent and position reported correctly
- T-FR-026-02 [PROP] Tessellation error within tolerance at low and high zoom
- T-FR-026-03 [GOLDEN] Reference curves
- T-FR-026-04 [INTEG] Export fidelity against core curve
