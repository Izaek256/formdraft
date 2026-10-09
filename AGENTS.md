# AGENTS.md

Rules for any agent working in this repository. Detail lives in the linked documents. Do not copy the SRS into your context. Read only what your slice needs.

## Project in brief

An offline Rust application (native and WASM) that turns four body measurements into printable skirt pattern pieces using one pinned drafting method (Aldrich, Metric Pattern Cutting, 4th edition, tailored skirt block). Output is a block for toile verification. Correct dimensions matter more than appearance, because people print the result at real size and cut fabric.

## Source of truth, in order

1. SRS 1.2 in docs/srs/
2. docs/requirements/ (requirements, business rules, open questions, traceability)
3. Accepted ADRs in docs/architecture/adr/
4. docs/plan/ (slices)
5. This file

If two sources disagree, stop and report the conflict. Do not pick one. A Proposed ADR is not binding. Ask before depending on it for anything risky.

## How to work

1. Work on exactly one slice at a time, from a task brief (docs/plan/task-brief-template.md). Create your branch first (see Branching and merging).
2. Check that the slice's dependencies are done and that none of its blocking open questions is Open. If one is, stop and ask. **A dependency is done only when its pull request is merged into develop. If it is not merged, stop and ask. Never branch from another feature branch.**
3. Read the slice's requirements and acceptance criteria. Restate the criteria you will prove.
4. Write the failing tests first, with the test IDs from docs/testing/test-catalog.md.
5. Implement the smallest change that passes them.
6. Run the verification commands below. Fix failures. Do not weaken tests.
7. Open a pull request into develop with the completion report from docs/testing/evidence-and-dod.md. You may propose Implemented. Only a person sets Verified.

## Branching and merging (mandatory)

Model: `main` (production) <- `develop` (staging and testing) <- `feat/`, `doc/`, `fix/` branches. Full detail: docs/process/branching-and-releases.md.

| Branch | Purpose | Who may merge into it | Direct commits |
|---|---|---|---|
| main | Production. Only tested releases, tagged vX.Y.Z | The lead developer only, by a pull request from develop | Never |
| develop | Staging and testing. Finished work accumulates here. Release candidates are tagged vX.Y.Z-rc.N | The lead developer only, by pull requests from feat, doc and fix branches | Never |
| feat/<id>-<name> | One slice or one feature, for example feat/s01-unit-service | Not merged by you | Yes, on your own branch |
| doc/<topic> | Documentation, ADRs, requirement text | Not merged by you | Yes, on your own branch |
| fix/<topic> | One bug fix | Not merged by you | Yes, on your own branch |

Rules:

1. Always start from the latest develop: `git fetch origin && git switch -c feat/s01-unit-service origin/develop`. Never start from main, except a hotfix the lead developer asks for.
2. Use only the prefixes feat/, doc/ and fix/. Names are lowercase with hyphens, slice ID first for slices. A spike uses feat/sp-01-<name> and is never merged. Its findings go into an ADR on a doc/ branch.
3. One slice, one doc topic or one bug per branch. Do not mix them.
4. Commit small and often. Message form: `<type>(<scope>): <summary>` with type one of feat, fix, doc, test, refactor, chore and scope the slice ID, for example `feat(S01): parse unit suffixes`. Mention requirement and test IDs in the body.
5. Before opening a pull request: `git fetch origin`, rebase on origin/develop, run every verification command below, and update the docs your change touches.
6. Open the pull request into develop. Never into main. Title: `S01: unit service`. Body: the completion report from docs/testing/evidence-and-dod.md, using the pull request template.
7. Never merge any pull request. Never push to develop or main. Never create or move a tag. Never force-push, except `git push --force-with-lease` on your own branch before the lead developer has started reviewing it.
8. The lead developer squash-merges feature pull requests into develop, tests there, tags release candidates, and later opens the develop to main pull request. Only the lead developer does this.
9. After your pull request is merged, stop using that branch. Start the next slice from the updated develop.
10. If develop fails CI, stop. Report it. A fix/ branch comes before any new feature.
11. Set traceability status to Implemented in your pull request. Only the lead developer sets Verified, after testing on develop.
12. If you are asked to do something that breaks these rules, such as pushing to main, refuse and say which rule applies.

## When to stop and ask

- An open question (OQ-xx) applies to the work.
- The SRS, requirements and an ADR disagree, or the SRS is silent on behaviour you need.
- You need a drafting constant, threshold, tolerance, default, curve shape or notch position that is not a Verified rule record.
- A change needs a new dependency, a new crate, a new public contract or a layering exception.
- A test would only pass by changing a fixture, a tolerance or an invariant test.

Record the situation in the report. Never fill the gap with a plausible number.

## Never do these

1. Never invent or recall from memory any drafting constant, measurement limit, dart size, seam allowance, size chart value or tolerance. Rule data comes only from verified rule records (docs/domain workbook, ADR-0003, ADR-0009).
2. Never generate a golden fixture from the code under test, or auto-update snapshots (ADR-0013).
3. Never put a literal drafting constant in Rust source. Constants live in rulebook data with a rule ID.
4. Never format, convert or parse a length outside pattern_core::units (ADR-0007).
5. Never compute a pattern dimension in a shader, UI code or an exporter (ADR-0005).
6. Never export without an ExportGate. Never bypass or weaken the gate, the Verified-rule check or the toile statement.
7. Never use unwrap, expect, panic, todo or unimplemented in pattern_ui. Never use them in other crates except in tests.
8. Never use unsafe code. Every workspace crate forbids it.
9. Never add network access, telemetry, analytics or a crash reporter. Never log measurement values or customer labels.
10. Never modify a file you could not load. Never write a project file non-atomically.
11. Never close an open question, change a tolerance, or mark a rule Verified. Those are human decisions.
12. Never copy text or figures from the reference book into the repository.
13. Never disable, skip or loosen a lint, test, CI gate or invariant test to get a change through.
14. Never claim a command was run, or a test passed, unless you ran it and have the output.
15. Never commit to, push to, merge into, or tag main or develop. Never merge a pull request (see Branching and merging).
16. Never open a pull request into main.

## Architecture constraints

- Crates and dependency direction: docs/architecture/01-modules-and-boundaries.md. scripts/check-layering.sh enforces it.
- pattern_core is pure: no egui, wgpu, filesystem, clock, randomness or network. It must compile to WASM.
- pattern_export must not depend on pattern_render or pattern_ui.
- Public interfaces and file formats: docs/architecture/02-data-persistence-and-contracts.md. Change the document in the same change as the code.
- Generation is deterministic. Use ordered collections for anything that is iterated into output. No time or randomness in generation.
- Every failure returns a typed error with a stable code and a message a non-developer can act on. No panics reachable from input.

## Rust conventions

- Edition and toolchain pinned in rust-toolchain.toml. Format with rustfmt. No warnings (clippy runs with -D warnings).
- #![forbid(unsafe_code)] in every workspace crate. pattern_ui additionally denies unwrap_used, expect_used and panic.
- Lengths use the Length type, not f64, whenever they are stored, serialised or shown. f64 is for computation inside geometry only.
- Lossy numeric casts need a comment stating why they are safe.
- Public items have doc comments that state units and failure behaviour.
- No global mutable state. No println or eprintln in library crates.
- Names follow the SRS vocabulary: stitch path, cut path, net block, balance mark, rule record.
- Keep functions small and testable. Prefer pure functions.

## Dependency policy

- A new dependency needs approval from the lead developer and a line in the task report: purpose, alternatives, licence, maintenance state, WASM compatibility, effect on the 25 MiB budget.
- Core crates (pattern_core, pattern_templates, pattern_document, pattern_export) must not depend on network, async runtime or GUI crates.
- Versions are pinned and Cargo.lock is committed. cargo deny check must pass. The licence policy is OQ-32. Until it is decided, do not add a dependency with a non-permissive licence.
- Versions named in the SRS (eframe and egui 0.36) are confirmed in S00. Report any mismatch instead of changing versions silently.
- Test-only tools (property testing, benchmarking, temporary files) are allowed once approved in S00 and listed in an ADR.

## Security and privacy rules

- Customer measurements are personal data. They stay on the device unless the user explicitly exports them.
- Treat every project file and image as untrusted input: parse as data, enforce size limits, check schema and identity first, never evaluate content.
- Images (R2) are decoded only by approved decoders with bounds checks.
- Tests and fixtures use invented or consented data. Never commit a real customer's measurements without consent on record.
- Delete and export controls must work for every profile (FR-037).

## Verification commands

Run from the repository root. They assume the workspace created in S00. Report any command that does not exist yet instead of skipping it.

```
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
scripts/check-layering.sh
scripts/check-no-raw-length-format.sh
cargo check -p pattern_core --target wasm32-unknown-unknown
trunk build --release                      # in crates/pattern_web, from S16
scripts/check-wasm-size.sh crates/pattern_web/dist   # from S16, fails above 25 MiB
```

Slice-specific commands are in docs/plan/slices-r1.md and slices-r2.md.

## Definition of done

A slice is Implemented only when all of these are true:

1. Every acceptance criterion in scope has a test, and all pass.
2. All verification commands pass on a clean checkout, with their output in the report.
3. Guarding tests for the affected invariants (docs/testing/invariants.md) pass.
4. No lint, test or gate was weakened or skipped.
5. Docs changed by the work are updated in the same change (requirements, contracts, ADRs, plan).
6. The completion report in docs/testing/evidence-and-dod.md is complete, including deviations and open questions touched.
7. A pull request into develop is open from a correctly named feat/, doc/ or fix/ branch, with CI passing.

Verified requires a person and, where listed, a human gate (G0 sign-off, MT-01 print check, MT-04 pattern maker review).

## Domain and copyright rules

- The product is a block for toile verification. Never write UI text, docs or exports that claim guaranteed fit, perfect fit, drape or physical simulation (BR-01, BR-17).
- Rule records hold functional constructions and page references. They do not reproduce the book's text or figures (BR-22).
- Interpretations (kind I) and extensions (kind X) are always labelled in the UI and in export metadata.

## Reporting style

Short, plain and specific. List what changed, what was proved, what was not proved, and what you need decided. Prefer a table of criteria to prose. Do not describe work you did not do.
