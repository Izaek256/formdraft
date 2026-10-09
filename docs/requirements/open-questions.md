# Open questions and preserved ambiguities

These are unresolved. The agent must not answer them by guessing. When work reaches one, stop, record the situation in the task report, and ask the lead developer. Closing an item means writing the decision here with a date and updating the affected requirement.

| ID | Question | Source | Affects | Owner | Status | Decision and date |
|---|---|---|---|---|---|---|
| OQ-01 | Permission to use and cite the source book, and the licence position for the rulebook. | SRS 15 item 1 | FR-020; release | Lead developer | Open | |
| OQ-02 | Plausibility thresholds and cross-field rules for the four measurements and bust. | SRS 15 item 2; Thresholds sheet | FR-019; S03 | Pattern maker | Open | |
| OQ-03 | Target population and size range. | SRS 15 item 3 | FR-042; FR-027 | Lead developer | Open | |
| OQ-04 | Offline storage model, customer consent record, and deletion semantics. | SRS 15 item 4 | FR-037 | Lead developer | Open | |
| OQ-05 | Reference printer and the numeric print tolerance. | SRS 15 item 5 | FR-032; NFR-015 | Lead developer | Open | |
| OQ-06 | Priority of native versus browser export. | SRS 15 item 6 | S12; S13; S16 | Lead developer | Open | |
| OQ-07 | DXF dialect and the two CAD viewers. | SRS 15 item 7 | FR-034 | Lead developer | Open | |
| OQ-08 | Initial host target and asset cap. | SRS 15 item 8 | NFR-019; S16 | Lead developer | Open | |
| OQ-09 | Whether a multi-language UI is required. | SRS 15 item 9 | FR-039; S14 | Lead developer | Open | |
| OQ-10 | Named owner of fit validation (qualified pattern maker). Blocks G0. | SRS 15 item 10 | FR-021; DOM-01 | Lead developer | Open | |
| OQ-11 | Numeric definition of the waistline and side seam curves. | SRS 5.6; ALD4-INT-001 to 004 | FR-026; FR-021 | Pattern maker with lead developer | Open | |
| OQ-12 | Notch positions and grainline position and length. | SRS 5.6; ALD4-INT-006, 007 | FR-024 | Pattern maker | Open | |
| OQ-13 | Numeric trigger for the small-waist rule. | SRS 5.4; ALD4-FIT-001 | FR-023 | Pattern maker | Open | |
| OQ-14 | Default seam allowances: side seams, hem, waist edge. | SRS FR-022; ALD4-SA-001 to 003 | FR-022 | Pattern maker | Open | |
| OQ-15 | Chart size selection when bust is not supplied. | SRS 5.3; ALD4-INT-009 | FR-042 | Pattern maker | Open | |
| OQ-16 | Default display unit per quantity class. | SRS 15 item 15 | FR-038 | Lead developer | Decided | 2026-10-09: Provisional defaults per quantity class: body measurements cm, pattern dimensions mm, allowances and tolerances mm, fabric width cm, fabric length m. The two fabric classes go beyond the SRS proposal. All five marked PROVISIONAL. |
| OQ-17 | Tolerance between code output and hand drafts. | SRS 15 item 16 | FR-021; NFR-014; AT-13 | Pattern maker | Open | |
| OQ-18 | SRS 1.0 requirements FR-001 to FR-018 and NFR-001 to NFR-013 were not provided. This register is incomplete without them. | Process gap | Whole register | Lead developer | Open | |
| OQ-19 | Corner policy for seam allowance joins. The source gives none. | FR-022; ALD4-SA-005 | FR-022 | Lead developer with pattern maker | Open | |
| OQ-20 | Definition of a severe self-intersection and of excessive offset curvature. | FR-022; FR-036 | FR-022; FR-036; AT-09 | Lead developer | Open | |
| OQ-21 | Project file format: JSON or a documented binary format, single file or directory. | FR-035; ADR-0010 | FR-035; S11 | Lead developer | Open | |
| OQ-22 | What anonymisable means for a profile. | FR-019 | FR-019 | Lead developer | Open | |
| OQ-23 | Reference hardware for NFR-017 and the print baseline. | SRS 10 | NFR-017 | Lead developer | Open | |
| OQ-24 | Minimum browser and operating system matrix. | NFR-023 | NFR-023; S16 | Lead developer | Open | |
| OQ-25 | What atomic means for a browser download and how the last valid version is protected. | FR-035 | FR-035 | Lead developer | Open | |
| OQ-26 | Tile overlap size, margins and registration target design. | FR-032 | FR-032 | Lead developer | Open | |
| OQ-27 | Dart legs, apex and drill point construction. | ALD4-INT-005 | FR-021; FR-023; FR-024 | Pattern maker | Open | |
| OQ-28 | Hem line shape and corner treatment after flare. | ALD4-INT-008 | FR-041 | Pattern maker | Open | |
| OQ-29 | Whether undo and redo are in R1 and how deep. | SRS 9; FR-025 | S11; S22 | Lead developer | Open | |
| OQ-30 | How production mode and developer mode are selected, and who may enable developer mode (affects UNVERIFIED exports). | FR-020; AT-19 | FR-020; S04; S10 | Lead developer | Open | |
| OQ-31 | PDF font embedding and non-Latin text for legends. | FR-032; OQ-09 | FR-032 | Lead developer | Open | |
| OQ-32 | Licence of the application and the dependency licence policy. | Process | AGENTS.md; cargo deny | Lead developer | Open | |
| OQ-33 | Display precision per unit. | FR-038 | FR-038 | Lead developer | Decided | 2026-10-09: Display precision (decimal places): mm 1, cm 1, m 3, in 2 (fraction mode: nearest 1/16), ft 3, yd 3. Inch fraction denominators: 2, 4, 8, 16, 32, 64; default 16. Input conventions: only the six abbreviations mm, cm, m, in, ft, yd, case-insensitive, with or without a space; dot decimal point only; fractions for inches only; no apostrophe or quote symbols in S01. Display rounding: half away from zero, display only, never stored values. |
| OQ-34 | CI host and release distribution channel for native builds. | Process | S00; S18 | Lead developer | Open | |
| OQ-35 | Resolution order of unit preferences across scope and quantity class. | FR-038 | FR-038; S01 | Lead developer | Decided | 2026-10-09: Most specific wins: project with quantity class, then global with quantity class, then project default, then global default. |

## Slices that each question blocks

| Question | Slices |
|---|---|
| OQ-01 | DOM-01 |
| OQ-02 | DOM-01, S03 |
| OQ-03 | S05 |
| OQ-04 | S17 |
| OQ-05 | S13, S18 |
| OQ-06 | SP-01, S12 |
| OQ-07 | SP-01, S21 |
| OQ-08 | S16 |
| OQ-09 | S14 |
| OQ-10 | DOM-01, S06, S18 |
| OQ-11 | DOM-01, S02, S06 |
| OQ-12 | DOM-01, S09 |
| OQ-13 | DOM-01, S06 |
| OQ-14 | DOM-01, S08 |
| OQ-15 | DOM-01, S05 |
| OQ-16 | S01 |
| OQ-17 | DOM-01, S06 |
| OQ-18 | none directly |
| OQ-19 | SP-02, S08 |
| OQ-20 | SP-02, S08, S10 |
| OQ-21 | S11 |
| OQ-22 | S03 |
| OQ-23 | S14 |
| OQ-24 | S16 |
| OQ-25 | S11 |
| OQ-26 | S13 |
| OQ-27 | DOM-01, S06 |
| OQ-28 | DOM-01, S07 |
| OQ-29 | S11, S22 |
| OQ-30 | S04, S10 |
| OQ-31 | SP-01, S13 |
| OQ-32 | S00 |
| OQ-33 | S01 |
| OQ-34 | S00 |
| OQ-35 | S01 |
