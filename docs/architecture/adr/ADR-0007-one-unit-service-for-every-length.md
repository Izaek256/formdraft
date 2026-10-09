# ADR-0007: One unit service for every length

- Status: Accepted
- Date: 2026-10-09
- Source: FR-038, NFR-028
- Requirements: FR-038; NFR-028; BR-12

## Context

Six units must work everywhere. Scattered formatting leads to unit bugs in the one place where they cost fabric.

## Decision

pattern_core::units is the only place that parses, converts or formats a length. A static script fails CI if any other module formats a length directly. Display units are preferences. They never change stored values.

## Consequences

UI, exports, warnings and reports call the service. Precision per unit is data (OQ-33).

## Alternatives considered

Per-module formatting was rejected as untestable.

## Revisit when

A required unit cannot be expressed by the service.
