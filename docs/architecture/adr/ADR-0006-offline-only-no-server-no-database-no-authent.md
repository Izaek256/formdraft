# ADR-0006: Offline-only: no server, no database, no authentication

- Status: Accepted
- Date: 2026-10-09
- Source: FR-037, NFR-018, NFR-024
- Requirements: FR-035; FR-037; NFR-018; NFR-024; BR-15

## Context

The SRS requires offline operation and no data leaving the device without explicit action.

## Decision

The product has no server and no database. The project file is the persistence unit. There are no accounts, so authentication and authorization do not apply. Trust boundaries and privacy controls replace them (see 03-dataflow-failure-security-deployment.md).

## Consequences

Cloud collaboration (R3) needs a new ADR and a threat model first. Backups are the user's responsibility unless a later decision says otherwise.

## Alternatives considered

A local database was rejected: a human-readable versioned file meets FR-035 with less risk.

## Revisit when

Any feature needs the network.
