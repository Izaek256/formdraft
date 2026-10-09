# Requirements: Render and non-functional

Derived from SRS 1.2. Wording is condensed and made testable. Numeric values the SRS does not give are marked TBD with an open question (OQ-xx). Nothing here invents an answer.

Status values in the traceability file: Not started, In progress, Implemented, Verified.

## REN-01 Authoritative geometry

- Release and priority: R1 / Must
- SRS reference: SRS 8 REN-01 (rev 1.2)
- Implementing slice: S02 (see ../plan/)
- Business rules: BR-13, BR-21
- Open questions: none
- Acceptance tests: AT-08

**Statement.** Canonical millimetre geometry on the CPU in f64 or a documented exact representation. The shader never defines sewing measurements.

**Acceptance criteria**

- AC-REN-01-1: All pattern calculations run on the CPU in canonical millimetre space.
- AC-REN-01-2: Rule constants are stored as exact decimals and converted once.
- AC-REN-01-3: No WGSL code computes a pattern dimension.
- AC-REN-01-4: CPU output is identical within test tolerance on native and WASM (AT-08).

**Planned tests**

- T-REN-01-01 [XTARGET] Reference pattern on native and WASM within tolerance
- T-REN-01-02 [STATIC] Review checklist: no pattern dimension in WGSL
- T-REN-01-03 [UNIT] Decimal converted once

## REN-02 GPU canvas separation

- Release and priority: R1 / Must
- SRS reference: SRS 8 REN-02
- Implementing slice: S15 (see ../plan/)
- Business rules: BR-13
- Open questions: none
- Acceptance tests: AT-05

**Statement.** The paint callback draws view effects and may draw tessellated vectors. It never modifies exported vectors.

**Acceptance criteria**

- AC-REN-02-1: The GPU layer draws shadow, checkerboard, grid and zoomed texture.
- AC-REN-02-2: A render-only sampling mode change produces byte-equivalent geometry export.

**Planned tests**

- T-REN-02-01 [INTEG] Export bytes identical across render modes
- T-REN-02-02 [STATIC] pattern_export has no dependency on pattern_render

## REN-03 Zoom and sampling

- Release and priority: R1 / Must
- SRS reference: SRS 8 REN-03
- Implementing slice: S15 (see ../plan/)
- Business rules: BR-13
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** Bilinear and trilinear sampling with explicit mipmaps, and a pixel grid at 8x zoom and above for raster assets only.

**Acceptance criteria**

- AC-REN-03-1: Bilinear and trilinear modes exist, with explicit mipmap generation where required.
- AC-REN-03-2: A 1 px grid appears at 8x zoom and above, for raster assets only.
- AC-REN-03-3: Switching mode changes the raster preview and leaves measured coordinates constant.

**Planned tests**

- T-REN-03-01 [UNIT] Measured coordinates constant across modes
- T-REN-03-02 [MANUAL] Visual check of both modes and the grid

## REN-04 Textile swatch orientation

- Release and priority: R2 / Should
- SRS reference: SRS 8 REN-04
- Implementing slice: S24 (see ../plan/)
- Business rules: BR-17
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** Texture scale in real units, rotation and repeat preview. Stretch and nap metadata kept apart from the image.

**Acceptance criteria**

- AC-REN-04-1: Scale is set in real units.
- AC-REN-04-2: Rotation and repeat preview exist.
- AC-REN-04-3: The legend shows physical repeat dimensions and implies no stretch simulation.

**Planned tests**

- T-REN-04-01 [UNIT] Repeat dimensions match the set scale
- T-REN-04-02 [MANUAL] Legend wording review

## NFR-014 Deterministic replay

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-014
- Implementing slice: S06 (see ../plan/)
- Business rules: BR-04
- Open questions: OQ-17
- Acceptance tests: none named in the SRS

**Statement.** Draft replay is deterministic for the same method ID, rulebook version and inputs.

**Acceptance criteria**

- AC-NFR-014-1: Draft replay is deterministic for the same method ID, rulebook version and inputs. Verification: Golden coordinate checksum and tolerance.

**Planned tests**

- T-NFR-014-01 [GOLDEN] Checksum fixtures per reference case
- T-NFR-014-02 [PROP] Repeat runs give identical checksums
- T-NFR-014-03 [XTARGET] Native and WASM checksums agree within tolerance

## NFR-015 Export length accuracy

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-015
- Implementing slice: S12 (see ../plan/)
- Business rules: none specific
- Open questions: OQ-05
- Acceptance tests: none named in the SRS

**Statement.** Digital export length error is 0.1 mm or less. Printing is verified separately.

**Acceptance criteria**

- AC-NFR-015-1: Digital export length error is 0.1 mm or less. Printing is verified separately. Verification: Compare path length and calibration square.

**Planned tests**

- T-NFR-015-01 [INTEG] Path length in file against core length
- T-NFR-015-02 [MANUAL] MT-01 print check

## NFR-016 No silently incomplete pattern

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-016
- Implementing slice: S10 (see ../plan/)
- Business rules: BR-14
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** No exported pattern is silently incomplete. An invalid contour gives a blocking error.

**Acceptance criteria**

- AC-NFR-016-1: No exported pattern is silently incomplete. An invalid contour gives a blocking error. Verification: Self-intersection and closure tests.

**Planned tests**

- T-NFR-016-01 [UNIT] Closure check on every path
- T-NFR-016-02 [UNIT] Self-intersection check
- T-NFR-016-03 [INTEG] Blocked export writes no file

## NFR-017 UI update time

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-017
- Implementing slice: S14 (see ../plan/)
- Business rules: none specific
- Open questions: OQ-23
- Acceptance tests: none named in the SRS

**Statement.** UI update is 150 ms or less for base skirt changes on reference hardware, with no freeze over 500 ms.

**Acceptance criteria**

- AC-NFR-017-1: UI update is 150 ms or less for base skirt changes on reference hardware, with no freeze over 500 ms. Verification: Local performance bench.

**Planned tests**

- T-NFR-017-01 [PERF] Core regeneration time for a measurement change
- T-NFR-017-02 [PERF] UI frame time during regeneration

## NFR-018 Offline operation

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-018
- Implementing slice: S17 (see ../plan/)
- Business rules: BR-15
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** All core drafting actions work offline. Browser local download and open are supported.

**Acceptance criteria**

- AC-NFR-018-1: All core drafting actions work offline. Browser local download and open are supported. Verification: Disable network, run test suite.

**Planned tests**

- T-NFR-018-01 [SECURITY] Test suite with the network disabled
- T-NFR-018-02 [E2E] Browser download and open

## NFR-019 WASM size

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-019
- Implementing slice: S16 (see ../plan/)
- Business rules: none specific
- Open questions: OQ-08
- Acceptance tests: none named in the SRS

**Statement.** WASM artifact is below 25 MiB after production optimisation.

**Acceptance criteria**

- AC-NFR-019-1: WASM artifact is below 25 MiB after production optimisation. Verification: CI size gate.

**Planned tests**

- T-NFR-019-01 [STATIC] CI script fails above 25 MiB on the hosted artifact

## NFR-020 Schema version checking

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-020
- Implementing slice: S11 (see ../plan/)
- Business rules: BR-16
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** 100% of loaded project files are version-checked, including method ID, rulebook version and edition pin. Unknown schemas are rejected safely.

**Acceptance criteria**

- AC-NFR-020-1: 100% of loaded project files are version-checked, including method ID, rulebook version and edition pin. Unknown schemas are rejected safely. Verification: Schema upgrade tests.

**Planned tests**

- T-NFR-020-01 [MIGRATION] Fixture per schema version
- T-NFR-020-02 [UNIT] Unknown version rejected
- T-NFR-020-03 [PROP] Fuzzed loader never panics

## NFR-021 Crash-safe saves

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-021
- Implementing slice: S11 (see ../plan/)
- Business rules: none specific
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** Native saves are crash-safe with deterministic file recovery.

**Acceptance criteria**

- AC-NFR-021-1: Native saves are crash-safe with deterministic file recovery. Verification: Fault injection.

**Planned tests**

- T-NFR-021-01 [FAULT] Failure at each write step
- T-NFR-021-02 [INTEG] Recovery returns the last valid version

## NFR-022 Lint policy

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-022
- Implementing slice: S00 (see ../plan/)
- Business rules: none specific
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** No unwrap, expect or panic in UI code and no unsafe in own workspace crates.

**Acceptance criteria**

- AC-NFR-022-1: No unwrap, expect or panic in UI code and no unsafe in own workspace crates. Verification: cargo clippy and deny policy.

**Planned tests**

- T-NFR-022-01 [STATIC] clippy with deny lints on pattern_ui
- T-NFR-022-02 [STATIC] forbid(unsafe_code) in every workspace crate

## NFR-023 Platform matrix

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-023
- Implementing slice: S16 (see ../plan/)
- Business rules: none specific
- Open questions: OQ-24
- Acceptance tests: none named in the SRS

**Statement.** A minimum reference browser and operating system matrix is recorded before release.

**Acceptance criteria**

- AC-NFR-023-1: A minimum reference browser and operating system matrix is recorded before release. Verification: Cross-platform tests.

**Planned tests**

- T-NFR-023-01 [XTARGET] Suite on each matrix entry
- T-NFR-023-02 [MANUAL] Matrix recorded in the release evidence

## NFR-024 No undisclosed transmission

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-024
- Implementing slice: S17 (see ../plan/)
- Business rules: BR-15
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** No undisclosed image upload, analytics or remote measurement storage.

**Acceptance criteria**

- AC-NFR-024-1: No undisclosed image upload, analytics or remote measurement storage. Verification: Network trace offline test.

**Planned tests**

- T-NFR-024-01 [SECURITY] Network trace during the full workflow
- T-NFR-024-02 [STATIC] Dependency audit for network crates in core crates

## NFR-025 Accessible workflow

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-025
- Implementing slice: S14 (see ../plan/)
- Business rules: none specific
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** The measurement form and export flow are accessible by keyboard and semantic tree.

**Acceptance criteria**

- AC-NFR-025-1: The measurement form and export flow are accessible by keyboard and semantic tree. Verification: Manual accessibility evaluation.

**Planned tests**

- T-NFR-025-01 [E2E] Keyboard-only run
- T-NFR-025-02 [MANUAL] MT-02 evaluation

## NFR-026 Rule record completeness

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-026
- Implementing slice: S04 (see ../plan/)
- Business rules: BR-05, BR-22
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** 100% of constants and formulas that influence production output have a rule record with page reference, kind and verifier.

**Acceptance criteria**

- AC-NFR-026-1: 100% of constants and formulas that influence production output have a rule record with page reference, kind and verifier. Verification: Automated rulebook completeness test.

**Planned tests**

- T-NFR-026-01 [CONTRACT] Every constant referenced in code exists in the rulebook with required fields

## NFR-027 Data checksums

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-027
- Implementing slice: S04 (see ../plan/)
- Business rules: none specific
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** Size chart and rule data are checksummed. Any change to transcribed data changes the rulebook version.

**Acceptance criteria**

- AC-NFR-027-1: Size chart and rule data are checksummed. Any change to transcribed data changes the rulebook version. Verification: Checksum test in CI.

**Planned tests**

- T-NFR-027-01 [CONTRACT] Data change without version bump fails CI

## NFR-028 Unit service coverage

- Release and priority: R1 / Must
- SRS reference: SRS 10 NFR-028
- Implementing slice: S01 (see ../plan/)
- Business rules: BR-12
- Open questions: none
- Acceptance tests: none named in the SRS

**Statement.** 100% of unit-bearing fields, labels and exports route through the unit service, for mm, cm, m, in, ft and yd.

**Acceptance criteria**

- AC-NFR-028-1: 100% of unit-bearing fields, labels and exports route through the unit service, for mm, cm, m, in, ft and yd. Verification: Static check plus AT-20 and AT-21.

**Planned tests**

- T-NFR-028-01 [STATIC] No module formats a length directly
- T-NFR-028-02 [INTEG] AT-20 and AT-21
