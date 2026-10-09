# ADR-0001: Rust-first single-language product

- Status: Accepted
- Date: 2026-10-09
- Source: SRS 9 language policy
- Requirements: NFR-022; FR-040

## Context

The SRS requires one language for desktop, web and pattern mathematics and limits other languages to narrow roles.

## Decision

Rust for all product code, desktop and WASM. WGSL only for the custom shader. TOML for Cargo, JSON for projects, SVG as an output format, minimal HTML and CSS for the Trunk host. No Python, JavaScript or C++ runtime in the product. Python or TypeScript scripts are allowed only under scripts/ for independent verification and are never a production dependency.

## Consequences

PDF, DXF and image libraries must work in Rust and, where the browser export is claimed, on WASM. Verification scripts such as the G0 readiness checker may be written in Python.

## Alternatives considered

A TypeScript web front end over a Rust core was rejected by the SRS.

## Revisit when

A required capability exists only outside Rust.
