# Evidence and definition of done

A feature is not done because the code exists. It is done when the evidence below exists and a reviewer can check it.

## States

| State | Meaning |
|---|---|
| Not started | No work |
| In progress | Work started, evidence incomplete |
| Implemented | Code merged and tests pass, evidence not yet reviewed |
| Verified | All evidence below attached and accepted by the lead developer. Human gates passed where required |

Only a person sets Verified in traceability.csv, after testing on develop. An agent may set Implemented in its pull request.

## Evidence for every slice

1. The list of requirement IDs and acceptance criteria IDs the slice covers, each marked met or not met with the test IDs that show it.
2. The full output of the slice's verification commands, with the commit hash.
3. The list of invariants the change could affect and the guarding tests that passed.
4. New or changed tests, and for a bug fix the failing-first test.
5. The open questions touched, and how they were handled (stopped and asked, or not relevant). No open question is closed by the agent.
6. Deviations from the slice or the SRS, stated plainly.
7. Dependencies added or changed, with the policy check result.
8. Documents updated: requirements, ADRs, contracts, plan.

## Extra evidence by kind of work

| Kind | Additional evidence |
|---|---|
| Drafting rule or template (S05 to S07, S09) | Rule IDs used, all Verified. Golden results for every reference case. Pattern maker sign-off recorded |
| Geometry or offset (S02, S08) | Property test seeds, reference cases, measured distance error against 0.1 mm, benchmark if on the budget path |
| Units (S01) | Exactness table for all pairs, AT-20 result, static script result |
| Export (S12, S13, S21) | Parsed-file test results, path length error, calibration square check, MT-01 record for PDF |
| Persistence (S11) | Fault injection log, migration fixtures, fuzz run summary |
| UI (S14, S15) | Keyboard-only run record, accessibility tree test, frame timing on the reference hardware |
| Web (S16) | Size gate output, native versus WASM comparison, browser matrix results |
| Privacy (S17) | Network-disabled run, trace summary |

## Human gates

| Gate | Who | Evidence recorded |
|---|---|---|
| G0 | Named pattern maker, lead developer | Completed workbook, G0_signoff, reference drafts, tolerance |
| Fit sign-off for FR-021 | Pattern maker | MT-04 record |
| Print acceptance (G4) | Lead developer | MT-01 record with printer and measured values |
| Accessibility review | Reviewer | MT-02 record |
| R2 CAD interoperability (G6) | Lead developer | MT-03 record |

## Completion report template

```
Slice: S__  Branch: ______  PR into develop: ______  Commit: ______
Requirements and criteria: FR-___ AC-___ (met / not met)
Tests added or changed: T-___
Verification commands and results: (paste)
Invariants affected and guarding tests run: INV-__ (T-___)
Open questions touched: OQ-__ (action taken)
Deviations: (none or list)
Dependencies changed: (none or list with policy check)
Docs updated: (list)
Known gaps: (list)
Status proposed: Implemented
```

## What an agent must never claim

- Verified, fit, accurate or compliant without the evidence above.
- That a test passes when it was not run.
- That a number is correct because the code produced it. Numbers are correct when they match a person's hand draft.
