# Data, persistence and contracts

There is no database and no network API. The contracts that matter are the crate interfaces and the file formats. Names below are illustrative. The semantics are binding. A slice may rename a type if the semantics hold, and must update this file when it does.

## 1. Length representation (ADR-0002, ADR-0007)

- Stored form: an exact integer count of nanometres in an i64. One millimetre is 1,000,000 units. This represents every value the six units can produce, including 1/64 inch (396,875 units) and 1 yard (914,400,000 units).
- Fractions of an inch convert exactly when the denominator is a power of two up to 64. Other denominators are rejected with a typed error (OQ-33 fixes the allowed set).
- Computation form: f64 millimetres, created from the stored form at the boundary into geometry. Results return to the stored form only where a value is saved or displayed.
- Serialised form: a decimal string in millimetres (for example "304.8"), never a JSON number.
- All parsing, conversion and formatting live in pattern_core::units.

## 2. Project file (ADR-0010, Proposed)

One UTF-8 JSON document in R1. Field order is fixed so files diff cleanly. Assets (R2) arrive with a container decision later.

| Field | Content | Notes |
|---|---|---|
| schema_version | Integer | Checked on every load (NFR-020) |
| app_version | String | Informational |
| created_at, updated_at | Timestamps | Saved, never used in geometry |
| method_ref | method_id, rulebook_version, edition, isbn | Edition pin. Mismatch is reported, never silently migrated |
| rule_hashes | rule_id to hash | Detects rulebook drift (FR-035) |
| profile | label, measurement_definition_version, protocol_version, values (decimal strings in mm with a source tag each), updated_at | Source tags: measured, chart-derived, estimated |
| style | Adaptation preset, swing, flare, per-edge allowances, style number, project display units | Lengths as decimal strings |
| pattern_checksum | Hash of the canonicalised generated geometry | Lets a reopen prove it rebuilt the same pattern (AT-06) |
| operations | Operation log | Only if undo is in R1 (OQ-29) |
| assets | List | R2 |

The generated pattern is not stored. It is rebuilt on load and checked against pattern_checksum. If they differ, the user sees a drift warning and the file is not changed.

### Versioning and migration

1. schema_version rises on any incompatible change.
2. Each migration is a pure function from version n to n+1 with a fixture pair.
3. A file with a higher version than the app, an unknown method_id, an unknown rulebook_version or an unknown edition is rejected with a typed error. The source file is never modified (AT-10).
4. Migration writes a new file. It never rewrites the original in place.

### Atomic write protocol

Native: serialise fully in memory, write to a temporary file in the same directory, flush to disk, then rename over the target. The previous version stays valid until the rename completes. A failure at any step leaves the previous file intact (NFR-021).

Browser: serialise fully, then trigger one download of the complete blob. The browser has no partial-write state. How the user's last valid version is protected is OQ-25.

## 3. Crate interface contracts

These sketches state responsibilities and error behaviour.

```rust
// pattern_core::units
pub struct Length(/* exact nanometre count */);
pub enum Unit { Mm, Cm, M, In, Ft, Yd }
pub fn parse_length(text: &str, default: Unit) -> Result<Length, UnitError>;
pub fn format_length(len: Length, spec: &FormatSpec) -> String;   // the only way to show a length

// pattern_core::rules
pub enum RuleKind { Book, Interpretation, Extension }              // B, I, X
pub enum RuleStatus { Open, Transcribed, Decided, Verified, Rejected, NotApplicable }
pub struct RuleRecord { /* id, page, kind, status, verifier, expression, deps */ }
pub trait Rulebook {
    fn rule(&self, id: &RuleId) -> Result<&RuleRecord, RuleError>;
    fn version(&self) -> &RulebookVersion;
    fn checksum(&self) -> Checksum;
}

// pattern_core::audit
pub struct ExportGate;            // cannot be constructed except by gate_check
pub fn gate_check(p: &GeneratedPattern, mode: Mode) -> Result<ExportGate, Vec<Blocker>>;

// pattern_templates
pub trait Template {
    fn method_ref(&self) -> MethodRef;
    fn required_inputs(&self) -> &[InputSpec];
    fn generate(&self, profile: &Profile, style: &Style, book: &dyn Rulebook)
        -> Result<GeneratedPattern, GenerateError>;                  // pure, no I/O, deterministic
}
// GeneratedPattern: pieces (net stitch path, cut path, marks), provenance, warnings, checksum

// pattern_export
pub trait Exporter {
    fn export(&self, p: &GeneratedPattern, gate: &ExportGate, opts: &ExportOptions)
        -> Result<ExportArtifact, ExportError>;                      // no gate, no export
}

// pattern_document
pub trait ProjectCodec {
    fn load(&self, bytes: &[u8]) -> Result<Project, LoadError>;
    fn save(&self, project: &Project) -> Result<Vec<u8>, SaveError>;
}
pub trait AtomicSink { fn write_atomic(&self, name: &str, bytes: &[u8]) -> Result<(), SinkError>; }
```

Binding behaviours:

- Template::generate does no I/O and reads no clock or randomness.
- No exporter can be called without an ExportGate. The type system enforces the gate.
- No public function panics on any input. Every failure returns a typed error.
- Mode is Production or Developer. In Production, gate_check refuses any pattern that used a rule that is not Verified. In Developer it passes with an UNVERIFIED flag that exporters print on every page. How the mode is selected is OQ-30.

## 4. Export file contracts

### SVG (FR-033)

- Physical units, origin at the top-left of the overall bounding box of laid-out pieces.
- Coordinates in millimetre-based user units regardless of display unit.
- Groups: one per piece with ID piece-<id>, and within it layer groups layer-stitch, layer-cut, layer-marks, layer-labels.
- Metadata element: method ID, rulebook version, edition pin, display unit, chart facts if any, warnings, toile statement, export checksum. Element names are fixed by S12 and then recorded here.
- No raster image carries a pattern boundary.

### PDF (FR-032)

- A4 and Letter. Coordinates derive from millimetres at 72 points per 25.4 mm.
- Each tile has overlap, registration targets, a page number of the form n of N, and the toile statement.
- Page 1 carries a 100 mm calibration square (and a 4 in square when the display unit is imperial) and the instruction to print at 100%.
- Tile size, overlap and margins are open (OQ-26).

### DXF (R2, FR-034)

Layers named stitch, cut, annotations, grainline, notches. Units header is millimetres. Dialect fixed by OQ-07.

## 5. Error taxonomy

| Type | Raised by | Severity | Example |
|---|---|---|---|
| UnitError | units | Input error, recoverable | Ambiguous suffix |
| ValidationError | measure | Input error, recoverable | Hips below waist limit |
| RuleError | rules | Critical in Production | Rule not Verified |
| GenerateError | templates | Critical | Missing rule record |
| Blocker | audit | Critical, blocks export | Severe self-intersection |
| Warning | audit | Recoverable, visible | Chart-derived value in use |
| LoadError | document | Critical for that file | Unknown schema version |
| SaveError, SinkError | document | Critical, last file kept | Disk full |
| ExportError | export | Critical | Library failure |

Every error has a stable code, a message a non-developer can act on, and no measurement values in logs.
