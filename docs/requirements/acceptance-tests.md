# Acceptance tests (SRS section 12)

| ID | Trigger | Expected outcome | Requirements | Slice |
|---|---|---|---|---|
| AT-01 | Enter the four valid measurements | Two named skirt pieces with required marks and a complete warnings list | FR-019, FR-021, FR-024 | S06 |
| AT-02 | Change hip circumference | Formula-linked waist, hip and side geometry updates; ruler changes by the expected amount | FR-021 | S06 |
| AT-03 | Delete any one mandatory measurement | Generation disabled with a specific missing-field prompt | FR-019 | S03 |
| AT-04 | Increase seam allowance | Cut path changes, stitch path identical | FR-022 | S08 |
| AT-05 | Preview textile at 3 repeat scales (R2) | Canvas changes; SVG and PDF geometry unchanged | FR-030, REN-02, FR-033 | S24 |
| AT-06 | Save and reopen | Project data and geometric output match before and after | FR-035, FR-019 | S11 |
| AT-07 | Export tiled PDF | A4 and Letter pages have alignment marks and a 100 mm scale box | FR-032 | S13 |
| AT-08 | Change native vs WASM target | Same reference pattern dimensions and warnings | REN-01, NFR-014 | S16 |
| AT-09 | Reject self-intersecting offset | Blocking message; no corrupted export | FR-022, FR-036 | S08 |
| AT-10 | Open unsupported schema version, method ID or edition pin | Safe failure; source file not modified | FR-040, FR-035, NFR-020 | S11 |
| AT-11 | Attempt invalid DXF construct (R2) | Export reports the unsupported feature rather than dropping geometry | FR-034 | S21 |
| AT-12 | Grainline-constrained marker on narrow fabric (R2) | Explicit failure or constrained alternative; no overlap | FR-031 | S25 |
| AT-13 | Size 12 chart values from the book's worked example | Output matches the pattern maker's hand draft within the G0 tolerance | FR-021, NFR-014 | S06 |
| AT-14 | Waist and hip combination just beyond and just inside the small-waist trigger | Beyond: darts widen and constants change with a warning. Inside: standard rule | FR-023 | S06 |
| AT-15 | No-adaptation preset, straight skirt preset, zero swing and flare | Zero parameters equal the block; preset changes outline and hem only | FR-041 | S07 |
| AT-16 | Chart-derived waist-to-hip with and without bust | Tagged chart-derived, warning raised, metadata records chart, size, page; no-bust fallback labelled interpretation | FR-042 | S05 |
| AT-17 | Export PDF and SVG | Toile statement with method ID, rulebook version, edition pin on every page; cannot be removed | FR-043 | S13 |
| AT-18 | Grade a base size with approved tables (R2) | Landmarks match the tables; out-of-range warns | FR-027 | S20 |
| AT-19 | Remove verification from one interpretation rule | Production export blocked; developer mode exports with UNVERIFIED legend | FR-020 | S04 |
| AT-20 | Same length as 304.8 mm, 30.48 cm, 0.3048 m, 12 in and 1 ft; switch display unit through all six and back | One identical stored value; stored values and geometry unchanged by every switch | FR-038, NFR-028 | S01 |
| AT-21 | Export SVG, PDF and DXF once per display unit | Geometry identical across display units; only legends and display-unit metadata differ; DXF units header millimetres; imperial adds 4 in square | FR-038, FR-032, FR-033 | S13 |

R2 tests (AT-05, AT-11, AT-12, AT-18) are not required to pass for the R1 gate.
