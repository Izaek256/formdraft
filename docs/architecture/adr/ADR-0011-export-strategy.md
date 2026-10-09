# ADR-0011: Export strategy

- Status: Proposed pending SP-01
- Date: 2026-10-09
- Source: FR-032, FR-033, FR-034, NFR-019
- Requirements: FR-032; FR-033; FR-034; NFR-019

## Context

PDF and DXF library support on WASM is a named risk in the SRS.

## Decision

SVG uses an own deterministic writer. PDF and DXF approaches are decided after SP-01: use a maintained crate if it works on native and WASM within the size cap, otherwise write a minimal writer for the subset needed. If browser export cannot run, narrow the supported formats and disclose it (SRS 13).

## Consequences

S12 can proceed before SP-01. S13 and S21 wait for it.

## Alternatives considered

Delegating export to an external service was rejected: offline requirement.

## Revisit when

SP-01 finishes.
