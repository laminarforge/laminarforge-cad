# LAMP Rev B Controller PCBA

Source-package ticket: `T-4C206871`<br>
Manufacturer-handoff ticket: `T-E36FA2C2`<br>
Electrical release evidence: `A-9F7C4851`

This directory is the deterministic source for the Rev B controller carrier. The promoted copper/electrical design is fabrication-clean. Manufacturer-order readiness is a separate fail-closed state owned by `lamp_rev_b_controller_fab_release`.

## Routing and release policy

All routing and manufacturer-release work is governed by the mandatory [LaminarForge PCBA Routing and Release Standard](../../docs/pcba_routing_and_release_standard.md). The board-specific fixture and evidence are:

- [`freerouting_benchmark.toml`](freerouting_benchmark.toml): pinned router/tool settings and the 1,200-second runtime cap;
- [`freerouting_benchmark.md`](freerouting_benchmark.md): harness operation and determinism limitations;
- [`benchmarks/freerouting_v2_1_0_pass_sweep.md`](benchmarks/freerouting_v2_1_0_pass_sweep.md) and its [JSON record](benchmarks/freerouting_v2_1_0_pass_sweep.json): exact 1→10→50 matrix evidence;
- [`benchmarks/freerouting_v2_1_0_promotion.md`](benchmarks/freerouting_v2_1_0_promotion.md): first-clean promotion-attempt evidence; and
- [`electrical_release_evidence.md`](electrical_release_evidence.md), [`fab_release.toml`](fab_release.toml), and [`release_manifest.toml`](release_manifest.toml): electrical, fabrication, and deterministic-package gates.

These files specialize the general policy for Rev B; they do not replace it. Routing and SES validation remain scratch-only until every promotion gate passes.

## Locked first-article population

- Variant: `rev_b_12v_first_article`; 5 assembled top-side PCBAs.
- `R25` and `R26`: mandatory DNP. Populating either bypasses its heater MOSFET and can energize the heater whenever `VIN_HEATER` is present.
- `R55`: populate only for this 12 V variant; DNP for 24 V unless an approved regulated 12 V VDRV path is present.
- `J24`: connect through an external normally-closed thermal cutoff or a correctly rated jumper before heater tests. Do not bridge `VIN_PROTECTED` to `VIN_HEATER` in copper.
- `LDD-1000H` and `P7812-500R` alternates: DNP for this release.
- Panelization: fabricator-owned from the released single-up board; portal preview approval is mandatory.

## Authoritative workflow

Run the repository contract and release command through the LaminarForge build tool:

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_check"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_fab_release"}
```

The release command requires a clean full commit equal to `origin/main` and KiCad `10.0.3`. It writes every KiCad report/export to two ignored temporary trees, normalizes known time metadata, compares every staged byte, and promotes only identical results.

When every external choice is controlled, it emits:

- nine fabrication Gerbers plus the Gerber job file;
- separate F/B paste Gerbers;
- separate PTH/NPTH Excellon drills, both drill maps, and drill report;
- fabrication and top/bottom assembly PDFs;
- formal BOM, CPL, DNP, and no-substitution CSVs;
- schematic PDF, top/bottom renders, board-only mechanical STEP, source snapshot, notes, and electrical evidence;
- `MANIFEST.json`, `SHA256SUMS`, upload units, and a normalized outer handoff ZIP.

If procurement, stackup/order fields, or portal approval remain unresolved, it still preserves the byte-reproducible review tree and exact blocker report, exits nonzero, and emits neither `upload/` nor the outer handoff ZIP.

## Current external blockers

Repository evidence does not define a selected manufacturer/order profile, physical dielectric stackup, fabrication tolerances/class, final finish specification, colors, or an impedance-policy decision. `parts.toml` also lacks explicit manufacturer/MPN fields and contains unresolved supplier prose/manual selections. No portal preview has been approved. These are manufacturing choices, not electrical design defects; do not invent them and do not treat the provisional artifact `A-D87AC12B` as vendor-cited research.

Any source, population, placement, schematic, stackup, or copper change invalidates prior evidence and requires fresh DRC/connectivity/ERC/BOM/CPL/reproducibility gates.
