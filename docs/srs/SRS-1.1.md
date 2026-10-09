**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

# Software Requirements Specification 

Automated Garment Pattern Generation System  •  Revision 1.1 

Document status: DRAFT — review with tailoring/pattern-making domain expert before approving formula and export claims. 

|**Control**|**Value**|
|---|---|
|**Baseline**|Revision of SRS 1.0 (18 FR, 13 NFR)|
|**Implementation policy**|Rust-first: eframe/egui 0.36, wgpu, egui_wgpu, WGSL,<br>egui_extras SVG, AccessKit, Trunk/WASM|
|**Scope**|2D measurement-driven garment pattern drafting and<br>output; material preview is non-physical|
|**Target clients**|Independent tailors, pattern makers, small tailoring<br>workshops|
|**Release**|R1 thesis MVP; R2 professional workflow; R3<br>experimental enhancements|



## **1. Revision strategy and evidence** 

SRS 1.0 supplied the project model and base functionality. SRS 1.1 retains the same product intent and adds garment-industry construction detail, measurable acceptance tests, material/fabric tools, and technical boundaries. The uploaded final-year concept specified parametric geometry, user measurements and PDF/DXF; it did not specify complete drafting recipes, cloth physics or AI. Those are not silently assumed. 

|**Evidence class**|**Examples**|**Treatment**|
|---|---|---|
|**Prior baseline**|FR-001–FR-018, NFR-001–NFR-013|Retained, refined and<br>supplemented|
|**External reference**|Seamly/Valentina workflows:<br>measurements, seam allowances,<br>notches, grainlines, tiled PDF, DXF|Evidence of domain-relevant<br>features, not proof of specific client<br>need|
|**Proposed product decision**|Straight skirt first, millimetre<br>model, local project files|Validate with target users|
|**Future investigation**|3D draping, body scanning,<br>photorealistic textile simulation|Out of R1 scope|



## **2. Problem, users and outcomes** 

Tailors currently repeat measurement calculations and manual drafting for variations in body size. The system shall create traceable, editable 2D sewing pattern pieces from measurement sets and a documented drafting rulebook. It must yield dimensionally reliable PDFs and interchange files, not merely attractive screen graphics. 

|**User**|**Jobs to be done**|**Primary risks**|
|---|---|---|
|**Pattern maker**|Draft blocks, adjust<br>ease/darts/curves, prepare cut lines|Incorrect fit, unreliable offsets|
|**Tailor**|Maintain customer measurements,<br>print patterns, mark fabric|Wrong unit or printer scaling|



Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

|**Workshop operator**|Estimate fabric and organise<br>project versions|Waste, confusing layouts|
|---|---|---|
|**Evaluator / administrator**|Repeat tests, inspect formula<br>provenance and error cases|Unverifiable accuracy|



## **3. Scope and release policy** 

R1 (must complete): straight-skirt base pattern, front/back pieces, waist/hip shaping and darts per a selected documented method; units; customer measurement profiles; visible construction geometry; seam/cut line distinction; notches, grainline, fold marks, labels; interactive pan/zoom; offline save/open; print-scale tiled PDF and SVG; repeatable testing. PDF and SVG exporters must preserve physical units. 

R2 (professional): more blocks (bodice/trousers only once each method is validated); grading rules, DXF interoperability, fabric marker planning, material presets and 2D print/texture previews; advanced editing and batch size exports. 

R3 (exploratory): interactive 3D drape or cloth physics, scanned-body measurements, automatic fit prediction, cloud collaboration; each requires separate research, dataset and validation. 

## **4. Terminology and domain model** 

|**Term**|**Exact interpretation**|
|---|---|
|**Block / sloper**|Basic fitted construction geometry before style<br>adaptations|
|**Pattern piece**|Named 2D region cut from fabric, with orientation,<br>quantity, labels and construction marks|
|**Stitch line**|Intended seam-joining geometry; distinct from outer<br>cutting line|
|**Seam allowance**|Specified offset region from stitch line to cut line, with<br>corner/curve treatment|
|**Ease**|Garment dimension beyond body dimension, defined<br>by a selected method and desired fit|
|**Dart**|Wedge of fabric folded/stitched to shape fit; includes<br>legs, apex and intake|
|**Grainline**|Required pattern orientation relative to fabric yarn<br>direction|
|**Nap/directional print**|Fabric property affecting allowable placement and<br>rotation|
|**Marker**|Arrangement of pattern pieces on fabric width to<br>estimate length and waste|
|**Grade rule**|Explicit changes to pattern landmarks across sizes, not<br>just uniform scaling|



## **5. Core workflows** 

WF-01 New project: select drafting method/version → select garment block → enter profile → validate inputs → generate named front/back pieces → inspect method assumptions → adjust ease/darts → validate geometry → save 

Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

→ tiled-print/export. Every change to a measurement shall recompute all dependent landmarks or report why it cannot. 

→ WF-02 Fabric workflow (R2): select fabric width, nap/direction, shrinkage assumptions and print/texture preset place cut pieces according to grainline and cut quantity → estimate minimum required length with stated heuristic and waste → export marker report. Estimates must never be advertised as guaranteed consumption. 

WF-03 Revision: load an existing project, apply versioned migration if needed, change measurements, compare difference overlay, inspect warnings and regenerate outputs with revision metadata. 

## **6. Functional requirements — domain-critical** 

### **FR-019 — Measurement profiles** 

Requirement: Create anonymisable client measurement profiles with named measurement definitions, units, lastedited time and provenance (manual/estimated). Reject missing, impossible or contradictory data using rulespecific thresholds; do not invent unavailable anatomical measures. 

Acceptance: Invalid fields identify exact issue; a saved profile reloads identically. 

Priority: Must  |  Target: R1 

### **FR-020 — Formula provenance and diagnostics** 

Requirement: Bind every supported template to named pattern-drafting methodology ID/version; expose formula inputs, calculated key lengths and dependencies. An undocumented rule shall not silently become production output. 

Acceptance: User can inspect rulebook and measurements used to calculate a selected construction point. 

Priority: Must  |  Target: R1 

### **FR-021 — Full straight-skirt block** 

Requirement: Generate documented front and back skirt pieces with waistline, hipline, hemline, centre lines, required dart(s) and shaping governed by validated method and measurements. 

Acceptance: Domain expert reviews ten varied realistic measurements and signs off construction against reference drafts. 

Priority: Must  |  Target: R1 

### **FR-022 — Stitch versus cut geometry** 

Requirement: Represent base/stitch boundaries separately from cutting boundaries. Support per-edge allowance, joined-corner policy and concave/convex curves; warn on invalid offset self-intersections. 

Acceptance: Changing allowance modifies cut boundary only; severe self intersections block export. 

Priority: Must  |  Target: R1 

### **FR-023 — Dart operations** 

Requirement: Display dart legs, apex, intake and fold direction; update dart geometry when measurements or style settings change, subject to drafting method. 

Acceptance: Dart labels remain valid and shape rules pass method-specific tests. 

Priority: Must  |  Target: R1 

Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

### **FR-024 — Construction marks** 

Requirement: Add grainline, fold indicator, notches, drill points (when appropriate), piece names, cut quantity, size, method/version, and orientation. 

Acceptance: Every exported R1 piece includes required legends and marks at correct dimensions. 

Priority: Must  |  Target: R1 

### **FR-025 — Manual drafting tools** 

Requirement: Enable point selection, move constrained points, dimension rulers, snapping, offset, mirror and curve-handle edits, with constraint warnings. Direct edits should be stored as overrides distinct from formuladriven geometry. 

Acceptance: Undoing a manual edit restores exact preceding geometry; constraints stay inspectable. 

Priority: Should  |  Target: R2 

### **FR-026 — Curve construction** 

Requirement: Support cubic Bézier and line segments, continuity inspection and an adaptive tessellation path independent of rendered zoom. Avoid representing a curve only by raster pixels. 

Acceptance: Reference curve tests cover join tangents, low/high zoom and export fidelity. 

Priority: Must  |  Target: R1 

### **FR-027 — Size grading** 

Requirement: Apply declared per-landmark grade rules for defined sizes; reject naïve uniform scaling as a substitute for pattern grading. 

Acceptance: Given approved rule tables, resulting landmark dimensions match within set tolerances. 

Priority: Should  |  Target: R2 

### **FR-028 — Pattern comparison** 

Requirement: Overlay two versions or two measurement profiles; show dimension deltas and highlight landmark movement. 

Acceptance: The revision summary lists changed input, method and affected pieces. 

Priority: Should  |  Target: R2 

### **FR-029 — Material library** 

Requirement: Store fabric label, width, weave/knit class, stretch direction/percentage when known, shrinkage allowance, nap/one-way design flag and optional swatch image. These fields are informational and used only where the relevant algorithm supports them. 

Acceptance: Fabric choice cannot quietly change body measurements; user can see assumptions. 

Priority: Should  |  Target: R2 

### **FR-030 — Material visualization** 

Requirement: Display a 2D swatch or pattern fill preview with scale, orientation and opacity controls; enable userprovided PNG/JPEG/SVG images through permitted decoders. Describe results as visual approximation, not physical drape prediction. 

Acceptance: A printed dimension remains unchanged when the material texture or sampling mode changes. 

Priority: Should  |  Target: R2 

Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

### **FR-031 — Fabric layout and consumption** 

Requirement: Arrange pieces within selected fabric width respecting grainlines, fabric fold, piece count, mirrored pairs, nap and rotation constraints; report total layout length and waste ratio. 

Acceptance: All placed pieces remain within width and do not overlap (apart from explicitly permitted shared folds). 

Priority: Should  |  Target: R2 

### **FR-032 — Printable tiled PDF** 

Requirement: Support page size A4/Letter, physical millimetre coordinates, tile overlap, page numbers, registration targets and a 100 mm calibration square; force 100% / actual-size print instructions. 

Acceptance: Measure printed calibration square at 100 mm within agreed printer tolerance, with no missing piece edges. 

Priority: Must  |  Target: R1 

### **FR-033 — SVG vector export** 

Requirement: Produce dimensioned SVG using physical units and a predictable coordinate origin; include layer/group IDs and machine-readable metadata without rasterising geometric boundaries. 

Acceptance: Automated parser validates bounding boxes, paths and unit metadata. 

Priority: Must  |  Target: R1 

### **FR-034 — DXF interchange** 

Requirement: Export named geometry layers for stitch, cut, annotations, grainline and notches; document supported DXF dialect and units. Verify against at least two selected external CAD viewers. 

Acceptance: Round-trip measurement and entity tests pass; unsupported constructs report limitations. 

Priority: Should  |  Target: R2 

### **FR-035 — Offline project package** 

Requirement: Save human-readable metadata, versioned geometry/rule references, measurements and optional assets as one user-transferable file or directory. Use atomic writes for native and user-mediated browser downloads. 

Acceptance: Opening after restart reconstructs the same pattern; crash during save does not destroy last valid version. 

Priority: Must  |  Target: R1 

### **FR-036 — Audit and warnings** 

Requirement: Record warnings for missing measurements, invalid contours, excessive offset curvature, impossible grading changes, insufficient fabric width and unknown formula sources; show remediation text. 

Acceptance: Critical errors prevent misleading export; recoverable warnings remain visible. 

Priority: Must  |  Target: R1 

### **FR-037 — Measurement and pattern privacy** 

Requirement: Default to offline local use with no telemetry or cloud transmission; give delete/export controls for customer measurements. 

Acceptance: Network-blocked app can draft, save and export; no measurement data leaves device without explicit action. 

Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

Priority: Must  |  Target: R1 

### **FR-038 — Localization and measurement conversion** 

Requirement: Display metric by default and optional inches while maintaining a single internal canonical unit. Avoid binary-float text conversion drift in stored values. 

Acceptance: Roundtrip unit conversion stays within 0.1 mm for representative measurements. 

Priority: Should  |  Target: R2 

### **FR-039 — Keyboard and accessibility workflow** 

Requirement: Expose labelled data fields, keyboard zoom/pan equivalents, error focus, contrast themes and AccessKit semantic controls; provide non-canvas numerical representation of critical dimensions. 

Acceptance: Key functionality usable without pointer; status announcements readable through accessibility APIs. 

Priority: Must  |  Target: R1 

### **FR-040 — Templates and constraints** 

Requirement: Allow new garment definitions through versioned Rust rule implementations and validated declarative settings; do not execute user-provided arbitrary code inside project files. 

Acceptance: Unknown template IDs fail safely with actionable error. 

Priority: Must  |  Target: R1 

## **7. Render and texture specifications** 

### **REN-01 — Authoritative geometry** 

Requirement: All physical pattern calculations shall use canonical millimetre-space Rust geometry with f64 (or a documented exact representation); the wgpu shader shall not define the authoritative sewing measurements. 

Acceptance: CPU output identical within test tolerance across native and WASM. 

Priority: Must  |  Target: R1 

### **REN-02 — GPU canvas separation** 

Requirement: egui_wgpu paint callback shall draw viewport effects (shadow, checkerboard, zoomed texture, grid) and may draw tessellated vector pieces; it must not silently modify exported vectors. 

Acceptance: A render-only sampling mode change produces byte-equivalent geometry export. 

Priority: Must  |  Target: R1 

### **REN-03 — Zoom/sampling** 

Requirement: Support bilinear and trilinear texture sampling with explicit mipmap generation where required, and show 1 px image pixel grid at 8× zoom and above for raster assets only. 

Acceptance: Switching mode visibly affects raster preview while measured coordinates stay constant. 

Priority: Must  |  Target: R1 

### **REN-04 — Textile swatch orientation** 

Requirement: When a user supplies a texture, offer scale in real units, rotation and repeat preview; mark stretch/nap metadata separately from the image. 

Acceptance: The preview legend displays physical repeat dimensions and does not imply simulated stretch. 

Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

Priority: Should  |  Target: R2 

## **8. Technical architecture and language policy** 

Primary development language: Rust for desktop, WASM and pattern mathematics. WGSL is required for the custom GPU shader and is not a replacement for Rust. TOML defines Cargo build configuration; JSON or a documented binary format stores projects; SVG is an output/asset vector format; HTML/CSS provide minimal Trunk web host structure. No Python, JavaScript or C++ runtime is required for the proposed product. A separate Python/TypeScript research harness may be used solely for independent numeric/export verification if desired, but is not a production dependency. 

|**Crate / layer**|**Responsibility**|**Compatibility rule**|
|---|---|---|
|**pattern_core**|Units, coordinates, dependencies,<br>drafting rules and geometry<br>validation|No egui/wgpu, no filesystem<br>requirement|
|**pattern_templates**|Selected method-specific garment<br>blocks and grading rules|Deterministic inputs/outputs|
|**pattern_document**|Projects, versioning, migration and<br>undo/redo|No UI types in file schema|
|**pattern_export**|SVG, PDF, DXF adapters and print<br>checks|Vector-first; test native and WASM<br>separately|
|**pattern_material**|Fabric metadata, repeats and R2<br>layout heuristics|No claims of physics|
|**pattern_ui**|eframe/egui 0.36, accesskit and<br>egui_extras SVG loader|Deny unwrap_used / expect_used /<br>panic via Clippy policy|
|**pattern_render**|egui_wgpu paint callback, wgpu<br>and WGSL shader|Graceful fallback and texture limits|
|**pattern_web**|Trunk host and wasm-release build|Built output under 25 MiB target<br>host constraint|



Suggested cargo profile (validate against real build): [profile.wasm-release] inherits="release"; opt-level="z"; lto="fat"; codegen-units=1; strip="debuginfo"; panic="abort". The profile alone does not guarantee a 25 MiB file. CI shall measure the actual hosted WASM artifact and fail if it exceeds 25 MiB; monitor whole deployment payload separately. 

Workspace policy: #![forbid(unsafe_code)] on workspace-owned crates (and workspace lints for members); Clippy denies unwrap_used, expect_used and panic in UI crate. These lint restrictions apply to first-party source, not automatically to compiled third-party dependencies. All Result handling must expose errors or propagate them. Shader inputs and file assets must be validated against bounds. 

## **9. Nonfunctional requirements and quantitative verification** 

|**ID**|**Metric / policy**|**Verification**|
|---|---|---|
|**NFR-014**|Draft replay deterministic for same<br>template version and inputs|Golden-case coordinate checksum /<br>tolerance|
|**NFR-015**|Export physical length error<br>0.1<br>≤<br>mm digital geometry; external<br>print verified separately|Compare path length and<br>calibration square|
|**NFR-016**|No silently incomplete exported|Self-intersection and closure tests|



Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

||pattern; blocking error on invalid<br>contour||
|---|---|---|
|**NFR-017**|UI update target<br>150 ms for base<br>≤<br>skirt changes on reference<br>hardware; no UI freeze >500 ms|Performance telemetry local bench|
|**NFR-018**|Offline all core drafting actions;<br>browser local download/open<br>supported|Disable network then execute test<br>suite|
|**NFR-019**|WASM artifact below 25 MiB after<br>production optimisation|CI size gate|
|**NFR-020**|100% of loaded project files version<br>checked; reject unknown schema<br>safely|Schema upgrade tests|
|**NFR-021**|Crash-safe native saves and<br>deterministic file recovery|Fault injection|
|**NFR-022**|No unwrap/expect/panic in UI and<br>no unsafe in own workspace|cargo clippy and deny policy|
|**NFR-023**|Minimum reference browser and<br>operating system matrix explicitly<br>recorded prior to release|Cross-platform tests|
|**NFR-024**|No undisclosed image upload,<br>analytics or remote measurement<br>storage|Network trace offline test|
|**NFR-025**|Accessible measurement form and<br>export flow via keyboard and<br>semantic tree|Manual accessibility evaluation|



All timing, accuracy and performance values in this revision are proposed acceptance targets pending a recorded hardware/printer baseline. The choice of method and template may make tighter or looser engineering tolerances appropriate. 

## **10. Conceptual data objects** 

|**Entity**|**Required fields**|
|---|---|
|**MeasurementProfile**|id, label, measurement_definition_version,<br>values_mm, source, updated_at|
|**DraftMethod**|method_id, version, citation/source, required_inputs,<br>constraints|
|**PatternProject**|id, version, garment_type, method_ref, profile_ref,<br>style_settings, assets|
|**PatternPiece**|id, label, closed_stitch_path, cut_path, grainline,<br>cut_quantity, annotations|
|**PatternOperation**|timestamp/revision, operation_type, parameters,<br>affected IDs|
|**FabricSpec**|name, usable_width_mm, nap, fold, shrinkage, repeat<br>size, swatch_ref|
|**ExportJob**|target format, units, media, profile, warnings, output<br>checksum|



Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

## **11. Acceptance test scenarios** 

|**Test**|**Trigger**|**Expected outcome**|
|---|---|---|
|**AT-01**|Enter valid measurement set|Two named skirt pieces with<br>required marks and complete<br>warnings list|
|**AT-02**|Change hip circumference|Formula-linked waist/hip/side<br>geometry updates; ruler changes by<br>expected amount|
|**AT-03**|Delete mandatory measure|Generation disabled with specific<br>missing-field prompt|
|**AT-04**|Increase seam allowance|Cut path changes, stitch path stays<br>identical|
|**AT-05**|Preview textile at 3 repeat scales|Canvas changes but SVG/PDF<br>geometry unchanged|
|**AT-06**|Save file and reopen|Project data and geometric output<br>match before/after|
|**AT-07**|Export tiled PDF|A4/Letter pages have alignment<br>marks and 100 mm scale box|
|**AT-08**|Change native vs WASM target|Same reference pattern dimensions<br>and warnings|
|**AT-09**|Reject self-intersecting offset|Blocking message; no silently<br>corrupted export|
|**AT-10**|Open unsupported schema version|Safe failure, source file not<br>modified|
|**AT-11**|Attempt invalid DXF construct|Export reports unsupported feature<br>rather than dropping geometry|
|**AT-12**|Put grainline-constrained marker<br>on narrow fabric|Explicit failure or constrained<br>alternative; no overlap|



## **12. Dependencies and risks** 

|**Risk**|**Mitigation / exit criterion**|
|---|---|
|**Missing recognised pattern-making formulas**|Select one published/qualified method and obtain<br>expert reference patterns before claiming fit|
|**Unsupported PDF/DXF libraries on WASM**|Prototype libraries early; if browser export cannot<br>run, narrow supported format and disclose|
|**Complex seam allowance corner cases**|Geometric offset verification including concavities and<br>rounded corners|
|**WASM 25 MiB package cap**|Use feature gating, optimisation and CI budget; no<br>promise without build measurement|
|**Overengineering graphics before pattern**<br>**correctness**|Finish CPU geometry and printing baseline before<br>material shader polish|
|**Textured preview mistaken for garment fit**<br>**simulation**|Label 2D visual preview and never claim drape<br>validation|
|**Commercial exchange compatibility**|Test representative file imports in nominated CAD|



Draft for academic and client validation  •  08 October 2026 

**GARMENT PATTERN GENERATION SYSTEM  |  SRS 1.1** 

software, publish profile 

## **13. Build order / project management gates** 

|**Gate**|**Deliverable**|**Pass condition**|
|---|---|---|
|**G0 Domain baseline**|Method references, measurement<br>list, 10 manual reference samples|Qualified pattern-maker agrees on<br>equations and measurement<br>conventions|
|**G1 Geometry engine**|Rust coordinate graph, curves and<br>skirt blocks|Golden numerical tests pass|
|**G2 Construction**|Allowances, darts, notches, piece<br>labelling|Construction and offset tests pass|
|**G3 App UI**|egui controls, project save, vector<br>canvas|Full offline workflow demonstrated|
|**G4 Export**|SVG + tiled PDF, physical<br>calibration|Print acceptance achieved|
|**G5 Web build**|Trunk WASM and shader fallback<br>tests|Browser tests + <25 MiB CI gate|
|**G6 R2 professional**|DXF, grading, marker and fabric<br>swatch|Domain-specific interoperability<br>tests pass|



## **14. Explicit exclusions and unresolved decisions** 

Not promised in v1.1: automatic perfect fit, physically simulated fabric behavior, photoreal 3D clothing, bodycamera scanning, AI recommendations, printer-driver control or guaranteed lowest-waste fabric nesting. These are separate possible products/research tracks and must not be implied by pattern preview. 

Approval required: (1) reference garment pattern drafting textbook/method and permission where applicable; (2) exact skirt measurement definitions and minimum set; (3) tailoring population/size ranges; (4) offline storage model and customer consent; (5) reference printer and print tolerance; (6) priority of native versus browser export; (7) selected DXF standard/compatible tools; (8) initial host target and asset cap; (9) whether users require multi-language UI; (10) formal ownership of fit validation. 

## **15. Public sources checked for v1.1 feature plausibility** 

- Final year project .docx (user-supplied): parametric geometry, measurement to vector pattern and staged curves/ease development. 

- SRS v1.0 (user-supplied): base architecture, functional/nonfunctional baseline and Rust toolchain decisions. 

- Seamly documentation index: https://wiki.seamly.io/wiki/Main_Page/en (measurements, drafting, curves and allowances). 

- Valentina software release notes: https://gitlab.com/smart-pattern/valentina/-/releases (grainline, seam allowance, printable PDF, DXF interoperability). 

- egui_extras 0.36.2 docs: https://docs.rs/egui_extras/latest/egui_extras/ (SVG loader support). 

- eframe 0.36 docs: https://docs.rs/crate/eframe/latest (native/WASM and wgpu backend). 

Review note: The cited project source is conceptual. Production drafting equations must be derived and tested from a separately identified drafting authority; do not convert illustrative formulas into default manufacturing values. 

Draft for academic and client validation  •  08 October 2026 

