# Requirements: Export and storage

Derived from SRS 1.2. Wording is condensed and made testable. Numeric values the SRS does not give are marked TBD with an open question (OQ-xx). Nothing here invents an answer.

Status values in the traceability file: Not started, In progress, Implemented, Verified.

## FR-032 Printable tiled PDF

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-032
- Implementing slice: S13 (see ../plan/)
- Business rules: BR-14, BR-01
- Open questions: OQ-05, OQ-26, OQ-31
- Acceptance tests: AT-07

**Statement.** Tiled PDF on A4 and Letter in physical millimetre coordinates with overlap, page numbers, registration targets, a 100 mm calibration square and a 100% print instruction.

**Acceptance criteria**

- AC-FR-032-1: A4 and Letter are supported.
- AC-FR-032-2: Page content is in physical millimetre coordinates.
- AC-FR-032-3: Pages have tile overlap, page numbers, registration targets and a 100 mm calibration square. A 4 in square is added when an imperial display unit is selected.
- AC-FR-032-4: The document instructs printing at 100% or actual size.
- AC-FR-032-5: The union of tiles covers every piece edge. No edge is missing (automated).
- AC-FR-032-6: A printed calibration square measures 100 mm within the printer tolerance (manual, tolerance OQ-05).

**Planned tests**

- T-FR-032-01 [UNIT] Tile coverage: union of tiles contains all piece geometry
- T-FR-032-02 [UNIT] Overlap and registration marks consistent between neighbours
- T-FR-032-03 [INTEG] Parsed PDF: page size, square size 100 mm within 0.1 mm, page numbers
- T-FR-032-04 [INTEG] A4 and Letter variants
- T-FR-032-05 [MANUAL] MT-01 print and measure the calibration square

**Note.** Library choice pending SP-01.

## FR-033 SVG vector export

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-033 (rev 1.2)
- Implementing slice: S12 (see ../plan/)
- Business rules: BR-12, BR-13
- Open questions: none
- Acceptance tests: AT-05

**Statement.** Dimensioned SVG in physical units with layer and group IDs and metadata, with no rasterised boundaries.

**Acceptance criteria**

- AC-FR-033-1: Physical units and a predictable coordinate origin.
- AC-FR-033-2: Layer and group IDs for stitch, cut and marks, and one group per piece.
- AC-FR-033-3: Machine-readable metadata: method ID, rulebook version, edition pin, display unit, toile statement, warnings.
- AC-FR-033-4: Geometry is written in millimetre-based physical coordinates whatever the display unit.
- AC-FR-033-5: An automated parser validates bounding boxes, paths and unit metadata (AT-05).

**Planned tests**

- T-FR-033-01 [INTEG] Parse exported SVG: bounding boxes, paths, units, metadata
- T-FR-033-02 [INTEG] Path length within 0.1 mm of core length
- T-FR-033-03 [INTEG] Export bytes unchanged when render mode changes (REN-02)

## FR-034 DXF interchange

- Release and priority: R2 / Should
- SRS reference: SRS 7 FR-034 (rev 1.2)
- Implementing slice: S21 (see ../plan/)
- Business rules: BR-12
- Open questions: OQ-07
- Acceptance tests: AT-11

**Statement.** DXF with named layers for stitch, cut, annotations, grainline and notches, a documented dialect, and unsupported-construct reporting.

**Acceptance criteria**

- AC-FR-034-1: Named layers: stitch, cut, annotations, grainline, notches.
- AC-FR-034-2: Dialect and units documented. The units header is millimetres.
- AC-FR-034-3: Verified in at least two external CAD viewers (OQ-07).
- AC-FR-034-4: Round-trip measurement and entity tests pass.
- AC-FR-034-5: An unsupported construct is reported and never silently dropped (AT-11).

**Planned tests**

- T-FR-034-01 [INTEG] Round-trip entity and measurement tests
- T-FR-034-02 [INTEG] AT-11 unsupported construct reported
- T-FR-034-03 [MANUAL] MT-03 open in two CAD viewers

## FR-035 Offline project package

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-035 (rev 1.2)
- Implementing slice: S11 (see ../plan/)
- Business rules: BR-04, BR-16
- Open questions: OQ-21, OQ-25
- Acceptance tests: AT-06, AT-10

**Statement.** Human-readable, versioned project package with measurements, rule references and optional assets, written atomically.

**Acceptance criteria**

- AC-FR-035-1: Saved data include metadata, versioned geometry and rule references, measurements with source tags and optional assets.
- AC-FR-035-2: The package records method ID, rulebook version, edition pin and the hash of each rule record used.
- AC-FR-035-3: Native saves are atomic. A crash during save leaves the last valid version intact.
- AC-FR-035-4: Browser saves produce one complete download. Atomic semantics are defined in OQ-25.
- AC-FR-035-5: Reopening after restart reconstructs the same pattern (AT-06).
- AC-FR-035-6: A rule-hash or edition mismatch is reported and never silently regenerated.

**Planned tests**

- T-FR-035-01 [INTEG] Save, restart, load gives identical pattern checksum
- T-FR-035-02 [FAULT] Failure injected at every write step keeps the last valid file
- T-FR-035-03 [CONTRACT] Schema golden files for each schema version
- T-FR-035-04 [UNIT] Hash mismatch reported
- T-FR-035-05 [MIGRATION] Older schema versions migrate to current
