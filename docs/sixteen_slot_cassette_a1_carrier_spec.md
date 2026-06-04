# 16-Slot Cassette A1 Carrier Subsystem Spec

This document is the A1 carrier subsystem specification for the first physical 16-slot LaminarForge AAV cassette build. It is controlled by `docs/sixteen_slot_cassette_a0_interface_spec.md` and uses `docs/cassette_incubator_manufacturing_readiness.md` as manufacturing context.

Scope is limited to the reusable lower carrier: chip pockets, datum pins/rails, carrier body/envelope, leak tray/drain features, barcode/condition ID surfaces, and manual/robot handling features. Lid/clamp, gasket, fluid path, sensor, and full DFM decisions are referenced only where they affect carrier interfaces.

This is not a production drawing, sterile-barrier claim, live-cell release criterion, or fluid-path validation.

## A1 Baseline

| Item | A1 carrier requirement |
| --- | --- |
| First-build format | 16 slots in a 4 x 4 grid. |
| Experimental unit | One cassette equals one AAV capsid/promoter/payload/dose/timing/media condition. Slots are readouts/replicates, not separate AAV candidates. |
| Active CAD baseline | `sixteen_slot_cassette_incubator_first_article`. |
| Carrier role | Reusable dry structural fixture. Wetted-path contact remains disposable commercial tubing/connectors or separately validated chip/fluid components. |
| Carrier envelope | 648.04 x 466.92 x 24.00 mm. |
| Coordinate origin | Center of 16-slot array, per A0. |
| Datum policy | Rear rail primary, left rail secondary, front low lip plus four datum bosses tertiary/assembly reference. |
| Material direction | CNC-machined 6061-T651 or 6061-T6 aluminum plate, clear anodized after DFM review; no sterile/wetted claim. |

## Slot And Pocket Geometry

A1 preserves the A0 slot centers and row-major slot numbering. Slot centers are basic dimensions from the A0 interface contract and must not be changed by the carrier agent.

### Pocket Function

Each pocket locates one Rev C chip in XY only loosely enough to permit manual placement, gauge-chip inspection, and thermal/humidity cycling without wedging. The pocket must not be used as the precision datum for imaging, sealing, or fluid routing. The dock rails, carrier datums, lid/clamp interfaces, and optical fiducials carry those repeatability requirements.

### Pocket Dimensions

| Feature | A0/current CAD | A1 drawing target |
| --- | ---: | ---: |
| Rev C chip nominal X | 127.76 | 127.76 |
| Rev C chip nominal Y | 85.48 | 85.48 |
| Current CAD pocket clearance | 1.20 mm per side | Retain for STL fit-check compatibility. |
| Current CAD pocket size | 130.16 x 87.88 | Fit-check baseline only. |
| A1 manufacturer-drawing target clearance | n/a | 0.80 mm per side from measured maximum chip lot size. |
| Nominal target if Rev C nominal is confirmed | n/a | 129.36 x 87.08 before drawing tolerance. |
| Minimum accepted clearance after chip-lot measurement | n/a | 0.50 mm per side against measured maximum chip size. |
| Pocket depth | 7.00 | 7.00 unless chip stackup measurement forces revision. |
| Carrier optical through-window cut | 103.76 x 61.48 | Preserve. |

A1 resolves the A0 clearance conflict by splitting the prototype and drawing baselines:

- Keep the current 1.20 mm per-side CAD clearance for first STL compatibility and dummy-chip fit checks.
- For manufacturer drawings, set pocket dimensions from the measured maximum Rev C chip lot plus 1.60 mm total XY clearance.
- If chip-lot data is unavailable at drawing release, use 129.36 x 87.08 mm as the provisional pocket size and require a gauge-chip inspection hold before machining production-like parts.

### Pocket Detail Requirements

- Pocket floors remain dry support features; do not create fluid reservoirs, adhesive wells, or sterile wetted surfaces in the carrier.
- Pocket sidewalls must include a machineable internal corner radius selected by the DFM agent or vendor. Use 1.5 mm radius as the starting note unless tooling feedback changes it.
- Add a small lead-in/chamfer on the top pocket edge for chip loading after DFM review; do not reduce the gasket land or optical keepout.
- Pocket floor and sidewall burrs are unacceptable because loose particles can compromise chip placement and labels. Drawing notes should require deburr without rounding datum-critical pocket edges.
- Slot pocket positions shall be controlled relative to the A0 rear/left physical datums, not independently dimensioned from each pocket.

## Datum Pins, Rails, And Registration

The A1 carrier uses deterministic physical registration while avoiding overconstraint from four hard pins.

### Physical Datum Scheme

| Datum | Physical feature | A1 requirement |
| --- | --- | --- |
| A | Carrier bottom plane / dock support plane | Establish carrier Z seating and flatness reference. |
| B | Carrier rear edge against dock rear rail | Primary in-plane datum; machined in same setup as pocket array when practical. |
| C | Carrier left edge against dock left rail | Secondary in-plane datum; machined square to datum B. |
| Tertiary | Front low retention lip | Prevents gross front lift/walkout without fighting B/C rails. |
| Assembly references | Four corner datum bosses at A0 coordinates | Preserve boss locations; do not treat all four holes as simultaneous hard locators. |

A0 datum boss centers remain locked:

| Datum boss | X | Y | A1 role |
| --- | ---: | ---: | --- |
| D1 front-left | -289.02 | -198.46 | Primary precision round hole for lid/tooling alignment. |
| D2 front-right | 289.02 | -198.46 | Secondary slotted/relieved locator or slip-clearance hole to avoid overconstraint. |
| D3 rear-left | -289.02 | 198.46 | Clearance/witness hole for lid/dock presence and assembly check. |
| D4 rear-right | 289.02 | 198.46 | Clearance/witness hole for lid/dock presence and assembly check. |

### Pin And Hole Implementation

| Feature | A1 decision |
| --- | --- |
| Nominal dowel family | 6 mm stainless ground dowel pins. |
| Boss diameter | 18.00 mm baseline preserved. |
| Boss height | 6.00 mm above carrier top baseline preserved. |
| Carrier bores | Reamed after rough drilling where they control alignment. |
| Round locating hole | 6 mm H7 slip-fit style hole, verified after finish or masked during finish. |
| Secondary locator | Slotted or diamond/relieved geometry around 6 mm pin preferred for the second locator. If the current round-bore CAD is retained for prototype STL, drawing notes must call out the intended relieved locator behavior. |
| Remaining holes | Clearance/witness only; do not use all four as press-fit hard locators. |
| Press fits | Avoid permanent press-fit pins in the carrier unless the integrator assigns a matched-lid or dock assembly strategy. Press-fit pins may live in replaceable mating tooling instead. |

Datum hole position, perpendicularity, and cylindricity should use GD&T in the drawing set. A1 starting targets for vendor discussion are:

- D1 true position: 0.10 mm relative to datums A/B/C.
- D2 true position/slot center: 0.15 mm relative to datums A/B/C and D1.
- D3/D4 witness hole position: 0.30 mm unless later promoted to functional locators.
- Pocket array true position: 0.25 mm relative to datums A/B/C.

Final tolerances remain DFM/vendor-confirmed because carrier size, material stock, anodize thickness, and selected inspection method affect achievable cost.

## Carrier Body And Envelope

The A1 carrier body remains a single reusable dry structural plate.

| Feature | Requirement |
| --- | --- |
| Envelope | 648.04 x 466.92 x 24.00 mm. |
| Slot array | 532.04 x 362.92 mm centered in carrier. |
| Margins | 58.00 mm per side in X; 52.00 mm per side in Y. |
| Perimeter mount pattern | Preserve A0 six-position carrier perimeter pattern unless integrator revises the dock/lid stack. |
| Top features | Chip pockets, gasket lands, per-slot label lands, datum bosses, leak gutter, service relief lands. |
| Bottom features | Flat seating plane plus future handling/lift lands only; no hidden fluid channels. |
| Structural policy | Carrier shall not be counted as a sterile wetted part. |

### Material And Finish

A1 selects 6061-T651 or 6061-T6 aluminum as the first-article carrier material because it is practical for a large CNC plate, supports reamed datum holes, and matches the readiness package direction for a dry structural first article.

Starting finish callout:

- Clear anodize after machining for corrosion/handling resistance, subject to DFM review.
- Mask or post-verify datum bores, datum edges, and any seal-adjacent critical surfaces where anodize buildup would affect fit.
- No coating inside datum holes unless the drawing tolerance explicitly accounts for coating thickness.
- No biological compatibility claim from carrier material or finish.

Escalate material if the build package later requires repeated autoclave exposure, aggressive chemical cleaning, or direct wetted contact. Those requirements would reopen the material decision toward stainless, polycarbonate, or another validated material.

## Leak Tray And Drain Features

A1 preserves leak capture as a visible witness and containment feature, not a validated leak-release method.

| Feature | A1 requirement |
| --- | --- |
| Perimeter leak gutter | Continuous rectangular gutter inside the carrier perimeter and outside the slot/gasket functional zones. |
| Current gutter basis | A0 rectangular frame cut, 7.00 mm width, positioned inside carrier. |
| Drain port | 8.00 mm diameter side-drilled feature near the front-right carrier corner. |
| Drain role | Route visible drips/condensate/leak-test liquid to a collection witness point without contacting chips or labels. |
| Drain interface | Keep accessible from the dock front/right service side and compatible with a removable collection tube or absorbent witness pad. |
| Cleanability | No blind pockets that trap liquid outside visible inspection surfaces. |

The gutter must not intersect chip pockets, optical windows, gasket lands, datum bosses, label areas, or handling pads. If DFM cannot machine a sloped gutter in this large plate without cost or warp risk, use a flat-bottom gutter for A1 and rely on visual/witness collection rather than gravity-drain completeness.

Leak-test pressure, pressure decay, dye/tracer method, drain collection hardware, and acceptance thresholds are escalated to validation/build-package agents.

## Barcode And Condition ID Area

Carrier identity must survive assembly handling and must link the physical cassette to the AAV condition and slot map. A1 defines geometry and placement only; final label stock and cleaning/condensation survivability are build-package/DFM decisions.

### Required Fields

- Cassette ID.
- AAV condition ID.
- Capsid ID.
- Promoter ID.
- Payload ID.
- Dose/MOI.
- Media recipe.
- Date/time and run ID.
- Slot map revision.

### Carrier Label Surfaces

| Surface | Requirement |
| --- | --- |
| Global cassette/condition label land | Add or reserve a front-left dry margin land outside gasket/leak features, minimum 90 x 24 mm, readable when docked. |
| Machine-readable code | Prefer 2D Data Matrix or QR on the global land; barcode format finalization remains build-package scope. |
| Human-readable text | Reserve adjacent text area for cassette ID and short condition ID. |
| Per-slot label lands | Preserve current 26.00 x 10.00 per-slot lands for slot number/witness marks, not full condition metadata. |
| Orientation mark | Add a visible front-left orientation mark tied to slot 1 and datum B/C. |
| RFID | Optional only; reserve physical space if selected, but do not require RFID for A1 carrier release. |

Labels must not sit on gasket lands, pocket seating surfaces, optical paths, datum bosses, rail contact edges, or handling pads.

## Manual And Robot Handling

The carrier must be movable without touching chip windows, gasket lands, tubing reliefs, or identity labels.

### Manual Handling

- Reserve long-edge dry handling bands on the left/right carrier margins.
- Add shallow finger reliefs or textured grip flats only in margin areas outside the leak gutter, datum bosses, and service reliefs.
- Maintain a clear visual orientation cue at the front-left corner so slot 1 is unambiguous during manual load/unload.
- Do not add handles that increase the A0 carrier envelope unless the cassette integrator revises the dock envelope.

### Robot Handling

- Preserve the dock robot lift-land relationship from A0 and do not block front/rear robot approach.
- Reserve underside/front-rear pickup lands on the carrier for future parallel-jaw or fork-style handling. Starting target: four dry contact lands, each at least 50 x 18 mm, outside pocket/window/gutter intersections.
- Robot contact features must be dry, cleanable, and outside label and drain witness areas.
- Robot handling must reference the same rear/left datum convention used by the dock; do not create a competing slot-based coordinate system.

Final gripper geometry, gripper force, robot approach height, and end-effector material are escalated to the robot/dock handling agent.

## Targeted Exa Research Pass

The research pass was intentionally narrow and used only for carrier datum, material, and pocket-tolerance decisions.

| Source | Relevant finding | A1 implication |
| --- | --- | --- |
| Fictiv, "Slip Fit Tolerances and Geometry" | Slip-fit dowels are useful for repeated assembly and jig alignment; GD&T is preferred for true position/cylindricity/perpendicularity; using too many hard pins can overconstrain an assembly. | Preserve four A0 bosses but make only the first two functional locators, with the second relieved/slotted and the others clearance/witness. Use GD&T instead of loose coordinate tolerancing. |
| Engineers Edge, "Dowel Pin Installation Design Tolerance Table Chart" | Dowel pins are typical for alignment between mating components; examples use aluminum base parts and stainless dowels, with separate press-fit and slip-fit holes. | Select 6 mm stainless dowels and distinguish carrier slip-fit/witness holes from any press-fit tooling holes. |
| SPIROL, "Hole Preparation for Press Fit Pins" | Ground dowel/straight pin press fits need precise, straight drilled and reamed holes; tight hole tolerances increase preparation cost. | Ream carrier datum holes only where they control alignment; do not make all four carrier bosses precision press-fit features. |
| Minahan et al., modular reusable perfusion-ready MPS cassette | A reusable acrylic cassette clamps interchangeable elastomeric inserts and supports imaging while separating reusable structure from consumable inserts. | Keep the LaminarForge carrier reusable/dry/structural and avoid making the carrier the validated wetted boundary. |
| STARTER modular organ-on-chip platform | Modular OoC platforms benefit from standardized footprints, interoperable modules, and reusable interface boards. | Treat the carrier as a standardized mechanical interface board with stable datums, slot map, and identity surfaces. |

Research URLs:

- https://www.fictiv.com/articles/slip-fit-tolerances-and-geometry
- https://www.engineersedge.com/dowel_pin.htm
- https://www.spirol.com/assets/files/pins-wp-hole-preparation-for-press-fit-pins-us.pdf
- https://pmc.ncbi.nlm.nih.gov/articles/PMC12914553/
- https://pmc.ncbi.nlm.nih.gov/articles/PMC12834090/

## A0 Decisions Resolved By A1

| A0 open decision | A1 resolution |
| --- | --- |
| Chip pocket clearance | Keep 1.20 mm per side for current CAD/STL fit-check compatibility; set manufacturer-drawing target to measured chip max plus 0.80 mm per side, with 0.50 mm per side minimum accepted clearance. |
| Final carrier material | Select CNC 6061-T651 or 6061-T6 aluminum, clear anodized, for the dry first-article carrier. |
| Datum pin diameter and hole preparation | Select 6 mm stainless dowel family; ream functional carrier holes; avoid using all four bosses as hard locators. |
| Datum implementation | Rear edge/rail is primary, left edge/rail secondary, front lip tertiary; D1/D2 are functional assembly references, D3/D4 are clearance/witness unless promoted by integrator. |
| Carrier leak/drain geometry | Preserve continuous visible perimeter leak gutter and 8 mm front-right side drain as witness/collection features. |
| Barcode/condition ID area | Reserve front-left global ID land plus per-slot 26 x 10 mm label lands; prefer 2D machine-readable code with human-readable cassette/condition ID. |
| Carrier handling surfaces | Reserve dry long-edge manual handling bands and future underside/front-rear robot pickup lands without changing the A0 envelope. |

## A0 Decisions Escalated

| A0 open decision | Escalation path |
| --- | --- |
| Final chip pocket drawing dimensions if chip lot data is missing | Integrator/DFM must measure Rev C chip lot or approve provisional 129.36 x 87.08 mm pocket target before manufacturer drawing release. |
| Gasket material, cross-section, groove dimensions, compression stops, and seal-land finish | A3 gasket and A4 DFM agents. A1 only preserves carrier gasket-land interfaces. |
| Fastener family, torque sequence, captive hardware, washers, inserts, and whether carrier holes are threaded/inserted/through-bolted | A2 lid/clamp and A4 DFM agents. A1 preserves the A0 mount/fastener interface locations. |
| Connector family, tubing OD/ID, bend radius, valve/pump interface, port assignment, and sterile connection method | A6 fluid-path agent. Carrier remains dry structure. |
| Sensor connector family, logger model, TEER/impedance electrode interface, cable routing, and incubation-module quick-connect | Sensor/module agents. A1 preserves dry keepouts and identity linkage. |
| Leak-test pressure, pressure-decay acceptance, dye/tracer method, and release criteria | A7 no-cell validation/build-package agents. |
| Final large-plate flatness, anodize masking, inspection plan, internal radii, and vendor-specific tolerances | A4 DFM/vendor RFQ. A1 provides starting targets only. |
| STEP/export pipeline and production drawing package | CAD integrator/build-package agent. A1 did not modify the final CAD generator. |

## CAD Handoff Notes

No CAD generator edit is required for this A1 spec. The current generator remains valid as the A0/A1 fit-check baseline, with these known documentation-to-CAD deltas for future drawing work:

- Current CAD pocket clearance remains 1.20 mm per side; A1 drawing target is tighter and chip-lot-driven.
- Current CAD models all four datum boss bores as round 6 mm holes; A1 wants only D1/D2 to function as controlled locators and prefers a relieved/slotted D2.
- Current CAD has per-slot label lands; A1 additionally reserves a global cassette/condition ID land.
- Current CAD has dock robot lift lands; A1 reserves carrier handling/pickup lands for future robot handling detail.

Any future CAD change must preserve A0 slot count, slot centers, one-condition semantics, dry structural carrier policy, imaging keepouts, and carrier envelope unless the cassette integrator revises the A0 contract.
