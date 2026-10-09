# ADR-0003: Pin the drafting method and track every rule

- Status: Accepted
- Date: 2026-10-09
- Source: SRS 5; FR-020; NFR-026
- Requirements: FR-020; FR-021; NFR-026; NFR-027; BR-04; BR-05; BR-22

## Context

Fit depends on a published method. Mixing editions or using untracked constants would make output untraceable.

## Decision

Method ID aldrich-mpc-4e-tailored-skirt, rulebook version 1.0.0, edition pin 4th edition (2004). Each constant is a rule record of kind B (book), I (interpretation) or X (extension) with a status. Only Verified rules reach Production export. The rulebook records functional constructions with page references and does not reproduce the book's text or figures.

## Consequences

The G0 workbook is the human source. Adopting another edition is a new method with a new reference set. Permission to use the book is unresolved (OQ-01).

## Alternatives considered

Hard-coding constants with comments was rejected: nothing enforces traceability.

## Revisit when

A later edition or a second method is adopted.
