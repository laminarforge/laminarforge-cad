# 16-Slot Cassette A9 STEP And Drawing Export Plan

Ticket: T-44807EA2

This A9 plan converts the A8 first-article build package into a concrete STEP and drawing release plan for vendor RFQ. It defines the export files, drawing sheets, critical dimensions, inspection tables, stackup drawing, and implementation rules needed to move from internal STL fit-checks to quote-ready manufacturing documents.

This is not a released drawing package, vendor quote packet, biological validation, sterile-barrier claim, AAV containment claim, or live-cell release criterion. It is the plan for producing those manufacturing files without silently treating STL output as production geometry.

## A9 Baseline

| Item | A9 requirement |
| --- | --- |
| First-build format | 16 slots in a 4 x 4 cassette. |
| Experimental unit | One cassette equals one AAV capsid/promoter/payload/dose/timing/media condition. |
| Active fit-check CAD | `src/bin/sixteen_slot_cassette_incubator_first_article.rs`. |
| Current export status | Seven STL fit-check files plus an implemented seven-file true B-rep STEP draft generated from the shared machine-readable A0 contract. The draft groove loops are square-cornered and omit the required entry break, so D2/D4 manufacturing geometry remains blocked. |
| RFQ export target | Implemented STEP drafts plus D0-D9 2D drawings, inspection tables, manifest, and release review. Drawings still block RFQ. |
| Manufacturing definition | Drawings control; STEP supports geometry; STL is not the manufacturing definition. |
| Wetted-path policy | Structural parts remain dry; bought disposable tubing/connectors remain the wetted path. |
| Biology status | No media-only, live-cell, or AAV release from drawing output alone. |

## Current Export Reality

The active STL fit-check and STEP draft generators export matching packages from `src/sixteen_slot_cassette_a0.rs`. Both complete additive lid/retention geometry before subtracting the same 1.80 x 3.20 mm underside groove loops; operation order is part of geometry parity, not merely an implementation detail:

| Fit-check STL | Implemented STEP draft | Drawing still required |
| --- | --- | --- |
| `output/sixteen_slot_cassette_lower_carrier.stl` | `output/rfq/sixteen_slot_cassette_lower_carrier.step` | D1 |
| `output/sixteen_slot_cassette_lid_clamp.stl` | `output/rfq/sixteen_slot_cassette_lid_clamp.step` | D2 |
| `output/sixteen_slot_cassette_window_placeholder.stl` | `output/rfq/sixteen_slot_cassette_window_placeholder.step` | D3 or controlled sheet drawing |
| `output/sixteen_slot_cassette_gasket_witness_coupon.stl` | `output/rfq/sixteen_slot_cassette_gasket_witness_coupon.step` | D4 |
| `output/sixteen_slot_incubator_dock_plate.stl` | `output/rfq/sixteen_slot_incubator_dock_plate.step` | D5 |
| `output/sixteen_slot_service_bulkhead_test_block.stl` | `output/rfq/sixteen_slot_service_bulkhead_test_block.step` | D6 |
| `output/sixteen_slot_cassette_incubator_first_article_assembly.stl` | `output/rfq/sixteen_slot_cassette_stackup_reference.step` | D7 assembly/section reference |

Do not send the current STL files alone to a manufacturer except as supplemental visual references. A4 and A8 both require STEP files and drawings for RFQ.

## STEP Export Strategy

The implemented `src/bin/sixteen_slot_cassette_first_article_step.rs` uses the repo's OpenCascade `step` feature and `Shape::write_step(...)` precedent. It emits true B-rep draft geometry rather than converting STL triangles.

Implemented direction and remaining release rules:

- Keep the implemented OpenCascade-backed cassette STEP export bin as the common-contract draft source rather than relying on `vcad` STL triangles as the release source.
- Keep the current `sixteen_slot_cassette_incubator_first_article` STL bin as fit-check and visual review output.
- Preserve parity-equivalent lid construction order in both generators: finish
  additive frame, crossbar, and retention features first, then subtract the
  shared-contract gasket grooves. A groove cut made before a later union is not
  equivalent and must fail parity review.
- Keep deterministic STEP names under `output/rfq/`; decide separately whether a reviewed release bundle should be committed. Re-import every file in a separate OpenCascade verifier pass and require one closed solid for each standalone part.
- Fail immediately if the selected STEP export dependency, feature flag, or converter is unavailable during an RFQ export run.
- Do not use the existing `stl_to_step` helper as the RFQ release path because it converts triangulated STL faces and currently skips/fails softly when `stltostp` is missing.
- If STL-to-STEP conversion is used at all, label it `non-release mesh-derived STEP` and require vendor approval before quoting.

Authoritative execution uses the LaminarForge MCP runner with `action: run`,
`bin: sixteen_slot_cassette_first_article_step`, and `features: ["step"]`.
The runner sets the required CMake compatibility environment. The underlying
Cargo command belongs in the evidence artifact, not the operator procedure.

The exporter creates six custom-part STEP files and one stackup/reference STEP
file, then asserts all files exist. Missing files, failed export, failed
separate re-import, or a standalone part with more than one closed solid fail
the release evidence gate. STEP headers contain timestamps, so file SHA-256
values identify a particular run and are not stable geometry baselines.

## Drawing Set

A9 requires the following drawing sheets before RFQ.

| Sheet | File | Purpose |
| --- | --- | --- |
| D0 | `sixteen_slot_cassette_drawing_index.pdf` | Drawing index, revision table, no-biological-claim note, file manifest, and controlling specs. |
| D1 | `sixteen_slot_cassette_lower_carrier.pdf` | Carrier manufacturing definition. |
| D2 | `sixteen_slot_cassette_lid_clamp.pdf` | Lid/clamp manufacturing definition. |
| D3 | `sixteen_slot_cassette_window_placeholder.pdf` | Window sheet/placeholder definition. |
| D4 | `sixteen_slot_cassette_gasket_witness_coupon.pdf` | Coupon manufacturing and inspection definition. |
| D5 | `sixteen_slot_incubator_dock_plate.pdf` | Dock plate manufacturing definition. |
| D6 | `sixteen_slot_service_bulkhead_test_block.pdf` | Service bulkhead placeholder definition. |
| D7 | `sixteen_slot_cassette_stackup_and_sections.pdf` | Cross-section stackup, gasket compression, chip seating, and window/lid/carrier relationship. |
| D8 | `sixteen_slot_cassette_inspection_plan.pdf` | Consolidated critical inspection table and pass/fail evidence log. |
| D9 | `sixteen_slot_cassette_rfq_notes.pdf` | Vendor questions, quote options, assumptions, and explicit blockers. |

Drawings should be PDF for RFQ plus editable source if the selected CAD/drawing tool supports it. Do not bury critical tolerances in markdown only.

## Part Drawing Requirements

### D1 Lower Carrier

Drawing must include:

- Base-body dimensions: 699.04 x 541.92 x 24.00 mm; true overall bounds: 699.04 x 541.92 x 31.35 mm.
- A0 slot map and S01-S16 row-major orientation.
- 24.00 x 24.00 mm inter-chip gutter, 151.76 x 109.48 mm pitch, 583.04 x 413.92 mm array, and X/Y center coordinates +/-227.64, +/-75.88 / +/-164.22, +/-54.74 mm.
- Rev C chip pocket size, pocket depth, and optical through-window size.
- Explicit note that 1.20 mm per-side pocket clearance is the current CAD fit-check baseline.
- Manufacturer-drawing pocket target from A1: measured maximum Rev C chip lot plus 0.80 mm per side, with 0.50 mm per side minimum accepted clearance.
- Bottom plane datum A seated directly on the dock support plane; nominal rear
  edge datum B and left edge datum C contact the corresponding dock rail inner
  faces, and the nominal front edge contacts the front lip.
- D1-D4 coordinates at X = +/-335.52 mm and Y = +/-204.96 mm. D1 is the 6.00 mm round functional locator; D2 is a 10.00 x 6.00 X-oriented relieved slot; D3/D4 are 9.00 mm clearance/witness bores.
- Sixteen independent per-slot gasket lands: 145.76 x 103.48 mm outer, 129.76 x 87.48 mm inner, 8.00 mm width, 7.35 mm height, and 6.00 mm gap between adjacent outer edges.
- Perimeter gasket land: 629.04 x 459.92 mm outer, 605.04 x 435.92 mm inner, 12.00 mm width, and 7.35 mm height.
- Nine 4.00 mm-diameter internal hard stops at the 3 x 3 inter-slot gap
  intersections and distributed 4.00 mm-wide perimeter stops centered in the
  5.00 mm seal-to-gutter web. All top out at 7.35 mm and no stop intersects a
  seal; per-slot corner stops are prohibited.
- Sixteen 3.30 mm carrier-side M4 tap-drill/pilot receiver placeholders matching
  the D2 fastener centers. Final thread/insert SKU remains open.
- Side service reliefs are 7.00 mm high above carrier top and below closure.
- Leak gutter: 651.04 x 481.92 mm outer, 639.04 x 469.92 mm inner, 6.00 mm width, 3.00 mm depth, and 5.00 mm intact web to the perimeter land.
- Drain: 8.00 mm diameter, center (315.52, -254.96, 10.00) mm, 40.00 mm length along Y.
- Global 96.00 x 12.00 mm barcode land at (-271.52, -263.96) mm and adjacent
  118.00 x 10.00 mm text land at (-159.52, -263.96) mm, both outside the leak
  gutter and clear of the front receiver row; per-slot labels, compression-stop
  references, and handling lands.
- Material/finish: 6061-T651/T6 dry structural aluminum baseline, clear Type II anodize option with critical surfaces controlled after finish.

Minimum inspection table:

| Feature | Starting target |
| --- | --- |
| Pocket array true position | 0.25 mm relative to A/B/C. |
| Local adjacent slot pitch | +/-0.10 mm. |
| Pocket X/Y size | +0.20 / -0.00 mm from selected clearance target. |
| Pocket depth | +/-0.10 mm. |
| D1 locator bore | 6 mm H7 or vendor-equivalent after pin selection. |
| D1 true position | 0.10 mm relative to A/B/C. |
| D2 slot/locator true position | 0.15 mm relative to A/B/C and D1. |
| D3/D4 witness hole position | 0.30 mm; non-locating clearance/witness features only. |
| Per-slot gasket land flatness | 0.05 mm over each seal loop. |
| Gasket land / hard-stop height | 7.35 mm nominal, +/-0.03 mm starting target after finish. |
| Carrier receiver pilot diameter | 3.30 mm at all 16 locked centers before final thread/insert process. |
| Carrier bottom support face flatness | 0.25 mm over full carrier. |
| Seal band finish | Ra 0.8 um target and Ra 1.6 um hard maximum; a gasket vendor may require smoother. |

### D2 Lid/Clamp

Drawing must include:

- Base-body dimensions: 717.04 x 559.92 x 10.00 mm; true overall bounds: 717.04 x 559.92 x 11.60 mm.
- One-part lid construction: 4.00 mm continuous underside sealing skin plus
  6.00 mm upper frame. The 635.04 x 457.92 mm center lightening relief cuts only
  the upper frame.
- Sixteen lid view openings: 113.76 x 71.48 mm.
- Sixteen underside chip-top reliefs: 130.16 x 87.88 mm x 0.50 mm deep, leaving
  2.20 mm to the inner gasket-groove edge in X/Y.
- Carrier optical opening reference: 103.76 x 61.48 mm.
- 16-fastener M4 baseline outside the gutter: side X = +/-332.52 mm at the four slot-center Y values and front/rear Y = +/-247.96 mm at the four slot-center X values.
- Captive screw counterbore/retainer placeholders or hold note if final hardware is not selected.
- Matching 3.30 mm carrier pilot receiver reference at every fastener center.
- Lid-to-carrier datum/alignment ears and orientation prevention detail.
- D1/D2 replaceable-pin seats: 6.00 mm diameter x 4.00 mm deep. Show the 5.80 mm
  diameter x 7.35 mm fit-check surrogate only in D7 assembly/section views;
  standalone lid manufacturing geometry contains no pins, and D3/D4 contain no
  pin seats.
- Crossbar network and imaging keepouts.
- Window retention ledge/shoulder or explicit placeholder hold.
- Torque sequence mark locations.
- Sixteen per-slot and one perimeter 1.80 mm-deep x 3.20 mm-wide underside
  groove loops, cut after additive lid/retention geometry so STEP and STL are
  parity-equivalent.
- Groove floor: 2.20 mm nominal / 2.00 mm minimum. Adjacent loop gap: 10.80 mm;
  per-slot array to perimeter groove: 8.80 mm; perimeter groove to lid edge:
  48.40 mm X / 54.40 mm Y.
- Material/finish and gasket-facing flatness callouts.

Minimum inspection table:

| Feature | Starting target |
| --- | --- |
| Lid view opening X/Y size | +/-0.20 mm. |
| Fastener hole position | +/-0.15 mm unless hardware requires tighter; verify every retainer clears the gutter. |
| Gasket groove | 1.80 mm depth x 3.20 mm width nominal; depth +/-0.03 mm after finish. |
| Alignment feature true position | 0.10-0.15 mm relative to lid datums after carrier strategy is frozen. |
| Gasket-facing flatness | 0.15 mm total over clamp face; tighter local targets if vendor review supports it. |
| Lid elastic deflection allowance | 0.10 mm planning allocation until FEA or metrology replaces it. |
| Captive hardware fit | Vendor/hardware drawing controlled. |

### D3 Window Placeholder

Drawing must include:

- Base panel: 667.04 x 489.92 x 3.00 mm; true overall bounds: 667.04 x 489.92 x 4.80 mm.
- Per-slot witness frames and three calibration rings. Every raised feature must
  be fused to the panel; calibration rings expose 1.80 mm above the panel with a
  0.20 mm modeling overlap. Floating solids fail D3 export review.
- Retention tab/ledge relationship to lid.
- Material options: polycarbonate fit-check baseline, borosilicate/glass alternate.
- Edge deburr/chamfer note and no point-loading note.
- Imaging surfaces and cleaning compatibility hold.

If a sheet drawing is simpler than STEP, D3 may use a flat pattern drawing plus thickness and material notes.

### D4 Gasket Witness Coupon

Drawing must include:

- Coupon envelope: 250.00 x 118.00 x 12.00 mm from active CAD.
- Same 1.80 mm groove depth, 3.20 mm groove width, seal-band finish, 7.35 mm closure-contact stop logic, and hardware style as final cassette gasket interfaces.
- A3 baseline: 2.40 mm gasket free height, 1.80 mm target compressed height, 1.68-1.92 mm accepted guard band.
- Locked nominal groove: 1.80 mm depth, 3.20 mm width, 0.20 mm minimum entry break, vendor-reviewed corner radius and tolerance.
- Coupon labeling for 20%, 25%, and 30% squeeze references.
- Leak, burst, reconnection, and soak coupon traceability fields.

Minimum inspection table:

| Feature | Starting target |
| --- | --- |
| Groove depth | +/-0.03 mm after finish/coating strategy. |
| Groove width | +/-0.05 mm unless vendor/gasket supplier changes target. |
| Compression stop height | +/-0.03 mm. |
| Seal land finish | Ra 0.8 um target and Ra 1.6 um hard maximum; a gasket vendor may require smoother. |
| Coupon flatness | Match cassette seal-band drawing notes. |

### D5 Incubator Dock Plate

Drawing must include:

- Base-body dimensions: 869.04 x 691.92 x 22.00 mm; true overall bounds: 869.04 x 691.92 x 40.00 mm.
- Rear rail, left rail, front low retention lip, dock leveling pads, logger pockets, robot lift lands, drain gutters, and visible sump.
- Direct datum-A support plane and rail/lip inner faces positioned to contact the
  nominal rear, left, and front carrier edges without spacer or recess offset.
- Note that CO2 incubation and environmental control are reserved, not solved by this dock.

Minimum inspection table:

| Feature | Starting target |
| --- | --- |
| Dock cassette support flatness | 0.25 mm over cassette support region. |
| Rear/left rail squareness and edge contact | Vendor-reviewed; inner faces contact nominal carrier edges and must not twist carrier. |
| Rail/contact edge burr state | No burrs or coating buildup that shifts cassette datum. |
| Drain/sump visibility | Visual inspection with cassette docked. |
| Logger pocket size | +/-0.20 mm unless logger SKU is selected. |

### D6 Service Bulkhead Test Block

Drawing must include:

- Base-body dimensions: 789.04 x 34.00 x 76.00 mm; true overall bounds: 789.04 x 60.00 x 76.00 mm.
- Current placeholder gas/media/waste holes and sensor/backplane cut.
- A6 logical port labels G0-G3, M0-M6, W0-W4.
- Sensor/backplane cut center X = 320.00 mm and center Z = 18.00 mm; preserve clearance from W4.
- Explicit note that placeholder diameters are not final connector cutouts.
- Cap/plug status and label-strip requirements.
- Dry structural role; no custom machined sterile port claim.

RFQ drawings should either leave connector cutouts as placeholders for no-cell fit-check or wait until selected bought connector datasheets define panel geometry.

### D7 Stackup And Sections

Stackup drawing must include:

- Carrier, Rev C chip surrogate, gasket, lid/clamp, window placeholder, hard stops, and dock support in section.
- Per-slot and perimeter gasket land height: 7.35 mm above carrier top on one common plane meeting the lid underside.
- Gasket free height: 2.40 mm.
- Target compressed height: 1.80 mm.
- Lid groove cavity: 1.80 mm deep x 3.20 mm wide; compressed gasket resides in
  this cavity rather than in a gap above a shorter land.
- Guard band: 1.68-1.92 mm.
- Pocket depth: 7.00 mm.
- Chip total height: 14.35 mm.
- Nominal chip protrusion and closure plane: 7.35 mm above carrier top.
- Lid local chip-top clearance: 0.50 mm; show that the sealing face still meets
  lands/stops and that the relief remains 2.20 mm from the groove.
- Replaceable D1/D2 pin stack: 4.00 mm lid embedment, 3.35 mm projection, and
  2.00 mm carrier-boss engagement; D3/D4 remain pin-free.
- Nine 4.00 mm-diameter inter-slot hard stops and 4.00 mm-wide perimeter web
  stops at the same 7.35 mm plane, with no seal intersection.
- Direct carrier-bottom contact on dock datum A and nominal rear/left/front edge
  contact at the dock rails/lip.
- Optical line-of-sight through carrier and lid openings.
- Drain/leak witness path and tubing keepout zones.

D7 must include a signed worst-case calculation, not a nominal-only review. At
every required seal witness location, define delta_local as the local
land-to-stop plane error after part flatness, coating, datum seating, stop/land
height variation, lid deflection, and measurement uncertainty. Prove:

1.68 mm <= groove_depth + delta_local <= 1.92 mm

using worst-case limits for release and showing RSS only as supplemental
process-capability information. List every contributor, sign, tolerance,
inspection method, and governing drawing dimension. A missing contributor or
failed inequality blocks D7 and RFQ release.

D7 remains the highest-risk drawing because it exposes whether the physical tolerance stack preserves the common 7.35 mm closure plane. The prior overlapping loops and 3.00/4.00 mm unequal land elevations were corrected in the shared A0 contract; drawings must not revive them.

### D8 Inspection Plan

D8 should consolidate all critical measurements into one vendor-facing table:

- Part number and revision.
- Feature ID.
- Drawing zone.
- Nominal value.
- Tolerance or GD&T frame.
- Datum reference.
- Inspection method.
- Sample quantity.
- Acceptance rule.
- Vendor report required: yes/no.

Feature IDs should be stable, for example:

- CAR-PKT-001 through CAR-PKT-016 for chip pockets.
- CAR-DAT-D1 through CAR-DAT-D4 for datum bosses/bores.
- CAR-SEAL-S01 through CAR-SEAL-S16 for slot seal lands.
- LID-WIN-S01 through LID-WIN-S16 for lid view openings.
- LID-FST-001 through LID-FST-016 for fastener holes.
- CAR-RCV-001 through CAR-RCV-016 for the 3.30 mm carrier pilot receivers.
- CAR-STP-I01 through CAR-STP-I09 and CAR-STP-P01 onward for internal and
  perimeter hard stops.
- GSK-GRV-001 for coupon groove.
- DCK-RAIL-B and DCK-RAIL-C for dock rails.
- BLK-M0 through BLK-W4 for service bulkhead logical ports.

### D9 RFQ Notes

D9 should carry the A4/A8 vendor notes:

- Quote one prototype set, three prototype sets, and pilot quantity separately.
- Separate DFM comments from price.
- Quote 6061-T651 clear anodized baseline, no-anodize/chem-film option, 316 stainless option, matched carrier/lid machining option, CMM inspection add-on, and flatness-relaxed option.
- Ask whether carrier and lid should be machined or inspected as a matched pair.
- Ask whether seal bands and datum bores should be masked, post-machined, or inspected after anodize.
- Ask for vendor minimum radii for pockets, gasket grooves, drain gutters, and bulkhead cutouts.
- Ask for cleaning compatibility notes for humid 37 C exposure and likely wipe agents.
- State that the first quote is mechanical/no-cell only.

## Drawing Notes To Carry Forward

Use these notes unless a selected vendor or subsystem revision replaces them:

- Material: 6061-T651 aluminum plate unless otherwise specified.
- Finish: clear Type II anodize, natural, with critical seal bands and precision bores controlled after finish or masked per drawing.
- General tolerance: ISO 2768-m unless otherwise specified.
- Break all noncritical sharp edges 0.25-0.50 mm.
- Do not break, polish, or blend gasket compression stops beyond specified tolerance.
- Seal bands: Ra 0.8 um target and Ra 1.6 um hard maximum; a gasket vendor may require smoother, never rougher.
- General machined faces: Ra 3.2 um max unless otherwise specified.
- No burrs, loose media, sharp chips, or embedded blasting residue.
- Dimensions for gasket groove depth, compression stop height, precision bores, and pocket size apply after coating unless explicitly marked pre-coat.
- Vendor to provide inspection report for critical dimensions listed on drawing.
- Part is dry reusable structural hardware for no-cell first-article testing; sterile/wetted path is not claimed by this drawing.

## Release Acceptance Criteria

The STEP implementation portion of A9 exists as a draft. A9 is not RFQ-complete until all of these pass:

- STEP export command creates all six part STEP files and one stackup/reference STEP file.
- Export command fails immediately if STEP dependencies, feature flags, or output writes fail.
- `sixteen_slot_cassette_step_verify` re-imports every STEP successfully and records closed-solid count, nonempty topology, A0 bounds, and file hash for the release commit.
- Drawing index lists every STEP, drawing, revision, and controlling spec.
- D0-D9 drawings exist as PDFs or selected drawing-source outputs.
- Drawings include no-biological-claim notes.
- Inspection table includes carrier pockets, datums, gasket lands, lid openings, fastener pattern, dock rails, bulkhead ports, surface finish, flatness, coating, and deburr checks.
- D7 stackup is reviewed against the common 7.35 mm land/stop elevation, lid
  underside contact, and 1.80 mm groove cavity.
- A8 build package is updated from "blocked by drawings" to "RFQ package generated" only after drawings exist.
- Ticket artifact records command output, file manifest, and unresolved deviations.

## Implementation Tasks

Recommended next implementation tasks:

1. Done: add `src/bin/sixteen_slot_cassette_first_article_step.rs` using the `step` feature and OpenCascade B-rep primitives.
2. Done: export the six custom part STEP files and stackup/reference STEP file with hard-fail assertions.
3. Decide whether generated `output/rfq/` output remains ignored or whether a reviewed zipped RFQ release artifact should be committed under a controlled release directory.
4. Create drawing-source templates or markdown-to-PDF drawing notes for D0-D9.
5. Add a manifest generator that records file names, commit hash, export command, dimensions, and no-biological-claim notes.
6. Update A8 from "blocked by drawings" only after the actual D0-D9 drawing package exists.

## Explicit Blockers

- Do not send STL files alone as a vendor manufacturing definition.
- Do not use mesh-derived STEP as release geometry unless explicitly labeled and accepted by the vendor.
- Do not silently skip STEP output when a dependency is missing.
- Do not release a drawing set without D7 stackup review of gasket compression.
- Do not reintroduce the obsolete overlapping per-slot lands, unequal 3.00/4.00
  mm or interim shorter land elevations, per-slot corner stops, fasteners inside
  the gutter, or pre-contract base dimensions into drawings.
- Do not claim sterility, live-cell readiness, AAV containment, or biological compatibility from STEP/drawing output.
- Do not change slot count, slot centers, one-condition semantics, or disposable wetted-path policy without revising A0.
