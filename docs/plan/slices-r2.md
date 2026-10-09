# R2 slices

Each slice delivers one working, testable capability. Work one slice per task. Do not start a slice before its dependencies are done and its blocking questions are answered. Commands assume the workspace created in S00 and are not yet run against real code.

## S20 Size grading

- Release: R2  Kind: code
- Depends on: S06, DOM-02
- Requirements: FR-027, AT-18
- ADRs: ADR-0003
- Blocked by open questions or domain work: DOM-02

**Delivers.** Per-landmark grading from the transcribed increment tables.

**Expected files or modules**

- crates/pattern_templates/src/aldrich4/skirt/grade.rs
- crates/pattern_templates/data/aldrich-mpc-4e/grade.toml

**Acceptance criteria**

- AT-18 passes.
- Uniform scaling is rejected.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_templates grade
```

## S21 DXF export

- Release: R2  Kind: code
- Depends on: SP-01, S12
- Requirements: FR-034, AT-11
- ADRs: ADR-0011
- Blocked by open questions or domain work: OQ-07

**Delivers.** DXF with named layers and unsupported-construct reporting.

**Expected files or modules**

- crates/pattern_export/src/dxf/*

**Acceptance criteria**

- AT-11 passes.
- Two external CAD viewers open the file (MT-03).

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_export dxf
```

## S22 Manual drafting tools and overrides

- Release: R2  Kind: code
- Depends on: S14, S11
- Requirements: FR-025
- ADRs: ADR-0010
- Blocked by open questions or domain work: OQ-29

**Delivers.** Constrained manual edits stored as overrides with exact undo.

**Expected files or modules**

- crates/pattern_core/src/overrides/*
- crates/pattern_ui/src/tools/*

**Acceptance criteria**

- Undo restores exact geometry.
- Overrides are stored apart from formula geometry.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## S23 Pattern comparison

- Release: R2  Kind: code
- Depends on: S14
- Requirements: FR-028
- ADRs: none
- Blocked by open questions or domain work: none

**Delivers.** Overlay and delta view for two versions or profiles.

**Expected files or modules**

- crates/pattern_core/src/compare/*
- crates/pattern_ui/src/compare.rs

**Acceptance criteria**

- Deltas and a revision summary are shown.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## S24 Material library and preview

- Release: R2  Kind: code
- Depends on: S15, S11
- Requirements: FR-029, FR-030, REN-04, AT-05
- ADRs: ADR-0005
- Blocked by open questions or domain work: none

**Delivers.** Fabric records and a 2D texture preview.

**Expected files or modules**

- crates/pattern_material/src/*
- crates/pattern_render/src/swatch.rs

**Acceptance criteria**

- AT-05 passes.
- Image decoding is bounds-checked.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## S25 Fabric layout and consumption

- Release: R2  Kind: code
- Depends on: S24
- Requirements: FR-031, AT-12
- ADRs: none
- Blocked by open questions or domain work: none

**Delivers.** Marker layout heuristic with a waste report.

**Expected files or modules**

- crates/pattern_material/src/layout/*

**Acceptance criteria**

- AT-12 passes.
- Results are labelled estimates.

**Verification commands**

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p pattern_material
```

## DOM-02 Domain baseline for R2 (grading tables)

- Release: R2  Kind: domain
- Depends on: DOM-01
- Requirements: FR-027
- ADRs: none
- Blocked by open questions or domain work: none

**Delivers.** Grade tables transcribed from the book's figures and verified.

**Expected files or modules**

- docs/domain/G0_Rule_Register_Aldrich_4e.xlsx (Grade_tables sheet completed)

**Acceptance criteria**

- Every point 1 to 19 has a landmark name from the figure and verified increments.

**Verification commands**

```
Read the Grade_tables sheet
```
