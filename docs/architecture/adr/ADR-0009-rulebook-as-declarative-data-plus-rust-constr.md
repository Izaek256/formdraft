# ADR-0009: Rulebook as declarative data plus Rust construction code

- Status: Proposed
- Date: 2026-10-09
- Source: FR-040, NFR-026, NFR-027
- Requirements: FR-020; FR-040; NFR-026; NFR-027; BR-21

## Context

Drafting constants must be traceable and replaceable without hiding them in code, while FR-040 forbids user-provided code.

## Decision

Construction logic is Rust per template. Every numeric constant, threshold and chart value is TOML data keyed by rule ID. Rust source contains no literal drafting constant. The loader validates data, the checksum covers it, and a completeness test fails when code references an unknown rule ID or a record lacks page, kind or verifier. The workbook Expression column is documentation. The executable form is the code, and the golden tests prove both agree.

## Consequences

Data and code can disagree. Reference drafts are the check (ADR-0013).

## Alternatives considered

An interpreted expression language was rejected: more risk, and the SRS prohibits executable project content.

## Revisit when

The number of rules makes typed keys impractical.
