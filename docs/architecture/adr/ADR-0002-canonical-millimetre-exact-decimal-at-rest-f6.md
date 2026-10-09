# ADR-0002: Canonical millimetre, exact decimal at rest, f64 for geometry

- Status: Accepted
- Date: 2026-10-09
- Source: FR-038, REN-01, NFR-015
- Requirements: FR-038; REN-01; NFR-028; BR-12; BR-21

## Context

Lengths arrive in six units and some come from book constants. Floating point text conversion drifts. Geometry needs fast numeric work.

## Decision

The canonical unit is the millimetre. Stored lengths are exact integer nanometre counts in an i64. Geometry is computed in f64 millimetres and the conversion happens once at the boundary. Serialised lengths are decimal strings. Fractions of an inch convert exactly only for power-of-two denominators up to 64.

## Consequences

Switching display unit cannot change stored values. Rule constants and chart data use the same type. Reviewers must watch boundary conversions.

## Alternatives considered

f64 everywhere was rejected: drift and text round-trip errors. A decimal crate is a valid alternative if the lead prefers a dependency over an own i64 type.

## Revisit when

The set of required fractions grows beyond denominator 64 (OQ-33).
