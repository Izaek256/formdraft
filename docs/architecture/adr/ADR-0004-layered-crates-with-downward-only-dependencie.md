# ADR-0004: Layered crates with downward-only dependencies

- Status: Accepted
- Date: 2026-10-09
- Source: SRS 9
- Requirements: REN-02; NFR-022

## Context

The SRS defines eight crates with compatibility rules, such as no egui or wgpu in the core.

## Decision

Use the dependency matrix in 01-modules-and-boundaries.md. scripts/check-layering.sh reads cargo metadata and fails CI on a violation. pattern_export must not depend on pattern_render or pattern_ui.

## Consequences

Core logic is testable without a GUI and compiles to WASM. Cross-crate shortcuts are blocked by CI, not by review alone.

## Alternatives considered

A single crate was rejected: it cannot enforce the separation.

## Revisit when

A crate needs a dependency the matrix forbids. That requires a new ADR.
