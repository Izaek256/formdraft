# ADR-0005: CPU-authoritative geometry, GPU for view only

- Status: Accepted
- Date: 2026-10-09
- Source: REN-01, REN-02
- Requirements: REN-01; REN-02; REN-03; BR-13

## Context

Exports must be dimensionally exact and independent of rendering choices.

## Decision

All pattern calculations run on the CPU. The wgpu layer draws shadows, grid, checkerboard, textures and may draw tessellated vectors. A render-only setting change must leave exports byte-identical.

## Consequences

A GPU fallback path is required (SP-03). No WGSL code computes a pattern dimension.

## Alternatives considered

Computing geometry in shaders was rejected for precision and reproducibility.

## Revisit when

GPU computation is proposed for layout in R2 or R3.
