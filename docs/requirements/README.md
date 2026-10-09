# Requirements and traceability

This folder splits SRS 1.2 into 44 uniquely identified requirements (36 for R1, 8 for R2) with 155 acceptance criteria and 131 planned tests.

## Files

| File | Content |
|---|---|
| 01-inputs-rules-charts.md | Inputs, rules and charts |
| 02-geometry-construction.md | Geometry and construction |
| 03-export-storage.md | Export and storage |
| 04-safety-privacy-access.md | Safety, privacy and access |
| 05-units.md | Units |
| 06-r2-editing-grading-fabric.md | R2: editing, grading, fabric |
| 07-render-and-nonfunctional.md | Render and non-functional |
| business-rules.md | Cross-cutting business rules BR-01 to BR-22 |
| open-questions.md | Unresolved ambiguities OQ-01 to OQ-34 |
| acceptance-tests.md | AT-01 to AT-21 mapped to requirements and slices |
| traceability.csv | One row per requirement: slice, tests, rules, open questions, status |

## Identifier scheme

- Requirement: the SRS ID (FR-019, REN-01, NFR-014).
- Acceptance criterion: AC-<requirement>-<n>.
- Planned test: T-<requirement>-<nn>. Levels: UNIT, PROP, GOLDEN, INTEG, CONTRACT, E2E, SECURITY, PERF, FAULT, MIGRATION, XTARGET, STATIC, MANUAL.
- Business rule: BR-nn. Open question: OQ-nn. Slice: Snn, spike SP-nn, domain track DOM-nn.

## Rules for changing this folder

1. A requirement text changes only when the SRS changes. Update the SRS first, then this folder, and bump the SRS revision note.
2. A new acceptance criterion needs a planned test. A new test needs a requirement.
3. Never replace TBD with a number unless the matching open question has a recorded decision.
4. Status in traceability.csv changes to Verified only with the evidence required by docs/testing/evidence-and-dod.md.

## Known gap

SRS 1.0 requirements FR-001 to FR-018 and NFR-001 to NFR-013 were not available when this register was written (OQ-18). The register is incomplete until they are added in the same format.
