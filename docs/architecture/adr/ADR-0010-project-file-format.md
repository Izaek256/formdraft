# ADR-0010: Project file format

- Status: Proposed (OQ-21)
- Date: 2026-10-09
- Source: FR-035, NFR-020
- Requirements: FR-035; NFR-020; NFR-021

## Context

The SRS allows JSON or a documented binary format and requires human-readable metadata.

## Decision

A single UTF-8 JSON file in R1 with fixed field order, schema_version, method reference, rule hashes, and lengths as decimal strings. The generated pattern is rebuilt on load and compared with a stored checksum. Assets (R2) need a container decision later.

## Consequences

Files are readable and diffable. Size is larger than binary, which is acceptable for single patterns.

## Alternatives considered

CBOR or another binary format was rejected for readability. A directory package is deferred to R2 assets.

## Revisit when

Assets or size force a container.
