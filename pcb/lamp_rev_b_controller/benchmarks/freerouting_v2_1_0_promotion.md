# Rev B controller Freerouting promotion

Ticket: `T-352986EA`

Base commit: `ba91540f71f67298c9d914408f8d560ce6c69c26`

## Promoted result

The first routed attempt from the fresh canonical input passed the promotion gates and was promoted to `pcb/lamp_rev_b_controller/lamp_rev_b_controller.kicad_pcb`. Attempts 2 and 3 were skipped under the first-clean stopping rule. A default-`java` preflight failed before routing because the macOS shim could not locate a runtime; the routed attempt used the installed OpenJDK 26 executable and the committed benchmark settings unchanged.

| Metric | Result |
| --- | ---: |
| Pass budget / observed passes | `10 / 10` (`exact_budget`) |
| Freerouting runtime | `14.318 s` |
| Freerouting routed / remaining | `42 / 1` |
| SES nets / wires / segments / vias | `76 / 488 / 982 / 170` |
| Physical DRC violations | `0` |
| DRC errors / warnings | `0 / 0` |
| Raw / ignored self-zone / real unconnected | `3 / 3 / 0` |

- Input board SHA-256: `2e2722f4e5b4acc72965f36f6fecc1a5f201de26a8b326ed44ce2b1f0d21d884`
- Fresh DSN SHA-256: `63f45876adb1d40e7fe629fd10340aca65b6791570441f99f2b40c20ed02449e`
- SES SHA-256: `190abb4ace95b46c654cc525709bb5ea746f18adc3c50c7235792416c9a9ac53`
- Promoted board SHA-256: `09800341d4b4c438354efa6a6cbc8280e5b86784c5fb4409be15c401effeb263`
- Routing seed SHA-256 (unchanged): `ce2717b38bb65e6556810f99e4e6e6def6801cd75e8b67a1e03f51a00c98bd06`
- Placement SHA-256 (unchanged): `9b8322d20315e5b93c3c6c8022be3f381e7684f7fb9f531cc0459c52c03b5f1e`
- Full effective settings and attempt metrics: `freerouting_v2_1_0_promotion/attempt_1/result.json`

## Promotion validation

- SES review accepted all `38` touched open-net candidates with no unknown nets or layers: `laminarforge-pcb-ses-import-review-20260710T032357Z-ef48bea450644b48b713e1725ae7d3fe`.
- MCP scratch validation: physical DRC `0`, raw unconnected `3`, identical self-zone `3`, real unconnected `0`: `laminarforge_pcb_scratch_validate-0f6a6206aa1e4aafaa568c3da4dee370`.
- Scratch diff preserved the `155` footprints, `80` board net entries (`79` named nets), `22` layers, `4` zones, and the `118 x 94 mm` outline; routed copper changed from `601 / 113` segments/vias to `982 / 170`: `laminarforge_pcb_scratch_diff-c704a17849804054b3d7fd8334cd2fbc`.
- Exact normalized component, placement, pad, and pad-net comparison matched across all `155` components and `409` pads. Normalized contract class, netclass, route-family, width, test-point, and rule coverage intent matched across all `79` named nets.
- Final promoted KiCad report runner after whitespace normalization: physical DRC `0`, raw unconnected `3`, real unconnected `0`: `laminarforge_pcb_kicad_report_runner-78e0762c74b84bbc8687dbe7cf8046dd`.
- Promoted constraint audit found no conflicts or zone-rule mismatches: `pcb_constraints_1574c261d484419aae720111667d0a36`.
- `lamp_rev_b_controller_check` passed with fab-release blocking gaps `0`.
- `lamp_rev_b_controller_route_report` reported DRC violations `0` and real unconnected items `0`.
- `mcp__agentic-mcp__laminarforge_build` completed successfully.

Freerouting 2.1.0 remains unseeded, so the SES is evidence of one successful fresh attempt, not a bit-reproducibility guarantee. No placement, constraints, source generator, or Freerouting source was changed.
