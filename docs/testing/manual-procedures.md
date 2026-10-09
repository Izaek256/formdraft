# Manual procedures

Each procedure produces a short record stored under docs/testing/evidence/. Records name the date, person, equipment and result. A blank record means the procedure was not done.

## MT-01 Print calibration (FR-032, NFR-015)

1. Export the tiled PDF for a reference pattern on A4, and on Letter if both are supported.
2. Print on the reference printer (OQ-05) at 100% or actual size. Do not use fit to page.
3. Measure the 100 mm calibration square along both sides with a steel rule, at least twice per page. For an imperial display unit also measure the 4 in square.
4. Tape the tiles along the registration marks. Check that edges meet and no piece edge is missing.
5. Record printer model, driver setting, measured values and the tolerance used. Pass when every measurement is within the tolerance set by OQ-05.

## MT-02 Accessibility review (FR-039, NFR-025)

1. Complete WF-01 using the keyboard only: enter four measurements, change units, generate, read warnings, save, export.
2. Repeat with one screen reader on the target platform. The reader choice is decided at S14 and recorded.
3. Check labels, focus after an error, status announcements, contrast themes and the numeric view of dimensions.
4. Record each failure with the step and the expected behaviour.

## MT-03 CAD viewer check (FR-034, R2)

1. Export DXF for two reference patterns.
2. Open each in the two viewers chosen by OQ-07.
3. Confirm layers (stitch, cut, annotations, grainline, notches), units, and the length of a known edge.
4. Record any unsupported construct and confirm the exporter reported it.

## MT-04 Pattern maker review (FR-021, G0)

1. Print each of the ten reference results at full scale.
2. The pattern maker lays each over their hand draft from the same edition.
3. Record the deviation in millimetres at each named landmark and whether it is within the G0 tolerance.
4. The pattern maker signs the record. Any rejected landmark returns to the rule register as Open.

## MT-05 Toile trial (recommended, not required by the SRS)

1. Cut and sew one reference pattern in calico, with the seam allowances the app produced.
2. Check balance, darts, ease and hang on a person or dress form.
3. Record findings. This is evidence about the method, not a software acceptance test, and it supports the toile statement (FR-043).
