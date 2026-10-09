# Critical invariants

An invariant is something that must always be true. A change that breaks one is a defect even if every feature test passes. Each has a statement, why it matters, and the tests that guard it.

| ID | Invariant | Why it matters | Guarding tests |
|---|---|---|---|
| INV-01 | Same method ID, rulebook version and inputs always give an identical geometry checksum | Trust in output and reproducible bugs | T-NFR-014-01, T-NFR-014-02, T-FR-021-03 |
| INV-02 | Changing the display unit never changes a stored value or any geometry | Unit mistakes waste fabric | T-FR-038-02, T-FR-038-05, T-FR-038-06 |
| INV-03 | Changing an allowance changes the cut path only | The stitch line is the true dimension | T-FR-022-01, T-FR-022-02 |
| INV-04 | If the export gate refuses, no file or partial file is written | A wrong pattern must not leave the app | T-FR-036-02, T-NFR-016-03, T-FR-022-05 |
| INV-05 | Every exported path is closed and free of severe self-intersection | A broken contour cuts wrong | T-NFR-016-01, T-NFR-016-02 |
| INV-06 | Production exports use only Verified rules | Unverified rules are untested fit claims | T-FR-020-03, T-FR-020-04 |
| INV-07 | Render and preview settings never change export bytes | The screen must not affect the print | T-REN-02-01, T-FR-033-03 |
| INV-08 | Save then load reproduces the same project and pattern checksum | Customer work must not drift | T-FR-035-01, T-FR-019-04 |
| INV-09 | A file that cannot be loaded is never modified | Protect the user's original | T-FR-040-03, T-NFR-020-02 |
| INV-10 | A failure during save leaves the last valid file loadable | Crash safety | T-NFR-021-01, T-NFR-021-02, T-FR-035-02 |
| INV-11 | No core flow performs network I/O | Customer privacy | T-NFR-024-01, T-NFR-024-02, T-NFR-018-01 |
| INV-12 | Only waist-to-hip may be chart-derived, and chart-derived or estimated values are always tagged and warned | Never invent a body measurement | T-FR-019-02, T-FR-042-03, T-FR-036-01 |
| INV-13 | The calibration square in the file is 100 mm within 0.1 mm | Print scale check | T-FR-032-03, T-NFR-015-01 |
| INV-14 | Native and WASM produce the same reference geometry within tolerance | Browser users get the same pattern | T-REN-01-01, T-NFR-014-03 |
| INV-15 | No input can make any library function panic | Stability and safety | T-NFR-020-03, T-NFR-022-01 |
| INV-16 | The toile statement appears on every output page and cannot be hidden in production | Never imply guaranteed fit | T-FR-043-01, T-FR-043-02 |
| INV-17 | Zero swing and zero flare give exactly the block | The adaptation is only an adaptation | T-FR-041-01 |
| INV-18 | The tile union covers every piece edge | No missing edges in print | T-FR-032-01 |
| INV-19 | A drafting constant exists only in rulebook data, with a rule record | Traceability | T-NFR-026-01, T-NFR-027-01 |
| INV-20 | Values survive parse, format, parse in every unit without drift | Exact decimal promise | T-FR-038-01, T-FR-038-02 |
| INV-21 | The method's deliberate asymmetries (off-centre side seam) are preserved | They are correct, not defects | T-FR-021-01, T-FR-021-04 |

## How to use this list

- Every slice report names which invariants its change could affect and which guarding tests ran.
- A new invariant is added here with its tests before the code that relies on it merges.
- A guarding test is never deleted or weakened without lead developer approval and a note here.
