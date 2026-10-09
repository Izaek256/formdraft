# **Software Requirements Specification** 

### _Automated Garment Pattern Generation System_ 

SRS version 1.0  |  Draft for stakeholder review  |  8 October 2026 

Implementation: Rust workspace • eframe/egui 0.36 • wgpu • egui_wgpu/WGSL canvas • egui_extras SVG • Trunk/WASM 

Status: DRAFT — unapproved garment equations, platform targets, data format, and numeric quality thresholds require validation. 

## **Document control** 

|**Field**|**Value**|
|---|---|
|**Document ID**|AGPGS-SRS-1.0|
|**Author**|Initial technical draft|
|**Baseline**|Source concept document + developer-selected<br>implementation constraints|
|**Change control**|Changes to “Must” requirements require<br>versioning and test update|
|**Audience**|Development team, project supervisor, tailoring<br>subject-matter experts, testers|



## **1. Introduction** 

#### **1.1 Purpose** 

This SRS defines an implementation-oriented baseline for a system that converts garment type and human body measurements into mathematically derived, dimensionally accurate, printable sewing pattern pieces. It specifies requirements for a Rust native/web application and its data, geometry, GPU rendering, quality, and verification. 

#### **1.2 Source basis and decisions** 

The supplied two-page project concept explicitly identifies: a measurement form; a formula-driven parametric geometry calculation engine; vector drawing with curved regions; PDF/DXF outputs; starting with one simple block; validation of curved drafting; and ease/style variables. It does not contain approved formulas, final garment catalogue, exact performance targets, storage rules, or target operating systems. 

Developer-mandated baseline: entire application implementation in Rust; eframe 0.36 / egui 0.36 UI; eframe wgpu and accesskit feature flags; egui_extras SVG loader; wgpu rendering; canvas implemented through egui_wgpu paint callback and proprietary WGSL; custom canvas shadow, checkerboard, bilinear/trilinear sampling, pixel-grid at ≥8×; web build with Trunk and wasm-release; web-host .wasm under 25 MiB; workspace-level forbid(unsafe_code); UI crate denies unwrap/expect/panic. 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

#### **1.3 Scope** 

- In scope: manually entered body measurements, garment templates, parametric construction of pieces, interactive preview, dimension and seam visualization, PDF/DXF export, local project persistence, desktop/web distribution, validation and error reporting. 

- Recommended v1.0 garment baseline: one fully validated straight-skirt block; shirt/trousers are later templates unless separately approved. 

- Out of scope for v1.0: automatic photo/AI body measurement, 3-D fitting, cloth simulation, sewing machine control, industrial cutter integration, online payments, remote collaboration, medical/bodyhealth inference. 

#### **1.4 Definitions** 

|**Term**|**Definition**|
|---|---|
|**Pattern piece**|2-D geometry specifying a cut/assembly component|
|**Block/sloper**|Base draft constructed from measurements using a<br>defined methodology|
|**Ease**|Added allowance for movement and intended fit,<br>distinct from seam allowance|
|**Seam allowance**|Offset from stitch line for joining fabric|
|**Notch/grainline**|Construction marks assisting assembly and cutting<br>orientation|
|**DXF**|Drawing Exchange Format used to exchange vector<br>CAD geometry|
|**WGSL**|WebGPU Shading Language for the canvas GPU<br>shader|
|**WASM**|WebAssembly build target for deployment in a<br>browser|
|**Source of truth**|Measurement/template-derived vector geometry;<br>never the GPU framebuffer|



## **2. Stakeholders, users and operation** 

|**Role**|**Responsibilities**|
|---|---|
|**Tailor / pattern maker**|Choose garment; capture measurements; inspect<br>and export pattern|
|**Student/developer**|Maintain crates, drafting algorithms, validation,<br>platform builds|
|**Fashion-domain validator**|Approve drafting method and verify paper/fabric<br>results|
|**Academic supervisor**|Review scope, requirements traceability and<br>evidence|



#### **2.1 Primary use case** 

User launches app → starts project → chooses Straight Skirt → inputs required measurements with centimetre or inch display → app validates values → selected methodology calculates piece geometry in 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

physical units → canvas previews sew lines, cut lines, markings and dimensions → user adjusts ease/measurements → app recalculates deterministically → user saves editable project and exports a fullsize PDF and optionally DXF → user verifies calibration mark and paper assembly. 

#### **2.2 Operating environments** 

Proposed: Windows and Linux native executables, plus modern browsers supporting the configured wgpu backend. macOS is an optional verification target. Offline project operation shall be possible after native install or web asset availability; web offline-cache/PWA behaviour is not promised in this version. Target browsers, screen-reader test environments and GPU fallback policies remain open. 

## **3. System architecture** 

Layers: UI crate (egui/eframe), application/controller crate (project commands, state transitions), drafting core crate (units, formula templates, validated geometry), persistence/export crates, canvas renderer crate (egui_wgpu paint callback and WGSL). Build desktop and wasm32 from shared domain/application Rust code. Optional platform-specific adapters must not alter drafting results. 

Proposed Rust workspace crates: pattern-domain; pattern-drafting; pattern-geometry; pattern-doc; patternexport; pattern-canvas; pattern-ui; pattern-app. Names are suggestions, not mandatory APIs. 

#### **3.1 GPU/data boundary** 

Physical dimensions and curves shall be represented as geometry in document units (prefer millimetres internally). The wgpu callback shall render projections of that geometry, with camera transforms for pan/zoom. Shader-only effects must not change geometric exports. SVG loading is for icons/reference assets/preview support and is not a substitute for parametric editable shape data. 

#### **3.2 Build and policy boundaries** 

Desktop uses eframe wgpu backend. Web uses Trunk and a named wasm-release Cargo profile configured for compact output. Measure actual emitted .wasm against a host-enforced <25 MiB limit (clarify whether raw or compressed size). The Rust workspace forbids unsafe code; UI crate adds deny policies for direct unwrap, expect, and panic via lint configuration. These rules apply to code authored in workspace, not a guarantee that transitive dependencies contain no unsafe implementation. 

## **4. Functional requirements** 

#### **FR-001 — Create and manage pattern projects** 

The user shall create, open, save, duplicate and rename pattern projects, with a garment template and drafting-method version stored with each project. 

Acceptance: Reopening preserves input values, geometry settings and original document unit conventions. 

Priority: Must 

#### **FR-002 — Select supported garment templates** 

The user shall select from a list of installed garment templates; only formally validated templates are enabled for export. 

Acceptance: Unvalidated templates cannot be presented as cutting-ready. 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

Priority: Must 

#### **FR-003 — Enter and validate measurements** 

Provide fields specific to the selected template, units and guidance. Reject non-finite, missing, zero/negative or implausible values according to per-field domain constraints, without crashing. 

Acceptance: Errors identify the exact field and prevent invalid generation; valid unit conversion is reversible within tolerance. 

Priority: Must 

#### **FR-004 — Apply named drafting methodology** 

A versioned garment template shall map measurements and fit variables to deterministic construction points, paths, darts and piece metadata using a documented, approved drafting method. 

Acceptance: Same input + template version produces equivalent geometry within specified numeric tolerance. 

Priority: Must 

#### **FR-005 — Support fitting and construction allowances** 

Users may configure approved ease and seam-allowance parameters separately; seam allowance offsets may be added to cut paths independently of stitched/finished paths. 

Acceptance: Changes yield expected geometry; the output distinguishes body dimensions, ease and seam allowances. 

Priority: Must 

#### **FR-006 — Construct geometric primitives** 

The geometry layer shall support points, line segments, closed/open paths and curves (e.g. cubic Bézier), with joins, transforms, orientation and bounds. 

Acceptance: Unit tests check geometric continuity, finiteness, closure and signed extents. 

Priority: Must 

#### **FR-007 — Produce required pattern pieces** 

The straight-skirt reference template shall generate all approved front/back pieces and associated construction lines/markings as specified by the chosen drafting standard. 

Acceptance: Domain expert checks piece inventory and corresponding dimensions against reference drafts. 

Priority: Must 

#### **FR-008 — Expose understandable pattern metadata** 

Show project name, garment type, measurement profile, units, template method/version, piece labels, grainlines, cut quantity and seam indications where applicable. 

Acceptance: Each exported piece can be identified and used without relying on UI-only labels. 

Priority: Must 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

#### **FR-009 — Live canvas and interaction** 

The document canvas shall show derived geometry with zoom, pan, fit-to-view, selection and optional measurement overlays; it shall be an egui_wgpu paint callback. 

Acceptance: User changes trigger fresh preview; scaling on screen cannot change the underlying millimetre lengths. 

Priority: Must 

#### **FR-010 — GPU canvas visual effects** 

The WGSL-based canvas shall draw a drop shadow and checkerboard; support bilinear or trilinear sampling for applicable raster textures; and draw a pixel grid when zoom is at least 8×. 

Acceptance: A 7.99× view omits pixel grid and 8× enables it; shader effects are visually testable and excluded from export geometry. 

Priority: Must 

#### **FR-011 — SVG asset support** 

Enable egui_extras SVG loader for suitable UI graphics and imported reference/preview imagery; clearly distinguish rasterized SVG display from editable parametric garment vectors. 

Acceptance: An SVG asset loads with a recoverable error path for invalid files. 

Priority: Must 

#### **FR-012 — PDF pattern export** 

Export full-scale pattern pieces with explicit mm units, calibration square, page sizing and multi-page tiling/overlap and registration marks if needed. 

Acceptance: Printed measured reference square and designated pattern dimensions meet approved physical tolerance. 

Priority: Must 

#### **FR-013 — DXF vector export** 

Export linework and curve approximations/representations with unit declaration and stable layers for cut/stitch/markings, using a documented DXF subset. 

Acceptance: Import in a selected CAD viewer preserves dimensions, layer names and piece inventory. 

Priority: Must 

#### **FR-014 — Local persistence and recovery** 

Save a versioned structured document (candidate: JSON or binary with explicit schema) through native file dialogs and appropriate browser download/import mechanisms. Warn on unsaved modifications. 

Acceptance: Round-trip tests compare semantic document data; malformed or newer documents fail gracefully. 

Priority: Must 

#### **FR-015 — Undo/redo** 

Support undo/redo for changed measurements, fit settings and layout operations (if piece positioning is implemented). 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

Acceptance: Undo and redo yield correct state and preview without stale geometry. 

Priority: Should 

#### **FR-016 — Accessible interaction** 

UI shall expose accessible labels and keyboard navigation for core workflows using eframe accesskit where supported; errors are text as well as colour. 

Acceptance: Keyboard-only test covers project creation to export; test supported accessibility paths on each target. 

Priority: Must 

#### **FR-017 — Error handling and user feedback** 

Validation, storage, export and GPU failures shall return typed recoverable errors, visible messages and logs without deliberate process termination. 

Acceptance: Fault injection demonstrates safe rejection or supported fallback and no UI-authored panic. 

Priority: Must 

#### **FR-018 — Built-in checks** 

Provide a project validation summary: unit consistency, geometry validity, bounding boxes, seam offsets, missing markings and printable page allocation. 

Acceptance: All failed checks are traceable to piece/field with actionable description. 

Priority: Should 

## **5. External interface requirements** 

#### **5.1 UI areas** 

Application shell: top project menu/status; left measurement and garment controls; central GPU document canvas; right piece/properties pane; bottom warnings/zoom/status; modal export workflow. Support keyboard focus, tooltips, units and consistent error markers. Screen layout is indicative, not approved visual design. 

#### **5.2 File and interchange interfaces** 

Project file: include schema_version, application version, template ID/version, measurements and unit, fit settings, user adjustments, piece placement, and metadata. PDF: use real physical paper dimensions and annotated print scaling; DXF: specify units and an interoperable entity subset. No remote service is required for baseline operation. 

#### **5.3 Rendering interface** 

Renderer input shall be a read-only snapshot of scene nodes and their transforms. Renderer shall handle surface resize, device-pixel ratio and loss/re-creation of GPU resources where supported by eframe/wgpu. Pixel-grid threshold is evaluated on effective canvas zoom, not monitor DPI alone. 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

## **6. Non-functional requirements** 

#### **NFR-001 — Rust implementation** 

Application-authored executable code and document generation logic must be Rust; WGSL is the explicitly selected shader language. Web shell glue generated by Trunk/tooling is allowed; no separately coded JS application logic is required. 

#### **NFR-002 — Version compatibility** 

Pin a coherent 0.36.x family for eframe/egui/egui_wgpu/egui_extras; Cargo.lock must be checked in for application builds. Feature selection must be tested separately for native and wasm. 

#### **NFR-003 — Safety lint policy** 

Apply #![forbid(unsafe_code)] for every workspace crate and configure the UI crate to deny clippy::unwrap_used, clippy::expect_used and clippy::panic (plus explicit no unwrap/expect/panic calls). CI must exercise appropriate clippy gates. 

#### **NFR-004 — Size budget** 

Web .wasm artifact shall be strictly below 25 MiB under the hosting limit; measure the actual output and publish a CI measurement. Do not assume wasm-release naming alone activates size tuning. 

#### **NFR-005 — Reproducibility** 

A fixed template/version, physical measurements and configuration generate deterministic semantic geometry on desktop and web within a documented numerical tolerance. 

#### **NFR-006 — Accuracy** 

Target dimensional, curve fit and printed-scale tolerances must be approved by a tailoring expert before release; no unverified “perfect fit” claim is permitted. 

#### **NFR-007 — Responsiveness** 

Proposed initial targets: input validation feedback within 100 ms and preview update within 250 ms for reference skirt on stated reference hardware. Targets remain proposed until benchmarked. 

#### **NFR-008 — Usability/accessibility** 

All essential actions use keyboard and visible text labels; available accesskit semantics should be populated; colour shall never be the sole failure indicator. 

#### **NFR-009 — Privacy/security** 

Keep body measurements local by default. No network telemetry without consent. Validate imported data lengths and formats; prevent path traversal and resource exhaustion through bounded parsing where applicable. 

#### **NFR-010 — Compatibility** 

Proposed baseline: Windows native, Linux native, current Chromium/Firefox with supported wgpu pathway; exact matrix to be confirmed. WebGPU availability and fallback must be tested rather than assumed. 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

#### **NFR-011 — Reliability** 

Failures in imports, exports, permissions, and rendering should not corrupt saved project files. Prefer atomic replacement for native save where supported. 

#### **NFR-012 — Maintainability** 

Separate UI, drafting/domain, exports and GPU canvas in crates; cover formulas by golden/reference tests; document method versioning and geometry units. 

#### **NFR-013 — Print integrity** 

PDF shall include a “100% / actual size” instruction and measurable calibration markers. End-user responsibility to disable printer scaling must be communicated. 

## **7. Data requirements and constraints** 

|**Entity**|**Principal fields / constraints**|
|---|---|
|**Project**|ID; title; created/updated times; schema_version;<br>template reference|
|**MeasurementSet**|unit; named typed lengths; source/manual tag;<br>validation outcome|
|**GarmentTemplate**|stable ID; drafting-method ID/version; required<br>measures; supported pieces|
|**PatternPiece**|piece ID; name; cut count; grainline; stitch/cut<br>paths; annotations|
|**StyleSettings**|ease; seam allowance; optional style variations<br>with supported limits|
|**ViewState**|zoom, pan, selected piece; not authoritative<br>geometry|
|**ExportOptions**|paper size, margin, tiling overlap, units, output<br>format|



All geometric calculations shall use explicitly defined length units and finite floating-point values; document conversions at the I/O boundary. Store original inputs and regenerated output provenance. Formula changes require method/template version migration, never silent overwriting. 

## **8. Detailed use cases** 

#### **UC-01 Create pattern** 

Pre: application loaded. Main: create project, select validated garment, input values, validate, generate. Alt: missing/invalid measurements yield field errors. Post: unsaved editable document with reproducible geometry. 

#### **UC-02 Adjust fit** 

Pre: valid document. Main: change ease/seam allowance, calculate and preview changes, display affected piece dimensions. Alt: unsupported parameter ranges are rejected. Post: updated undoable configuration. 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

#### **UC-03 Save and reopen** 

Pre: document open. Main: choose storage location, save versioned file, reopen, migrate if supported, reconstruct geometry. Alt: damaged/unsupported version shows recoverable diagnostic. 

#### **UC-04 PDF export** 

Pre: geometry passes validity. Main: choose paper size, actual-size setting and tiling; export PDF. Alt: offpage geometry warns or tiles; write failures recover gracefully. 

#### **UC-05 DXF export** 

Pre: supported geometry passes checks. Main: select DXF flavour/layers and units, export, verify in external CAD. Alt: unsupported entities convert with recorded approximation tolerance or block export. 

#### **UC-06 Navigate canvas** 

Pre: document open. Main: zoom, pan, fit selection, inspect dimension overlay. Canvas visual effects respond to zoom. Alt: GPU resource failure displays error; preserve document. 

## **9. Verification and acceptance plan** 

|**Test ID**|**Test method**|**Pass condition**|
|---|---|---|
|**T-001**|Unit/validation: empty, NaN,<br>infinite, negatives and extremes|Invalid entries cannot produce<br>pattern output|
|**T-002**|Golden draft: compare<br>constructed points and piece<br>outlines|Matches signed-off manual<br>reference within tolerance|
|**T-003**|Metamorphic: modify one<br>measurement|Expected dimensions change;<br>unrelated settings preserved|
|**T-004**|Geometry fuzz/property-based<br>tests|No non-finite or invalid paths for<br>accepted range|
|**T-005**|Native vs WASM numerical<br>comparison|Geometry within agreed<br>tolerance|
|**T-006**|PDF print and measure|Calibration mark and specified<br>edges within signed-off tolerance|
|**T-007**|DXF reimport in CAD viewer|Unit scale, layers and piece sizes<br>preserved|
|**T-008**|Zoom 7.99× versus 8×|Pixel grid toggles at defined<br>threshold|
|**T-009**|GPU visuals/shader snapshot|Shadow/checkerboard/filter<br>modes observable; exports<br>unchanged|
|**T-010**|Clippy, compile, review|Workspace unsafe forbidden; UI<br>unwrap/expect/panic denied|
|**T-011**|Web artifact budget|Emitted .wasm measured <25<br>MiB|
|**T-012**|Keyboard/a11y workflow|Required tasks achievable<br>withoutpointer on supported|



AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

|||target|
|---|---|---|
|**T-013**|Corrupted/imported file and save|Clear error, no crash or silent|
||failure|document corruption|



## **10. Suggested repository and build conventions** 

Cargo workspace: crates/domain, drafting, geometry, document, export, canvas, ui, app. Shared logic must avoid binding to native-only file I/O. Keep renderer backends isolated from formulas; conditionally include platform adapters for native dialogs and web download/upload. 

Suggested Cargo profiles (illustrative; benchmark needed): [profile.wasm-release] inherits = "release"; optlevel = "z"; lto = true; codegen-units = 1; panic = "abort"; strip = "symbols". A “panic = abort” compiledruntime setting does not authorize use of panic!, expect or unwrap in UI code. Check output path/config with actual Trunk version. Add size checks after compilation. 

Suggested quality gates: cargo fmt --check; cargo clippy --workspace --all-targets with locked policy; cargo test --workspace; target-specific cargo check; Trunk build; artifact size assertion; golden-pattern tests; manual print validation. 

## **11. Dependencies, assumptions, risks** 

|**ID**|**Risk / dependency**|**Mitigation**|
|---|---|---|
|**R-01**|No approved garment formulas<br>provided|Select published/established<br>method; version formulas; obtain<br>tailor sign-off|
|**R-02**|GPU/WASM differences,<br>unsupported browsers|Test capability matrix; preserve<br>safe error/fallback behaviour|
|**R-03**|Large wgpu/WASM dependency<br>size|Audit features, profile size,<br>remove unused loaders, measure<br>CI <25 MiB|
|**R-04**|Print scaling varies across<br>browsers/printers|Calibration square and<br>instructions; physical reference<br>test|
|**R-05**|Unsafe requirement<br>misunderstood as third-party<br>guarantee|Explicitly limit forbid to<br>workspace code; audit<br>dependencies separately|
|**R-06**|SVG loader confused with<br>editable vectors|Model native editable geometry;<br>SVG images as assets/preview<br>only|
|**R-07**|Pressure to support many<br>garments|Release one expert-validated<br>garment block before expanding|
|**R-08**|PDF/DXF libraries may differ in<br>WASM support|Prototype required export paths<br>early and test both targets|



AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

## **12. Open decisions before v1.0 baseline approval** 

OD-01: Which named drafting textbook/standard, specific skirt style and measurement definitions will be authoritative? 

OD-02: Exactly which pattern pieces, darts, notches, grainlines and labels are mandatory for the reference skirt? 

OD-03: Minimum/maximum supported measurement ranges and expert-approved numeric tolerance? 

OD-04: Is the 25 MiB host limit applied to raw .wasm, compressed transfer, or full deployable site? Default acceptance assumes raw emitted .wasm. 

OD-05: Which native OS/browser/device matrix and whether WebGPU fallback to WebGL is permissible with the selected wgpu configuration? 

OD-06: Which project file format/versioning rules, PDF writer and DXF subset are selected? 

OD-07: Are local-only projects sufficient, or is account sync a future requirement? 

OD-08: Should the pixel grid apply to the whole pattern vector view or specifically raster reference imagery? 

OD-09: What are acceptance-performance targets on the actual development laptop? 

OD-10: Who signs off the printed reference patterns as correct? 

## **13. Traceability and delivery milestones** 

|**Milestone**|**Required demonstrable**<br>**evidence**|**Requirements**|
|---|---|---|
|**M1: requirements & method**|Signed-off measurement schema<br>and paper reference draft|FR-002–007; NFR-006|
|**M2: Rust drafting kernel**|Geometry generated by Rust<br>unit/golden tests|FR-003–008|
|**M3: native UI & GPU**|eframe/wgpu callback with 8×<br>pixel grid and styling|FR-009–011|
|**M4: documents**|Saved project; dimensionally<br>correct PDF and DXF|FR-012–014|
|**M5: browser release**|Trunk wasm-release build under<br>host budget|NFR-001–005; NFR-010|
|**M6: validation**|Tailor comparison, print test,<br>accessibility/safety checks|FR-016–018; NFR-006–013|



## **14. Acceptance criteria for first academic release** 

- A tailor can enter a valid reference measurement set and obtain a complete, labelled straight-skirt pattern conforming to the approved reference methodology. 

- Pattern calculation, save/reopen, PDF export and DXF output have passing verification evidence and real-size print checks. 

- Same Rust domain layer runs for native and web (platform-dependent interactions may differ). 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

- Required WGSL canvas effects, zoom behaviour, accessibility features and lint policy are demonstrably enforced. 

- The WASM result is measured strictly below the agreed 25 MiB hosting limit; if not, the web build is not accepted as compliant. 

- No claim of universal perfect fit is made without sample-based fitting validation. 

## **Appendix A. Normative and supporting references** 

User-supplied “Final year project .docx”, 2 pages (conceptual basis); developer’s 8 October 2026 stack and safety constraints. Supporting implementation references: docs.rs/eframe, docs.rs/egui_extras (svg feature), docs.rs/egui-wgpu. Formal source for the garment drafting methodology remains to be selected. 

AGPGS • SRS v1.0 DRAFT • 08 Oct 2026 

