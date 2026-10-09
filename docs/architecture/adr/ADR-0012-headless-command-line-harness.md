# ADR-0012: Headless command-line harness

- Status: Proposed
- Date: 2026-10-09
- Source: NFR-014, NFR-018
- Requirements: NFR-014; NFR-018; AT-01 to AT-21

## Context

Agents and CI need to generate and export without a GUI to verify behaviour.

## Decision

tools/pattern-cli uses the same crates to generate a pattern from a profile file and export it. It is not shipped and holds no logic of its own.

## Consequences

Faster verification and reproducible bug reports. A small additional crate to maintain.

## Alternatives considered

Testing only through the UI was rejected as slow and fragile.

## Revisit when

It starts to hold logic. Move it into a library crate.
