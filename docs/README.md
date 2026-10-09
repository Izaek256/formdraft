# Project documentation

Project: Automated Garment Pattern Generation System (offline Rust application that turns four body measurements into printable skirt patterns, using one pinned drafting method).

These documents let an agent build the system in small, checked steps. They are designed from SRS 1.2. Where the SRS is silent they preserve the gap as an open question instead of inventing an answer.

| Folder or file | Purpose |
|---|---|
| ../AGENTS.md | Rules every agent follows |
| srs/ | The SRS, the top source of truth (to be added) |
| requirements/ | 44 uniquely identified requirements, business rules, open questions, acceptance tests, traceability |
| architecture/ | Modules, data ownership, contracts, flow, failure handling, security, deployment, ADRs |
| plan/ | Dependency-aware slices with files, criteria and verification commands |
| testing/ | Quality strategy, invariants, evidence required, manual procedures, test catalog |
| domain/ | The G0 rule register and reference drafts, owned by people |
| process/ | Branching strategy and release process |
| ../.github/pull_request_template.md | Pull request template used by agents |

## Reading order for a new agent

1. AGENTS.md
2. requirements/README.md, then the requirement groups for the slice
3. requirements/open-questions.md
4. architecture/01 and 02, then the ADRs the slice names
5. plan/slices for the slice, plus testing/invariants.md

## What is not here yet

- SRS 1.0 requirements (OQ-18).
- Any drafting constant, tolerance or threshold. They come from the completed G0 workbook (DOM-01).
- Code, CI configuration and the scripts the plan refers to. S00 creates them.
