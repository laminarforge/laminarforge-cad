# Dry P0 Cartridge Coupon CAD Suite

Ticket: `T-A2021311`  
Revision: `P0-R0`  
Authoritative boundary: `A-A1A77D11`, `A-696CE730`, `A-CE59D39F`

## Scope

This suite turns the cartridge architecture and engineering verification matrix into deterministic parametric Rust/vcad geometry. It produces empty, nonsterile P0 engineering coupons plus one reusable alignment nest. It does not define or execute wet-lab chemistry, select a swab or material grade, authorize purchasing, change reader electronics, or establish clinical, manufacturing, biocompatibility, containment, sterility, transport, or shelf-life validation.

Every dimension is a proposed CAD envelope for comparison and fixture planning. Values are centralized in `src/p0_cartridge_coupons.rs` and may be revised from measured interface evidence. They are not validated tolerances. Adhesive edges are never treated as datums.

## Build and verify

Use the repository build service:

```text
mcp__agentic_mcp.laminarforge_build action=run repo=laminarforge-cad bin=p0_cartridge_coupon_suite
mcp__agentic_mcp.laminarforge_build action=run repo=laminarforge-cad bin=p0_cartridge_coupon_verify
```

The generator defaults to all three material-stack configurations. For local geometry development, the binary also accepts `--stack coc_cop_target`, `--stack pmma_control`, or `--stack pet_comparator`. Unknown stack names fail immediately.

Generated files live in `output/p0_cartridge_coupons/`:

- 33 stack-specific coupon STL files: 11 families × 3 stack configurations;
- `shared_3_2_1_alignment_nest.stl`, a reusable dry fixture;
- `manifest.json`, containing source artifacts, scope notices, parameters, output byte counts, and SHA-256 hashes.

The `output/` directory is intentionally generated and ignored by git. The tracked source of truth is the Rust geometry, this document, the static manifest at `manifests/p0_cartridge_coupon_suite.toml`, and the tests. The verifier fails on missing files, duplicate paths, changed bytes, bad hashes, incomplete family/stack coverage, or boundary metadata drift.

## Shared interface contract

Every disposable coupon uses the same 86 × 54 mm proposed frame and carries:

- a primary round registration hole and a same-width elongated relief slot;
- an asymmetric corner notch that prevents a 180-degree or face-flipped assembly from appearing correctly keyed;
- three cross fiducials outside the protected fluidic/optical areas;
- a font-independent geometric revision mark, family-ID ladder, and material-stack ladder that survive STL export;
- a common origin and layer order: base, channel/spacer, then cover/window.

The reusable alignment nest implements the 3-2-1 constraint system explicitly:

1. three non-collinear support pads establish the primary plane;
2. two separated edge contacts establish the secondary direction;
3. one end stop establishes the final translation without overconstraining the opposite edge.

Round and slot pins provide layer registration while allowing the slot direction to absorb material-dimensional change. A keyed guard mirrors the asymmetric coupon notch. These are deterministic geometric interfaces, not a promise of fabrication capability.

## Material-stack configurations

| Stack | Proposed base / spacer / cover | Role | Status |
| --- | ---: | --- | --- |
| `coc_cop_target` | 1.00 / 0.14 / 0.20 mm | COC/COP-centered target-faithful comparator | Material family only; exact grade, surface treatment, adhesive, and bond process unresolved |
| `pmma_control` | 1.50 / 0.14 / 0.20 mm | Rapid-fabrication PMMA control | Engineering control only; no optical or contact compatibility implied |
| `pet_comparator` | 0.25 / 0.14 / 0.125 mm | All-film PET process comparator | Conditional; do not advance without traceable optical/diagnostic-grade evidence |

The shared 0.14 mm spacer makes each 20 × 8 mm chamber a nominal 22.4 µL CAD void, inside the proposed 20–25 µL architecture envelope. That arithmetic checks geometry only. It is not a reaction-volume prescription, and actual volume depends on fabricated thickness, bond intrusion, bow, and fill state.

No vendor SKU appears in the CAD or manifest. Material and adhesive lots remain external evidence fields for future coupon builds.

## Coupon families

| Family | Matrix mapping | Geometry exercised |
| --- | --- | --- |
| `material_contact` | M-02 | Separate round component-contact wells and larger complete-stack exposure wells, allowing future direct/extract contact comparisons to remain localized |
| `optical_window` | M-01 / O-01 | Two chamber-sized optical ROIs, adhesive-edge witness, fiducials, and deliberate bubble/wrinkle positive-failure geometry |
| `thermal_evaporation` | T-01 | Paired 22.4 µL nominal chamber voids, fill bus, window span, heater-registration fiducials, and bubble witnesses |
| `bond_registration` | B-01 / R-01 | Straight path, 90-degree corner, T-junction, narrow land, edge route, and raised registration-offset ladder |
| `metering_debris` | F-01 / F-02 | Inlet, settling pocket, turn/weir, barrier-insert window, nominal 2.5 µL meter envelope, overflow-to-waste path, and outlet |
| `vent_waste` | F-03 | Captive headspace, shortest liquid route, two splash baffles, vent membrane bond land, and empty-versus-absorbent comparator bay |
| `seal_backflow` | F-04 | Pressure inlet, narrow controlled-burst throat, collection chamber, reverse-flow labyrinth, seal-land transitions, and reverse-pressure port |
| `dual_lane_isolation` | F-05 | Two physically separate inlets, meters, chamber voids, terminal waste areas, and vent routes with a continuous inter-lane land |
| `swab_dock_retention` | H-01 | Dry shell and shaft-bore envelope, hard insertion stop, two latch witnesses, tip pocket, and drainage proxy |
| `conditional_blister` | H-02 | At most one blister seal-land envelope, external actuator target, captive puncture guard, outlet, and anti-rebound terminal; foil stays outside optical ROIs |
| `sealed_containment` | C-01 | Representative closed fluidic perimeter, fill closure, terminal vent capture, weak-corner perimeter, external witness moat, and tamper bridge |

The material-contact family is included because the authoritative verification matrix separates contact effects from optical background. The bond/registration family combines two tightly coupled fabrication risks, while metering/debris combines the downstream sequence that must be evaluated without hiding debris-driven meter bias.

## Parametric design boundaries

The common proposed values in revision P0-R0 are:

| Parameter | Value | Interpretation |
| --- | ---: | --- |
| Frame | 86 × 54 mm | Common fixture and handling envelope |
| Registration hole / slot width | 3.2 / 3.2 mm | Same nominal width avoids two competing lateral constraints |
| Slot length | 8.0 mm | Relief direction for material dimensional change |
| Key notch | 8 × 7 mm | Asymmetric orientation feature |
| Channel width | 1.2 mm | Visible P0 fluidic surrogate feature |
| Minimum represented seal land | 3.0 mm | CAD comparison variable, not a released minimum |
| Chamber planform | 20 × 8 mm | 22.4 µL only at the proposed 0.14 mm spacer |
| Meter | 2.5 µL nominal | Mid-envelope CAD pocket; the 0.5–5 µL range remains the architecture boundary |

Changing spacer thickness automatically changes calculated chamber volume and is guarded by tests. A configuration that moves the paired chamber outside 20–25 µL or the meter outside 0.5–5 µL fails immediately. Dimensional capability, actual bond squeeze-in, optical ROI, heater footprint, vent grade, and pressure allocation still require measured evidence before any drawing release.

## Reusable versus disposable boundary

The alignment nest, future platen/roller, optical backer, heater registration holder, leak/pressure fixture, dock force fixture, vent placement nest, and external blister actuator are reusable dry equipment. The generated `shared_3_2_1_alignment_nest.stl` is the only reusable part emitted by this suite.

Every stack-specific coupon represents disposable geometry. Any later base, spacer/adhesive, window, dock liner/gasket, barrier, vent membrane, absorbent, blister, or terminal seal that becomes wetted remains disposable. The reader and reusable nest must stay dry.

## Inspection and evidence handoff

Before using a generated coupon revision, retain the static manifest, runtime hash manifest, selected stack slug, source commit, slicer/conversion settings if any, and an image of the geometric ID ladders. For fabricated dry articles, record measured layer thickness, round-hole and slot geometry, fiducial offsets, key orientation, seal-land intrusion, and visible defects at unit level.

The generated shapes expose interfaces; they do not encode pass/fail evidence. Any later engineering screen should use the acceptance framework from A-696CE730 and must keep proposed engineering gates distinct from validated assay or manufacturing requirements. Passing dry geometry inspection authorizes only the next engineering comparison, never diagnostic use or production release.

## Explicit exclusions

- No pathogen, clinical specimen, amplicon, primer, enzyme, buffer composition, lysis condition, LoD, or diagnostic threshold is defined.
- No material, adhesive, swab, vent membrane, absorbent, blister laminate, or converter is selected.
- No vendor is contacted and no purchase is authorized.
- No PCB, firmware, reader actuator bank, or Rev C electronics are modified or specified.
- No sterility, biocompatibility, aerosol containment, transport, shelf-life, clinical, manufacturing-capability, or reliability claim is made.
- The PET stack and blister family remain conditional comparators and may be deleted after evidence review.
