# Data flow, failure handling, security and deployment

## 1. Data flow

### Generate

1. The user enters a value in any supported unit. The unit service parses it to an exact Length.
2. Measurement validation checks the four values and cross-field rules against thresholds from the rulebook. Failures stop here with the exact field named.
3. If waist-to-hip is chart-derived, the chart lookup supplies it, tags it and raises a warning.
4. The template reads constants only through rule records, builds the net back and front pieces, and applies the adaptation if selected.
5. The offset stage builds each cut path from its stitch path using per-edge allowance.
6. The marks stage adds grainline, notches, labels and the style number.
7. The audit stage collects warnings and blockers and records provenance.
8. The result is a GeneratedPattern with a checksum. The UI shows geometry, warnings and provenance.

### Save and open

Save: project -> codec -> bytes -> atomic sink. Open: bytes -> version and identity checks -> migrate if needed -> project -> regenerate -> compare pattern_checksum -> show drift warning if different.

### Export

pattern -> gate_check -> (ExportGate) -> exporter -> bytes -> sink. If gate_check fails, nothing is written.

### Render

The same GeneratedPattern is tessellated for display. The GPU layer adds view effects. Nothing flows back from the GPU to geometry.

## 2. Failure handling

| Failure | Detection | Behaviour | User sees | Test |
|---|---|---|---|---|
| Mandatory measurement missing | Validation | Generation disabled | Field named | AT-03 |
| Value implausible in its unit | Validation | Warning, not silent guess | Warning with unit | FR-036 tests |
| Rule not Verified | Gate (Production) | Export refused | Which rule and why | AT-19 |
| Self-intersecting offset, severe | Offset detect | Export blocked, nothing written | Where and what to change | AT-09 |
| Self-intersection, minor | Offset detect | Warning stays visible | Warning | T-FR-022 |
| Unknown schema, method, edition | Load | Typed error, file untouched | Actionable message | AT-10 |
| Rule hash mismatch on load | Load | Drift warning, no silent regenerate | Warning | T-FR-035 |
| Crash or disk error during save | Atomic protocol | Previous file remains valid | Save failed message | NFR-021 |
| Library failure in PDF or DXF | Exporter | Typed error, nothing partial written | Export failed message | S13 tests |
| GPU feature missing | Render init | Fallback to CPU drawing | Notice | SP-03 |
| Texture too large (R2) | Image decode | Rejected before upload | Message | FR-030 test |
| Malformed image (R2) | Decoder | Rejected, no panic | Message | FR-030 test |
| Panic in a non-UI crate | None allowed | Treated as a defect | n/a | clippy and fuzz tests |

Principle: fail closed on export, fail open on editing. A bad pattern never leaves the app. A bad input never loses the user's other work.

## 3. Security and privacy

Authentication and authorization do not apply. There are no accounts, no server and one local user. This is deliberate (ADR-0006). Adding any network feature, including the R3 cloud collaboration idea, requires a new ADR and a threat model first.

### Assets

Customer measurements (personal data about real people), project files, exports, the rulebook (integrity matters because it drives output).

### Trust boundaries

| Boundary | Untrusted input | Control |
|---|---|---|
| Opening a project file | File from any source | Parse as data only. Size limit. Schema and identity checks. No evaluation. Fuzzed loader |
| Loading an image (R2) | User-chosen file | Permitted decoders only. Bounds and dimension limits. Reject on failure |
| Typed input | Any text | Unit service rejects malformed text with typed errors |
| Dependencies | Third-party code | Policy in AGENTS.md. cargo deny. Pinned versions. WASM feature gating |
| Browser host | Hosting page | Static files only. No third-party scripts. Content security policy to be set at S16 |

### Privacy controls

1. No network I/O in any core crate. A static check blocks network crates in pattern_core, pattern_templates, pattern_document and pattern_export.
2. No telemetry or analytics. No crash reporter that sends data.
3. Logs never contain measurement values or customer labels.
4. Delete and export controls exist for every profile (FR-037). Embedded copies in saved project files are the user's files. Deletion semantics for those are OQ-04.
5. Anonymisation meaning is OQ-22. Nothing is built until it is answered.

### Integrity controls

Rule status gate in Production mode, rule hashes in project files, data checksums with version bump, completeness test (NFR-026, NFR-027).

## 4. Deployment topology

```
 Developer machine --push--> CI host --builds--> native artifacts (Windows, Linux; others per OQ-24)
                                      \-builds--> web artifact (Trunk WASM, size gate 25 MiB)
                                                      |
                                                      v
                                             static file host (OQ-08)
```

| Item | Decision | Status |
|---|---|---|
| Native targets | Windows and Linux first | Proposed. macOS and others per OQ-24 |
| Web build | Trunk and wasm-release profile, static hosting only | Accepted from SRS 9 |
| Size gate | CI fails if the hosted WASM artifact exceeds 25 MiB. The whole deployment payload is monitored separately | Accepted from SRS 9 |
| CI stages | fmt, clippy, test, deny, layering check, WASM check, size gate, offline test | Proposed |
| CI host and distribution | Not decided | OQ-34 |
| Servers, databases, queues | None | ADR-0006 |
| Environments | Local, CI, and two long-lived branches: develop is staging and testing, main is production. No staging server because there is no server | Accepted. See docs/process/branching-and-releases.md |
| Release artifacts | Signed or checksummed binaries and the web bundle, with the evidence pack | Proposed |
| Reference hardware and printer | Recorded before NFR-017 and NFR-015 are claimed | OQ-23, OQ-05 |
