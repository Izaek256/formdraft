# Task brief template

Copy this for each agent task. Keep it short. The agent reads AGENTS.md and the linked documents itself.

```
Slice: S__ name
Branch: feat/s__-name, created from the latest develop. PR target: develop
Goal (one sentence):
Requirements and acceptance criteria in scope: FR-___ (AC-___ to AC-___)
Out of scope: (name anything nearby that must not be touched)
Dependencies confirmed done: S__, S__
Open questions that apply and their current status: OQ-__ Open / Decided (see open-questions.md)
ADRs that apply and their status: ADR-____ Accepted / Proposed
Expected files or modules: (from docs/plan)
Verification commands: (from docs/plan)
Invariants to protect: INV-__
Evidence required: docs/testing/evidence-and-dod.md
Stop and ask if: (any situation not covered by the documents)
```

Rules for writing a brief:

- One slice per brief.
- Name what is out of scope. Agents drift into neighbouring work.
- Do not paste the SRS. Link to the requirement IDs.
- If an open question applies, say whether to stop at it or to build around it.
