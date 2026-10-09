# Business rules

Rules that cut across requirements. Each is traceable to the SRS. Requirement files cite them as BR-xx.

| ID | Rule | Source |
|---|---|---|
| BR-01 | Output is a block for toile verification. The system never claims fit, perfect fit, drape or physics. | SRS 5.7, 15; FR-043 |
| BR-02 | Never invent an unavailable anatomical measure. A chart-derived value is allowed only for waist-to-hip, tagged, with a warning. | SRS 5.3; FR-019; FR-042 |
| BR-03 | Every measurement value carries a source tag: measured, chart-derived or estimated. | SRS 5.3; FR-019 |
| BR-04 | The method is pinned by method ID, rulebook version and edition pin (Aldrich 4th ed.). Editions are never mixed in one project. | SRS 5.1; FR-020; FR-035 |
| BR-05 | Rules have kind B, I or X. Only Verified rules reach production export. Kind X is labelled. Developer mode exports carry an UNVERIFIED legend. | SRS 5.5; FR-020 |
| BR-06 | The block is net. Seam allowance is added afterwards. No allowance on a fold line. | SRS 5.4; FR-022 |
| BR-07 | Stitch path and cut path are separate objects. Changing allowance changes the cut path only. | FR-022; AT-04 |
| BR-08 | Ease is embedded in rule constants. User-adjustable ease is an extension (kind X). | SRS 5.4 |
| BR-09 | The side seam is deliberately off-centre. The asymmetry is correct and must not be normalised. | SRS 5.4 |
| BR-10 | The back has two darts and the front has one. A small-waist conditional rule widens darts and changes two waist constants. The trigger is not numeric in the source. | SRS 5.4; FR-023 |
| BR-11 | The straight skirt is an adaptation of the block. Zero swing and zero flare equal the block. | SRS 4; FR-041 |
| BR-12 | The canonical unit is the millimetre, stored as an exact decimal. Display unit never changes stored values. All conversions are exact. | FR-038 |
| BR-13 | Pattern geometry is authoritative on the CPU. The GPU layer is view only and never alters exports. | REN-01; REN-02 |
| BR-14 | Critical errors block export. Recoverable warnings stay visible. No silently incomplete output. | FR-036; NFR-016 |
| BR-15 | Offline by default. No telemetry. Measurements leave the device only by explicit user action. | FR-037; NFR-018; NFR-024 |
| BR-16 | Unknown schema, template, method or edition fails safely and never modifies the source file. | FR-040; NFR-020; AT-10 |
| BR-17 | Fabric consumption and material previews are estimates and visual approximations, never guarantees or physics. | FR-030; FR-031; SRS 15 |
| BR-18 | Grading is per-landmark increments from tables, never uniform scaling. | FR-027 |
| BR-19 | Chart size is chosen by bust when supplied. The hips fallback is an interpretation and warns. | SRS 5.3 |
| BR-20 | Qualitative curves, notch positions, grainline placement and dart construction are interpretations (kind I) decided and verified at G0. | SRS 5.6 |
| BR-21 | Rule constants and chart data are stored as printed with their unit and converted once to exact millimetres. | SRS 5.5; REN-01 |
| BR-22 | The rulebook records functional constructions with page references. It does not reproduce the book's text or figures. | SRS 5.1 |

## Requirements that cite each rule

| Rule | Requirements |
|---|---|
| BR-01 | FR-043, FR-032 |
| BR-02 | FR-019, FR-042, FR-036 |
| BR-03 | FR-019 |
| BR-04 | FR-020, FR-035, NFR-014 |
| BR-05 | FR-020, FR-036, FR-025, NFR-026 |
| BR-06 | FR-021, FR-022 |
| BR-07 | FR-022 |
| BR-08 | FR-021, FR-025 |
| BR-09 | FR-021 |
| BR-10 | FR-021, FR-023 |
| BR-11 | FR-041 |
| BR-12 | FR-019, FR-033, FR-034, FR-038, NFR-028 |
| BR-13 | FR-033, FR-030, REN-01, REN-02, REN-03 |
| BR-14 | FR-032, FR-036, NFR-016 |
| BR-15 | FR-037, NFR-018, NFR-024 |
| BR-16 | FR-040, FR-035, NFR-020 |
| BR-17 | FR-029, FR-030, FR-031, REN-04 |
| BR-18 | FR-027 |
| BR-19 | FR-042 |
| BR-20 | FR-021, FR-024, FR-026 |
| BR-21 | FR-038, REN-01 |
| BR-22 | FR-020, NFR-026 |
