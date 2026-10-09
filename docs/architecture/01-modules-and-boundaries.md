# Modules and boundaries

## System context

```
 Tailor / pattern maker
          |
          v
 +----------------------+        local files only
 |  Native app (egui)   |<------------------------> project files, exports, settings
 |  or Web app (WASM)   |        browser download / file picker on the web
 +----------------------+
          no network, no server, no database
```

External systems: none at runtime. At build time: crates.io and the CI host. Outside the software: a printer, CAD viewers (R2), and a pattern maker who verifies rules.

## Principles

1. The core is pure. Same inputs and same rulebook always give the same geometry (INV-01).
2. Dependencies point downward only (ADR-0004).
3. The CPU computes every pattern dimension. The GPU only draws (ADR-0005).
4. Every length passes through the unit service (ADR-0007).
5. Every drafting constant has a rule record (ADR-0003, ADR-0009).
6. Nothing leaves the device unless the user acts (ADR-0006).

## Modules

| Module | Responsibility | Owns | Must not |
|---|---|---|---|
| pattern_core | Unit service, exact decimal length, geometry (points, paths, Béziers, tessellation, offset), measurement profile and validation, rule record types and gate, warning model, export gate | Types and algorithms. No stored data | Depend on egui, wgpu, the filesystem, the clock or randomness |
| pattern_templates | The Aldrich 4th edition tailored skirt block, straight skirt adaptation, rulebook data, size charts, thresholds, grading tables (R2) | Rulebook and chart data (read only at runtime) | Do I/O. Contain a literal drafting constant in Rust source |
| pattern_document | Project schema, versioning, migration, load and save, atomic write logic, undo and redo | The project file format | Contain UI types. Execute anything found in a file |
| pattern_export | SVG, PDF and (R2) DXF writers, tiling, calibration, legends, print checks | Export file formats | Depend on pattern_render or pattern_ui. Write without passing the export gate |
| pattern_material | Fabric records, repeats, layout heuristics (R2) | Fabric data | Claim physical behaviour. Change body measurements |
| pattern_ui | egui application, forms, canvas, settings, keyboard and accessibility | App settings, unit preferences (global scope) | Use unwrap, expect or panic. Format a length itself. Compute a pattern dimension |
| pattern_render | wgpu paint callback and WGSL shader, texture sampling | Nothing persistent | Change geometry. Be used by exporters |
| pattern_web | Trunk host and the wasm-release build | The hosted artifact | Add logic. Add network calls |
| tools/pattern-cli | Headless generate and export for tests and CI (ADR-0012, Proposed) | Nothing | Ship to users |

## Allowed dependencies

A row may depend on a column marked yes. Everything else is forbidden. scripts/check-layering.sh reads cargo metadata and fails on a violation.

| depends on > | core | templates | document | export | material | render | ui |
|---|---|---|---|---|---|---|---|
| pattern_core | - | no | no | no | no | no | no |
| pattern_templates | yes | - | no | no | no | no | no |
| pattern_document | yes | yes | - | no | no | no | no |
| pattern_export | yes | no | no | - | no | no | no |
| pattern_material | yes | no | no | no | - | no | no |
| pattern_render | yes | no | no | no | no | - | no |
| pattern_ui | yes | yes | yes | yes | yes | yes | - |
| pattern_web | yes | yes | yes | yes | yes | yes | yes |
| tools/pattern-cli | yes | yes | yes | yes | no | no | no |

## Responsibilities that are easy to misplace

| Concern | Lives in | Reason |
|---|---|---|
| Warnings and the export gate | pattern_core | Every exporter and the UI share one definition. Exporters take the gate as a required argument |
| Measurement validation limits | pattern_templates data, enforced by pattern_core | Limits belong to the method, the logic is generic |
| Display unit preferences | Global in pattern_ui settings, project scope in pattern_document | Global scope is an app concern. Project scope travels with the file |
| Toile statement text | pattern_export, parameters from the project | Must appear in every output and cannot be hidden |
| Provenance queries | pattern_core | The UI and exports both need them |

## Data ownership (there is no database)

| Data | Defined in | Persisted by | Loaded by | Mutable at runtime |
|---|---|---|---|---|
| Measurement profile | pattern_core | pattern_document, inside the project file | pattern_document | Yes, by the user |
| Style settings, adaptation, allowances | pattern_core | pattern_document | pattern_document | Yes |
| Rulebook (rule records and constants) | pattern_templates | Compiled into the binary from data files | pattern_templates | No |
| Size charts and thresholds | pattern_templates | Compiled into the binary | pattern_templates | No |
| Project file | pattern_document | pattern_document through a store sink | pattern_document | By save only |
| Unit preference, global | pattern_ui | Platform settings file or browser storage | pattern_ui | Yes |
| Export artifacts | pattern_export produces bytes | The app layer writes them via a sink | n/a | No |
| Fabric records (R2) | pattern_material | pattern_document | pattern_document | Yes |
| Logs | pattern_ui | Local file, optional | n/a | Must never contain measurements (see security) |

Rule: a crate that does not own a data set does not parse or write it.
