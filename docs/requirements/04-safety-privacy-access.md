# Requirements: Safety, privacy and access

Derived from SRS 1.2. Wording is condensed and made testable. Numeric values the SRS does not give are marked TBD with an open question (OQ-xx). Nothing here invents an answer.

Status values in the traceability file: Not started, In progress, Implemented, Verified.

## FR-036 Audit and warnings

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-036 (rev 1.2)
- Implementing slice: S10 (see ../plan/)
- Business rules: BR-14, BR-02, BR-05
- Open questions: OQ-20
- Acceptance tests: AT-09

**Statement.** Warnings and errors with remediation text. Critical errors block export. Recoverable warnings stay visible.

**Acceptance criteria**

- AC-FR-036-1: Each warning has a code, severity, location and remediation text.
- AC-FR-036-2: Categories include: missing measurement, invalid contour, excessive offset curvature, impossible grading change, insufficient fabric width, unknown formula source, chart-derived or estimated value, interpretation or extension rule in use, unverified rule, small-waist rule fired, wide deviation from chart, grading outside chart range, implausible value for its unit.
- AC-FR-036-3: Critical errors block export. Recoverable warnings stay visible and are recorded in export metadata.
- AC-FR-036-4: Numeric thresholds such as excessive curvature are TBD (OQ-20).

**Planned tests**

- T-FR-036-01 [UNIT] Each warning category raised by its trigger
- T-FR-036-02 [INTEG] Critical error blocks every exporter
- T-FR-036-03 [UNIT] Remediation text present for every code

## FR-037 Measurement and pattern privacy

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-037
- Implementing slice: S17 (see ../plan/)
- Business rules: BR-15
- Open questions: OQ-04
- Acceptance tests: none named in the SRS

**Statement.** Offline by default with no telemetry. Delete and export controls for customer measurements.

**Acceptance criteria**

- AC-FR-037-1: Drafting, saving and exporting work with the network blocked.
- AC-FR-037-2: No telemetry or cloud transmission exists.
- AC-FR-037-3: A user can delete a customer profile and export it. Deletion semantics for project files that embed it are defined by OQ-04.
- AC-FR-037-4: No measurement data leaves the device without an explicit user action.

**Planned tests**

- T-FR-037-01 [SECURITY] Full workflow with network blocked
- T-FR-037-02 [SECURITY] Network trace shows no outbound traffic
- T-FR-037-03 [E2E] Delete and export a profile

## FR-039 Keyboard and accessibility workflow

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-039
- Implementing slice: S14 (see ../plan/)
- Business rules: none specific
- Open questions: OQ-09
- Acceptance tests: none named in the SRS

**Statement.** Labelled fields, keyboard zoom and pan, error focus, contrast themes, AccessKit semantics, and a numeric representation of critical dimensions.

**Acceptance criteria**

- AC-FR-039-1: Every data field has an accessible label.
- AC-FR-039-2: Zoom and pan have keyboard equivalents.
- AC-FR-039-3: An error moves focus to the offending field.
- AC-FR-039-4: Contrast themes exist.
- AC-FR-039-5: Critical dimensions are available as text outside the canvas.
- AC-FR-039-6: The key workflow is usable without a pointer.
- AC-FR-039-7: Status announcements are readable through accessibility APIs.

**Planned tests**

- T-FR-039-01 [E2E] Keyboard-only run: enter measurements, generate, export
- T-FR-039-02 [UNIT] Semantic tree contains labelled fields and status
- T-FR-039-03 [MANUAL] MT-02 screen reader and keyboard review
