# Architecture decision records

Short records of decisions. Status Accepted means it comes from the SRS. Status Proposed means it is my design and waits for the lead developer. Agents treat a Proposed ADR as not yet binding and ask before depending on it for risky work.

|ID|Title|Status|Source|
|-|-|-|-|
|ADR-0001|[Rust-first single-language product](ADR-0001-rust-first-single-language-product.md)|Accepted|SRS 9 language policy|
|ADR-0002|[Canonical millimetre, exact decimal at rest, f64 for geometry](ADR-0002-canonical-millimetre-exact-decimal-at-rest-f6.md)|Accepted|FR-038, REN-01, NFR-015|
|ADR-0003|[Pin the drafting method and track every rule](ADR-0003-pin-the-drafting-method-and-track-every-rule.md)|Accepted|SRS 5; FR-020; NFR-026|
|ADR-0004|[Layered crates with downward-only dependencies](ADR-0004-layered-crates-with-downward-only-dependencie.md)|Accepted|SRS 9|
|ADR-0005|[CPU-authoritative geometry, GPU for view only](ADR-0005-cpu-authoritative-geometry-gpu-for-view-only.md)|Accepted|REN-01, REN-02|
|ADR-0006|[Offline-only: no server, no database, no authentication](ADR-0006-offline-only-no-server-no-database-no-authent.md)|Accepted|FR-037, NFR-018, NFR-024|
|ADR-0007|[One unit service for every length](ADR-0007-one-unit-service-for-every-length.md)|Accepted|FR-038, NFR-028|
|ADR-0008|[Stitch path and cut path as separate objects; offset algorithm to be chosen](ADR-0008-stitch-path-and-cut-path-as-separate-objects.md)|Accepted|FR-022, BR-06, BR-07|
|ADR-0009|[Rulebook as declarative data plus Rust construction code](ADR-0009-rulebook-as-declarative-data-plus-rust-constr.md)|Accepted|FR-040, NFR-026, NFR-027|
|ADR-0010|[Project file format](ADR-0010-project-file-format.md)|Accepted (OQ-21)|FR-035, NFR-020|
|ADR-0011|[Export strategy](ADR-0011-export-strategy.md)|Accepted pending SP-01|FR-032, FR-033, FR-034, NFR-019|
|ADR-0012|[Headless command-line harness](ADR-0012-headless-command-line-harness.md)|Accepted|NFR-014, NFR-018|
|ADR-0013|[Golden test policy](ADR-0013-golden-test-policy.md)|Proposed|FR-021, NFR-014|

## Template

Context, Decision, Consequences, Alternatives considered, Revisit when. Keep each ADR under one page. A decision that reverses an earlier one gets a new ADR and the old one is marked Superseded.

