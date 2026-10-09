# Test catalog

All 134 planned tests by level. IDs match docs/requirements. A test ID is stable: never reuse an ID for a different test.

## Where each level lives

| Level | Meaning | Location convention |
|---|---|---|
| UNIT | One function or type in isolation | tests inside the crate (src/**/tests or crates/<crate>/tests) |
| PROP | Property-based test over generated inputs | crates/<crate>/tests/prop_*.rs |
| GOLDEN | Output compared with fixtures made from hand drafts | crates/<crate>/tests/golden_*.rs with fixtures in tests/fixtures |
| INTEG | Several modules or crates together | crates/<crate>/tests/integ_*.rs or tests/ at the workspace root |
| CONTRACT | File schema, rulebook, chart or public interface contract | crates/<crate>/tests/contract_*.rs |
| E2E | Through the application UI | crates/pattern_ui/tests/e2e_*.rs |
| SECURITY | Network, privacy and malformed input | tests/security/ |
| PERF | Timing and size budgets | benches/ and tests/perf/ |
| FAULT | Injected failures such as a crash mid-save | crates/pattern_document/tests/fault.rs |
| MIGRATION | Older schema versions load and upgrade | crates/pattern_document/tests/migrate.rs |
| XTARGET | Same input on native and WASM | tests/xtarget/ |
| STATIC | Lints, scripts and dependency checks | scripts/ and CI |
| MANUAL | Done by a person, recorded as evidence | docs/testing/manual-procedures.md |

## UNIT (43)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-019-01 | FR-019 | Validation per field and per cross-field rule | S03 | R1 |
| T-FR-019-02 | FR-019 | Tag rules: chart-derived accepted only on waist-to-hip | S03 | R1 |
| T-FR-019-03 | FR-019 | AT-03: each of the four mandatory fields removed in turn names that field | S03 | R1 |
| T-FR-020-01 | FR-020 | Provenance chain returned for sampled construction points | S04 | R1 |
| T-FR-020-05 | FR-020 | Mixed-edition rulebook rejected | S04 | R1 |
| T-FR-021-04 | FR-021 | Structural invariants: closed paths, hipline below waistline, hem below hipline, dart counts | S06 | R1 |
| T-FR-040-01 | FR-040 | Each unknown identifier kind returns the expected typed error | S04 | R1 |
| T-FR-041-01 | FR-041 | Zero parameters give geometry identical to the block | S07 | R1 |
| T-FR-041-03 | FR-041 | Out-of-range swing or flare rejected | S07 | R1 |
| T-FR-041-04 | FR-041 | Input measurements unchanged after adaptation | S07 | R1 |
| T-FR-042-01 | FR-042 | Lookup by bust returns the expected size | S05 | R1 |
| T-FR-042-02 | FR-042 | Hips fallback is labelled as an interpretation | S05 | R1 |
| T-FR-043-02 | FR-043 | Production mode has no way to hide the statement | S13 | R1 |
| T-FR-022-02 | FR-022 | Per-edge allowance and fold-line exemption | S08 | R1 |
| T-FR-022-04 | FR-022 | Self-intersection detection | S08 | R1 |
| T-FR-023-02 | FR-023 | AT-14 trigger boundary: just inside and just beyond | S06 | R1 |
| T-FR-023-03 | FR-023 | Labels valid after recompute | S06 | R1 |
| T-FR-024-01 | FR-024 | Every required mark present per piece | S09 | R1 |
| T-FR-026-01 | FR-026 | Join tangent and position reported correctly | S02 | R1 |
| T-FR-032-01 | FR-032 | Tile coverage: union of tiles contains all piece geometry | S13 | R1 |
| T-FR-032-02 | FR-032 | Overlap and registration marks consistent between neighbours | S13 | R1 |
| T-FR-035-04 | FR-035 | Hash mismatch reported | S11 | R1 |
| T-FR-036-01 | FR-036 | Each warning category raised by its trigger | S10 | R1 |
| T-FR-036-03 | FR-036 | Remediation text present for every code | S10 | R1 |
| T-FR-039-02 | FR-039 | Semantic tree contains labelled fields and status | S14 | R1 |
| T-FR-038-01 | FR-038 | Exact conversion for every unit pair | S01 | R1 |
| T-FR-038-03 | FR-038 | Suffix override and fractional inch parsing | S01 | R1 |
| T-FR-038-08 | FR-038 | Preference resolution order across scope and quantity class | S01 | R1 |
| T-FR-038-09 | FR-038 | Display rounding is half away from zero | S01 | R1 |
| T-FR-025-01 | FR-025 | Undo restores exact geometry | S22 | R2 |
| T-FR-025-02 | FR-025 | Override isolation from formula geometry | S22 | R2 |
| T-FR-027-02 | FR-027 | Uniform scaling rejected | S20 | R2 |
| T-FR-027-03 | FR-027 | Out-of-range warning | S20 | R2 |
| T-FR-028-01 | FR-028 | Delta computation against known pairs | S23 | R2 |
| T-FR-029-01 | FR-029 | Selecting a fabric leaves measurements unchanged | S24 | R2 |
| T-FR-031-02 | FR-031 | AT-12 narrow fabric | S25 | R2 |
| T-FR-031-03 | FR-031 | Nap and rotation constraints | S25 | R2 |
| T-REN-01-03 | REN-01 | Decimal converted once | S02 | R1 |
| T-REN-03-01 | REN-03 | Measured coordinates constant across modes | S15 | R1 |
| T-REN-04-01 | REN-04 | Repeat dimensions match the set scale | S24 | R2 |
| T-NFR-016-01 | NFR-016 | Closure check on every path | S10 | R1 |
| T-NFR-016-02 | NFR-016 | Self-intersection check | S10 | R1 |
| T-NFR-020-02 | NFR-020 | Unknown version rejected | S11 | R1 |

## PROP (10)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-019-04 | FR-019 | Any valid profile survives serialise then deserialise unchanged | S03 | R1 |
| T-FR-021-03 | FR-021 | Same inputs always give the same geometry checksum | S06 | R1 |
| T-FR-022-01 | FR-022 | Offset distance holds on generated convex and concave polygons | S08 | R1 |
| T-FR-023-04 | FR-023 | Recompute is stable and repeatable | S06 | R1 |
| T-FR-026-02 | FR-026 | Tessellation error within tolerance at low and high zoom | S02 | R1 |
| T-FR-038-02 | FR-038 | Parse then format then parse is stable for every unit | S01 | R1 |
| T-FR-038-07 | FR-038 | Arbitrary text never panics the parser | S01 | R1 |
| T-FR-031-01 | FR-031 | No overlap and within width for generated cases | S25 | R2 |
| T-NFR-014-02 | NFR-014 | Repeat runs give identical checksums | S06 | R1 |
| T-NFR-020-03 | NFR-020 | Fuzzed loader never panics | S11 | R1 |

## INTEG (26)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-019-05 | FR-019 | Profile persisted and reloaded through the project file (needs S11) | S03 | R1 |
| T-FR-020-03 | FR-020 | Production export blocked when one used rule is not Verified | S04 | R1 |
| T-FR-020-04 | FR-020 | Developer-mode export carries UNVERIFIED legend on every page | S04 | R1 |
| T-FR-040-03 | FR-040 | Opening an unsupported file leaves its bytes unchanged (hash before and after) | S04 | R1 |
| T-FR-042-03 | FR-042 | Tag, warning and metadata present after a chart-derived entry | S05 | R1 |
| T-FR-043-01 | FR-043 | Statement present in SVG metadata and visible text, and on every PDF page | S13 | R1 |
| T-FR-022-05 | FR-022 | Severe self-intersection blocks export and writes nothing | S08 | R1 |
| T-FR-024-03 | FR-024 | Exported SVG parsed: all marks present at correct coordinates | S09 | R1 |
| T-FR-026-04 | FR-026 | Export fidelity against core curve | S02 | R1 |
| T-FR-032-03 | FR-032 | Parsed PDF: page size, square size 100 mm within 0.1 mm, page numbers | S13 | R1 |
| T-FR-032-04 | FR-032 | A4 and Letter variants | S13 | R1 |
| T-FR-033-01 | FR-033 | Parse exported SVG: bounding boxes, paths, units, metadata | S12 | R1 |
| T-FR-033-02 | FR-033 | Path length within 0.1 mm of core length | S12 | R1 |
| T-FR-033-03 | FR-033 | Export bytes unchanged when render mode changes (REN-02) | S12 | R1 |
| T-FR-034-01 | FR-034 | Round-trip entity and measurement tests | S21 | R2 |
| T-FR-034-02 | FR-034 | AT-11 unsupported construct reported | S21 | R2 |
| T-FR-035-01 | FR-035 | Save, restart, load gives identical pattern checksum | S11 | R1 |
| T-FR-036-02 | FR-036 | Critical error blocks every exporter | S10 | R1 |
| T-FR-038-05 | FR-038 | AT-20 same length in five units stores one value | S01 | R1 |
| T-FR-038-06 | FR-038 | AT-21 exports across display units | S01 | R1 |
| T-FR-030-01 | FR-030 | Export bytes unchanged across texture settings | S24 | R2 |
| T-REN-02-01 | REN-02 | Export bytes identical across render modes | S15 | R1 |
| T-NFR-015-01 | NFR-015 | Path length in file against core length | S12 | R1 |
| T-NFR-016-03 | NFR-016 | Blocked export writes no file | S10 | R1 |
| T-NFR-021-02 | NFR-021 | Recovery returns the last valid version | S11 | R1 |
| T-NFR-028-02 | NFR-028 | AT-20 and AT-21 | S01 | R1 |

## CONTRACT (6)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-020-02 | FR-020 | Rulebook completeness: every constant referenced by template code exists in the rulebook (NFR-026) | S04 | R1 |
| T-FR-040-02 | FR-040 | Schema review test: no executable or path-evaluating fields | S04 | R1 |
| T-FR-042-04 | FR-042 | Checksum changes when chart data change | S05 | R1 |
| T-FR-035-03 | FR-035 | Schema golden files for each schema version | S11 | R1 |
| T-NFR-026-01 | NFR-026 | Every constant referenced in code exists in the rulebook with required fields | S04 | R1 |
| T-NFR-027-01 | NFR-027 | Data change without version bump fails CI | S04 | R1 |

## GOLDEN (10)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-021-01 | FR-021 | AT-13 worked example against the hand draft | S06 | R1 |
| T-FR-021-02 | FR-021 | REF-01 to REF-10 against their hand drafts | S06 | R1 |
| T-FR-041-02 | FR-041 | REF-09 straight skirt reference | S07 | R1 |
| T-FR-042-05 | FR-042 | Chart rows equal the verified transcription | S05 | R1 |
| T-FR-022-03 | FR-022 | Reference offset cases: concave notch, rounded corner, tight curve | S08 | R1 |
| T-FR-023-01 | FR-023 | Dart geometry against reference drafts | S06 | R1 |
| T-FR-024-02 | FR-024 | Mark positions against the verified reference | S09 | R1 |
| T-FR-026-03 | FR-026 | Reference curves | S02 | R1 |
| T-FR-027-01 | FR-027 | Graded landmarks against the approved tables | S20 | R2 |
| T-NFR-014-01 | NFR-014 | Checksum fixtures per reference case | S06 | R1 |

## MANUAL (10)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-021-05 | FR-021 | MT-04 pattern maker review and sign-off | S06 | R1 |
| T-FR-032-05 | FR-032 | MT-01 print and measure the calibration square | S13 | R1 |
| T-FR-034-03 | FR-034 | MT-03 open in two CAD viewers | S21 | R2 |
| T-FR-039-03 | FR-039 | MT-02 screen reader and keyboard review | S14 | R1 |
| T-FR-030-03 | FR-030 | Label wording review | S24 | R2 |
| T-REN-03-02 | REN-03 | Visual check of both modes and the grid | S15 | R1 |
| T-REN-04-02 | REN-04 | Legend wording review | S24 | R2 |
| T-NFR-015-02 | NFR-015 | MT-01 print check | S12 | R1 |
| T-NFR-023-02 | NFR-023 | Matrix recorded in the release evidence | S16 | R1 |
| T-NFR-025-02 | NFR-025 | MT-02 evaluation | S14 | R1 |

## FAULT (2)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-035-02 | FR-035 | Failure injected at every write step keeps the last valid file | S11 | R1 |
| T-NFR-021-01 | NFR-021 | Failure at each write step | S11 | R1 |

## MIGRATION (2)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-035-05 | FR-035 | Older schema versions migrate to current | S11 | R1 |
| T-NFR-020-01 | NFR-020 | Fixture per schema version | S11 | R1 |

## SECURITY (5)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-037-01 | FR-037 | Full workflow with network blocked | S17 | R1 |
| T-FR-037-02 | FR-037 | Network trace shows no outbound traffic | S17 | R1 |
| T-FR-030-02 | FR-030 | Malformed image rejected without panic | S24 | R2 |
| T-NFR-018-01 | NFR-018 | Test suite with the network disabled | S17 | R1 |
| T-NFR-024-01 | NFR-024 | Network trace during the full workflow | S17 | R1 |

## E2E (7)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-037-03 | FR-037 | Delete and export a profile | S17 | R1 |
| T-FR-039-01 | FR-039 | Keyboard-only run: enter measurements, generate, export | S14 | R1 |
| T-FR-025-03 | FR-025 | Edit, save, reload keeps overrides | S22 | R2 |
| T-FR-028-02 | FR-028 | Overlay and summary shown | S23 | R2 |
| T-FR-029-02 | FR-029 | Assumptions visible in the UI | S24 | R2 |
| T-NFR-018-02 | NFR-018 | Browser download and open | S17 | R1 |
| T-NFR-025-01 | NFR-025 | Keyboard-only run | S14 | R1 |

## STATIC (8)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-FR-038-04 | FR-038 | Script fails when any module formats a length directly | S01 | R1 |
| T-REN-01-02 | REN-01 | Review checklist: no pattern dimension in WGSL | S02 | R1 |
| T-REN-02-02 | REN-02 | pattern_export has no dependency on pattern_render | S15 | R1 |
| T-NFR-019-01 | NFR-019 | CI script fails above 25 MiB on the hosted artifact | S16 | R1 |
| T-NFR-022-01 | NFR-022 | clippy with deny lints on pattern_ui | S00 | R1 |
| T-NFR-022-02 | NFR-022 | forbid(unsafe_code) in every workspace crate | S00 | R1 |
| T-NFR-024-02 | NFR-024 | Dependency audit for network crates in core crates | S17 | R1 |
| T-NFR-028-01 | NFR-028 | No module formats a length directly | S01 | R1 |

## XTARGET (3)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-REN-01-01 | REN-01 | Reference pattern on native and WASM within tolerance | S02 | R1 |
| T-NFR-014-03 | NFR-014 | Native and WASM checksums agree within tolerance | S06 | R1 |
| T-NFR-023-01 | NFR-023 | Suite on each matrix entry | S16 | R1 |

## PERF (2)

| Test | Requirement | Verifies | Slice | Release |
|---|---|---|---|---|
| T-NFR-017-01 | NFR-017 | Core regeneration time for a measurement change | S14 | R1 |
| T-NFR-017-02 | NFR-017 | UI frame time during regeneration | S14 | R1 |
