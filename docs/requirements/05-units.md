# Requirements: Units

Derived from SRS 1.2. Wording is condensed and made testable. Numeric values the SRS does not give are marked TBD with an open question (OQ-xx). Nothing here invents an answer.

Status values in the traceability file: Not started, In progress, Implemented, Verified.

## FR-038 Global unit system

- Release and priority: R1 / Must
- SRS reference: SRS 7 FR-038 (rev 1.2)
- Implementing slice: S01 (see ../plan/)
- Business rules: BR-12, BR-21
- Open questions: OQ-16, OQ-33
- Acceptance tests: AT-20, AT-21

**Statement.** Length units mm, cm, m, in, ft and yd everywhere a length is entered, shown or exported, over one canonical millimetre value stored as an exact decimal.

**Acceptance criteria**

- AC-FR-038-1: Six units are supported with exact factors: 1 cm = 10 mm, 1 m = 1000 mm, 1 in = 25.4 mm, 1 ft = 304.8 mm, 1 yd = 914.4 mm.
- AC-FR-038-2: Inches display as decimals or fractions. The default denominator is 16 and is configurable.
- AC-FR-038-3: A typed suffix such as 94 cm or 37 3/8 in overrides the field default.
- AC-FR-038-4: Unit preferences exist at global scope, project scope and per quantity class. Defaults per quantity class are decided in OQ-16 (2026-10-09) and marked PROVISIONAL.
- AC-FR-038-5: All unit parsing, conversion and formatting go through one unit service. No other module formats a length (NFR-028).
- AC-FR-038-6: Switching display unit never changes a stored value or any geometry (AT-20).
- AC-FR-038-7: Exports carry geometry in millimetre coordinates and record the display unit in metadata (AT-21).
- AC-FR-038-8: Values round only at display time, to a documented precision per unit (OQ-33, decided 2026-10-09, rounding half away from zero).

**Planned tests**

- T-FR-038-01 [UNIT] Exact conversion for every unit pair
- T-FR-038-02 [PROP] Parse then format then parse is stable for every unit
- T-FR-038-03 [UNIT] Suffix override and fractional inch parsing
- T-FR-038-04 [STATIC] Script fails when any module formats a length directly
- T-FR-038-05 [INTEG] AT-20 same length in five units stores one value
- T-FR-038-06 [INTEG] AT-21 exports across display units
- T-FR-038-07 [PROP] Arbitrary text never panics the parser
- T-FR-038-08 [UNIT] Preference resolution order across scope and quantity class
- T-FR-038-09 [UNIT] Display rounding is half away from zero
