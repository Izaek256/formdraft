# ADR-0008: Stitch path and cut path as separate objects; offset algorithm to be chosen

- Status: Model Accepted; algorithm Proposed
- Date: 2026-10-09
- Source: FR-022, BR-06, BR-07
- Requirements: FR-022; NFR-015; NFR-016

## Context

The SRS separates net stitch geometry from cut geometry and requires correct offsets on curves and corners.

## Decision

A piece holds a net stitch path and a cut path derived from it with per-edge allowance. The cut path is never edited independently. The algorithm is chosen by spike SP-02. It must hold distance within 0.1 mm, detect self-intersections, report an allowance larger than a feature, and apply a corner policy (OQ-19).

## Consequences

Candidate approaches are flatten then offset a polygon, or offset segments analytically and approximate curve offsets. SP-02 records the result.

## Alternatives considered

Editing cut paths directly was rejected: it breaks AT-04.

## Revisit when

SP-02 shows no candidate meets the tolerance.
