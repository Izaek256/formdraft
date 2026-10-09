# Implementation plan

The project is cut into small vertical slices. Each slice delivers one working, testable capability, lists its dependencies, expected files, acceptance criteria and verification commands, and names the open questions that block it.

## Files

| File | Content |
|---|---|
| slices-r1.md | R1 slices, spikes and the domain baseline DOM-01 |
| slices-r2.md | R2 slices and DOM-02 |
| dependency-graph.md | Graph, one valid build order, parallel tracks, critical path |
| task-brief-template.md | The brief to give an agent for one slice |

## How to run a slice

1. Pick a slice whose dependencies are done and whose blocking questions are answered (open-questions.md).
2. Fill in task-brief-template.md and give it to the agent with AGENTS.md.
3. The agent creates a feat/ branch from develop, writes failing tests first, implements, runs the verification commands and opens a pull request into develop with the completion report from docs/testing/evidence-and-dod.md.
4. A person reviews the evidence, squash merges into develop, and updates traceability.csv. Work then accumulates on develop and moves to main by release pull request (docs/process/branching-and-releases.md).
5. One slice per task. If a slice grows, split it and update this folder first.

## Gates mapped to slices

| SRS gate | Slices | Pass condition (SRS 14) |
|---|---|---|
| G0 Domain baseline | DOM-01 | Qualified pattern maker agrees on equations and conventions |
| G1 Geometry engine | S01 to S07 | Golden numerical tests pass |
| G2 Construction | S08 to S10 | Construction and offset tests pass |
| G3 App UI | S11, S14, S17 | Full offline workflow demonstrated |
| G4 Export | SP-01, S12, S13 | Print acceptance achieved |
| G5 Web build | SP-03, S15, S16 | Browser tests and the 25 MiB CI gate |
| R1 release | S18 | Every R1 acceptance test passes and the evidence pack is complete |
| G6 R2 professional | S20 to S25, DOM-02 | Domain-specific interoperability tests pass |

## Rules for the plan

- A slice may not be started while a blocking open question is Open, unless the slice is written to run with the value unset and to fail clearly (as S03 does for thresholds).
- Slices that need verified drafting data (S05, S06, S07, S09) wait for DOM-01. Until then the agent may build only the parts that need no drafting constant, using placeholder rule records with status Open.
- Spikes produce a decision and a throwaway prototype. Spike code is not merged into the product crates.
- Verification commands assume the workspace from S00. They were not run, because no code exists yet. S00 is also the check that they work.
