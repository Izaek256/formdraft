# Software Requirements Specification: Automated Garment Pattern Generation System

Revision 1.2 · Draft for academic and client validation · 09 October 2026

## Document control

| Control | Value |
| --- | --- |
| Baseline | Revision of SRS 1.1 (08 October 2026), itself a revision of SRS 1.0 (18 FR, 13 NFR). FR-001 to FR-018 and NFR-001 to NFR-013 of SRS 1.0 are retained unchanged. |
| Document status | DRAFT. Every drafting rule and export claim requires review by a qualified pattern maker before approval. |
| Drafting method | Winifred Aldrich, Metric Pattern Cutting, 4th edition (Blackwell Publishing, 2004, ISBN 1-4051-0278-0). Tailored skirt block with the straight skirt adaptation. See Section 5. |
| Implementation policy | Unchanged from 1.1. Rust-first: eframe/egui 0.36, wgpu, egui\_wgpu, WGSL, egui\_extras SVG, AccessKit, Trunk/WASM. |
| Scope | 2D measurement-driven garment pattern drafting and output. Material preview is non-physical. |
| Target clients | Independent tailors, pattern makers, small tailoring workshops. |
| Releases | R1 thesis MVP; R2 professional workflow; R3 experimental enhancements. |

### Summary of changes from 1.1 to 1.2

1. The drafting method is selected and pinned to one edition (new Section 5). Approval items 1 and 2 are closed in part (Section 15).
2. Skirt scope is clarified. The tailored skirt block is the block; the straight skirt is an adaptation of it (FR-021, new FR-041).
3. The minimum measurement set is fixed at four values, each carrying a source tag. Waist-to-hip may be chart-derived (FR-019, new FR-042).
4. Provenance is extended with edition, page, rule kind and verifier (FR-020, Section 11).
5. The block is net (no seam allowance). Seam allowance defaults come from the book's general guide (FR-022).
6. The construction-marks list is aligned with the book's pattern-instruction list, and style number is added (FR-024).
7. Curves the book gives only qualitatively must be mapped to documented numeric interpretations (FR-026).
8. Grading is defined as per-point increment tables (FR-027).
9. Outputs carry a toile-verification statement (new FR-043).
10. Rule constants are stored as exact decimals (REN-01).
11. Acceptance tests AT-13 to AT-21 are added, gate G0 deliverables are made specific and backed by a rule register workbook (Appendix A), the reference set composition is fixed, risks are added, and six approval items are added.

Amendment of 09 October 2026 (revision number unchanged): (a) unit support is made global. FR-038 becomes Must for R1 and supports mm, cm, m, in, ft and yd through one unit service, with new NFR-028, AT-20, AT-21 and a UnitPreference entity. (b) Appendix A defines the G0 rule register workbook and the composition of the ten reference skirts. (c) G0 requires every rule to be checked against the book's figures as well as its text. (d) Approval items 15 (display units) and 16 (code-versus-hand tolerance) are added. (e) A unit-confusion risk is added.

Requirements revised in 1.2 are marked (revised 1.2). New requirements are marked (new 1.2). All other requirements are unchanged from 1.1.

## 1. Revision strategy and evidence

SRS 1.0 supplied the project model and base functionality. SRS 1.1 added garment-industry construction detail, measurable acceptance tests, material tools and technical boundaries. SRS 1.2 binds the product to a specific, published drafting method so that rules can be implemented, verified and traced. Nothing in this revision claims that the method guarantees fit. The method source itself requires a calico toile to verify fit (Section 5.7).

| Evidence class | Examples | Treatment |
| --- | --- | --- |
| Prior baseline | FR-001 to FR-040, NFR-001 to NFR-025 | Retained, refined and supplemented |
| Selected method (new 1.2) | Aldrich, Metric Pattern Cutting, 4th ed. (2004): size charts, tailored skirt block, straight skirt adaptation, seam allowance guide, pattern instructions, grading, individual figures | Source of drafting rules. Each rule is transcribed, interpreted where needed, and verified by a pattern maker. |
| External reference | Seamly/Valentina workflows: measurements, seam allowances, notches, grainlines, tiled PDF, DXF | Evidence of domain-relevant features, not proof of specific client need |
| Proposed product decision | Millimetre model, local project files, default seam allowances | Validate with target users |
| Future investigation | 3D draping, body scanning, photorealistic textile simulation, 6th edition blocks | Out of R1 scope |

## 2. Problem, users and outcomes

Tailors repeat measurement calculations and manual drafting for variations in body size. The system shall create traceable, editable 2D sewing pattern pieces from measurement sets and a documented drafting rulebook. It must yield dimensionally reliable PDFs and interchange files, not merely attractive screen graphics.

| User | Jobs to be done | Primary risks |
| --- | --- | --- |
| Pattern maker | Draft blocks, adjust ease and darts, prepare cut lines | Incorrect fit, unreliable offsets |
| Tailor | Maintain customer measurements, print patterns, mark fabric | Wrong unit or printer scaling |
| Workshop operator | Estimate fabric and organise project versions | Waste, confusing layouts |
| Evaluator / administrator | Repeat tests, inspect formula provenance and error cases | Unverifiable accuracy |

## 3. Scope and release policy

**R1 (must complete):** the tailored skirt block (front and back) and the straight skirt adaptation per Section 5; waist and hip shaping and darts per the selected method; four-measurement customer profiles with source tags; standard size chart lookup for waist-to-hip; global unit support for every length (mm, cm, m, in, ft, yd) over one canonical millimetre model; visible construction geometry; seam allowance and cut line distinct from the net stitch line; notches, grainline, fold marks, labels and toile statement; interactive pan and zoom; offline save and open; print-scale tiled PDF and SVG; repeatable testing. PDF and SVG exporters must preserve physical units.

**R2 (professional):** other skirt adaptations from the same chapter (A-line first candidate; panel, pleated, gored and flared variants each need their own rule transcription and verification); waistbands; bodice and trouser blocks only once each method is validated; grading using the book's increment tables; DXF interoperability; fabric marker planning; material presets and 2D print and texture previews; advanced editing and batch size exports.

**R3 (exploratory):** interactive 3D drape or cloth physics, scanned-body measurements, automatic fit prediction, cloud collaboration, adoption of a later edition of the method. Each requires separate research, dataset and validation.

## 4. Terminology and domain model

| Term | Exact interpretation |
| --- | --- |
| Block / sloper | Basic fitted construction geometry before style adaptations |
| Tailored skirt block | The method's block for a separate skirt not attached to a bodice. It carries less hip ease than the bodice-attached skirt and moves the side seam forward. |
| Straight skirt (adaptation) | A pattern derived from the tailored skirt block by an optional slight centre back swing and an optional small hem flare at the side seams. Both may be zero. |
| Net pattern | A pattern without seam allowance. Seam allowance is added after drafting. |
| Pattern piece | Named 2D region cut from fabric, with orientation, quantity, labels and construction marks |
| Stitch line | Intended seam-joining geometry; distinct from the outer cutting line. In this system the net block is the stitch line. |
| Seam allowance | Specified offset region from stitch line to cut line, with corner and curve treatment |
| Ease | Garment dimension beyond body dimension. In this method, ease is embedded in the rule constants. |
| Dart | Wedge of fabric folded and stitched to shape fit; includes legs, apex and intake |
| Balance mark (notch) | Mark that ensures pattern pieces are sewn together at the correct points |
| Grainline | Required pattern orientation relative to fabric yarn direction |
| Toile | A trial garment made up in calico to check the fit of a block before cutting the final fabric |
| Nap / directional print | Fabric property affecting allowable placement and rotation |
| Marker | Arrangement of pattern pieces on fabric width to estimate length and waste |
| Grade rule | Explicit per-point changes to pattern landmarks across sizes, not uniform scaling |
| Rule record | A traceable unit of the rulebook: rule ID, page reference, kind (book rule, interpretation, extension), expression, verifier (Section 5.5) |

Canonical unit: the millimetre, stored as an exact decimal. Display unit: the unit a person sees and types, one of mm, cm, m, in, ft or yd, set globally with project and quantity-class overrides (FR-038). Quantity class: body measurements, pattern dimensions, allowances and tolerances, fabric width, or fabric length.

## 5. Drafting method baseline (new 1.2)

### 5.1 Method identity

| Field | Value |
| --- | --- |
| Method ID | aldrich-mpc-4e-tailored-skirt |
| Rulebook version | 1.0.0 (versions the transcription and interpretations, independent of the edition) |
| Source | Winifred Aldrich, Metric Pattern Cutting, 4th edition, Blackwell Publishing, 2004, ISBN 1-4051-0278-0 |
| Edition pin | 4th edition (2004) for all of R1. The 6th edition (Wiley, 2015) updates the skirt blocks; it is not used. Adopting it requires a new method ID, a new rule transcription and a new reference sample set. Rules from different editions must never be mixed in one project. |
| Rights | The source text and diagrams are copyrighted. Permission and citation are tracked under approval item 1 (Section 15). The rulebook records rules as functional constructions with page references and does not reproduce the book's text or figures. |

### 5.2 Source sections used

| Topic | Printed pages |
| --- | --- |
| Standard body measurement charts (4 cm and 6 cm increments; 5 cm increments; S M L XL) | 11 to 12 |
| Seam allowances | 32 |
| Pattern instructions (marks required on a pattern) | 33 |
| Tailored skirt block | 78 |
| Straight skirt adaptation | 80 |
| Basic grading techniques, including tailored skirt block tables | 163 to 165 |
| Drafting blocks for individual figures: measurement-taking, toiles, alterations | 169 to 173 |

### 5.3 Required inputs

| Measurement | Definition and protocol | Allowed source tags |
| --- | --- | --- |
| Waist | Taken round the natural waist, comfortably. A string is tied firmly round the waist so that vertical measurements start from a consistent line. | measured |
| Hips | Widest part of the hips, approximately 21 cm below the waistline. | measured |
| Waist to hip | Vertical distance from the waist string to the hip line. The method treats this as a standard-chart value even for individual figures. | measured; chart-derived; estimated |
| Skirt length | Finished length from the waist string down to the required hem. Fashion-dependent. | measured |
| Bust (optional, chart lookup only) | Used only to pick the standard chart size when waist to hip is chart-derived. Never used in skirt geometry. | measured |

Plausibility and contradiction thresholds are set at gate G0 by the pattern maker. The 4 cm and 6 cm increment chart (sizes 8 to 26) spans waist 620 to 1040 mm, hips 860 to 1280 mm and waist to hip 200 to 227 mm, and is used as the starting range for threshold discussion only.

When waist to hip is chart-derived, the chart size is chosen by the method's own rule (nearest bust size) if bust is supplied. If bust is not supplied, the nearest chart size by hips is used. This fallback is an interpretation (kind I), is always shown as such, and raises a warning.

### 5.4 Construction scope

The block is a sequence of construction points defined from the four inputs. The rule engine needs only these primitives: a fraction of a measurement plus a constant; a perpendicular line from a point; a fixed-length offset; division of a line into equal parts; and curves defined by a documented interpretation (Section 5.6).

- Pieces: one back and one front, drafted net.
- Ease is embedded in rule constants. There is no separate ease input in R1. User-adjustable ease is an extension (kind X) and is labelled as such.
- Darts: two on the back and one on the front. Dart position is a fraction of a construction line, dart length is a constant, dart width is a standard constant.
- One conditional rule: for a waist small in proportion to the hips, the method widens the darts and changes two waist constants. The trigger is not numeric in the source and is set at G0.
- The side seam is deliberately off-centre (moved forward). The asymmetry between pieces is correct and must not be normalised.
- Straight skirt adaptation: optional centre back swing and optional hem flare at the side seams (FR-041).
- Out of R1: waistbands, other skirt adaptations, trousers, bodices.

### 5.5 Rule records and kinds

Every construction constant, formula or decision is a rule record with these fields: rule\_id, method\_ref, page, kind, expression, dependencies, verifier, verified\_on, notes.

| Kind | Meaning | Production policy |
| --- | --- | --- |
| B (book rule) | Transcribed from the source with a page reference | Allowed once verified by a pattern maker |
| I (interpretation) | The source gives a qualitative or incomplete instruction and the project chose a numeric reading | Allowed once verified, always labelled |
| X (extension) | Behaviour beyond the source, such as user-adjustable ease | Allowed only if labelled in the UI and export metadata |

Rule constants are stored as exact decimals (integer tenths of a millimetre, or decimal strings) and converted once to f64 for geometry.

### 5.6 Required interpretations

The following items are not defined numerically in the source and must be decided and verified at G0 before FR-021 can be signed off: waistline curve; side seam outward curve; dart leg and apex construction; balance mark (notch) positions; grainline position; hem line shape after flare; and the small-waist trigger. Each becomes a kind I rule record.

### 5.7 Fit statement

The source requires a calico toile to confirm fit. Output of this system is a block for toile verification and is never presented as a guaranteed fit (FR-043).

## 6. Core workflows

**WF-01 New project:** method is fixed for R1 (method ID, rulebook version, edition pin) → enter the four measurements with source tags, or request chart-derived waist to hip → validate inputs → generate net front and back blocks → apply straight skirt adaptation parameters (optional) → add seam allowances → place marks and toile statement → inspect method assumptions and rule provenance → validate geometry → save → tiled-print or export. Every change to a measurement shall recompute all dependent landmarks or report why it cannot.

**WF-02 Fabric workflow (R2):** select fabric width, nap and direction, shrinkage assumptions and print or texture preset → place cut pieces according to grainline and cut quantity → estimate minimum required length with a stated heuristic → export a marker report. Estimates must never be advertised as guaranteed consumption.

**WF-03 Revision:** load an existing project → check method ID, rulebook version and edition pin → apply versioned migration if needed → change measurements → compare difference overlay → inspect warnings → regenerate outputs with revision metadata.

## 7. Functional requirements

### FR-019: Measurement profiles (revised 1.2)

Requirement: Create anonymisable client measurement profiles with named measurement definitions, units, last-edited time, definition version and provenance. A skirt profile has the four measurements of Section 5.3. Every value carries a source tag (measured, chart-derived, estimated), may be entered in any supported unit (FR-038), and is stored as an exact canonical millimetre value. Reject missing, impossible or contradictory data using thresholds fixed at G0. Do not invent unavailable anatomical measures; a chart-derived value is permitted only where Section 5.3 allows it and is always tagged.

Acceptance: Invalid fields identify the exact issue. A saved profile reloads identically including source tags. Chart-derived and estimated values are visibly distinguished in the UI and recorded in export metadata.

Priority: Must | Target: R1

### FR-020: Formula provenance and diagnostics (revised 1.2)

Requirement: Bind every supported template to a method ID, rulebook version and edition pin. Every calculation is a rule record (Section 5.5). For any constructed point the user can inspect the rule IDs, page references, rule kinds, measurements used and dependencies. A rule that is unverified is blocked from production export; in developer mode it may export only with an UNVERIFIED legend on every page. Extension rules (kind X) are labelled in the UI and export metadata. Rules from different editions are never combined.

Acceptance: The user can trace a selected construction point to its rules and measurements. Export of a project containing an unverified rule is blocked in production mode.

Priority: Must | Target: R1

### FR-021: Tailored skirt block (revised 1.2)

Requirement: Generate net front and back pieces of the tailored skirt block per Section 5.4: waistline, hipline, hemline, centre lines, side seams, two back darts, one front dart and shaping governed by verified rule records and the four measurements.

Acceptance: A pattern maker reviews ten varied realistic measurement sets and signs off the construction against reference drafts hand-drafted from the same edition. The ten sets follow the composition in Appendix A.2. The method's worked example (AT-13) is reproduced within the tolerance recorded at G0.

Priority: Must | Target: R1

### FR-022: Stitch versus cut geometry (revised 1.2)

Requirement: Represent the net stitch boundary separately from the cutting boundary. Support per-edge allowance, joined-corner policy and concave and convex curves; warn on invalid offset self-intersections. Default allowances are proposals within the source's general guide: basic seams 10 to 15 mm, hems 10 to 50 mm, enclosed seams 5 mm. The proposed defaults are side seams 15 mm and hem 30 mm. The source calls its figures a general guide, so defaults are user-adjustable per edge within validated limits. No allowance is added on a fold line. The allowance is marked on the pattern by line or notch.

Acceptance: Changing allowance modifies the cut boundary only. Severe self-intersections block export.

Priority: Must | Target: R1

### FR-023: Dart operations (revised 1.2)

Requirement: Display dart legs, apex, intake and fold direction for the two back darts and one front dart. Update dart geometry when measurements or style settings change. Implement the small-waist conditional rule with a trigger fixed at G0 and shown to the user when it fires.

Acceptance: Dart labels remain valid and shape rules pass method-specific tests, including AT-14.

Priority: Must | Target: R1

### FR-024: Construction marks (revised 1.2)

Requirement: Add the marks the method requires on a pattern: piece name; centre back and centre front lines; cut quantity; fold indicator; balance marks (notches); seam allowance marking; construction lines (darts); grainline; size; and style number. Also add drill points where appropriate, method ID, rulebook version, edition pin and orientation. Grainlines are placed before a pattern is divided into sections. Dimension labels and legends use the project display unit; millimetre values are recorded in metadata. Notch and grainline positions are interpretations (Section 5.6) until verified.

Acceptance: Every exported R1 piece includes the required legends and marks at correct dimensions.

Priority: Must | Target: R1

### FR-025: Manual drafting tools

Requirement: Enable point selection, move constrained points, dimension rulers, snapping, offset, mirror and curve-handle edits, with constraint warnings. Direct edits should be stored as overrides distinct from formula-driven geometry.

Acceptance: Undoing a manual edit restores exact preceding geometry; constraints stay inspectable.

Priority: Should | Target: R2

### FR-026: Curve construction (revised 1.2)

Requirement: Support cubic Bezier and line segments, continuity inspection and an adaptive tessellation path independent of rendered zoom. Do not represent a curve only by raster pixels. Where the method gives a curve only qualitatively (for example a slight curve at the waistline, or a side seam curving outwards), the curve must be defined by a documented numeric interpretation (kind I) that fixes control points or an arc definition. Golden tests reference the interpretation version.

Acceptance: Reference curve tests cover join tangents, low and high zoom and export fidelity. Each qualitative curve has a rule record with a verifier.

Priority: Must | Target: R1

### FR-027: Size grading (revised 1.2)

Requirement: Apply declared per-landmark grade rules for defined sizes; reject uniform scaling as a substitute for grading. A grade rule is a horizontal and vertical increment per landmark per size step, taken from the method's tailored skirt block tables (back and front). The method grades by drafting one size, grading the extreme size, and stepping the intermediate sizes along lines through each landmark. The tables assume the method's 5 cm increment size chart. Grading outside the chart range raises a warning. Mapping from the book's numbered points to landmark IDs depends on the source figures and is transcribed and verified at G0.

Acceptance: Given the approved rule tables, resulting landmark dimensions match within set tolerances (AT-18).

Priority: Should | Target: R2

### FR-028: Pattern comparison

Requirement: Overlay two versions or two measurement profiles; show dimension deltas and highlight landmark movement.

Acceptance: The revision summary lists changed input, method and affected pieces.

Priority: Should | Target: R2

### FR-029: Material library

Requirement: Store fabric label, width, weave or knit class, stretch direction and percentage when known, shrinkage allowance, nap or one-way design flag and optional swatch image. These fields are informational and used only where the relevant algorithm supports them.

Acceptance: Fabric choice cannot quietly change body measurements; user can see assumptions.

Priority: Should | Target: R2

### FR-030: Material visualization

Requirement: Display a 2D swatch or pattern fill preview with scale, orientation and opacity controls; enable user-provided PNG, JPEG or SVG images through permitted decoders. Describe results as visual approximation, not physical drape prediction.

Acceptance: A printed dimension remains unchanged when the material texture or sampling mode changes.

Priority: Should | Target: R2

### FR-031: Fabric layout and consumption

Requirement: Arrange pieces within selected fabric width respecting grainlines, fabric fold, piece count, mirrored pairs, nap and rotation constraints; report total layout length and waste ratio.

Acceptance: All placed pieces remain within width and do not overlap (apart from explicitly permitted shared folds).

Priority: Should | Target: R2

### FR-032: Printable tiled PDF

Requirement: Support page size A4 and Letter, physical millimetre coordinates, tile overlap, page numbers, registration targets and a 100 mm calibration square (plus a 4 in square when an imperial display unit is selected); force 100% / actual-size print instructions.

Acceptance: Measure the printed calibration square at 100 mm within agreed printer tolerance, with no missing piece edges.

Priority: Must | Target: R1

### FR-033: SVG vector export

Requirement: Produce dimensioned SVG using physical units and a predictable coordinate origin; include layer and group IDs and machine-readable metadata (including the project display unit) without rasterising geometric boundaries. Geometry is always written in millimetre-based physical coordinates, whatever the display unit.

Acceptance: Automated parser validates bounding boxes, paths and unit metadata.

Priority: Must | Target: R1

### FR-034: DXF interchange

Requirement: Export named geometry layers for stitch, cut, annotations, grainline and notches; document the supported DXF dialect and units; the DXF units header is set to millimetres. Verify against at least two selected external CAD viewers.

Acceptance: Round-trip measurement and entity tests pass; unsupported constructs report limitations.

Priority: Should | Target: R2

### FR-035: Offline project package (revised 1.2)

Requirement: Save human-readable metadata, versioned geometry and rule references, measurements with source tags, and optional assets as one user-transferable file or directory. The package records the method ID, rulebook version, edition pin and the identity (hash) of each rule record used. Use atomic writes for native and user-mediated browser downloads.

Acceptance: Opening after restart reconstructs the same pattern. A crash during save does not destroy the last valid version. A project whose edition pin or rule hashes do not match the installed rulebook is reported, not silently regenerated.

Priority: Must | Target: R1

### FR-036: Audit and warnings (revised 1.2)

Requirement: Record warnings for missing measurements, invalid contours, excessive offset curvature, impossible grading changes, insufficient fabric width and unknown formula sources, with remediation text. New in 1.2: a chart-derived or estimated measurement is in use; an interpretation (kind I) or extension (kind X) rule is in use; an unverified rule is present; the small-waist rule fired; a measurement differs widely from the standard chart value for the nearest size (the method advises re-measuring in that case); grading is outside the chart range; an entered length is implausible in its field's unit (for example a hip of 94 read as inches).

Acceptance: Critical errors prevent misleading export; recoverable warnings remain visible.

Priority: Must | Target: R1

### FR-037: Measurement and pattern privacy

Requirement: Default to offline local use with no telemetry or cloud transmission; give delete and export controls for customer measurements.

Acceptance: Network-blocked app can draft, save and export; no measurement data leaves the device without explicit action.

Priority: Must | Target: R1

### FR-038: Global unit system (revised 1.2)

Requirement: Support these length units everywhere a length is entered, displayed or exported: millimetre (mm), centimetre (cm), metre (m), inch (in), foot (ft) and yard (yd). Inches may be shown as decimals or as fractions (nearest 1/16 by default, configurable). The system keeps one canonical unit, the millimetre, stored as an exact decimal. Every supported unit converts exactly: 1 cm = 10 mm, 1 m = 1000 mm, 1 in = 25.4 mm, 1 ft = 304.8 mm, 1 yd = 914.4 mm. Unit choice is a display and input preference only. It is set globally, may be overridden per project, and may be set per quantity class (body measurements, pattern dimensions, allowances and tolerances, fabric width, fabric length). Any input field accepts a typed unit suffix (for example 94 cm or 37 3/8 in) that overrides the field default. All unit-bearing values, labels, rulers, legends, warnings, tolerances, reports and exports go through one unit service in pattern\_core; no other module formats a length. Changing the display unit never changes a stored value or any geometry. Exports carry geometry in millimetre-based physical coordinates and record the display unit in metadata. Rule constants and chart data are stored as printed, with their unit, and converted once.

Acceptance: AT-20 and AT-21. Entering the same physical length in each supported unit stores an identical value. Switching display units any number of times leaves stored values and exported geometry unchanged. Values are rounded only at display time, to a documented precision per unit.

Priority: Must | Target: R1 (raised from Should, R2 in 1.1)

### FR-039: Keyboard and accessibility workflow

Requirement: Expose labelled data fields, keyboard zoom and pan equivalents, error focus, contrast themes and AccessKit semantic controls; provide a non-canvas numerical representation of critical dimensions.

Acceptance: Key functionality is usable without a pointer; status announcements are readable through accessibility APIs.

Priority: Must | Target: R1

### FR-040: Templates and constraints (revised 1.2)

Requirement: Allow new garment definitions through versioned Rust rule implementations and validated declarative settings; do not execute user-provided arbitrary code inside project files. Unknown template IDs, method IDs or edition pins fail safely.

Acceptance: Unknown template, method or edition identifiers fail safely with an actionable error.

Priority: Must | Target: R1

### FR-041: Straight skirt adaptation (new 1.2)

Requirement: Provide the straight skirt as an adaptation of the tailored skirt block with two optional parameters: centre back swing and hem flare at the side seams. A preset applies the method's adaptation (values come from verified rule records). A second preset applies no adaptation. Both parameters are adjustable within validated limits. Zero swing and zero flare reproduce the block exactly. The adaptation changes the piece outlines and adjusts the hem line; it must not change the four input measurements.

Acceptance: AT-15.

Priority: Must | Target: R1

### FR-042: Standard size charts (new 1.2)

Requirement: Store the method's standard body measurement charts as versioned, verified data: the 4 cm and 6 cm increment chart (sizes 8 to 26) and the 5 cm increment chart (sizes 10 to 24) in R1; the S M L XL chart in R2. R1 uses only the bust, waist, hips and waist-to-hip columns. Short and tall vertical adjustments are stored for later use. Chart lookup supplies a chart-derived waist to hip according to Section 5.3, always tagged and always producing a warning. Chart data are transcribed and verified like rules (kind B) and carry page references.

Acceptance: AT-16. Chart values reload identically and match the verified transcription.

Priority: Must | Target: R1 (S M L XL chart: Should, R2)

### FR-043: Toile verification statement (new 1.2)

Requirement: Every export page, and the on-screen summary, states that the output is a block for toile verification and that fit must be confirmed on a calico toile before final fabric is cut. The statement includes the method ID, rulebook version and edition pin. It cannot be removed in production mode.

Acceptance: AT-17.

Priority: Must | Target: R1

## 8. Render and texture specifications

### REN-01: Authoritative geometry (revised 1.2)

Requirement: All physical pattern calculations shall use canonical millimetre-space Rust geometry with f64 (or a documented exact representation). Rule constants are stored as exact decimals and converted once. The wgpu shader shall not define the authoritative sewing measurements.

Acceptance: CPU output identical within test tolerance across native and WASM.

Priority: Must | Target: R1

### REN-02: GPU canvas separation

Requirement: The egui\_wgpu paint callback shall draw viewport effects (shadow, checkerboard, zoomed texture, grid) and may draw tessellated vector pieces; it must not silently modify exported vectors.

Acceptance: A render-only sampling mode change produces byte-equivalent geometry export.

Priority: Must | Target: R1

### REN-03: Zoom and sampling

Requirement: Support bilinear and trilinear texture sampling with explicit mipmap generation where required, and show a 1 px image pixel grid at 8x zoom and above for raster assets only.

Acceptance: Switching mode visibly affects the raster preview while measured coordinates stay constant.

Priority: Must | Target: R1

### REN-04: Textile swatch orientation

Requirement: When a user supplies a texture, offer scale in real units, rotation and repeat preview; mark stretch and nap metadata separately from the image.

Acceptance: The preview legend displays physical repeat dimensions and does not imply simulated stretch.

Priority: Should | Target: R2

## 9. Technical architecture and language policy

Primary development language: Rust for desktop, WASM and pattern mathematics. WGSL is required for the custom GPU shader and is not a replacement for Rust. TOML defines Cargo build configuration; JSON or a documented binary format stores projects; SVG is an output and asset vector format; HTML and CSS provide minimal Trunk web host structure. No Python, JavaScript or C++ runtime is required for the proposed product. A separate Python or TypeScript research harness may be used solely for independent numeric and export verification, and is not a production dependency.

| Crate / layer | Responsibility | Compatibility rule |
| --- | --- | --- |
| pattern\_core | Unit service (all unit parsing, conversion and formatting), exact-decimal constants, coordinates, dependencies, rule-engine primitives, rule record types, geometry validation | No egui or wgpu, no filesystem requirement |
| pattern\_templates | Aldrich 4th ed. tailored skirt block and straight skirt adaptation, size chart data, rule records, grading tables (R2) | Deterministic inputs and outputs; every constant has a rule record |
| pattern\_document | Projects, versioning, migration, undo and redo, edition pin and rule hash checks | No UI types in file schema |
| pattern\_export | SVG, PDF, DXF adapters and print checks | Vector-first; test native and WASM separately |
| pattern\_material | Fabric metadata, repeats and R2 layout heuristics | No claims of physics |
| pattern\_ui | eframe/egui 0.36, AccessKit and egui\_extras SVG loader | Deny unwrap\_used, expect\_used and panic via Clippy policy |
| pattern\_render | egui\_wgpu paint callback, wgpu and WGSL shader | Graceful fallback and texture limits |
| pattern\_web | Trunk host and wasm-release build | Built output under 25 MiB target host constraint |

Suggested cargo profile (validate against a real build): \[profile.wasm-release\] inherits = release; opt-level = z; lto = fat; codegen-units = 1; strip = debuginfo; panic = abort. The profile alone does not guarantee a 25 MiB file. CI shall measure the actual hosted WASM artifact and fail if it exceeds 25 MiB, and monitor the whole deployment payload separately.

Workspace policy: forbid(unsafe\_code) on workspace-owned crates (and workspace lints for members). Clippy denies unwrap\_used, expect\_used and panic in the UI crate. These lint restrictions apply to first-party source, not automatically to compiled third-party dependencies. All Result handling must expose errors or propagate them. Shader inputs and file assets must be validated against bounds.

## 10. Nonfunctional requirements and quantitative verification

| ID | Metric / policy | Verification |
| --- | --- | --- |
| NFR-014 | Draft replay deterministic for the same method ID, rulebook version and inputs | Golden-case coordinate checksum and tolerance |
| NFR-015 | Export physical length error 0.1 mm or less for digital geometry; external print verified separately | Compare path length and calibration square |
| NFR-016 | No silently incomplete exported pattern; blocking error on invalid contour | Self-intersection and closure tests |
| NFR-017 | UI update target 150 ms or less for base skirt changes on reference hardware; no UI freeze over 500 ms | Local performance bench |
| NFR-018 | Offline: all core drafting actions; browser local download and open supported | Disable network, then execute test suite |
| NFR-019 | WASM artifact below 25 MiB after production optimisation | CI size gate |
| NFR-020 | 100% of loaded project files version-checked, including method ID, rulebook version and edition pin; reject unknown schema safely | Schema upgrade tests |
| NFR-021 | Crash-safe native saves and deterministic file recovery | Fault injection |
| NFR-022 | No unwrap, expect or panic in UI; no unsafe in own workspace | cargo clippy and deny policy |
| NFR-023 | Minimum reference browser and operating system matrix explicitly recorded prior to release | Cross-platform tests |
| NFR-024 | No undisclosed image upload, analytics or remote measurement storage | Network trace offline test |
| NFR-025 | Accessible measurement form and export flow via keyboard and semantic tree | Manual accessibility evaluation |
| NFR-026 (new 1.2) | 100% of constants and formulas that influence production output have a rule record with page reference, kind and verifier | Automated rulebook completeness test |
| NFR-027 (new 1.2) | Size chart and rule data checksummed; any change to transcribed data changes the rulebook version | Checksum test in CI |
| NFR-028 (new 1.2) | 100% of unit-bearing fields, labels and exports route through the unit service; entry, display and export work for mm, cm, m, in, ft and yd | Static check that no module formats a length directly; AT-20 and AT-21 |

All timing, accuracy and performance values in this revision are proposed acceptance targets pending a recorded hardware and printer baseline. The tolerance between code output and a hand-drafted reference is not fixed by this revision. It is recorded at gate G0 by the pattern maker, because hand drafting has its own precision limit.

## 11. Conceptual data objects

| Entity | Required fields |
| --- | --- |
| MeasurementProfile | id, label, measurement\_definition\_version, values\_mm with a source tag per value (measured, chart-derived, estimated), protocol\_version, updated\_at |
| DraftMethod | method\_id, rulebook\_version, edition, isbn, citation, required\_inputs, constraints |
| RuleRecord (new) | rule\_id, method\_ref, page, kind (B, I, X), expression, dependencies, verifier, verified\_on, notes |
| SizeChart (new) | chart\_id, source\_page, increment\_type, size\_rows (bust, waist, hips, waist\_to\_hip, other columns), vertical\_adjustments, checksum |
| PatternProject | id, version, garment\_type, method\_ref, rule\_hashes, profile\_ref, style\_settings (swing, hem\_flare, allowances, style\_no), assets |
| PatternPiece | id, label, closed\_stitch\_path, cut\_path, grainline, cut\_quantity, balance\_marks, annotations, style\_no |
| GradeRuleTable (new, R2) | method\_ref, size\_step, per-landmark dx and dy increments, source\_page |
| PatternOperation | timestamp and revision, operation\_type, parameters, affected IDs |
| FabricSpec | name, usable\_width\_mm, nap, fold, shrinkage, repeat size, swatch\_ref |
| ExportJob | target format, units, media, profile, warnings, toile statement, output checksum |
| UnitPreference (new) | scope (global or project), quantity\_class, display\_unit (mm, cm, m, in, ft, yd), inch\_format (decimal or fraction), fraction\_denominator, display\_precision per unit |

## 12. Acceptance test scenarios

| Test | Trigger | Expected outcome |
| --- | --- | --- |
| AT-01 | Enter a valid set of the four required measurements | Two named skirt pieces with required marks and a complete warnings list |
| AT-02 | Change hip circumference | Formula-linked waist, hip and side geometry updates; ruler changes by the expected amount |
| AT-03 | Delete any one mandatory measurement | Generation disabled with a specific missing-field prompt |
| AT-04 | Increase seam allowance | Cut path changes, stitch path stays identical |
| AT-05 | Preview textile at 3 repeat scales | Canvas changes but SVG and PDF geometry unchanged |
| AT-06 | Save file and reopen | Project data and geometric output match before and after |
| AT-07 | Export tiled PDF | A4 and Letter pages have alignment marks and a 100 mm scale box |
| AT-08 | Change native vs WASM target | Same reference pattern dimensions and warnings |
| AT-09 | Reject self-intersecting offset | Blocking message; no silently corrupted export |
| AT-10 | Open unsupported schema version, method ID or edition pin | Safe failure, source file not modified |
| AT-11 | Attempt invalid DXF construct | Export reports unsupported feature rather than dropping geometry |
| AT-12 | Put a grainline-constrained marker on narrow fabric | Explicit failure or constrained alternative; no overlap |
| AT-13 (new) | Enter the size 12 chart values used in the method's worked example (waist 700 mm, hips 940 mm, waist to hip 206 mm) with a skirt length chosen by the pattern maker | Output matches the pattern maker's hand draft from the same edition within the G0 tolerance |
| AT-14 (new) | Enter a waist and hip combination just beyond the small-waist trigger, then just inside it | Beyond: darts widen and the two waist constants change, and the warning is shown. Inside: the standard rule applies. |
| AT-15 (new) | Select the no-adaptation preset, then the straight skirt preset, then set swing and flare to zero | No adaptation and zero parameters give a piece identical to the block. The preset changes only the outline and hem. |
| AT-16 (new) | Supply no waist-to-hip value and request a chart-derived value, with and without bust | The value is tagged chart-derived; a warning is raised; export metadata records chart, size and page; the no-bust fallback is labelled an interpretation |
| AT-17 (new) | Export PDF and SVG | Every page carries the toile statement with method ID, rulebook version and edition pin; it cannot be removed in production mode |
| AT-18 (new, R2) | Grade a base size using the approved increment tables | Landmark positions match the tables within tolerance; grading outside the chart range warns |
| AT-19 (new) | Remove the verification from one interpretation rule | Production export is blocked; developer mode exports with an UNVERIFIED legend on every page |
| AT-20 (new) | Enter the same length as 304.8 mm, 30.48 cm, 0.3048 m, 12 in and 1 ft, then switch the display unit through all six units and back | All five entries store the identical canonical value; stored values and geometry are unchanged by every switch |
| AT-21 (new) | Export SVG, PDF and DXF once per display unit | Geometry coordinates are identical across display units; only legends and the display-unit metadata differ; DXF units header is millimetres; an imperial unit adds the 4 in calibration square |

## 13. Dependencies and risks

| Risk | Mitigation / exit criterion |
| --- | --- |
| Missing recognised pattern-making formulas | Resolved for the method choice (Aldrich 4th ed.). Remaining exit criterion: expert verification of the transcription and ten reference drafts before claiming fit. |
| Edition drift: the 6th edition has updated skirt blocks | Pin the 4th edition in the method ID; reject mixed rules; adopting a later edition is a new method with a new reference set |
| Qualitative curve instructions (new 1.2) | Document numeric interpretations (kind I) in G0; golden tests reference the interpretation version |
| Notch and grainline positions not defined in the block text (new 1.2) | Pattern maker defines them at G0; they remain kind I until verified |
| Source figures needed to map numbered points for grading (new 1.2) | Transcribe from the figures at G0; do not infer from text alone |
| Rights to use and cite the source (new 1.2) | Resolve approval item 1; record rules as functional constructions with page references, not reproduced text |
| Chart-derived waist to hip may not match an individual figure (new 1.2) | Tag and warn; encourage direct measurement; compare against chart and warn on wide deviation |
| Single-source method with no cross-check (new 1.2) | Expert review of the hand drafts; treat a second method as an R2 option |
| Unsupported PDF and DXF libraries on WASM | Prototype libraries early; if browser export cannot run, narrow supported format and disclose |
| Complex seam allowance corner cases | Geometric offset verification including concavities and rounded corners |
| WASM 25 MiB package cap | Use feature gating, optimisation and CI budget; no promise without build measurement |
| Overengineering graphics before pattern correctness | Finish CPU geometry and printing baseline before material shader polish |
| Textured preview mistaken for garment fit simulation | Label as 2D visual preview and never claim drape validation |
| Commercial exchange compatibility | Test representative file imports in nominated CAD software; publish profile |
| Unit confusion at input, for example inches read as centimetres (new 1.2) | Show the unit on every field; accept a typed suffix override; warn on implausible values (FR-036); never guess a unit silently; test with AT-20 |

## 14. Build order and project management gates

| Gate | Deliverable | Pass condition |
| --- | --- | --- |
| G0 Domain baseline (revised 1.2) | (a) Rule register workbook (Appendix A) completed: one rule record per constant with page reference, kind and verifier, each checked against the book's figure as well as its text. (b) Size charts transcribed. (c) Decisions on every interpretation in Section 5.6 and on the small-waist trigger. (d) Validation thresholds for the four measurements. (e) Default seam allowances. (f) Ten hand-drafted reference skirts from the 4th edition with the composition in Appendix A.2, at least three from real consenting clients. (g) Code-versus-hand-draft tolerance. (h) Named verifier. (i) Permission status for the source. | A qualified pattern maker verifies the transcription, interpretations and reference drafts |
| G1 Geometry engine | Rust coordinate graph, curves and skirt blocks | Golden numerical tests pass |
| G2 Construction | Allowances, darts, notches, piece labelling, straight skirt adaptation, toile statement | Construction and offset tests pass |
| G3 App UI | egui controls, project save, vector canvas | Full offline workflow demonstrated |
| G4 Export | SVG and tiled PDF, physical calibration | Print acceptance achieved |
| G5 Web build | Trunk WASM and shader fallback tests | Browser tests and under 25 MiB CI gate |
| G6 R2 professional | DXF, grading, marker and fabric swatch | Domain-specific interoperability tests pass |

## 15. Explicit exclusions and unresolved decisions

Not promised in v1.2: automatic perfect fit, physically simulated fabric behaviour, photoreal 3D clothing, body-camera scanning, AI recommendations, printer-driver control, guaranteed lowest-waste fabric nesting, and any claim that output fits without a toile. These are separate possible products or research tracks and must not be implied by pattern preview. Later editions of the method, other garment blocks and waistbands are outside R1.

| # | Approval item | Status in 1.2 |
| --- | --- | --- |
| 1 | Reference drafting textbook and method, and permission where applicable | Method selected (Aldrich, 4th ed.). Permission to use and cite: open. |
| 2 | Exact skirt measurement definitions and minimum set | Defined in Section 5.3. Plausibility thresholds: open, set at G0. |
| 3 | Tailoring population and size ranges | Partly known from chart ranges (sizes 8 to 26). Target population: open. |
| 4 | Offline storage model and customer consent | Open |
| 5 | Reference printer and print tolerance | Open |
| 6 | Priority of native versus browser export | Open |
| 7 | Selected DXF standard and compatible tools | Open |
| 8 | Initial host target and asset cap | Open |
| 9 | Whether users require multi-language UI | Open |
| 10 | Formal ownership of fit validation | Open. Blocks G0. |
| 11 (new) | Numeric interpretation of qualitative curves, notch positions and grainline position | Open. Owner: pattern maker with lead developer. |
| 12 (new) | Numeric trigger for the small-waist rule | Open |
| 13 (new) | Default seam allowance values within the source's general guide | Proposed in FR-022; open |
| 14 (new) | Chart-size selection when bust is not supplied | Proposed in Section 5.3; open |
| 15 (new) | Default display units per quantity class | Proposed: cm for body measurements, mm for allowances and tolerances, m for fabric length; all six units selectable. Open. |
| 16 (new) | Code-versus-hand-draft tolerance | Set by the pattern maker from the hand drafts' own precision (Appendix A.2). Open. |

## 16. Sources

- Winifred Aldrich, Metric Pattern Cutting, 4th edition, Blackwell Publishing, 2004, ISBN 1-4051-0278-0. Pages used are listed in Section 5.2. This is the pinned method source.
- Metric Pattern Cutting for Women's Wear, 6th edition, Wiley, 2015. Publisher listing consulted only to establish that later skirt blocks were revised. Its content was not used.
- Final year project .docx (user-supplied): parametric geometry, measurement to vector pattern and staged curves and ease development.
- SRS v1.0 and SRS v1.1 (user-supplied): base architecture, functional and nonfunctional baseline and Rust toolchain decisions.
- Seamly documentation index: https://wiki.seamly.io/wiki/Main\_Page/en (measurements, drafting, curves and allowances).
- Valentina software release notes: https://gitlab.com/smart-pattern/valentina/-/releases (grainline, seam allowance, printable PDF, DXF interoperability).
- egui\_extras 0.36.2 docs: https://docs.rs/egui\_extras/latest/egui\_extras/ (SVG loader support).
- eframe 0.36 docs: https://docs.rs/crate/eframe/latest (native, WASM and wgpu backend).

Review note: The method source supplies constructions, not guarantees. Production drafting equations must be transcribed, interpreted where the source is qualitative, and verified by a qualified pattern maker at G0 before any default manufacturing value is released. Illustrative values in this document, such as the proposed default seam allowances, are proposals and not manufacturing values.

## Appendix A. G0 rule register and reference set (new 1.2)

### A.1 Register workbook

The workbook G0\_Rule\_Register\_Aldrich\_4e.xlsx accompanies this SRS. The pattern maker and lead developer fill it from the 4th edition. It holds 70 rule slots (65 for R1, 5 for R2) with IDs of the form ALD4-GROUP-NNN, so every constant the code uses has a page reference, a kind (B, I or X), a status and a verifier.

| Sheet | Purpose |
| --- | --- |
| README | Instructions, kinds, status flow, colour legend |
| Rule\_register | Every rule slot: ID, group, piece, source page, release, kind, expression, constant and unit, status, verifier, figure check |
| Thresholds | Validation limits for the four measurements and optional bust, and cross-field rules |
| Size\_charts | Slots for the EU (sizes 8 to 26) and UK 5 cm (sizes 10 to 24) charts, the S M L XL chart (R2) and the short and tall adjustments |
| Reference\_drafts | The ten reference skirts, with the required composition (A.2) |
| Grade\_tables | R2 per-point grading increments for the back (points 1 to 11) and front (points 12 to 19) |
| Decisions | The 16 approval items and G0 decisions |
| G0\_signoff | Readiness counts computed from the other sheets, and the verifier's sign-off |
| Lists | Units with exact millimetre factors, and the dropdown lists |

### A.2 Reference set composition

1. The book's worked example (size 12 chart values).
2. A chart size at the small end of the range.
3. A chart size at the large end of the range.
4. An individual figure with proportions close to the chart.
5. An individual figure with a small waist for the hips, just inside the small-waist trigger.
6. An individual figure with a small waist for the hips, beyond the trigger.
7. A short waist to hip, measured directly.
8. A long waist to hip, measured directly.
9. An individual figure drafted with the straight skirt adaptation preset.
10. An extreme skirt length, short or long.

At least three cases come from real clients who have consented to their measurements being used. Every draft is made by hand from the same edition the code implements. The pattern maker records each draft's own precision, and the code-versus-hand tolerance (approval item 16) is set from it.

### A.3 Status workflow and readiness

A rule moves from Open to Transcribed (kind B) or Decided (kinds I and X), then to Verified by the pattern maker. Only Verified rules reach production export (FR-020). The G0\_signoff sheet reports READY FOR G0 REVIEW only when every R1 rule and every R1 interpretation is Verified, all ten reference drafts have passed, no decision is Open, and all eighteen EU and UK chart rows are verified. The verifier still makes the final call.

### A.4 Units in the register

Every constant, threshold, chart value and reference measurement is recorded with its own unit (mm, cm, m, in, ft or yd) and converted to millimetres by formula using the exact factors of FR-038. The register stores what the book prints; the code stores canonical millimetres.
