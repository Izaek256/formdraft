# R1 slices

Each slice delivers one working, testable capability. Work one slice per task. Do not start a slice before its dependencies are done and its blocking questions are answered. Commands assume the workspace created in S00 and are not yet run against real code.

## DOM-01 Domain baseline (gate G0)

- Release: R1  Kind: domain
- Depends on: nothing
- Requirements: FR-019, FR-020, FR-021, FR-023, FR-024, FR-026, FR-041, FR-042
- ADRs: ADR-0003
- Blocked by open questions or domain work: OQ-01, OQ-02, OQ-10, OQ-11, OQ-12, OQ-13, OQ-14, OQ-15, OQ-17, OQ-27, OQ-28

**Delivers.** The G0 rule register workbook completed and verified by a named pattern maker, with ten hand-drafted reference skirts.

**Expected files or modules**

- docs/domain/G0_Rule_Register_Aldrich_4e.xlsx (completed)
- docs/domain/reference-drafts/REF-01 to REF-10 scans or photos
- docs/domain/g0-signoff.md (exported sign-off)

**Acceptance criteria**

- Every R1 rule and every R1 interpretation has status Verified, a verifier name and a date.
- Every rule is marked as checked against the book's figure.
- Ten reference drafts exist with the composition in the Reference_drafts sheet, at least three from consenting real clients.
- Thresholds, default allowances, small-waist trigger, curve interpretations and code-versus-hand tolerance are decided.
- G0_signoff reports READY FOR G0 REVIEW and the verifier has signed.

**Verification commands**

```
Open the workbook and read G0_signoff status
Run scripts/check-g0-readiness.py (created in S04) which reads the workbook and fails unless ready
```

## SP-01 Spike: PDF and DXF libraries on native and WASM

- Release: R1  Kind: spike
- Depends on: S00
- Requirements: FR-032, FR-033, FR-034, NFR-019
- ADRs: ADR-0011
- Blocked by open questions or domain work: OQ-06, OQ-07, OQ-31

**Delivers.** A written finding and a throwaway prototype showing which PDF and DXF approach works on both targets within the size cap.

**Expected files or modules**

- spikes/sp-01-export/ (prototype crate, not part of the workspace release)
- docs/architecture/adr/ADR-0011-export-strategy.md (updated to Accepted or revised)

**Acceptance criteria**

- A PDF with a 100 mm square and two tiled pages is produced natively and in a WASM build.
- Measured WASM size impact of each candidate is recorded.
- A DXF with two layers is produced natively and the WASM outcome is recorded.
- The ADR records the chosen approach, rejected options and the fallback if browser export cannot run (SRS 13).

**Verification commands**

```
cargo run -p sp01-export
trunk build --release in the spike host
scripts/check-wasm-size.sh <artifact>
```

## SP-02 Spike: polygon and curve offset approach

- Release: R1  Kind: spike
- Depends on: S02
- Requirements: FR-022
- ADRs: ADR-0008
- Blocked by open questions or domain work: OQ-19, OQ-20

**Delivers.** A comparison of offset strategies and a decision on the one S08 will implement.

**Expected files or modules**

- spikes/sp-02-offset/
- docs/architecture/adr/ADR-0008-stitch-cut-model.md (algorithm section updated)

**Acceptance criteria**

- Candidate strategies are run on five hand-built cases: convex corner, concave corner, tight curve, near-degenerate sliver, allowance larger than a feature.
- Distance error, self-intersection behaviour and runtime are recorded per case.
- The ADR names the strategy and the corner policy proposal for OQ-19 and the severity proposal for OQ-20.

**Verification commands**

```
cargo test -p sp02-offset
cargo bench -p sp02-offset
```

## SP-03 Spike: egui_wgpu paint callback and texture limits

- Release: R1  Kind: spike
- Depends on: S14
- Requirements: REN-02, REN-03
- ADRs: ADR-0005
- Blocked by open questions or domain work: none

**Delivers.** A working paint callback drawing grid and a sampled texture on native and WASM, with documented fallback and texture size limits.

**Expected files or modules**

- spikes/sp-03-render/
- docs/architecture/03-dataflow-failure-security-deployment.md (render fallback section updated)

**Acceptance criteria**

- Grid and a bilinear and trilinear sampled texture draw on native and in a browser.
- A device without the required feature falls back without crashing.
- The maximum texture size is queried and enforced.

**Verification commands**

```
cargo run -p sp03-render
trunk serve in the spike host
```

## S00 Workspace and CI skeleton

- Release: R1  Kind: code
- Depends on: nothing
- Requirements: NFR-022, NFR-019
- ADRs: ADR-0001, ADR-0004, ADR-0012
- Blocked by open questions or domain work: OQ-32, OQ-34

**Delivers.** A compiling workspace with the eight crates from the SRS, lint policy, formatting, dependency policy and a CI pipeline that already runs and fails correctly.

**Expected files or modules**

- Cargo.toml (workspace)
- crates/pattern_core, pattern_templates, pattern_document, pattern_export, pattern_material, pattern_ui, pattern_render, pattern_web (each Cargo.toml and src/lib.rs or main.rs)
- rustfmt.toml
- clippy.toml
- deny.toml
- rust-toolchain.toml
- scripts/check-wasm-size.sh
- scripts/check-no-raw-length-format.sh (stub)
- scripts/check-layering.sh
- tools/pattern-cli/ (empty binary that grows with later slices)
- CI pipeline definition (host per OQ-34)

**Acceptance criteria**

- cargo build --workspace succeeds on Windows and Linux.
- Every workspace crate has forbid(unsafe_code).
- pattern_ui denies unwrap_used, expect_used and panic via Clippy.
- The dependency direction rules in ADR-0004 are enforced by a test or script that fails on a violation.
- cargo deny check passes with the policy in AGENTS.md.
- A WASM check build of pattern_core passes.
- The CI pipeline runs fmt, clippy, test, deny, and the WASM check, and a deliberately broken commit fails it.
- CI triggers on pull requests into develop and main, on pushes to develop and main, and on tags starting with v.
- Crate versions named in the SRS (egui 0.36, eframe 0.36) are confirmed or the discrepancy is recorded in an ADR.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo check -p pattern_core --target wasm32-unknown-unknown
scripts/check-layering.sh
```

## S01 Unit service

- Release: R1  Kind: code
- Depends on: S00
- Requirements: FR-038, NFR-028
- ADRs: ADR-0002, ADR-0007
- Blocked by open questions or domain work: OQ-16, OQ-33

**Delivers.** Parse, convert and format lengths in mm, cm, m, in, ft and yd with exact decimal arithmetic, as a standalone tested library.

**Expected files or modules**

- crates/pattern_core/src/units/{mod,unit,length,decimal,parse,format,preference}.rs
- crates/pattern_core/tests/units_*.rs
- scripts/check-no-raw-length-format.sh (real implementation)

**Acceptance criteria**

- Exact conversion for all six units with the factors in FR-038.
- Parsing accepts decimals, fractions (37 3/8 in) and a unit suffix, and rejects ambiguous or malformed input with a typed error.
- Formatting follows a precision table that is data, not code (OQ-33).
- Preference scopes global, project and quantity class resolve in that override order.
- AT-20 passes as an integration test on the library.
- The static script fails when a length is formatted outside the unit service.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_core units
scripts/check-no-raw-length-format.sh
```

## S02 Geometry primitives

- Release: R1  Kind: code
- Depends on: S00, S01
- Requirements: REN-01, FR-026, NFR-014
- ADRs: ADR-0002, ADR-0005
- Blocked by open questions or domain work: OQ-11

**Delivers.** Points, paths, lines, cubic Béziers, continuity inspection, adaptive tessellation and path length, in canonical millimetre geometry.

**Expected files or modules**

- crates/pattern_core/src/geom/{mod,point,path,bezier,tessellate,measure,validate}.rs
- crates/pattern_core/tests/geom_*.rs

**Acceptance criteria**

- Paths and curves are never represented as raster data.
- Continuity at joins is reported for position and tangent.
- Tessellation deviation stays within a documented tolerance at low and high zoom.
- Path length is computed deterministically.
- Closed-path and basic validity checks exist.
- The same tests pass on a WASM check build.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_core geom
cargo check -p pattern_core --target wasm32-unknown-unknown
```

## S03 Measurement profile and validation

- Release: R1  Kind: code
- Depends on: S01
- Requirements: FR-019
- ADRs: ADR-0009
- Blocked by open questions or domain work: OQ-02, OQ-22

**Delivers.** A measurement profile type with source tags, validation driven by data, and precise error messages.

**Expected files or modules**

- crates/pattern_core/src/measure/{mod,profile,tag,validate,limits}.rs
- crates/pattern_core/tests/measure_*.rs
- crates/pattern_templates/data/aldrich-mpc-4e/thresholds.toml (placeholder until G0)

**Acceptance criteria**

- All FR-019 acceptance criteria except anonymisation hold.
- Limits are read from the thresholds data file, which contains no invented numbers: it ships empty or marked UNSET.
- With limits UNSET, validation reports that limits are not configured and does not guess.
- AT-03 passes for each of the four fields.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_core measure
```

## S04 Rule records and rulebook loader

- Release: R1  Kind: code
- Depends on: S00, S01
- Requirements: FR-020, FR-040, NFR-026, NFR-027, AT-19
- ADRs: ADR-0003, ADR-0009
- Blocked by open questions or domain work: OQ-30

**Delivers.** Rule record types, a validated rulebook loader, provenance queries, verification gating, checksums and a completeness test, using placeholder data.

**Expected files or modules**

- crates/pattern_core/src/rules/{mod,record,kind,status,provenance,gate}.rs
- crates/pattern_templates/data/aldrich-mpc-4e/rulebook.toml (slots only, status Open)
- crates/pattern_templates/src/rulebook/{mod,load,checksum}.rs
- scripts/check-g0-readiness.py
- crates/pattern_templates/tests/rulebook_*.rs

**Acceptance criteria**

- A rule record has rule ID, group, page, kind, expression, dependencies, status, verifier and date.
- The loader rejects duplicate IDs, unknown kinds and mixed editions.
- Provenance returns the rule chain for a construction point.
- The completeness test fails when code references a constant with no record.
- Changing data without changing the rulebook version fails the checksum test.
- Production mode refuses a rule that is not Verified. Developer mode marks output UNVERIFIED (AT-19).
- No real book constant is hard-coded anywhere in source files.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_core rules
cargo test -p pattern_templates rulebook
python scripts/check-g0-readiness.py --help
```

## S05 Size charts and chart lookup

- Release: R1  Kind: code
- Depends on: S01, S04, DOM-01
- Requirements: FR-042
- ADRs: ADR-0003
- Blocked by open questions or domain work: OQ-03, OQ-15

**Delivers.** Verified EU and UK 5 cm chart data and a lookup that produces a tagged, warned, chart-derived waist-to-hip.

**Expected files or modules**

- crates/pattern_templates/data/aldrich-mpc-4e/charts.toml
- crates/pattern_templates/src/charts/{mod,lookup}.rs
- crates/pattern_templates/tests/charts_*.rs

**Acceptance criteria**

- All FR-042 acceptance criteria hold.
- Chart data equals the verified transcription row by row.
- AT-16 passes.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_templates charts
```

## S06 Tailored skirt block with darts

- Release: R1  Kind: code
- Depends on: S02, S03, S04, S05, DOM-01
- Requirements: FR-021, FR-023, NFR-014
- ADRs: ADR-0003, ADR-0009, ADR-0013
- Blocked by open questions or domain work: OQ-10, OQ-11, OQ-13, OQ-17, OQ-27

**Delivers.** Back and front net pieces with darts generated from the four measurements and verified rules, with provenance, matching the reference drafts.

**Expected files or modules**

- crates/pattern_templates/src/aldrich4/skirt/{mod,block,back,front,darts,small_waist}.rs
- crates/pattern_templates/tests/skirt_*.rs
- crates/pattern_templates/tests/fixtures/ref-01 to ref-10 (from hand drafts)

**Acceptance criteria**

- AT-01, AT-02, AT-13 and AT-14 pass.
- All ten reference sets match within the G0 tolerance.
- Every constant used resolves to a Verified rule record.
- Generation is deterministic and has no I/O.
- Fixtures come from hand drafts and are never produced by this code (ADR-0013).

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_templates skirt
cargo test -p pattern_core
```

## S07 Straight skirt adaptation

- Release: R1  Kind: code
- Depends on: S06
- Requirements: FR-041
- ADRs: ADR-0013
- Blocked by open questions or domain work: OQ-28

**Delivers.** Swing and hem flare presets applied to the block.

**Expected files or modules**

- crates/pattern_templates/src/aldrich4/skirt/adapt.rs
- crates/pattern_templates/tests/adapt_*.rs

**Acceptance criteria**

- AT-15 passes.
- REF-09 matches its hand draft.
- Limits are enforced from the rulebook.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_templates adapt
```

## S08 Seam allowance and offset

- Release: R1  Kind: code
- Depends on: S02, SP-02, S06
- Requirements: FR-022
- ADRs: ADR-0008
- Blocked by open questions or domain work: OQ-14, OQ-19, OQ-20

**Delivers.** Cut paths generated from stitch paths with per-edge allowance, the chosen corner policy and self-intersection detection.

**Expected files or modules**

- crates/pattern_core/src/offset/{mod,edge,corner,detect}.rs
- crates/pattern_core/tests/offset_*.rs

**Acceptance criteria**

- AT-04 and AT-09 pass.
- Offset distance holds within 0.1 mm on the reference cases and generated shapes.
- Fold edges receive no allowance.
- Severe and minor self-intersections follow the OQ-20 decision.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_core offset
cargo bench -p pattern_core offset
```

## S09 Construction marks

- Release: R1  Kind: code
- Depends on: S06, S08
- Requirements: FR-024
- ADRs: ADR-0003
- Blocked by open questions or domain work: OQ-12

**Delivers.** Grainline, notches, fold marks, labels, quantity, size and style number on each piece.

**Expected files or modules**

- crates/pattern_core/src/marks/{mod,grainline,notch,label}.rs
- crates/pattern_templates/tests/marks_*.rs

**Acceptance criteria**

- All FR-024 criteria hold.
- Mark positions match the verified interpretation rules.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_templates marks
```

## S10 Audit, warnings and export gate

- Release: R1  Kind: code
- Depends on: S03, S04, S08
- Requirements: FR-036, NFR-016
- ADRs: ADR-0004
- Blocked by open questions or domain work: OQ-20, OQ-30

**Delivers.** A single warning model and an export gate that every exporter must pass through.

**Expected files or modules**

- crates/pattern_core/src/audit/{mod,warning,severity,gate}.rs
- crates/pattern_core/tests/audit_*.rs

**Acceptance criteria**

- Every warning category in FR-036 is raised by its trigger.
- Critical errors make the gate refuse export.
- Remediation text exists for every code.
- The gate is a required argument of every exporter, so bypassing it does not compile.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_core audit
```

## S11 Project document

- Release: R1  Kind: code
- Depends on: S03, S04, S06
- Requirements: FR-035, FR-040, NFR-020, NFR-021, AT-06, AT-10
- ADRs: ADR-0010, ADR-0006
- Blocked by open questions or domain work: OQ-21, OQ-25, OQ-29

**Delivers.** Versioned project save and load with migrations and atomic writes on native, and complete single-download save for browsers.

**Expected files or modules**

- crates/pattern_document/src/{lib,schema,project,migrate,store,atomic}.rs
- crates/pattern_document/tests/{roundtrip,migrate,fault,unsupported}.rs
- crates/pattern_document/tests/fixtures/schema-v1/*

**Acceptance criteria**

- AT-06 and AT-10 pass.
- Fault injection at every write step leaves the last valid file loadable.
- Every loaded file is version-checked, including method ID, rulebook version and edition pin.
- The fuzzed loader never panics.
- The schema contains no executable content.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_document
cargo test -p pattern_document --test fault
```

## S12 SVG export

- Release: R1  Kind: code
- Depends on: S09, S10
- Requirements: FR-033, NFR-015
- ADRs: ADR-0011
- Blocked by open questions or domain work: OQ-06

**Delivers.** Dimensioned SVG with groups, metadata and the toile statement, validated by a parser test.

**Expected files or modules**

- crates/pattern_export/src/svg/{mod,writer,metadata}.rs
- crates/pattern_export/tests/svg_*.rs

**Acceptance criteria**

- FR-033 criteria hold.
- Path length is within 0.1 mm of the core length.
- Output is identical across display units except legends and display-unit metadata.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_export svg
```

## S13 Tiled PDF export and toile statement

- Release: R1  Kind: code
- Depends on: SP-01, S09, S10, S12
- Requirements: FR-032, FR-043, AT-07, AT-17, AT-21
- ADRs: ADR-0011
- Blocked by open questions or domain work: OQ-05, OQ-26, OQ-31

**Delivers.** A tiled A4 and Letter PDF with calibration, plus the toile statement on every page.

**Expected files or modules**

- crates/pattern_export/src/pdf/{mod,tile,calibration,legend}.rs
- crates/pattern_export/tests/pdf_*.rs

**Acceptance criteria**

- AT-07, AT-17 and AT-21 pass.
- The union of tiles covers every piece edge.
- The calibration square is 100 mm within 0.1 mm in the file.
- MT-01 print calibration is recorded in the evidence pack.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_export pdf
```

## S14 Application shell and UI

- Release: R1  Kind: code
- Depends on: S01, S03, S06, S08, S09, S10, S11
- Requirements: FR-039, NFR-017, NFR-025
- ADRs: ADR-0004, ADR-0005, ADR-0007
- Blocked by open questions or domain work: OQ-09, OQ-23

**Delivers.** A native egui app: unit-aware measurement form, generation, a CPU-tessellated vector canvas, warnings panel, save and open, keyboard access.

**Expected files or modules**

- crates/pattern_ui/src/{main,app,form,canvas,warnings,settings}.rs
- crates/pattern_ui/tests/*

**Acceptance criteria**

- The WF-01 workflow runs end to end offline in the app.
- FR-039 criteria hold.
- Regeneration after a measurement change is within the NFR-017 budget on the reference hardware.
- No unwrap, expect or panic exists in pattern_ui.
- No length is formatted outside the unit service.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_ui
cargo run -p pattern_ui
```

## S15 GPU render layer

- Release: R1  Kind: code
- Depends on: S14, SP-03
- Requirements: REN-02, REN-03
- ADRs: ADR-0005
- Blocked by open questions or domain work: none

**Delivers.** View-only GPU effects and sampling modes with a safe fallback.

**Expected files or modules**

- crates/pattern_render/src/{lib,callback,shader.wgsl,texture}.rs
- crates/pattern_render/tests/*

**Acceptance criteria**

- Export bytes are identical across render modes.
- pattern_export does not depend on pattern_render.
- A fallback path works without the GPU feature.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_render
scripts/check-layering.sh
```

## S16 Web build

- Release: R1  Kind: code
- Depends on: S14, S12, S13
- Requirements: NFR-019, NFR-023, REN-01, AT-08
- ADRs: ADR-0011
- Blocked by open questions or domain work: OQ-08, OQ-24

**Delivers.** A Trunk WASM build that passes the cross-target reference tests and the size gate.

**Expected files or modules**

- crates/pattern_web/{index.html,Trunk.toml,Cargo.toml}
- scripts/check-wasm-size.sh (real thresholds)
- docs/release/browser-matrix.md

**Acceptance criteria**

- AT-08 passes: native and WASM reference output agree within tolerance.
- The hosted WASM artifact is under 25 MiB and CI fails otherwise.
- Browser download and open work.
- The browser and OS matrix is recorded.

**Verification commands**

```
trunk build --release
scripts/check-wasm-size.sh crates/pattern_web/dist
cargo test --workspace
```

## S17 Offline and privacy harness

- Release: R1  Kind: code
- Depends on: S11, S13, S14
- Requirements: FR-037, NFR-018, NFR-024
- ADRs: ADR-0006
- Blocked by open questions or domain work: OQ-04

**Delivers.** Automated proof that the full workflow runs with no network and no outbound traffic.

**Expected files or modules**

- tests/offline/*
- scripts/network-trace.*
- docs/testing/evidence/offline-trace.md

**Acceptance criteria**

- The suite passes with the network disabled.
- A trace of the full workflow shows no outbound connections.
- Profile delete and export work.

**Verification commands**

```
cargo test --workspace
scripts/network-trace.*
```

## S18 R1 acceptance and evidence pack

- Release: R1  Kind: code
- Depends on: S16, S17, S15
- Requirements: FR-021
- ADRs: none
- Blocked by open questions or domain work: OQ-05, OQ-10

**Delivers.** One run that executes AT-01 to AT-21 (R1 subset) and the manual procedures and assembles the evidence pack for gate sign-off.

**Expected files or modules**

- tests/acceptance/at_*.rs
- docs/testing/evidence/r1-evidence-pack.md

**Acceptance criteria**

- Every R1 acceptance test passes.
- MT-01, MT-02 and MT-04 are recorded.
- The evidence pack lists every R1 requirement with its test results and open questions.

**Verification commands**

```
cargo test --workspace
scripts/build-evidence-pack.*
```
