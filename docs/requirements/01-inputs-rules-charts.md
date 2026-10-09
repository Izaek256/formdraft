# Requirements: Inputs, rules and charts

Derived from SRS 1.2. Wording is condensed and made testable. Numeric values the SRS does not give are marked TBD with an open question (OQ-xx). Nothing here invents an answer.

Status values in the traceability file: Not started, In progress, Implemented, Verified.

## FR-019 Measurement profiles

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-019 (rev 1.2); 5.3
- Implementing slice: S03 (see ../plan/)
- Business rules: BR-02, BR-03, BR-12
- Open questions: OQ-02, OQ-22
- Acceptance tests: AT-01, AT-03, AT-06

**Statement.** Anonymisable customer measurement profiles with named definitions, units, last-edited time, definition version and provenance. Four required skirt measurements, a source tag per value, rejection of missing, impossible or contradictory data, and no invented measures.

**Acceptance criteria**

- AC-FR-019-1: A skirt profile requires waist, hips, waist-to-hip and skirt length. Bust is optional and used only for chart lookup (FR-042).
- AC-FR-019-2: Every value is stored as an exact canonical millimetre value with a source tag from {measured, chart-derived, estimated}.
- AC-FR-019-3: Only waist-to-hip may carry the tag chart-derived. Any other field with that tag is rejected.
- AC-FR-019-4: A missing mandatory value disables generation and names the exact field (AT-03).
- AC-FR-019-5: A value outside its hard limits, or a combination that breaks a cross-field rule, is rejected with the field name and rule ID. Limits and rules are data from the rulebook, not code constants. Numeric limits are TBD (OQ-02).
- AC-FR-019-6: A profile stores label, measurement_definition_version, protocol_version and updated_at.
- AC-FR-019-7: Saving then loading gives an identical profile, including source tags (AT-06).
- AC-FR-019-8: Anonymisation is not implemented until OQ-22 is answered.

**Planned tests**

- T-FR-019-01 [UNIT] Validation per field and per cross-field rule
- T-FR-019-02 [UNIT] Tag rules: chart-derived accepted only on waist-to-hip
- T-FR-019-03 [UNIT] AT-03: each of the four mandatory fields removed in turn names that field
- T-FR-019-04 [PROP] Any valid profile survives serialise then deserialise unchanged
- T-FR-019-05 [INTEG] Profile persisted and reloaded through the project file (needs S11)

## FR-020 Formula provenance and diagnostics

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-020 (rev 1.2); 5.1; 5.5
- Implementing slice: S04 (see ../plan/)
- Business rules: BR-04, BR-05, BR-22
- Open questions: OQ-01
- Acceptance tests: AT-19

**Statement.** Every template is bound to a method ID, rulebook version and edition pin. Every calculation is a rule record. Provenance is inspectable for any constructed point. Unverified rules cannot reach production export.

**Acceptance criteria**

- AC-FR-020-1: Every template carries method ID, rulebook version and edition pin.
- AC-FR-020-2: Every constant used in generated geometry resolves to a rule record with rule ID, source page, kind and verification status.
- AC-FR-020-3: For any constructed point the API returns the rule IDs, input measurements and dependency chain that produced it.
- AC-FR-020-4: Production mode blocks export when any rule used is not Verified. Developer mode exports with an UNVERIFIED legend on every page (AT-19).
- AC-FR-020-5: Kind X rules are labelled in the UI and in export metadata.
- AC-FR-020-6: Rules from different editions cannot be combined. Loading such a rulebook fails.

**Planned tests**

- T-FR-020-01 [UNIT] Provenance chain returned for sampled construction points
- T-FR-020-02 [CONTRACT] Rulebook completeness: every constant referenced by template code exists in the rulebook (NFR-026)
- T-FR-020-03 [INTEG] Production export blocked when one used rule is not Verified
- T-FR-020-04 [INTEG] Developer-mode export carries UNVERIFIED legend on every page
- T-FR-020-05 [UNIT] Mixed-edition rulebook rejected

## FR-021 Tailored skirt block

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-021 (rev 1.2); 5.4
- Implementing slice: S06 (see ../plan/)
- Business rules: BR-06, BR-08, BR-09, BR-10, BR-20
- Open questions: OQ-02, OQ-10, OQ-11, OQ-12, OQ-13, OQ-17, OQ-27
- Acceptance tests: AT-01, AT-02, AT-13

**Statement.** Net front and back pieces of the tailored skirt block: waistline, hipline, hemline, centre lines, side seams, two back darts, one front dart, shaped by verified rule records and the four measurements.

**Acceptance criteria**

- AC-FR-021-1: Generates one back piece and one front piece, both net.
- AC-FR-021-2: Each piece has waistline, hipline, hemline, centre line and side seam geometry. The back has two darts and the front has one.
- AC-FR-021-3: No separate ease input exists in R1. Ease lives in rule constants (BR-08).
- AC-FR-021-4: The off-centre side seam is preserved (BR-09).
- AC-FR-021-5: The book's worked example (size 12 chart values) matches the pattern maker's hand draft within the G0 tolerance (AT-13, OQ-17).
- AC-FR-021-6: Each of the ten Reference_drafts cases matches its hand draft within that tolerance.
- AC-FR-021-7: A qualified pattern maker signs off the construction (human gate, OQ-10).
- AC-FR-021-8: Output is deterministic (INV-01).

**Planned tests**

- T-FR-021-01 [GOLDEN] AT-13 worked example against the hand draft
- T-FR-021-02 [GOLDEN] REF-01 to REF-10 against their hand drafts
- T-FR-021-03 [PROP] Same inputs always give the same geometry checksum
- T-FR-021-04 [UNIT] Structural invariants: closed paths, hipline below waistline, hem below hipline, dart counts
- T-FR-021-05 [MANUAL] MT-04 pattern maker review and sign-off

**Note.** Cannot be completed until the G0 rule register is Verified for the ALD4-TSB, ALD4-FIT and ALD4-INT rules (DOM-01).

## FR-040 Templates and constraints

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-040 (rev 1.2)
- Implementing slice: S04 (see ../plan/)
- Business rules: BR-16
- Open questions: none
- Acceptance tests: AT-10

**Statement.** New garment definitions through versioned Rust rule implementations plus validated declarative settings. No arbitrary code in project files. Unknown identifiers fail safely.

**Acceptance criteria**

- AC-FR-040-1: Garment definitions are versioned Rust rule implementations plus validated declarative settings.
- AC-FR-040-2: The project schema is data only. No field is evaluated, executed or loaded as code.
- AC-FR-040-3: An unknown template ID, method ID, rulebook version or edition pin returns a typed error with an actionable message, does not panic and does not modify the source file (AT-10).

**Planned tests**

- T-FR-040-01 [UNIT] Each unknown identifier kind returns the expected typed error
- T-FR-040-02 [CONTRACT] Schema review test: no executable or path-evaluating fields
- T-FR-040-03 [INTEG] Opening an unsupported file leaves its bytes unchanged (hash before and after)

## FR-041 Straight skirt adaptation

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-041 (new 1.2); 5.4
- Implementing slice: S07 (see ../plan/)
- Business rules: BR-11
- Open questions: OQ-28
- Acceptance tests: AT-15

**Statement.** The straight skirt is an adaptation of the tailored block with optional centre back swing and hem flare.

**Acceptance criteria**

- AC-FR-041-1: Two presets exist: no adaptation, and straight skirt.
- AC-FR-041-2: Swing and hem flare are adjustable within validated limits. Limits come from the rulebook and are TBD.
- AC-FR-041-3: Zero swing and zero flare reproduce the block exactly (AT-15).
- AC-FR-041-4: The adaptation changes outlines and hem only. The four input measurements never change.
- AC-FR-041-5: Preset values come from Verified rule records.

**Planned tests**

- T-FR-041-01 [UNIT] Zero parameters give geometry identical to the block
- T-FR-041-02 [GOLDEN] REF-09 straight skirt reference
- T-FR-041-03 [UNIT] Out-of-range swing or flare rejected
- T-FR-041-04 [UNIT] Input measurements unchanged after adaptation

## FR-042 Standard size charts

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-042 (new 1.2); 5.3
- Implementing slice: S05 (see ../plan/)
- Business rules: BR-02, BR-19
- Open questions: OQ-03, OQ-15
- Acceptance tests: AT-16

**Statement.** The method's size charts as versioned verified data, used to supply a chart-derived waist-to-hip. The S M L XL chart is R2.

**Acceptance criteria**

- AC-FR-042-1: EU (sizes 8 to 26) and UK 5 cm (sizes 10 to 24) charts are versioned data with page references.
- AC-FR-042-2: Chart size is chosen by bust when bust is supplied. Otherwise it is chosen by hips. This fallback is an interpretation and warns (OQ-15).
- AC-FR-042-3: A chart-derived waist-to-hip is tagged chart-derived, raises a warning, and records chart, size and page in export metadata (AT-16).
- AC-FR-042-4: Chart data carry a checksum. Any change to the data changes the rulebook version (NFR-027).
- AC-FR-042-5: Short and tall adjustments are stored but not used in R1.

**Planned tests**

- T-FR-042-01 [UNIT] Lookup by bust returns the expected size
- T-FR-042-02 [UNIT] Hips fallback is labelled as an interpretation
- T-FR-042-03 [INTEG] Tag, warning and metadata present after a chart-derived entry
- T-FR-042-04 [CONTRACT] Checksum changes when chart data change
- T-FR-042-05 [GOLDEN] Chart rows equal the verified transcription

**Note.** Blocked on DOM-01 chart transcription.

## FR-043 Toile verification statement

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-043 (new 1.2); 5.7
- Implementing slice: S13 (see ../plan/)
- Business rules: BR-01
- Open questions: none
- Acceptance tests: AT-17

**Statement.** Every export page and the on-screen summary state that the output is a block for toile verification.

**Acceptance criteria**

- AC-FR-043-1: Every export page and the on-screen summary state that the output is a block for toile verification.
- AC-FR-043-2: The statement includes method ID, rulebook version and edition pin.
- AC-FR-043-3: It cannot be removed in production mode (AT-17).

**Planned tests**

- T-FR-043-01 [INTEG] Statement present in SVG metadata and visible text, and on every PDF page
- T-FR-043-02 [UNIT] Production mode has no way to hide the statement
