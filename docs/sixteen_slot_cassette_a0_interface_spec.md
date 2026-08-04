# 16-Slot Cassette A0 Interface Control Spec

Architecture status: active first-build interface contract. The 16-slot, 4 x 4
cassette is the first physical LaminarForge AAV cassette architecture.
`docs/standard_footprint_aav_condition_module_a0.md` is a supplemental compact
module and local-coupon study; it does not supersede this contract or satisfy a
16-slot first-article gate.

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
- Precision policy: local coupons and compact-module studies may retire chip-fit,
  gasket, connector, and imaging risks before full-carrier fabrication, but they
  may not change the slot map, datums, condition semantics, or active first-build
  package without an explicit A0 revision.

## A0 Revision Basis

The current contract replaces the original 7.00 mm-gutter fit-check geometry.
That draft could not serve as an interface contract: its raised land opening was
smaller than the chip, adjacent gasket loops overlapped, the closure plane sat
below the nominal chip top, slot/global labels intersected functional zones,
fastener retainers crossed the perimeter seal, the gutter crossed the seal land,
and the STEP carrier omitted the drain. Those geometries are rejected and must
not be restored as compatibility variants.

`src/sixteen_slot_cassette_a0.rs` now fails closed on the controlling
non-intersection and parity checks. The full-size STL and STEP generators import
the same contract rather than carrying separate interface constants.

## Coordinate System

All dimensions are millimeters.

- X/Y origin: center of the 16-slot cassette array, coincident with the carrier
  and dock X/Y centerlines.
- X axis: chip long edge / cassette columns.
- Y axis: chip short edge / cassette rows.
- Assembly Z origin: dock base-body midplane. Z is positive upward. The dock
  support/datum-A plane is therefore Z = +11.00, not Z = 0.
- Individual part files use part-local Z = 0 at the base-body midplane. The
  stackup reference applies the transforms below; do not mix a part-local Z
  coordinate with the assembly frame without that transform.
- Slot numbering: row-major from front/negative Y to rear/positive Y, and left/negative X to right/positive X.

| Part/reference | Part-local base-body span | Assembly transform / controlling face |
| --- | --- | --- |
| Dock | Z = -11.00 to +11.00 | No translation; top support/datum A = +11.00. |
| Carrier | Z = -12.00 to +12.00 | Translate +23.00; bottom/datum A = +11.00, base top = +35.00, closure plane = +42.35. |
| Lid | Z = -5.00 to +5.00 before retained features | Translate +47.35; underside sealing face = +42.35. |
| Window placeholder | Z = -1.50 to +1.50 before retained features | Translate +55.85 in the stackup reference. |
| Service bulkhead | Part-local block centered at Z = 0 | Translate +49.00; bottom = +11.00. |

| Slot row | Slot numbers |
| --- | --- |
| Row 1, Y = -164.22 | 1, 2, 3, 4 |
| Row 2, Y = -54.74 | 5, 6, 7, 8 |
| Row 3, Y = 54.74 | 9, 10, 11, 12 |
| Row 4, Y = 164.22 | 13, 14, 15, 16 |

## Core Dimensions

These values are sourced from the machine-readable contract in
`src/sixteen_slot_cassette_a0.rs`. The STL and STEP generators both import that
contract. “Base body” describes the starting plate/block; “overall” includes
additive rails, lands, retainers, shelves, or witness features.

| Interface | Value |
| --- | ---: |
| Rev C chip length, X | 127.76 |
| Rev C chip width, Y | 85.48 |
| Rev C chip total height | 14.35 |
| Cassette columns x rows | 4 x 4 |
| Slot count | 16 |
| Slot gutter X | 24.00 |
| Slot gutter Y | 24.00 |
| Slot pitch X | 151.76 |
| Slot pitch Y | 109.48 |
| Slot array X | 583.04 |
| Slot array Y | 413.92 |
| Carrier margin X | 58.00 per side |
| Carrier margin Y | 64.00 per side |
| Carrier base body | 699.04 x 541.92 x 24.00 |
| Carrier generated overall | 699.04 x 541.92 x 31.35 |
| Lid/clamp base body | 717.04 x 559.92 x 10.00 |
| Lid/clamp generated overall | 717.04 x 559.92 x 11.60 |
| Lid underside seal skin / upper frame | 4.00 continuous / 6.00 relieved |
| Lid chip-top relief per slot | 130.16 x 87.88 x 0.50 deep |
| Window placeholder base panel | 667.04 x 489.92 x 3.00 |
| Window placeholder generated overall | 667.04 x 489.92 x 4.80 |
| Dock plate base body | 869.04 x 691.92 x 22.00 |
| Dock plate generated overall | 869.04 x 691.92 x 40.00 |
| Service bulkhead base body | 789.04 x 34.00 x 76.00 |
| Service bulkhead generated overall | 789.04 x 60.00 x 76.00 |

## Exa Research Anchors

These references were checked during A0 to keep the interface spec aligned with current organ-chip and seal/fluid-interface practice.

| Source | Useful finding | A0 implication |
| --- | --- | --- |
| Minahan et al., modular reusable perfusion-ready MPS cassette, PMC12914553 | Uses a reusable acrylic cassette and compressible elastomeric inserts; clamping forms reversible leak-tight channels and preserves imaging access. | Keep LaminarForge's carrier reusable and structural, with disposable or separately validated culture/fluid-contact layers. Treat screw/clamp compression and imaging access as load-bearing requirements. |
| Sun et al., reusable standardized universal interface module, Micromachines 2019 | Reusable interface module supports plug-and-play organ-chip coupling; clamping-based approaches are explicitly evaluated for leakage-free reversible assembly. | Make datums, reversible clamping, and repeated assembly/disassembly part of the interface contract, not later accessories. |
| Vollertsen et al., fluidic circuit board for parallelized cell culture, PMC8433198 | A fluidic circuit board can operate multiple microfluidic building blocks with standardized interfaces and high parallelization. | Treat cassette ports and bulkhead zones as a frozen interface board. Downstream agents should avoid ad hoc tubing layouts that cannot be reproduced. |
| STARTER open modular organ-chip platform, PMC12834090 | Open modular platform uses swappable pump, sensor, and organ-chip modules within standard footprint logic and emphasizes interoperability. | Preserve module/dock interfaces and sensor zones even though active incubation control is deferred. |
| Static O-ring face seal guidance, allorings.com | Face seals require selected gland dimensions, compression, corner radii, and surface finish; typical recommendations call out smoother surfaces for gas sealing than liquids. | A0 fixes the 2.40 mm cross-section, 1.80 x 3.20 mm lid groove, and 25% nominal squeeze; the gasket supplier, compound, tolerances, finish, and groove-fill evidence still require vendor review. |
| TapeTech microfluidic connectors, PMC11795533 | World-to-chip connections are a major source of setup burden, pressure surges, priming issues, and errors; scalable connector strategy matters. | A6 must design prime/waste/bubble-visible routing and connector topology as a first-class interface, not loose tubing afterthoughts. |
| Sterile fluid transfer for cell therapy manufacturing, PMC8931546 | Closed systems commonly rely on sterile connectors or tube welders; sterile transfer has device, training, tubing material, and connection-frequency constraints. | First article should assume bought sterile connectors/tubing and document connector compatibility instead of custom sterile claims. |
| Integrated micro-gasket multi-organ chip connection, Micromachines 2025 | Thermoplastic elastomer gasket connections are tested for burst pressure, durability, repeated detachment/reconnection, and incubation exposure. | Gasket witness coupons and no-cell leak/burst/reconnection tests should be part of validation before live-cell or AAV work. |

## Slot Centers

| Slot | X | Y |
| ---: | ---: | ---: |
| 1 | -227.64 | -164.22 |
| 2 | -75.88 | -164.22 |
| 3 | 75.88 | -164.22 |
| 4 | 227.64 | -164.22 |
| 5 | -227.64 | -54.74 |
| 6 | -75.88 | -54.74 |
| 7 | 75.88 | -54.74 |
| 8 | 227.64 | -54.74 |
| 9 | -227.64 | 54.74 |
| 10 | -75.88 | 54.74 |
| 11 | 75.88 | 54.74 |
| 12 | 227.64 | 54.74 |
| 13 | -227.64 | 164.22 |
| 14 | -75.88 | 164.22 |
| 15 | 75.88 | 164.22 |
| 16 | 227.64 | 164.22 |

## Chip Pocket Interface

Current CAD baseline:

- Pocket size: Rev C chip length/width plus 1.20 mm clearance per side.
- Pocket X: 130.16.
- Pocket Y: 87.88.
- Pocket depth: 7.00.
- Optical through-window cut: 103.76 x 61.48.
- Lid view opening: 113.76 x 71.48.
- Nominal Rev C chip protrusion above the carrier top: 7.35.
- Nominal lid closure plane: 7.35 above the carrier top.
- Slot label land: 26.00 x 10.00 per slot, moved to the dry rear margin
  outside the gasket and leak-gutter zones.

Open issue for carrier/DFM agents: the manufacturing readiness note proposes 0.5-0.8 mm chip pocket clearance per side until real chip lot tolerance is measured. The current CAD uses 1.20 mm per side. Keep 1.20 mm as the A0 baseline for compatibility with the current generator, but resolve this before manufacturer drawings.

The 7.35 mm closure plane is not optional: it equals the nominal 14.35 mm chip
height minus the 7.00 mm pocket depth. Both carrier seal-land families and all
hard stops top out 7.35 mm above the carrier top and meet the lid sealing face at
closure. Each chip receives a 130.16 x 87.88 x 0.50 mm local underside relief so
the nominal chip top does not carry clamp load. That relief remains 2.20 mm from
the inner edge of its gasket groove in both X and Y. The 1.80 mm-deep lid groove
contains the 2.40 mm gasket at 25% nominal squeeze; it is a cavity above the
mating face, not an additional 1.80 mm plate-to-plate gap. Any chip-lot height,
pocket-depth, or underside-relief change requires an A0 revision plus a D7
section review.

## Datum Scheme

Use the cassette center as coordinate origin, but make physical registration
deterministic with a 3-2-1-style scheme:

| Datum | Physical feature | Function |
| --- | --- | --- |
| A | Carrier bottom plane seated directly on the dock support plane | Establishes Z seating and the carrier flatness reference; no spacer or recess offset is permitted in the active stack. |
| B | Nominal carrier rear edge against the dock rear-rail inner face | Primary in-plane datum. |
| C | Nominal carrier left edge against the dock left-rail inner face | Secondary in-plane datum, square to B. |
| Retention | Nominal carrier front edge against the front low retention lip | Prevents gross front lift or walkout without opposing B/C. |

- Carrier-to-lid alignment uses the four corner boss/ear locations, but only D1
  and a relieved or slotted D2 may be functional hard locators. D3 and D4 remain
  clearance/witness features unless an explicit revision proves the assembly is
  not overconstrained.
- Dock-to-bench/module alignment uses dock leveling-pad lands and perimeter mount
  holes; those features do not replace carrier datums A/B/C.

Current carrier datum pin boss centers:

| Datum | X | Y |
| --- | ---: | ---: |
| D1 front-left | -335.52 | -204.96 |
| D2 front-right | 335.52 | -204.96 |
| D3 rear-left | -335.52 | 204.96 |
| D4 rear-right | 335.52 | 204.96 |

Current datum boss assumptions:

- Boss diameter: 18.00.
- D1 round locator bore: 6.00 diameter.
- D2 relieved locator: 10.00 x 6.00 slot, oriented along X between D1 and D2
  so the pair constrains Y without overconstraining X.
- D3/D4 clearance/witness bores: 9.00 diameter; these are explicitly
  non-locating.
- Boss height: 6.00 above carrier top.

First-article assumption from A1: use a 6 mm stainless ground-dowel family, ream
the functional D1/D2 carrier features after drilling, use a round D1 and
relieved/slotted D2 strategy, and keep D3/D4 as clearance/witness holes. Final
bore tolerances and post-finish inspection remain drawing-release decisions.

The lid manufacturing part contains replaceable-pin seats, not integral pins.
D1 and D2 each use a 6.00 mm-diameter x 4.00 mm-deep lid seat for a 5.80 mm
fit-check pin surrogate. The surrogate is 7.35 mm long: 4.00 mm embedded in the
lid seat and 3.35 mm projecting below the lid, giving 2.00 mm engagement in the
carrier boss at closure. D3/D4 receive no pins or pin seats. Pin surrogates appear
only in assembly/stackup reference output; select the commercial pin SKU and
final fits before drawing release.

## Fastener Pattern

First-article lid/clamp assumptions:

- Use sixteen M4 x 0.7 stainless commercial captive panel screws.
- Screws provide clamp load only; the datum system aligns the lid.
- Use positive-drive heads and commercial retainers. Loose screws,
  self-tapping screws, adhesive-only retention, and printed receiver threads are
  not acceptable first-article baselines.

Current lid/clamp fastener centers:

- Side fasteners: X = +/-332.52 at Y = -164.22, -54.74, 54.74, 164.22.
- Front/rear fasteners: Y = +/-247.96 at X = -227.64, -75.88, 75.88, 227.64.
- Count: 16 lid/clamp fasteners.
- Current lid clearance hole diameter: 4.80, labeled as M4 clearance in CAD.
- Matching carrier-side receiver placeholders: 3.30 diameter M4 tap-drill/pilot
  holes at all 16 fastener centers. The final thread form or insert/nut-plate SKU
  remains a drawing-release selection.

Current carrier and dock perimeter mount holes:

- Carrier perimeter mount holes: X/Y corner pattern at +/-327.52, +/-248.96 plus mid-front/mid-rear at X = 0, Y = +/-248.96.
- Carrier perimeter hole diameter: 5.40, labeled as M5 in CAD.
- Dock perimeter holes use the same six-position perimeter pattern relative to dock dimensions.

Remaining drawing-release decisions: screw length/head/retainer SKU,
carrier-side thread/insert/nut-plate selection around the fixed 3.30 mm pilots,
washer or load-spreader details,
thread engagement, and the measured torque-to-hard-stop value. The documented
paired cross-pattern sequence and compression witness govern assembly; torque
alone is not a gasket-compression control.

## Gasket Interface

Current CAD baseline:

- Per-slot raised gasket land: Rev C chip length/width plus 18.00 mm.
- Per-slot gasket land outer: 145.76 x 103.48.
- Per-slot gasket land inner opening: 129.76 x 87.48; this passes the
  127.76 x 85.48 nominal chip with 1.00 mm clearance per side.
- Per-slot gasket land wall width: 8.00.
- Per-slot gasket land height: 7.35 above carrier top; the land face meets the
  lid underside at closure.
- Adjacent per-slot land separation: 6.00 in X and Y; the sixteen land and
  groove loops are independent and do not overlap.
- Perimeter gasket land outer: 629.04 x 459.92.
- Perimeter gasket land inner: 605.04 x 435.92.
- Perimeter gasket land wall width: 12.00.
- Perimeter gasket land height: 7.35 above carrier top; it shares the per-slot
  land and chip-top closure plane.
- Internal hard stops: nine 4.00 diameter cylindrical stops at the 3 x 3
  inter-slot gap intersections, X = -151.76, 0.00, 151.76 and Y = -109.48,
  0.00, 109.48. They replace the obsolete per-slot corner-stop pattern and do
  not intersect a seal.
- Perimeter hard-stop radial width: 4.00, centered in the 5.00 seal-to-gutter
  web to preserve 0.50 mm clearance from both functional zones.
- Gasket witness coupon exists as a separate first-article output.

First-article seal assumptions resolved by A3/A4:

- Replaceable continuous axial face seals; no adhesive-only or RTV baseline.
- 2.40 mm nominal round cord/O-ring cross-section.
- Lid-side grooves: 1.80 mm deep x 3.20 mm wide with at least a 0.20 mm entry
  break and vendor-reviewed corner radius. At closure, each carrier land meets
  the lid underside and the 1.80 mm groove cavity sets 25% nominal compression
  of the 2.40 mm gasket.
- The lid is constructed as a continuous 4.00 mm underside sealing skin plus a
  6.00 mm upper frame. The upper 635.04 x 457.92 mm lightening relief does not
  penetrate the sealing skin. The groove floor is therefore 2.20 mm thick.
- Adjacent per-slot groove loops retain 10.80 mm solid gaps in both axes. The
  per-slot groove array retains 8.80 mm to the perimeter groove, and the
  perimeter groove retains 48.40 mm X / 54.40 mm Y to the lid edge.
- Platinum-cured silicone, 50-60 Shore A, is the first quote candidate;
  high-purity EPDM, 60-70 Shore A, is the alternate.
- Hard compression stops and a representative witness coupon are mandatory.
- Liquid/no-cell seal lands target Ra 0.8 um; Ra 1.6 um is the absolute
  provisional maximum pending gasket-vendor approval and successful no-cell
  evidence. A future gas/vacuum claim requires a separately justified finish.

Gasket compression assumption from the gasket-install station:

- Nominal gasket thickness: 2.40.
- Target squeeze: 25%.
- Guard band: 20-30%.
- Witness steps: 20%, 25%, and 30% references.

The A0 contract uses one common 7.35 mm land/stop height and one 7.35 mm lid
underside closure plane for per-slot and perimeter loops. The gasket compresses
inside the 1.80 mm lid groove rather than spanning a gap above a shorter land.
The prior 3.00/4.00 mm split, the interim shorter-land stack, and per-slot
corner stops are rejected.

Remaining release decisions: supplier/compound/lot, actual cross-section
tolerance, splice or molded-loop strategy, coating/masking/post-machining plan,
final torque-to-stop value, and vendor-confirmed groove fill. Do not claim
sterility or leak release from CAD alone.

## Imaging And Window Keepouts

Current CAD baseline:

- Carrier optical window cut per slot: 103.76 x 61.48.
- Lid view opening per slot: 113.76 x 71.48.
- Window witness frame per slot: 109.76 x 67.48 with 3.00 wall.
- Window placeholder base panel: 667.04 x 489.92 x 3.00; generated witness
  features extend the overall Z to 4.80.
- Witness frames expose 1.20 mm and retention tabs expose 1.40 mm above the
  panel; both use a 0.20 mm modeling overlap so every raised feature is fused to
  the panel rather than merely tangent.
- Per-slot controlling optical keepout: a 103.76 x 61.48 vertical prism centered
  on each slot, normal to datum A, from the carrier bottom through the window
  panel. The current contract does not authorize angled imaging through clamp,
  tube, or label zones.
- Calibration fiducial centers on the window coordinate system:
  top-left (-315.52, 228.96), top-right (315.52, 228.96), and bottom-left
  (-315.52, -228.96). Starting position tolerance is +/-0.10 mm relative to
  cassette datums A/B/C, pending imaging calibration evidence.
- Each calibration ring has 1.80 mm exposed height and a 0.20 mm modeling overlap
  into the 3.00 mm panel; floating fiducial solids are prohibited.

Downstream agents must preserve line-of-sight through the chip chamber/well region and keep clamp bars, gasket lands, labels, tubing, and fasteners out of the imaging keepout unless the integrator changes the optical strategy.

## Leak, Drain, And Condensate Interfaces

Carrier:

- Perimeter leak gutter: 651.04 x 481.92 outer, 639.04 x 469.92 inner,
  6.00 wide x 3.00 deep. A 5.00 mm minimum solid web separates it from the
  perimeter gasket land.
- Carrier drain port: 8.00 diameter x 40.00 long side-drilled feature with
  centerline X = 315.52, Y = -254.96, Z = 10.00, oriented along Y. It crosses
  the front carrier wall and reaches the front gutter band.
- Side service relief lands: left/right tubing/service relief features are
  7.00 mm high above the carrier top, below the 7.35 mm closure plane. Each land
  is notched around every datum with an 11.00 mm radial keepout, and datum bores
  are subtracted again after all carrier unions so service geometry cannot refill
  D1-D4.

Dock:

| Interface | Locked A0 geometry |
| --- | --- |
| Support deck | 869.04 x 691.92 x 22.00 base body, local Z = -11.00 to +11.00; the top face at local Z = +11.00 is datum A. |
| Carrier recesses | Sixteen 137.76 x 95.48 top recesses at the slot centers, 5.50 deep, with exact floors at local Z = +5.50. |
| Datum rails / retention lip | Rear rail 743.04 x 16.00 x 18.00 centered at (0.00, 278.96) mm; left rail 16.00 x 587.92 x 18.00 centered at (-357.52, 0.00) mm; front lip 743.04 x 10.00 x 10.00 centered at (0.00, -275.96) mm. These are additive above datum A. |
| Air bypass openings | Five 657.04 x 8.00 full-thickness cuts centered at Y = -218.96, -109.48, 0.00, 109.48, and 218.96 mm. |
| Front drain opening | 799.04 x 10.00 full-thickness cut centered at (0.00, -307.96) mm. |
| Right drain opening | 10.00 x 615.92 full-thickness cut centered at (392.52, 0.00) mm. |
| Drain-visibility opening | 58.00 x 38.00 full-thickness cut centered at (376.52, -287.96) mm. |
| Position-token lands | Sixteen 24.00 x 10.00 x 3.00 additive lands. Each center is `(slot X, slot Y - 58.74)` mm. |
| Logger reservation lands | Four solid 48.00 x 32.00 x 8.00 additive lands centered at X = +/-383.52 mm and Y = +/-240.96 mm. They are not pockets or recesses. |
| Robot-lift lands | Two 160.00 x 20.00 x 7.00 additive lands centered at (0.00, +/-271.96) mm. |
| Leveling-pad lands | Four 32.00 mm-diameter x 3.00 additive lands centered at X = +/-392.52 mm and Y = +/-303.96 mm. |
| Perimeter mounts | Six 5.40 mm-diameter full-thickness holes at X = +/-412.52 mm, Y = +/-323.96 mm plus X = 0.00 mm, Y = +/-323.96 mm. |

The front, right, visibility, air-bypass, and mount cutters use a 24.00 mm
through-cut height centered at local Z = 0, providing 1.00 mm overtravel beyond
each face of the 22.00 mm base deck. Their XY locations are locked and must not
be silently moved to avoid later additive features.

The A0 generators subtract these openings from the base deck before unioning
top-side lands. The result is deliberately segmented/bridged fit-check geometry:
the front lip bridges part of the drain-visibility opening; the two right-side
logger reservation lands bridge the right drain; the front-left leveling land
bridges the front drain; the front-right leveling land bridges the front, right,
and visibility intersection; and the rear-right leveling land bridges the right
drain. At those overlaps, the base-body section below datum A remains void while
the additive land above datum A is solid. Clear spans of every named opening
remain through-deck. This topology is not evidence of gravity flow, condensate
capacity, cleanability, or drain performance.

D5 must dimension the unbridged clear spans and show sections through every
bridge. No-cell validation must demonstrate collection and drainage with the
as-modeled topology. Moving an opening or removing a bridge requires an explicit
A0 revision; a vendor or downstream agent may not reinterpret these fit-check
features as continuous gutters or a completed sump.

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

- Sixteen 26.00 x 10.00 per-slot label lands form a dry rear-margin row at
  Y = 255.96, with 36.00 mm X pitch in S01-S16 order. They do not overlap chip
  pockets, optical keepouts, gasket lands, or the leak gutter.
- Global machine-readable-code land: 96.00 x 12.00 in the front-left dry carrier
  margin, centered at X = -271.52, Y = -263.96.
- Adjacent human-readable land: 118.00 x 10.00, centered at X = -159.52,
  Y = -263.96.
- Both front label lands clear the leak gutter and the front fastener/receiver
  row at Y = -247.96.
- A front-left orientation marker ties slot 1 to datums B/C.
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

Bulkhead port coordinates are local to the 789.04 x 34.00 x 76.00 base body:
origin at the bulkhead body center, X along the long edge, Y through the panel,
and Z upward. The cassette-facing interface plane is local Y = -17.00 with
outward normal -Y. G0-G3 are centered at Z = 18.00; M0-M6 and W0-W4 are
centered at Z = 0.00. The A6 X coordinate/role tables are normative.

Fluid-path agent constraints:

- One AAV condition per cassette.
- Do not route multiple AAV candidates through a single cassette.
- Preserve prime path, waste path, bubble visibility, and dead-volume measurability.
- Use bought sterile connectors/tubing for first article; do not make printed or machined structural parts the validated wetted path.

The A6 logical G0-G3, M0-M6, and W0-W4 roles are the active port map. Remaining
release decisions are bought connector family/SKUs, final panel cutouts, tube
OD/ID and bend radius, valve/pump interface, and sterile connection method.

A6 handoff: `docs/sixteen_slot_cassette_a6_disposable_fluid_path_spec.md` resolves the first-article fluid path as a disposable, one-condition, pressure-limited single-pass harness with bought sterile connector/tube-welding assumptions. It assigns logical G0-G3, M0-M6, and W0-W4 port roles while keeping final connector SKUs, tubing OD/ID, pump type, and vendor connector cutouts open for RFQ/validation.

A7 handoff: `docs/sixteen_slot_cassette_a7_no_cell_validation_fixture_spec.md` defines the no-cell bench gate for the 16-slot first article: fit/datum, compression witness, harness topology, prime/debubble, leak/pressure decay, flow balance, restriction/occlusion detection, dye recovery, waste/backflow, and repeat assembly tests. Older 4 x 5 / 20-position validation fixtures are reference concepts only until ported to this slot map and A6 port plan.

A8 handoff: `docs/sixteen_slot_cassette_a8_first_article_build_package.md` consolidates A0-A7 into the first-article build package. It records the verified seven-STL internal fit-check export, states that vendor RFQ still requires STEP files and 2D drawings, defines custom and bought-component package assumptions, sets assembly/QA order, and lists blockers before media-only, live-cell, or AAV work.

A9 handoff: `docs/sixteen_slot_cassette_a9_step_drawing_export_plan.md` defines the STEP and drawing release plan required before vendor RFQ. It selects true STEP/B-rep export as the intended path, rejects STL-only or silently skipped conversion for release, defines D0-D9 drawing sheets, and lists critical inspection tables and stackup review requirements.

## Reserved Sensor And Module Interfaces

Current first-article service/dock reserves:

- Sensor/backplane connector cut: 88.00 x 18.00 at local X = 320.00,
  Z = 18.00, through the bulkhead Y thickness. Moving it from X = 300.00 gives
  positive clearance from W4.
- Cable strain-relief shelf and zip-slot on the service bulkhead.
- Four solid dock logger reservation lands: 48.00 x 32.00 x 8.00.
- Dock leveling pads at all four dock corners.
- Dock robot lift lands at front and rear.

Sensor/readout agent constraints:

- Keep electrical/readout interfaces dry.
- Support per-slot or per-cassette measurements without compromising the disposable fluid path.
- Reserve TEER/impedance, temperature/RH logging, imaging fiducials, and run identity capture.

The connector aperture and its 120.00 x 26.00 cable shelf reserve are dry
mechanical keepouts, not a selected connector. The final connector must declare
engagement depth, plug/removal sweep, cable bend volume, pinout/power/data
limits, and ingress/cleaning strategy within this reserved zone. Any connector
that exceeds it requires an A0 revision.

Open issue for sensor/module agents: final connector family, cable routing, strain relief, logger model, TEER/impedance electrode interface, and incubation-module quick-connect are not frozen.

## Downstream Agent Contracts

| Agent | May change | Must not change without integrator approval |
| --- | --- | --- |
| A1 Carrier | Pocket detail, datum implementation, drain/gutter details, label surfaces, handling lands | 4 x 4 slot count, slot centers, one-condition semantics, dry structural policy |
| A2 Lid/Clamp | Clamp bar section, window framing, final fastener hardware/receiver SKU, captive screw details | Imaging keepouts, carrier datum relationship, locked fastener centers/pilots, groove geometry, or stop layout without an A0 revision |
| A3 Gasket | Compound/vendor/tolerance recommendation, groove-fill review, finish, and witness coupon details | Locked 2.40 mm cross-section, 1.80 x 3.20 mm lid groove, land/stop geometry, 20-30% guard band, or nominal 25% target without an A0 revision |
| A4 DFM | Material/process/tolerance recommendations, radii, surface finishes, drawing notes | Disposable wetted-path policy and 16-slot first-build baseline |
| A6 Fluid Path | Port assignment, tubing route, connector recommendation, prime/waste/bubble plan | One AAV condition per cassette and bought disposable wetted path for first article |
| A7 No-Cell Validation | Leak/flow/bubble/dead-volume fixture details and acceptance evidence plan | Shared slot map, datums, and condition ID linkage |
| A8 Build Package | Output manifest, BOM assumptions, assembly/QA checklist, RFQ readiness blockers | No biological claims, no STL-only vendor release, no mixed-AAV routing |
| A9 STEP/Drawings | STEP export plan, drawing sheet list, inspection tables, stackup review plan | No mesh-derived release geometry, no silent export fallback, no RFQ without drawings |

## Downstream Resolution Register

The A1-A9 work refined A0 without changing its locked slot map or experimental
semantics:

| Interface | Current first-article baseline | Still blocks release |
| --- | --- | --- |
| Carrier | 6061-T651/T6 dry structural plate; 1.20 mm/side CAD fit-check clearance; provisional drawing target is measured chip maximum plus 0.80 mm/side. | Measure the actual Rev C chip lot, freeze the pocket drawing size, and finalize anodize/masking and inspection. |
| Datums | A/B/C rear-left 3-2-1 registration; 6 mm dowel family; round D1 and relieved/slotted D2. | Freeze bore tolerances and post-finish metrology. |
| Lid/clamp | Sixteen captive M4 x 0.7 screws, mechanical window retention, paired cross-pattern tightening. | Select hardware/receiver SKUs and validate torque-to-stop plus lid deflection. |
| Gasket | 2.40 mm axial face-seal baseline, 25% target squeeze, 20-30% guard band, locked 1.80 x 3.20 mm lid groove, and 7.35 mm land/stop closure plane. | Resolve supplier/compound/tolerance, finish/coating, groove fill, and no-cell evidence. |
| Fluid path | Disposable one-condition single-pass harness with A6 G/M/W logical port map. | Select connector/tubing/pump SKUs and replace placeholder bulkhead cuts with vendor geometry. |
| Validation | A7 defines fit, compression, prime, bubble, leak, flow, restriction, recovery, waste, and repeat-assembly gates. | Build/source the fixture and set final thresholds below the weakest selected component limits. |
| Manufacturing | A9 selects true B-rep STEP plus controlled 2D drawings; STL is internal fit-check only. | Complete and review the D0-D9 drawing/RFQ package, including the gasket stack section. |
| Sensors/module | Dry connector reserve, solid logger reservation lands, fiducials, lift lands, and leveling pads remain allocated. | Select sensor/logger connectors and any later incubation-module quick-connect. |

## A0 Open Decisions

These release decisions remain deliberately open:

1. Final manufacturer pocket size after Rev C chip-lot measurement.
2. Carrier/lid receiver hardware, bore tolerances, final torque-to-stop, and
   clamp-deflection acceptance.
3. Gasket supplier/compound/tolerance, seal-finish/coating plan,
   vendor-confirmed groove fill, and no-cell evidence. The land-height and
   nominal groove stack are fixed.
4. Final imaging-window material and optical/cleaning acceptance.
5. Bought connector, tubing, pump/pressure-control, and panel-cutout selections.
6. Sensor connector/logger and later incubation-module quick-connect selections.
7. Final no-cell thresholds tied to selected hardware and the weakest-component
   pressure limit.
8. Reviewed 2D drawing, inspection, stackup, and RFQ release package.

## A0 Requirement Traceability

| Ticket requirement | Controlling section | Verification source |
| --- | --- | --- |
| Shared dimensions and coordinate origin | Coordinate System; Core Dimensions; Slot Centers | `sixteen_slot_cassette_incubator_first_article` constants and layout assertions. |
| Chip pocket assumptions | Chip Pocket Interface | A1 carrier split between CAD fit-check and measured-lot drawing target. |
| Datum scheme | Datum Scheme | A1 A/B/C scheme and active CAD rail/boss geometry. |
| Fastener assumptions | Fastener Pattern | A2 captive M4 baseline and active 16-point CAD pattern. |
| Gasket assumptions | Gasket Interface | A3/A4 gland, material, finish, stop, and validation assumptions. |
| Imaging keepouts | Imaging And Window Keepouts | Carrier/lid/window generated geometry and 16-slot line-of-sight rule. |
| Leak/drain zones | Leak, Drain, And Condensate Interfaces | Carrier gutter/drain plus the dock's locked segmented through-deck openings and additive bridges. |
| Barcode/condition ID | Barcode And Condition ID | Global and per-slot CAD lands plus slot-1 orientation marker. |
| Fluid/sensor/module reserves | Reserved Fluid Path Interfaces; Reserved Sensor And Module Interfaces | A6 port map, service bulkhead reserve, solid logger reservation lands, lift lands, and leveling pads. |

## Machine Verification

`src/sixteen_slot_cassette_a0.rs` is the shared machine-readable interface
contract consumed by both active STL and STEP generators. Both lid builders must
construct the continuous seal skin and upper frame, subtract view/chip reliefs,
add retained features, cut D1/D2 replaceable-pin seats, and subtract the 1.80 x
3.20 mm gasket grooves last. Use the LaminarForge MCP runner for the STL
generator and then `sixteen_slot_cassette_a0_verify`; use the same runner with
the `step` feature for `sixteen_slot_cassette_first_article_step`.

The STL verifier fails on an invalid contract or manifest, malformed mesh,
non-finite geometry, envelope drift over 0.05 mm, disconnected window geometry,
or failed probes for exact carrier/dock floors, carrier datum bores, lid view and
chip reliefs, groove cavities/floors/shoulders, D1/D2 pin seats, D3/D4 no-pin
policy, window-fiducial attachment, and assembly-only pin surrogates. It prints
triangle count, envelope, and SHA-256 for the evidence record. STEP evidence
must additionally re-import all seven files in a separate OpenCascade verifier
pass and confirm one closed solid for each standalone part; existence alone is
not geometry verification. STEP hashes identify a run because headers contain
timestamps and are not stable geometry baselines.

## Required Handoff From A0

The cassette integrator should not accept downstream subsystem work unless it references this spec and states which A0 open decisions it resolves. Any change to slot count, slot centers, datum scheme, one-condition semantics, or disposable wetted-path policy requires an explicit revision to this document.
