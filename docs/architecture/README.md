# Architecture

Design for the Automated Garment Pattern Generation System, based on SRS 1.2. It is an offline, single-user application with no server and no database, so the usual architecture headings are adapted. The mapping is explicit:

| Usual heading | What it means here | Where |
|---|---|---|
| Module boundaries | Eight Rust crates from the SRS plus a headless tool | 01-modules-and-boundaries.md |
| Database ownership | Ownership of every persisted or loaded data set. There is no database | 02-data-persistence-and-contracts.md |
| API contracts | Public crate interfaces and the file contracts (project, SVG, PDF, DXF) | 02-data-persistence-and-contracts.md |
| Authentication and authorization | Not applicable. Replaced by trust boundaries and privacy controls | 03-dataflow-failure-security-deployment.md, ADR-0006 |
| Data flow, failure handling, deployment | As named | 03-dataflow-failure-security-deployment.md |
| Decisions | Architecture decision records | adr/ |

## Reading order

1. 01-modules-and-boundaries.md
2. 02-data-persistence-and-contracts.md
3. 03-dataflow-failure-security-deployment.md
4. adr/README.md for the decision index and status

## Status of this design

Decisions copied from the SRS are marked Accepted in the ADRs. Decisions I derived from the SRS but the SRS does not state are marked Proposed. The lead developer accepts or changes them before any slice that depends on them starts. Contract sketches use illustrative names. The behaviour described is binding. The exact Rust names are not.
