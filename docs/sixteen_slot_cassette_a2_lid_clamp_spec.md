# 16-Slot Cassette A2 Lid/Clamp Subsystem Spec

Ticket: T-2E65D751

This A2 spec defines the reusable lid/clamp subsystem for the first physical
LaminarForge 16-slot AAV cassette. It is subordinate to
`docs/sixteen_slot_cassette_a0_interface_spec.md` and uses
`docs/cassette_incubator_manufacturing_readiness.md` for manufacturing context.

This document does not change the lower carrier, gasket, disposable fluid path,
incubator dock, or final CAD generator. It defines the lid/clamp interfaces
those agents must be able to mate with.

This is not a biological protocol, sterile-barrier claim, AAV-containment
claim, live-cell release criterion, or final vendor drawing. One cassette
remains one AAV capsid/promoter/payload/dose/timing/media condition; the lid
does not create per-slot candidate isolation.

## Controlling Inputs

| Input | A2 implication |
| --- | --- |
| First-build format | Preserve 16 slots in a 4 x 4 grid. |
| One-condition semantics | Lid markings and access must not imply per-slot AAV candidate mixing. |
| Active CAD baseline | `sixteen_slot_cassette_incubator_first_article`. |
| Carrier base body / true overall | 699.04 x 541.92 x 24.00 mm base; 699.04 x 541.92 x 31.35 mm true overall. |
| Lid/clamp base body / true overall | 717.04 x 559.92 x 10.00 mm base; 717.04 x 559.92 x 11.60 mm true overall. |
| Lid structural split | 4.00 mm continuous underside sealing skin plus 6.00 mm relieved upper frame; 635.04 x 457.92 mm center relief is upper-frame-only. |
| Window placeholder base panel / true overall | 667.04 x 489.92 x 3.00 mm base; 667.04 x 489.92 x 4.80 mm true overall. |
| Slot pitch and centers | 151.76 mm X and 109.48 mm Y; X = +/-227.64, +/-75.88 mm and Y = +/-164.22, +/-54.74 mm. |
| Lid view opening per slot | 113.76 x 71.48 mm. |
| Carrier optical through-window per slot | 103.76 x 61.48 mm. |
| Gasket squeeze assumption | 2.40 mm nominal gasket in 1.80 mm-deep x 3.20 mm-wide lid grooves, 25% target squeeze, 20-30% guard band; carrier lands and stops meet the lid underside at 7.35 mm above carrier top. |
| Fastener baseline | Sixteen lid/clamp fasteners outside the gutter, with 4.80 mm lid clearance holes and matching 3.30 mm carrier pilot receivers. |
| Lid-to-carrier alignment baseline | Four lid alignment ears over the carrier bosses; replaceable D1/D2 pins only, with D3/D4 as clearance/witness positions. |

## Subsystem Boundaries

A2 owns:

- Reusable lid/clamp frame geometry intent.
- Imaging window retention and lid-side keepouts.
- Lid fastener family, captive screw strategy, and tightening sequence.
- Lid-to-carrier registration requirements.
- Clamp stiffness, clearance, and compression-uniformity requirements.

A2 does not own:

- Chip pocket dimensions or carrier datum bore tolerances.
- Final gasket compound/vendor tolerance, groove-fill evidence, or leak-test pressure. The nominal 1.80 x 3.20 mm lid grooves are locked by A0.
- Disposable tubing, connectors, port assignments, or sterile boundary claims.
- Dock/rack geometry except for clearance around the assembled lid.
- Final vendor drawing tolerances outside lid/clamp-specific notes.

## Targeted Exa Research Pass

The research pass was limited to clamp force distribution, imaging window
retention, and captive fastener strategy.

| Topic | Source | A2 finding | Design decision |
| --- | --- | --- | --- |
| Clamp force distribution | Fluid Sealing Association, `GASKET HANDBOOK`, and ASME PCC-1 bolt torque guidance summaries | Gasket joints depend on controlled seating stress, bolt preload, flange/lid stiffness, and tightening sequence. Point loads without adequate load spreading can over-compress near fasteners and under-compress between them. | Keep the 16-fastener baseline, require a documented cross-pattern torque sequence, and treat lid flatness/deflection as a release requirement before live-cell use. |
| Clamp force distribution | Amorim gasket design guidance and Henkel sealing guidelines | Seal performance is system-level: gasket material, gland geometry, surface finish, fastener preload, and part stiffness interact. | A2 resolves the mechanical clamp architecture only; exact torque value waits for A3 gasket material/cross-section and A4 vendor DFM. |
| Imaging window retention | University of Arizona optomechanical window design tutorial | Optical windows should be retained by mechanical seats/retainers that avoid asymmetric point loads and protect optical surfaces. | Use a mechanically retained window panel or per-slot retained inserts with perimeter support. Adhesive may be a secondary anti-rattle or environmental seal only. |
| Imaging window retention | Low-retaining-force optical viewport seal reference | Window seals can be designed around low, controlled retaining force to avoid window distortion. | Retention must not bow the 3 mm window panel into the imaging path or put screw heads inside view openings. |
| Captive fasteners | PEM captive panel screw engineering guide | Captive panel screws are intended for repeated access panels and keep hardware retained in the removable panel. | First article assumes captive M4 panel screws retained in the lid/clamp frame. |
| Captive fasteners | Southco captive screw product family | Captive screw products support repeated service access with configurable head style, spring loading, and retainer options. | Use bought captive hardware for the first article rather than custom printed or improvised retention. |

Research URLs checked:

- https://www.fluidsealing.com/wp-content/uploads/FSA-Gasket-Handbook-June2017Rev2025v2.pdf
- https://engineersedge.com/calculators/bolt_torque_for_gasket_15778.htm
- https://amorimcorksolutions.com/media/7347/tb-gasket-design-guidelines_2021_v1.pdf
- https://dm.henkel-dam.com/is/content/henkel/brochure-worldwide-sealing-guidelines-cured-in-place-gaskets-print-en
- https://wp.optics.arizona.edu/optomech/wp-content/uploads/sites/53/2016/10/WillisteinTutorial1.pdf
- https://pubmed.ncbi.nlm.nih.gov/17477698/
- https://www.pemnet.com/products/engineering-guides/captive-panel-screws/
- https://southco.com/en_us_int/fasteners/captive-screws

## Lid Frame Architecture

The lid/clamp is a reusable dry structural part. It clamps the gasketed chip
stack and provides imaging access without becoming a validated wetted surface.

Required first-article architecture:

- Lid base body: 717.04 x 559.92 x 10.00 mm; true overall bounds are 717.04 x 559.92 x 11.60 mm including raised retention detail.
- Lid perimeter overhang relative to carrier: 9.00 mm per side.
- Outer frame: continuous perimeter ring, machined or waterjet rough-cut plus
  secondary-machined datum/fastener/window features.
- Underside seal skin: retain a continuous 4.00 mm plate across the lid before
  view openings, local chip reliefs, grooves, fastener holes, and D1/D2 pin seats
  are cut. The 1.80 mm grooves leave a 2.20 mm minimum floor.
- Inner relief: the 635.04 x 457.92 mm lightened center is removed only from the
  6.00 mm upper frame. It must not cut through or weaken the underside seal skin.
- Crossbars: retain a 3 x 3 internal crossbar network between slot rows and
  columns, represented as 10.00 mm wide bars through the 6.00 mm upper frame.
- Crossbar purpose: distribute clamp load between the perimeter and per-slot
  gasket lands while staying outside the lid view openings.
- Labeling: lid may carry cassette orientation marks and torque sequence marks,
  but condition identity remains controlled by the cassette ID and A0 barcode
  fields.

Recommended first-article material direction:

- 6061-T6 aluminum, clear anodized, is the baseline quote material for A2
  unless A4 selects stainless or autoclavable engineering plastic for the full
  cassette package.
- Stainless 304/316 is acceptable if vendor review finds aluminum distortion,
  cleaning compatibility, or thread durability unacceptable.
- Polycarbonate or other plastic is not the preferred lid/clamp baseline unless
  stiffness and creep under gasket preload are explicitly verified.

## Imaging Windows

The lid/clamp must preserve line-of-sight through all 16 chip chamber regions.

Required openings and keepouts:

| Feature | Requirement |
| --- | --- |
| Per-slot lid view opening | 113.76 x 71.48 mm, centered on each A0 slot center. |
| Per-slot chip-top relief | 130.16 x 87.88 x 0.50 mm deep from the underside; 2.20 mm material remains to the gasket-groove inner edge in X and Y. |
| Per-slot carrier optical cut reference | 103.76 x 61.48 mm, centered on each A0 slot center. |
| Window witness frame reference | 109.76 x 67.48 mm with 3.00 mm wall and 1.20 mm exposed height; frames overlap the panel by 0.20 mm. |
| Window panel baseline | 667.04 x 489.92 x 3.00 mm base panel; 667.04 x 489.92 x 4.80 mm true overall with witness/retention features. |
| Fiducials | Preserve three calibration fiducials outside the slot array at the A0 top-left, top-right, and bottom-left reference positions. Rings expose 1.80 mm above the panel and overlap the panel by 0.20 mm; floating rings are rejected. |
| Retention tabs | Tabs expose 1.40 mm above the panel and overlap it by 0.20 mm; every raised window feature must form one connected manufacturing part. |

Window retention decision:

- Resolve A2 baseline as a mechanically retained window, not adhesive-only.
- Use either one full-panel removable window retained by the lid frame or 16
  per-slot window inserts retained by local shoulders. The first quote package
  should prefer one full-panel placeholder because the current CAD baseline
  already represents it and it simplifies early fit-check.
- Retention features must remain outside the lid view openings and outside the
  carrier optical through-windows.
- The current STL and STEP drafts include a continuous lid retention lip and
  panel retention tabs. D2/D3 drawings must convert those draft features into
  selected-material seats, clearances, edge protection, and bought retention
  hardware detail; the window remains a non-release interface placeholder.
- Adhesive, tape, or sealant may be used only as a secondary anti-rattle,
  dust, or condensation-control aid after material compatibility review.
- No screw head, washer, clip, or retainer may protrude into an imaging opening
  or shadow the chamber/well region under the intended camera angle.

Window material assumption:

- Baseline: optically clear polycarbonate sheet for mechanical fit-check and
  non-release imaging trials.
- Escalate to borosilicate/glass if autofluorescence, chemical compatibility,
  scratch resistance, or imaging distortion makes polycarbonate unacceptable.
- Do not claim optical assay compatibility until the A7 validation plan tests
  window material, thickness, cleaning, condensation, and imaging signal.

## Fastener Layout

The A0 16-fastener baseline remains locked for A2 first article.

| Pattern segment | Coordinates |
| --- | --- |
| Left/right side fasteners | X = +/-332.52 at Y = -164.22, -54.74, 54.74, 164.22 mm. |
| Front/rear fasteners | Y = +/-247.96 at X = -227.64, -75.88, 75.88, 227.64 mm. |
| Count | 16 |
| Lid clearance | 4.80 mm current M4 clearance in CAD |
| Carrier receiver placeholders | 3.30 mm M4 tap-drill/pilot holes at all 16 matching centers; final thread or insert SKU remains open. |

Resolved A2 fastener family assumption:

- Use stainless M4 x 0.7 captive panel screws in the lid/clamp frame.
- Use low-profile socket, Torx, or similar positive-drive heads compatible
  with a torque driver.
- Use captive screw retainers, flared retainers, or commercial captive panel
  screw assemblies so lid hardware remains attached during removal.
- Use flat washers or integrated load-spreading captive heads only where they
  do not enter imaging or gasket keepouts.
- Use carrier-side stainless threaded inserts, installed nut plates, or
  replaceable threaded receiver hardware for first article durability. Final
  receiver type is an A1/A4 detail, but the lid interface assumes M4 female
  threads on the carrier side and preserves the locked 3.30 mm pilot geometry.

Torque sequence requirement:

1. Seat all 16 screws finger-tight with the lid on carrier datums.
2. Tighten in a paired cross-pattern that alternates opposite sides and avoids
   fully seating one edge before the opposite edge.
3. Use at least two torque passes: low seating torque, then final torque.
4. Repeat the final pass once after the gasket has settled for the A3-defined
   wait interval.
5. Record torque driver setting, screw lot, gasket lot, and witness coupon
   result for each no-cell validation run.

Recommended first-article paired sequence:

| Pair | First point | Opposite point |
| ---: | --- | --- |
| 1 | X = -332.52, Y = -54.74 | X = 332.52, Y = 54.74 |
| 2 | X = 332.52, Y = -54.74 | X = -332.52, Y = 54.74 |
| 3 | X = -75.88, Y = -247.96 | X = 75.88, Y = 247.96 |
| 4 | X = 75.88, Y = -247.96 | X = -75.88, Y = 247.96 |
| 5 | X = -332.52, Y = -164.22 | X = 332.52, Y = 164.22 |
| 6 | X = 332.52, Y = -164.22 | X = -332.52, Y = 164.22 |
| 7 | X = -227.64, Y = -247.96 | X = 227.64, Y = 247.96 |
| 8 | X = 227.64, Y = -247.96 | X = -227.64, Y = 247.96 |

Exact torque is not resolved by A2 because it depends on gasket material and
tolerance, thread receiver type, lubrication, surface finish, and measured lid
stiffness. A2 requires torque to be selected by A3/A4/A7 testing so every stop
is seated without driving gasket compression outside the 20-30% guard band.

## Lid-To-Carrier Alignment

The lid must register to the carrier before clamp load is applied.

Required alignment baseline:

- Preserve the four A0 carrier datum boss locations:

| Datum | X | Y |
| --- | ---: | ---: |
| D1 front-left, round functional locator | -335.52 | -204.96 |
| D2 front-right, relieved functional locator | 335.52 | -204.96 |
| D3 rear-left, clearance/witness | -335.52 | 204.96 |
| D4 rear-right, clearance/witness | 335.52 | 204.96 |

- Lid alignment ears remain centered over those four datum locations, but only
  D1 and D2 establish functional location. The lid manufacturing part contains
  6.00 mm-diameter x 4.00 mm-deep seats at D1/D2 for replaceable commercial
  pins. The fit-check surrogate is 5.80 mm diameter x 7.35 mm long, with 4.00 mm
  seated in the lid, 3.35 mm projecting, and 2.00 mm engaging the carrier boss.
  Pin solids appear only in the assembly reference. D3/D4 have neither pins nor
  pin seats and must remain clearance/witness locations.
- Current CAD ear size is 34.00 x 22.00 x 10.00 mm; keep this envelope unless
  A1/A4 changes the datum boss implementation.
- Alignment must be feature-to-feature, not screw-shank registration. Screws
  supply clamp load only.
- Lid datum features must tolerate repeated installation without galling or
  burrs that shift the imaging windows.
- Add orientation asymmetry in the final detail if the carrier datum scheme does
  not already prevent 180-degree lid installation.

Remaining alignment detail for A1/A4:

- Final dowel/boss diameter, hole tolerance, reaming operation, and whether the
  lid uses close-fit holes, slots, bushings, or relieved pockets must be frozen
  with the carrier datum design.

## Clamp Stiffness And Clearance

The lid/clamp must create repeatable gasket compression without bending into
the imaging path or contacting chip features.

Clamp stiffness requirements:

- Maintain gasket compression inside the A0 guard band: 20-30% squeeze around
  the nominal 25% target.
- For the 2.40 mm nominal gasket assumption, this means the compressed gasket
  height target is 1.80 mm with an allowed guard-band height of 1.68-1.92 mm.
- The 7.35 mm-high carrier lands and hard stops meet the flat lid underside at
  closure. Nominal gasket height is set by the 1.80 mm-deep lid groove cavity;
  do not model a 1.80 mm gap above a shorter carrier land.
- Replace all per-slot corner stops with exactly nine 4.00 mm-diameter internal
  stops at the 3 x 3 inter-slot gap intersections. Retain 4.00 mm-wide
  perimeter stops centered in the 5.00 mm seal-to-gutter web. No stop may
  intersect a seal.
- Lid flatness, carrier flatness, gasket thickness variation, and clamp
  deflection must be budgeted together. A2 allocates no more than 0.10 mm of
  local compression-height error to lid/clamp elastic deflection until FEA or
  metrology replaces this planning value.
- Crossbars and perimeter ring must be stiff enough that fastener-adjacent
  compression does not visibly exceed witness targets while mid-span regions
  remain under-compressed.
- Production drawings should require a flatness inspection of the gasket-facing
  lid surface after machining and finishing.

Clearance requirements:

- No lid surface, screw, washer, retainer, or window feature may contact the Rev
  C chip except through the intended gasket/compression stack.
- Preserve the 0.50 mm local chip-top relief over every nominal Rev C chip; do
  not deepen or widen it into the gasket groove without an A0 stackup revision.
- Preserve all 16 lid view openings and carrier optical window cuts.
- Keep clamp bars, window retainers, screw heads, washers, labels, and captive
  screw hardware out of the A0 imaging keepouts.
- Keep service-side clearance for future tubing/bulkhead work. Do not add lid
  overhangs or latch features into the reserved side service relief areas
  without integrator approval; those carrier reliefs are 7.00 mm high and stay
  below the 7.35 mm closure plane.
- The lid should be removable vertically after screws are released; do not
  require sliding motion that could smear gasket surfaces or disturb chips.

Recommended verification before live-cell/AAV use:

- Dry fit with chip surrogates or gauge blocks.
- Blueing/pressure film or witness shim check across all 16 gasket regions.
- Dial indicator or CMM flatness check of lid gasket-facing surface.
- Repeat assembly/disassembly cycle check for captive hardware retention.
- Imaging test with the lid installed, window retained, and fiducials visible.

## Captive Fastener Strategy

Resolved A2 assumption:

- The removable lid/clamp carries all 16 screws captive.
- Captive hardware is commercially sourced rather than custom-machined for the
  first article.
- Screws are stainless and compatible with humid 37 C incubation and cleaning
  assumptions selected by A4.
- Thread engagement into carrier receivers must be long enough for repeated
  torque cycles without damaging the carrier.
- Hardware must be removable/rebuildable in the lab if a screw, retainer, or
  insert is damaged.

Preferred first-article implementation:

- Commercial M4 captive panel screws in counterbored or spotfaced lid holes.
- Carrier-side stainless threaded inserts or replaceable nut plates.
- Torque-driver-compatible head style.
- Captive retainers below the screw head or within the panel bore, outside all
  imaging and gasket keepouts.

Rejected for first article:

- Loose screws that can be dropped into the cassette, incubator dock, or lab
  workspace.
- Adhesive-only screw retention.
- Self-tapping screws into the reusable carrier.
- Printed plastic threads as the primary clamp receiver.

## Open Decisions Resolved By A2

| Decision | A2 resolution |
| --- | --- |
| Preserve or replace the 16-fastener baseline | Preserve the 16-fastener baseline. |
| Lid fastener family | M4 x 0.7 stainless captive panel screws for first article. |
| Screw role in alignment | Screws provide clamp load only; datums align the lid. |
| Captive hardware | Required on the removable lid. Use commercial captive hardware. |
| Window retention | Mechanical retention required; adhesive-only is rejected. |
| Window baseline for first quote | Use one retained 667.04 x 489.92 x 3.00 mm base panel, 4.80 mm true overall, unless imaging tests require per-slot inserts or glass. |
| Lid construction | One machined part with a 4.00 mm continuous underside seal skin and 6.00 mm relieved upper frame; no through-lightening cut beneath gasket grooves. |
| Alignment pins | Replaceable D1/D2 commercial-pin direction; no integral lid pins and no D3/D4 pin features. |
| Clamp verification | Require witness/pressure-film/metrology evidence before live-cell/AAV use. |

## Decisions Escalated

| Decision | Owner | Reason |
| --- | --- | --- |
| Final gasket compound/vendor tolerance, groove-fill evidence, and exact torque value | A3 with A4/A7 input | The nominal 2.40 mm gasket, 1.80 x 3.20 mm lid groove, and hard-stop geometry are fixed; torque and clamp preload still require the selected material and hardware. |
| Carrier-side threaded receiver type | A1/A4 | Receiver choice affects carrier machining, inserts, serviceability, and durability. |
| Final lid material and finish | A4 | Depends on vendor capability, cleaning chemistry, flatness, galling, and cost. |
| Final datum bore/pin tolerance and lid alignment feature detail | A1/A4 | Must be frozen with carrier datum implementation and inspection method. |
| Window material for assay imaging | A7 with A4 input | Requires optical, cleaning, condensation, and biological validation evidence. |
| FEA or analytical clamp-deflection release limit | A4/A7, using A2 geometry | A2 sets the 0.10 mm planning allocation; release requires analysis or metrology. |
| Drawing release | Integrator/A4/A9 | The STL and true B-rep STEP drafts now consume the shared A0 contract; D0-D9 drawings, hardware selections, and release inspection remain open. |

## Handoff Requirements

Before manufacturer drawings are released, the integrated cassette package must:

- Preserve the shared A0 correction that gives all sixteen seal loops and the
  perimeter loop a common 7.35 mm land elevation, 6.00 mm inter-land clearance,
  and a lid underside closure plane 7.35 mm above the carrier top. The 1.80 x
  3.20 mm grooves are cut into the lid underside; the earlier shorter-land gap
  stack is obsolete.
- Preserve the nine 4.00 mm-diameter internal stops and the 4.00 mm-wide
  perimeter web stops; do not restore per-slot corner stops or let any stop
  intersect a seal.

- Update production CAD with retained-window details rather than the current
  placeholder-only panel.
- Add lid-side detail for captive screw counterbores, retainers, and hardware
  clearance.
- Preserve all sixteen 3.30 mm carrier-side M4 pilot receivers and confirm the
  final thread/insert SKU and thread engagement.
- Add lid-to-carrier datum detail that does not rely on screw shanks.
- Add drawing notes for lid material, finish, gasket-facing flatness, burr
  removal, edge breaks, and inspection datums.
- Add an assembly note requiring the A2 torque sequence and A3 witness coupon
  result before no-cell leak or imaging validation.
