# 16-Slot Cassette A0 Interface Control Spec

This is the shared interface contract for the first physical LaminarForge AAV cassette build. Downstream carrier, lid/clamp, gasket, DFM, fluid-path, validation-fixture, and build-package agents must design against this spec unless the cassette integrator updates it.

This is not a production drawing, biological protocol, sterile-barrier claim, or live-cell release criterion. It is the geometry and interface baseline for the first article.

## Locked Decisions

- First build format: 16 slots in a 4 x 4 grid.
- Experimental unit: one cassette equals one AAV capsid/promoter/payload/dose/timing/media condition.
- Slot role: the 16 positions are cell-type and technical readouts under the same exposure, not separate AAV candidates.
- Active CAD baseline: `sixteen_slot_cassette_incubator_first_article`.
- Older 20-chip / 4 x 5 CAD: reference and future scale direction only until the 16-slot cassette is validated.
- Wetted-path policy: first article uses disposable commercial tubing/connectors. Structural cassette parts are dry fixtures unless separately validated.
- Incubation policy: do not solve CO2 incubation in this package. Reserve dock/module interfaces so a cassette-level controlled module can be added later.

## Coordinate System

All dimensions are millimeters.

- Origin: center of the 16-slot cassette array.
- X axis: chip long edge / cassette columns.
- Y axis: chip short edge / cassette rows.
- Z axis: upward from the cassette/dock plane.
- Slot numbering: row-major from front/negative Y to rear/positive Y, and left/negative X to right/positive X.

| Slot row | Slot numbers |
| --- | --- |
| Row 1, Y = -138.72 | 1, 2, 3, 4 |
| Row 2, Y = -46.24 | 5, 6, 7, 8 |
| Row 3, Y = 46.24 | 9, 10, 11, 12 |
| Row 4, Y = 138.72 | 13, 14, 15, 16 |

## Core Dimensions

These values are sourced from `src/bin/sixteen_slot_cassette_incubator_first_article.rs` and Rev C chip constants in `src/lib.rs`.

| Interface | Value |
| --- | ---: |
| Rev C chip length, X | 127.76 |
| Rev C chip width, Y | 85.48 |
| Rev C chip total height | 14.35 |
| Cassette columns x rows | 4 x 4 |
| Slot count | 16 |
| Slot gutter X | 7.00 |
| Slot gutter Y | 7.00 |
| Slot pitch X | 134.76 |
| Slot pitch Y | 92.48 |
| Slot array X | 532.04 |
| Slot array Y | 362.92 |
| Carrier margin X | 58.00 per side |
| Carrier margin Y | 52.00 per side |
| Carrier envelope | 648.04 x 466.92 x 24.00 |
| Lid/clamp envelope | 666.04 x 484.92 x 10.00 |
| Window placeholder envelope | 616.04 x 438.92 x 3.00 |
| Dock plate envelope | 818.04 x 616.92 x 22.00 |
| Service bulkhead envelope | 738.04 x 34.00 x 76.00 |

## Exa Research Anchors

These references were checked during A0 to keep the interface spec aligned with current organ-chip and seal/fluid-interface practice.

| Source | Useful finding | A0 implication |
| --- | --- | --- |
| Minahan et al., modular reusable perfusion-ready MPS cassette, PMC12914553 | Uses a reusable acrylic cassette and compressible elastomeric inserts; clamping forms reversible leak-tight channels and preserves imaging access. | Keep LaminarForge's carrier reusable and structural, with disposable or separately validated culture/fluid-contact layers. Treat screw/clamp compression and imaging access as load-bearing requirements. |
| Sun et al., reusable standardized universal interface module, Micromachines 2019 | Reusable interface module supports plug-and-play organ-chip coupling; clamping-based approaches are explicitly evaluated for leakage-free reversible assembly. | Make datums, reversible clamping, and repeated assembly/disassembly part of the interface contract, not later accessories. |
| Vollertsen et al., fluidic circuit board for parallelized cell culture, PMC8433198 | A fluidic circuit board can operate multiple microfluidic building blocks with standardized interfaces and high parallelization. | Treat cassette ports and bulkhead zones as a frozen interface board. Downstream agents should avoid ad hoc tubing layouts that cannot be reproduced. |
| STARTER open modular organ-chip platform, PMC12834090 | Open modular platform uses swappable pump, sensor, and organ-chip modules within standard footprint logic and emphasizes interoperability. | Preserve module/dock interfaces and sensor zones even though active incubation control is deferred. |
| Static O-ring face seal guidance, allorings.com | Face seals require selected gland dimensions, compression, corner radii, and surface finish; typical recommendations call out smoother surfaces for gas sealing than liquids. | The 25% squeeze target is only a starting point. Gasket agent must specify actual cross-section, groove dimensions, finish, and tolerances before drawings. |
| TapeTech microfluidic connectors, PMC11795533 | World-to-chip connections are a major source of setup burden, pressure surges, priming issues, and errors; scalable connector strategy matters. | A6 must design prime/waste/bubble-visible routing and connector topology as a first-class interface, not loose tubing afterthoughts. |
| Sterile fluid transfer for cell therapy manufacturing, PMC8931546 | Closed systems commonly rely on sterile connectors or tube welders; sterile transfer has device, training, tubing material, and connection-frequency constraints. | First article should assume bought sterile connectors/tubing and document connector compatibility instead of custom sterile claims. |
| Integrated micro-gasket multi-organ chip connection, Micromachines 2025 | Thermoplastic elastomer gasket connections are tested for burst pressure, durability, repeated detachment/reconnection, and incubation exposure. | Gasket witness coupons and no-cell leak/burst/reconnection tests should be part of validation before live-cell or AAV work. |

## Slot Centers

| Slot | X | Y |
| ---: | ---: | ---: |
| 1 | -202.14 | -138.72 |
| 2 | -67.38 | -138.72 |
| 3 | 67.38 | -138.72 |
| 4 | 202.14 | -138.72 |
| 5 | -202.14 | -46.24 |
| 6 | -67.38 | -46.24 |
| 7 | 67.38 | -46.24 |
| 8 | 202.14 | -46.24 |
| 9 | -202.14 | 46.24 |
| 10 | -67.38 | 46.24 |
| 11 | 67.38 | 46.24 |
| 12 | 202.14 | 46.24 |
| 13 | -202.14 | 138.72 |
| 14 | -67.38 | 138.72 |
| 15 | 67.38 | 138.72 |
| 16 | 202.14 | 138.72 |

## Chip Pocket Interface

Current CAD baseline:

- Pocket size: Rev C chip length/width plus 1.20 mm clearance per side.
- Pocket X: 130.16.
- Pocket Y: 87.88.
- Pocket depth: 7.00.
- Optical through-window cut: 103.76 x 61.48.
- Lid view opening: 113.76 x 71.48.
- Slot label land: 26.00 x 10.00 per slot.

Open issue for carrier/DFM agents: the manufacturing readiness note proposes 0.5-0.8 mm chip pocket clearance per side until real chip lot tolerance is measured. The current CAD uses 1.20 mm per side. Keep 1.20 mm as the A0 baseline for compatibility with the current generator, but resolve this before manufacturer drawings.

## Datum Scheme

Use the cassette center as coordinate origin, but make physical registration deterministic:

- Primary datum: dock rear rail against cassette/carrier rear edge.
- Secondary datum: dock left rail against cassette/carrier left edge.
- Tertiary datum: front low retention lip and four carrier datum pin bosses.
- Carrier-to-lid alignment: four corner datum bosses/ears.
- Dock-to-bench/module alignment: dock leveling pad lands plus perimeter mount holes.

Current carrier datum pin boss centers:

| Datum | X | Y |
| --- | ---: | ---: |
| D1 front-left | -289.02 | -198.46 |
| D2 front-right | 289.02 | -198.46 |
| D3 rear-left | -289.02 | 198.46 |
| D4 rear-right | 289.02 | 198.46 |

Current datum boss assumptions:

- Boss diameter: 18.00.
- Bore diameter: 6.00.
- Boss height: 6.00 above carrier top.

Open issue for carrier/DFM agents: final dowel pin diameter, bore tolerance, and whether holes are reamed must be selected before drawings.

## Fastener Pattern

Current lid/clamp fastener centers:

- Side fasteners: X = +/-290.02 at Y = -138.72, -46.24, 46.24, 138.72.
- Front/rear fasteners: Y = +/-205.46 at X = -202.14, -67.38, 67.38, 202.14.
- Count: 16 lid/clamp fasteners.
- Current lid clearance hole diameter: 4.80, labeled as M4 clearance in CAD.

Current carrier and dock perimeter mount holes:

- Carrier perimeter mount holes: X/Y corner pattern at +/-302.02, +/-211.46 plus mid-front/mid-rear at X = 0, Y = +/-211.46.
- Carrier perimeter hole diameter: 5.40, labeled as M5 in CAD.
- Dock perimeter holes use the same six-position perimeter pattern relative to dock dimensions.

Open issue for lid/clamp/DFM agents: choose final screw family, captive hardware strategy, torque sequence, washer/load-spreader needs, and whether carrier holes are clearance, threaded, inserted, or through-bolted.

## Gasket Interface

Current CAD baseline:

- Per-slot raised gasket land: Rev C chip length/width plus 14.00 mm.
- Per-slot gasket land outer: 141.76 x 99.48.
- Per-slot gasket land wall width: 8.00.
- Per-slot gasket land height: 3.00 above carrier top.
- Perimeter gasket land outer: 604.04 x 428.92.
- Perimeter gasket land wall width: 12.00.
- Perimeter gasket land height: 4.00 above carrier top.
- Gasket witness coupon exists as a separate first-article output.

Gasket compression assumption from the gasket-install station:

- Nominal gasket thickness: 2.40.
- Target squeeze: 25%.
- Guard band: 20-30%.
- Witness steps: 20%, 25%, and 30% references.

Open issue for gasket/DFM agents: final gasket material, cross-section, groove depth/width, compression stop strategy, surface finish, and leak-test pressure are not frozen. Do not claim sterility or leak release from CAD alone.

## Imaging And Window Keepouts

Current CAD baseline:

- Carrier optical window cut per slot: 103.76 x 61.48.
- Lid view opening per slot: 113.76 x 71.48.
- Window witness frame per slot: 109.76 x 67.48 with 3.00 wall.
- Window placeholder panel: 616.04 x 438.92 x 3.00.
- Calibration fiducials: three window fiducials outside the slot array at top-left, top-right, and bottom-left reference positions.

Downstream agents must preserve line-of-sight through the chip chamber/well region and keep clamp bars, gasket lands, labels, tubing, and fasteners out of the imaging keepout unless the integrator changes the optical strategy.

## Leak, Drain, And Condensate Interfaces

Carrier:

- Perimeter leak gutter: rectangular frame cut inside carrier.
- Carrier drain port: 8.00 diameter side-drilled feature near front-right corner.
- Side service relief lands: left/right tubing/service relief features along the slot array.

Dock:

- Front condensate gutter.
- Right-side drain gutter.
- Visible drain sump near front-right corner.
- Air bypass windows between slot rows.
- Four logger pockets outside the carrier envelope.

Open issue for validation agents: leak-test pressure, allowable pressure decay, dye/tracer method, drain collection interface, and acceptance thresholds remain external validation requirements.

## Barcode And Condition ID

Required identity fields:

- Cassette ID.
- AAV condition ID.
- Capsid ID.
- Promoter ID.
- Payload ID.
- Dose/MOI.
- Media recipe.
- Date/time and run ID.
- Slot map revision.

Current CAD surfaces:

- Per-slot label lands exist on the carrier.
- Bulkhead label strip exists on the service block.
- Manufacturing readiness requires barcode/RFID labels compatible with humidity/condensation.

Open issue for DFM/build-package agents: define durable label material, barcode size, RFID option, human-readable text area, and cleaning/condensation survivability.

## Reserved Fluid Path Interfaces

Current first-article service bulkhead reserves:

- Four gas ports, currently modeled as 8.00 diameter cuts.
- Seven media ports, currently modeled as 6.40 diameter cuts.
- Five waste ports, currently modeled as 6.40 diameter cuts.
- Twelve tubing strain-relief comb teeth.
- Disposable tubing/connectors remain off-the-shelf selections.

Fluid-path agent constraints:

- One AAV condition per cassette.
- Do not route multiple AAV candidates through a single cassette.
- Preserve prime path, waste path, bubble visibility, and dead-volume measurability.
- Use bought sterile connectors/tubing for first article; do not make printed or machined structural parts the validated wetted path.

Open issue for fluid-path agent: connector family, tube OD/ID, bend radius, port assignment, valve/pump interface, and sterile connection method are not frozen.

A6 handoff: `docs/sixteen_slot_cassette_a6_disposable_fluid_path_spec.md` resolves the first-article fluid path as a disposable, one-condition, pressure-limited single-pass harness with bought sterile connector/tube-welding assumptions. It assigns logical G0-G3, M0-M6, and W0-W4 port roles while keeping final connector SKUs, tubing OD/ID, pump type, and vendor connector cutouts open for RFQ/validation.

A7 handoff: `docs/sixteen_slot_cassette_a7_no_cell_validation_fixture_spec.md` defines the no-cell bench gate for the 16-slot first article: fit/datum, compression witness, harness topology, prime/debubble, leak/pressure decay, flow balance, restriction/occlusion detection, dye recovery, waste/backflow, and repeat assembly tests. Older 4 x 5 / 20-position validation fixtures are reference concepts only until ported to this slot map and A6 port plan.

A8 handoff: `docs/sixteen_slot_cassette_a8_first_article_build_package.md` consolidates A0-A7 into the first-article build package. It records the verified seven-STL internal fit-check export, states that vendor RFQ still requires STEP files and 2D drawings, defines custom and bought-component package assumptions, sets assembly/QA order, and lists blockers before media-only, live-cell, or AAV work.

A9 handoff: `docs/sixteen_slot_cassette_a9_step_drawing_export_plan.md` defines the STEP and drawing release plan required before vendor RFQ. It selects true STEP/B-rep export as the intended path, rejects STL-only or silently skipped conversion for release, defines D0-D9 drawing sheets, and lists critical inspection tables and stackup review requirements.

## Reserved Sensor And Module Interfaces

Current first-article service/dock reserves:

- Sensor/backplane connector cut: 88.00 x 18.00 at X = 300.00 on the service bulkhead.
- Cable strain-relief shelf and zip-slot on the service bulkhead.
- Four dock logger pockets: 48.00 x 32.00 x 8.00.
- Dock leveling pads at all four dock corners.
- Dock robot lift lands at front and rear.

Sensor/readout agent constraints:

- Keep electrical/readout interfaces dry.
- Support per-slot or per-cassette measurements without compromising the disposable fluid path.
- Reserve TEER/impedance, temperature/RH logging, imaging fiducials, and run identity capture.

Open issue for sensor/module agents: final connector family, cable routing, strain relief, logger model, TEER/impedance electrode interface, and incubation-module quick-connect are not frozen.

## Downstream Agent Contracts

| Agent | May change | Must not change without integrator approval |
| --- | --- | --- |
| A1 Carrier | Pocket detail, datum implementation, drain/gutter details, label surfaces, handling lands | 4 x 4 slot count, slot centers, one-condition semantics, dry structural policy |
| A2 Lid/Clamp | Clamp bar section, window framing, fastener hardware approach, captive screw details | Imaging keepouts, carrier datum relationship, 16-fastener baseline without documented replacement |
| A3 Gasket | Groove/cross-section recommendation, compression stops, witness coupon details | 20-30% guard band and nominal 25% target unless justified |
| A4 DFM | Material/process/tolerance recommendations, radii, surface finishes, drawing notes | Disposable wetted-path policy and 16-slot first-build baseline |
| A6 Fluid Path | Port assignment, tubing route, connector recommendation, prime/waste/bubble plan | One AAV condition per cassette and bought disposable wetted path for first article |
| A7 No-Cell Validation | Leak/flow/bubble/dead-volume fixture details and acceptance evidence plan | Shared slot map, datums, and condition ID linkage |
| A8 Build Package | Output manifest, BOM assumptions, assembly/QA checklist, RFQ readiness blockers | No biological claims, no STL-only vendor release, no mixed-AAV routing |
| A9 STEP/Drawings | STEP export plan, drawing sheet list, inspection tables, stackup review plan | No mesh-derived release geometry, no silent export fallback, no RFQ without drawings |

## A0 Open Decisions

These items are deliberately not finalized in A0:

1. Chip pocket clearance: 1.20 mm per side in current CAD versus 0.5-0.8 mm per side in manufacturing-readiness guidance.
2. Final material: 6061-T6 aluminum, stainless, polycarbonate, or another autoclavable engineering plastic.
3. Gasket material, cross-section, and final groove dimensions.
4. Fastener family, torque sequence, captive hardware, and inserts.
5. Connector family and port assignment.
6. Sensor connector and logger model.
7. Leak-test pressure, pressure-decay acceptance, and dye/tracer method.
8. STEP/export pipeline and 2D drawing tolerance callouts.

## Required Handoff From A0

The cassette integrator should not accept downstream subsystem work unless it references this spec and states which A0 open decisions it resolves. Any change to slot count, slot centers, datum scheme, one-condition semantics, or disposable wetted-path policy requires an explicit revision to this document.
