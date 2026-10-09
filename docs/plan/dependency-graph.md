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
  S14["S14 Application shell and UI"]
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
  S00 --> SP01
  S02 --> SP02
  S14 --> SP03
  S00 --> S01
  S00 --> S02
  S01 --> S02
  S01 --> S03
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
  S01 --> S14
  S03 --> S14
  S06 --> S14
  S08 --> S14
  S09 --> S14
  S10 --> S14
  S11 --> S14
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
| 8 | S04 | R1 | code | Rule records and rulebook loader |
| 9 | S05 | R1 | code | Size charts and chart lookup |
| 10 | S06 | R1 | code | Tailored skirt block with darts |
| 11 | S08 | R1 | code | Seam allowance and offset |
| 12 | S09 | R1 | code | Construction marks |
| 13 | S10 | R1 | code | Audit, warnings and export gate |
| 14 | S11 | R1 | code | Project document |
| 15 | S14 | R1 | code | Application shell and UI |
| 16 | SP-03 | R1 | spike | Spike: egui_wgpu paint callback and texture limits |
| 17 | S07 | R1 | code | Straight skirt adaptation |
| 18 | S12 | R1 | code | SVG export |
| 19 | S13 | R1 | code | Tiled PDF export and toile statement |
| 20 | S15 | R1 | code | GPU render layer |
| 21 | S16 | R1 | code | Web build |
| 22 | S17 | R1 | code | Offline and privacy harness |
| 23 | S18 | R1 | code | R1 acceptance and evidence pack |
| 24 | DOM-02 | R2 | domain | Domain baseline for R2 (grading tables) |
| 25 | S20 | R2 | code | Size grading |
| 26 | S21 | R2 | code | DXF export |
| 27 | S22 | R2 | code | Manual drafting tools and overrides |
| 28 | S23 | R2 | code | Pattern comparison |
| 29 | S24 | R2 | code | Material library and preview |
| 30 | S25 | R2 | code | Fabric layout and consumption |

## Parallel tracks

- DOM-01 (people) runs in parallel with S00 to S04. It must finish before S05, S06 and S07 can be completed.
- S01 and S02 can proceed in parallel after S00. SP-01 and SP-02 can run as soon as their dependencies finish.
- S03 and S04 can proceed in parallel after S01.

## Critical path

DOM-01 then S05, S06, S07, S08 (needs SP-02), S09, S10, S12, S13 (needs SP-01), S14, S15, S16, S17, S18. The longest human-dependent step is DOM-01, because it needs a named pattern maker (OQ-10).
