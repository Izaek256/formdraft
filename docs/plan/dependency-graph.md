# Dependency graph and build order

```mermaid
flowchart TD
  DOM01(["DOM-01 Domain baseline (gate G0)"])
  SP01{{"SP-01 Spike: PDF and DXF libraries on native and WASM"}}
  SP02{{"SP-02 Spike: polygon and curve offset approach"}}
  SP03{{"SP-03 Spike: egui_wgpu paint callback and texture limits"}}
  S00["S00 Workspace and CI skeleton"]
  S01["S01 Unit service"]
  S02["S02 Geometry primitives"]
  S03["S03 Measurement profile and validation"]
  SK01["SK-01 Workspace skeleton (professional layout, placeholder shape)"]
  S04["S04 Rule records and rulebook loader"]
  S05["S05 Size charts and chart lookup"]
  S06["S06 Tailored skirt block with darts"]
  S07["S07 Straight skirt adaptation"]
  S08["S08 Seam allowance and offset"]
  S09["S09 Construction marks"]
  S10["S10 Audit, warnings and export gate"]
  S11["S11 Project document"]
  S12["S12 SVG export"]
  S13["S13 Tiled PDF export and toile statement"]
  S14["S14 Application UI (container: SK-01 plus S14c to S14f)"]
  S14c["S14c Properties, warnings and provenance panels"]
  S14d["S14d Pieces panel and layer visibility"]
  S14e["S14e File and export flows"]
  S14f["S14f Keyboard and accessibility pass"]
  S15["S15 GPU render layer"]
  S16["S16 Web build"]
  S17["S17 Offline and privacy harness"]
  S18["S18 R1 acceptance and evidence pack"]
  S20["S20 Size grading"]
  S21["S21 DXF export"]
  S22["S22 Manual drafting tools and overrides"]
  S23["S23 Pattern comparison"]
  S24["S24 Material library and preview"]
  S25["S25 Fabric layout and consumption"]
  DOM02(["DOM-02 Domain baseline for R2 (grading tables)"])
  SP04{{"SP-04 Spike: dockable panels with the pinned egui version"}}
  S00 --> SP01
  S02 --> SP02
  S14 --> SP03
  S00 --> S01
  S00 --> S02
  S01 --> S02
  S01 --> S03
  S01 --> SK01
  S02 --> SK01
  S03 --> SK01
  S00 --> S04
  S01 --> S04
  S01 --> S05
  S04 --> S05
  DOM01 --> S05
  S02 --> S06
  S03 --> S06
  S04 --> S06
  S05 --> S06
  DOM01 --> S06
  S06 --> S07
  S02 --> S08
  SP02 --> S08
  S06 --> S08
  S06 --> S09
  S08 --> S09
  S03 --> S10
  S04 --> S10
  S08 --> S10
  S03 --> S11
  S04 --> S11
  S06 --> S11
  S09 --> S12
  S10 --> S12
  SP01 --> S13
  S09 --> S13
  S10 --> S13
  S12 --> S13
  SK01 --> S14
  S14c --> S14
  S14d --> S14
  S14e --> S14
  S14f --> S14
  S03 --> S14c
  S04 --> S14c
  S10 --> S14c
  SK01 --> S14c
  S06 --> S14d
  S08 --> S14d
  S09 --> S14d
  SK01 --> S14d
  S11 --> S14e
  S12 --> S14e
  S13 --> S14e
  SK01 --> S14e
  SK01 --> S14f
  S14c --> S14f
  S14d --> S14f
  S14e --> S14f
  S14 --> S15
  SP03 --> S15
  S14 --> S16
  S12 --> S16
  S13 --> S16
  S11 --> S17
  S13 --> S17
  S14 --> S17
  S16 --> S18
  S17 --> S18
  S15 --> S18
  S06 --> S20
  DOM02 --> S20
  SP01 --> S21
  S12 --> S21
  S14 --> S22
  S11 --> S22
  S14 --> S23
  S15 --> S24
  S11 --> S24
  S24 --> S25
  DOM01 --> DOM02
  SK01 --> SP04
```

Hexagons are spikes. Rounded boxes are domain work done by people, not code.

## One valid build order

| Step | Slice | Release | Kind | Name |
|---|---|---|---|---|
| 1 | DOM-01 | R1 | domain | Domain baseline (gate G0) |
| 2 | S00 | R1 | code | Workspace and CI skeleton |
| 3 | SP-01 | R1 | spike | Spike: PDF and DXF libraries on native and WASM |
| 4 | S01 | R1 | code | Unit service |
| 5 | S02 | R1 | code | Geometry primitives |
| 6 | SP-02 | R1 | spike | Spike: polygon and curve offset approach |
| 7 | S03 | R1 | code | Measurement profile and validation |
| 8 | SK-01 | R1 | code | Workspace skeleton (professional layout, placeholder shape) |
| 9 | S04 | R1 | code | Rule records and rulebook loader |
| 10 | S05 | R1 | code | Size charts and chart lookup |
| 11 | S06 | R1 | code | Tailored skirt block with darts |
| 12 | S08 | R1 | code | Seam allowance and offset |
| 13 | S10 | R1 | code | Audit, warnings and export gate |
| 14 | S14c | R1 | code | Properties, warnings and provenance panels |
| 15 | S09 | R1 | code | Construction marks |
| 16 | S14d | R1 | code | Pieces panel and layer visibility |
| 17 | S11 | R1 | code | Project document |
| 18 | S12 | R1 | code | SVG export |
| 19 | S13 | R1 | code | Tiled PDF export and toile statement |
| 20 | S14e | R1 | code | File and export flows |
| 21 | S14f | R1 | code | Keyboard and accessibility pass |
| 22 | S14 | R1 | code | Application UI (container: SK-01 plus S14c to S14f) |
| 23 | SP-03 | R1 | spike | Spike: egui_wgpu paint callback and texture limits |
| 24 | S07 | R1 | code | Straight skirt adaptation |
| 25 | S15 | R1 | code | GPU render layer |
| 26 | S16 | R1 | code | Web build |
| 27 | S17 | R1 | code | Offline and privacy harness |
| 28 | S18 | R1 | code | R1 acceptance and evidence pack |
| 29 | DOM-02 | R2 | domain | Domain baseline for R2 (grading tables) |
| 30 | S20 | R2 | code | Size grading |
| 31 | S21 | R2 | code | DXF export |
| 32 | S22 | R2 | code | Manual drafting tools and overrides |
| 33 | S23 | R2 | code | Pattern comparison |
| 34 | S24 | R2 | code | Material library and preview |
| 35 | S25 | R2 | code | Fabric layout and consumption |
| 36 | SP-04 | R2 | spike | Spike: dockable panels with the pinned egui version |

## Parallel tracks

- DOM-01 (people) runs in parallel with S00 to S04. It must finish before S05, S06 and S07 can be completed.
- S01 and S02 can proceed in parallel after S00. SP-01 and SP-02 can run as soon as their dependencies finish.
- S03 and S04 can proceed in parallel after S01.

## Critical path

DOM-01 then S05, S06, S07, S08 (needs SP-02), S09, S10, S12, S13 (needs SP-01), S14, S15, S16, S17, S18. The longest human-dependent step is DOM-01, because it needs a named pattern maker (OQ-10).
