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
| Carrier base body | 699.04 x 541.92 x 24.00 mm. |
| Carrier true overall bounds | 699.04 x 541.92 x 31.35 mm, including the 7.35 mm closure-plane lands and stops above the 24.00 mm base body. |
| Coordinate origin | Center of 16-slot array, per A0. |
| Datum policy | Datum A is the carrier bottom plane seated directly on the dock support plane. The dock rear- and left-rail inner faces contact the nominal rear and left carrier edges, and the front lip contacts the nominal front edge. D1 is the functional round locator, D2 is the functional relieved locator, and D3/D4 are clearance/witness features. |
| Material direction | CNC-machined 6061-T651 or 6061-T6 aluminum plate, clear anodized after DFM review; no sterile/wetted claim. |

## Slot And Pocket Geometry

A1 preserves the A0 slot centers and row-major slot numbering. Slot centers are basic dimensions from the A0 interface contract and must not be changed by the carrier agent.

The machine-readable A0 contract uses a 24.00 x 24.00 mm inter-chip gutter, 151.76 mm X pitch, 109.48 mm Y pitch, and a 583.04 x 413.92 mm centered array. The four X center coordinates are -227.64, -75.88, 75.88, and 227.64 mm; the four Y center coordinates are -164.22, -54.74, 54.74, and 164.22 mm.

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
| A | Carrier bottom plane seated directly on dock support plane | Establish carrier Z seating and flatness reference; no intermediate spacer or recess offset is part of the active stack. |
| B | Nominal carrier rear edge against dock rear-rail inner face | Primary in-plane datum; machined in same setup as pocket array when practical. |
| C | Nominal carrier left edge against dock left-rail inner face | Secondary in-plane datum; machined square to datum B. |
| Retention feature, not a datum | Nominal carrier front edge against front low retention lip | Prevents gross front lift/walkout without competing with the A/B/C datum scheme. |
| Assembly references | Four corner datum bosses at A0 coordinates | Only D1 and D2 locate. D3/D4 remain clearance/witness features and must not create a four-pin constraint. |

A0 datum boss centers remain locked:

| Datum boss | X | Y | A1 role |
| --- | ---: | ---: | --- |
| D1 front-left | -335.52 | -204.96 | Primary precision round hole for lid/tooling alignment. |
| D2 front-right | 335.52 | -204.96 | Secondary relieved locator to avoid overconstraint. |
| D3 rear-left | -335.52 | 204.96 | Clearance/witness hole for lid/dock presence and assembly check. |
| D4 rear-right | 335.52 | 204.96 | Clearance/witness hole for lid/dock presence and assembly check. |

### Pin And Hole Implementation

| Feature | A1 decision |
| --- | --- |
| Nominal dowel family | Replaceable 6 mm stainless ground dowel pins carried by D1/D2 lid seats; the fit-check surrogate is 5.80 mm diameter. |
| Boss diameter | 18.00 mm baseline preserved. |
| Boss height | 6.00 mm above carrier top baseline preserved. |
| Carrier bores | Reamed after rough drilling where they control alignment. |
| Round locating hole | D1 uses a 6 mm H7 slip-fit style hole, verified after finish or masked during finish. |
| Secondary locator | D2 uses the A0 10.00 x 6.00 X-oriented slot around the 6 mm pin. The drawing must preserve that functional relief. |
| Remaining holes | D3/D4 use 9.00 mm clearance/witness bores; do not use all four as press-fit hard locators. |
| Press fits | No permanent or integral carrier pins. D1/D2 pins are replaceable mating hardware seated 4.00 mm into the lid; the current surrogate extends 3.35 mm below the lid and engages the carrier boss by 2.00 mm. |

Datum hole position, perpendicularity, and cylindricity should use GD&T in the drawing set. A1 starting targets for vendor discussion are:

- D1 true position: 0.10 mm relative to datums A/B/C.
- D2 true position/slot center: 0.15 mm relative to datums A/B/C and D1.
- D3/D4 witness hole position: 0.30 mm; they remain non-locating clearance/witness features.
- Pocket array true position: 0.25 mm relative to datums A/B/C.

Final tolerances remain DFM/vendor-confirmed because carrier size, material stock, anodize thickness, and selected inspection method affect achievable cost.

## Carrier Body And Envelope

The A1 carrier body remains a single reusable dry structural plate.

| Feature | Requirement |
| --- | --- |
| Base-body dimensions | 699.04 x 541.92 x 24.00 mm. |
| True overall bounds | 699.04 x 541.92 x 31.35 mm. Do not call the 24.00 mm base thickness the overall Z bound. |
| Slot array | 583.04 x 413.92 mm centered in carrier. |
| Margins | 58.00 mm per side in X; 64.00 mm per side in Y. |
| Lid fastener interface | Preserve the A0 16-position pattern outside the gutter: side X = +/-332.52 mm at the four slot-center Y values; front/rear Y = +/-247.96 mm at the four slot-center X values. Add matching 3.30 mm carrier-side M4 tap-drill/pilot receiver placeholders at all 16 centers. |
| Top features | Chip pockets; sixteen independent, nonoverlapping per-slot gasket lands; perimeter gasket land; nine internal and distributed perimeter hard stops; label lands outside the leak gutter; datum bosses; leak gutter; and 7.00 mm-high service relief lands below closure. Service lands preserve an 11.00 mm-radius keepout at every datum, and D1-D4 bores are final subtraction features after all unions. |
| Bottom features | Flat seating plane plus future handling/lift lands only; no hidden fluid channels. |
| Structural policy | Carrier shall not be counted as a sterile wetted part. |

Seal geometry on the carrier is no longer an open or overlapping prototype layout. Each of the sixteen loops has a 145.76 x 103.48 mm outer boundary, 129.76 x 87.48 mm inner boundary, 8.00 mm width, and 7.35 mm height, with 6.00 mm clear between adjacent outer edges. The perimeter land is 629.04 x 459.92 mm outer, 605.04 x 435.92 mm inner, 12.00 mm wide, and also 7.35 mm high. Both seal families meet the lid underside on the common 7.35 mm nominal closure/chip-top plane; the 2.40 mm gasket compresses to the 1.80 mm lid-groove depth rather than spanning a gap above a shorter land.

The obsolete four-corner-stop pattern around every slot is replaced by nine
4.00 mm-diameter internal hard stops at the 3 x 3 inter-slot gap intersections:
X = -151.76, 0.00, 151.76 mm and Y = -109.48, 0.00, 109.48 mm. Distributed
perimeter stops remain 4.00 mm wide and centered in the 5.00 mm
seal-to-gutter web. Every stop tops out at 7.35 mm and no stop may intersect a
seal. Final thread or insert hardware remains open, but the carrier must retain
the locked 3.30 mm pilot-receiver geometry at each of the 16 lid fasteners.

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
| Gutter geometry | Rectangular frame with 651.04 x 481.92 mm outer bounds, 639.04 x 469.92 mm inner bounds, 6.00 mm width, and 3.00 mm depth. |
| Separation from perimeter land | A 5.00 mm intact web separates the gutter inner edge from the 629.04 x 459.92 mm perimeter land outer edge. |
| Drain port | 8.00 mm diameter, centered at X = 315.52, Y = -254.96, Z = 10.00 mm, with 40.00 mm length along Y. |
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
| Global cassette/condition label lands | Preserve the 96.00 x 12.00 barcode land at (-271.52, -263.96) mm and the 118.00 x 10.00 text land at (-159.52, -263.96) mm in the front dry margin, entirely outside the leak gutter and clear of the front fastener/receiver row. |
| Machine-readable code | Prefer 2D Data Matrix or QR on the global land; barcode format finalization remains build-package scope. |
| Human-readable text | Reserve adjacent text area for cassette ID and short condition ID. |
| Per-slot label lands | Preserve the 26.00 x 10.00 per-slot lands in the rear dry margin outside the leak gutter for slot number/witness marks, not full condition metadata. |
| Orientation mark | Add a visible front-left orientation mark tied to slot 1 and datum B/C. |
| RFID | Optional only; reserve physical space if selected, but do not require RFID for A1 carrier release. |

Labels must not sit on gasket lands, pocket seating surfaces, optical paths, datum bosses, rail contact edges, or handling pads.

## Manual And Robot Handling

The carrier must be movable without touching chip windows, gasket lands, tubing reliefs, or identity labels.

### Manual Handling

- Reserve long-edge dry handling bands on the left/right carrier margins.
- Add shallow finger reliefs or textured grip flats only in margin areas outside the leak gutter, datum bosses, and service reliefs.
- Maintain a clear visual orientation cue at the front-left corner so slot 1 is unambiguous during manual load/unload.
- Do not add handles that increase the A0 699.04 x 541.92 mm carrier XY bounds unless the cassette integrator revises the dock interface.

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

## A1 Decisions Carried Downstream

| Carrier decision | A1 resolution |
| --- | --- |
| Chip pocket clearance | Keep 1.20 mm per side for current CAD/STL fit-check compatibility; set manufacturer-drawing target to measured chip max plus 0.80 mm per side, with 0.50 mm per side minimum accepted clearance. |
| Final carrier material | Select CNC 6061-T651 or 6061-T6 aluminum, clear anodized, for the dry first-article carrier. |
| Datum pin diameter and hole preparation | Select 6 mm stainless dowel family; ream functional carrier holes; avoid using all four bosses as hard locators. |
| Datum implementation | A/B/C are bottom/rear/left. D1 is round, D2 is relieved, and only those two bosses locate; D3/D4 are clearance/witness features. |
| Carrier leak/drain geometry | Preserve the 6 mm-wide, 3 mm-deep gutter, its 5 mm separating web, and the 8 mm drain at the locked A0 coordinates. |
| Barcode/condition ID area | Preserve the global 96 x 12 mm code and 118 x 10 mm text lands at Y = -263.96 mm plus the per-slot 26 x 10 mm label lands outside the leak gutter; prefer a 2D machine-readable code with human-readable cassette/condition ID. |
| Carrier handling surfaces | Reserve dry long-edge manual handling bands and future underside/front-rear robot pickup lands without changing the A0 XY bounds. |

## Remaining Subsystem Decisions

| Remaining decision | Escalation path |
| --- | --- |
| Final chip pocket drawing dimensions if chip lot data is missing | Integrator/DFM must measure Rev C chip lot or approve provisional 129.36 x 87.08 mm pocket target before manufacturer drawing release. |
| Gasket compound/vendor tolerance, groove-fill evidence, seal-land finish, and no-cell validation | A3 gasket and A4 DFM agents. A1 preserves the locked 2.40 mm cross-section, 1.80 x 3.20 mm lid groove, and carrier land/stop geometry. |
| Fastener family, torque sequence, captive hardware, washers, and final thread/insert/nut-plate SKU | A2 lid/clamp and A4 DFM agents. A1 preserves the A0 fastener centers and all sixteen 3.30 mm carrier pilot receivers. |
| Connector family/SKU, tubing OD/ID, bend radius, valve/pump interface, and sterile connection method | A6 fluid-path work. The G0-G3, M0-M6, and W0-W4 logical role map is already resolved and must be preserved; the carrier remains dry structure. |
| Sensor connector family, logger model, TEER/impedance electrode interface, cable routing, and incubation-module quick-connect | Sensor/module agents. A1 preserves dry keepouts and identity linkage. |
| Leak-test pressure, pressure-decay acceptance, dye/tracer method, and release criteria | A7 no-cell validation/build-package agents. |
| Final large-plate flatness, anodize masking, inspection plan, internal radii, and vendor-specific tolerances | A4 DFM/vendor RFQ. A1 provides starting targets only. |
| Production drawing package | The true B-rep STEP draft is implemented; D0-D9 drawings, inspection tables, and release review remain with A9/build-package work. |

## CAD Handoff Notes

The STL fit-check and STEP draft generators now consume the shared machine-readable A0 contract. That integration corrected the earlier overlapping gasket-loop layout, unequal gasket-land elevation, undersized carrier margins, gutter/label conflicts, per-slot corner stops, and fasteners inside the gutter envelope. The remaining documentation-to-release items are narrower:

- Current CAD pocket clearance remains 1.20 mm per side; A1 drawing target is tighter and chip-lot-driven.
- D1 must remain the round functional locator and D2 the relieved functional locator; D3/D4 are non-locating clearance/witness features.
- Global and per-slot label lands must remain outside the leak gutter.
- The carrier must retain the 7.35 mm land/stop plane, nine internal stops,
  perimeter web stops, 3.30 mm pilot receivers, and 7.00 mm service reliefs.
- Datum A seats directly on the dock support plane; the rear/left rail inner
  faces and front lip contact the corresponding nominal carrier edges.
- Current CAD has dock robot lift lands; A1 reserves carrier handling/pickup lands for future robot handling detail.

Any future CAD change must preserve A0 slot count, slot centers, one-condition semantics, dry structural carrier policy, imaging keepouts, base-body dimensions, and true overall bounds unless the cassette integrator revises the shared A0 contract.
