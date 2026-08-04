# 16-Slot Cassette A8 First-Article Build Package

Ticket: T-E0187ED3

This A8 package consolidates A0-A7 into the first practical build package for the LaminarForge 16-slot AAV cassette. It defines what can be built now, what must be generated or selected before vendor RFQ, how the first article should be assembled, and which acceptance gates block cells or AAV.

This is not a final vendor drawing set, sterile-barrier validation, biological protocol, AAV containment claim, or live-cell release criterion. It is a first-article engineering package for a dry reusable cassette/dock stack plus a disposable one-condition fluid path.

The active first-build architecture is the full 16-slot, 4 x 4 A0 cassette.
`docs/standard_footprint_aav_condition_module_a0.md` is a supplemental compact
module and local-coupon study only; it does not supersede this package or close
any 16-slot first-article gate.

## Package Status

| Lane | Status | Meaning |
| --- | --- | --- |
| Internal CAD fit-check package | Implemented draft | The active Rust CAD bin consumes the shared A0 contract and exports seven STL fit-check files. |
| True B-rep STEP package | Implemented draft | `sixteen_slot_cassette_first_article_step` exports six custom-part STEP drafts plus one stackup reference from the same A0 contract. These are not released manufacturing definitions. |
| Mechanical RFQ package | Blocked by drawings | D0-D9 drawings, inspection tables, selected hardware/connector details, and release review are still required. STEP draft availability does not make the package quote-ready. |
| Disposable fluid-path package | Architecture ready, SKUs open | A6 defines topology and port roles, but final tubing, connectors, pump interface, and harness vendor are not selected. |
| No-cell validation package | Layout mockup only; functional fixture blocked | `sixteen_slot_cassette_no_cell_validation_fixture` exports dry placement/envelope geometry. Its current tokens are solid, the dead-volume dye station is absent, and no characterized resistance articles or measurement hardware exist. It cannot execute or pass A7. |
| Media-only planning | Blocked until A7 gates pass | No-cell fit, leak, flow, bubble, dye recovery, and waste tests must pass first. |
| Live-cell/AAV work | Blocked | Requires A7 no-cell pass, media-only/aseptic evidence, safety/IBC workflow, and material compatibility data. |

## Controlling Inputs

| Input | A8 use |
| --- | --- |
| `docs/sixteen_slot_cassette_a0_interface_spec.md` | Controls slot map, dimensions, datums, one-condition semantics, dry structural policy, and downstream contracts. |
| `src/sixteen_slot_cassette_a0.rs` | Machine-readable mechanical contract consumed by both the STL fit-check and STEP draft generators. |
| `docs/sixteen_slot_cassette_a1_carrier_spec.md` | Controls carrier pocket, datum, drain, label, material, and handling assumptions. |
| `docs/sixteen_slot_cassette_a2_lid_clamp_spec.md` | Controls lid/clamp, imaging windows, captive fasteners, torque sequence, and stiffness requirements. |
| `docs/sixteen_slot_cassette_a3_gasket_spec.md` | Controls gasket material candidates, 2.40 mm cross-section, 25% target squeeze, grooves, hard stops, and leak/burst/reconnection logic. |
| `docs/sixteen_slot_cassette_a4_dfm_spec.md` | Controls manufacturing process, material, tolerance, finish, flatness, inspection, and RFQ requirements. |
| `src/bin/sixteen_slot_cassette_incubator_first_article.rs` | Active A5 integrated CAD generator and export source. |
| `docs/sixteen_slot_cassette_a6_disposable_fluid_path_spec.md` | Controls disposable one-condition single-pass harness, G/M/W port roles, prime/debubble path, waste path, and run record. |
| `docs/sixteen_slot_cassette_a7_no_cell_validation_fixture_spec.md` | Controls no-cell fixture architecture, test sequence, acceptance gates, and blockers before biology. |

## Locked Build Decisions

- First build format: 16 slots in a 4 x 4 cassette.
- Architecture authority: the active 16-slot A0 interface contract; compact
  standard-footprint work is supplemental risk retirement only.
- Experimental unit: one cassette equals one AAV capsid/promoter/payload/dose/timing/media condition.
- Slot role: cell-type and technical readouts under the same exposure, not separate AAV candidates.
- Active CAD bin: `sixteen_slot_cassette_incubator_first_article`.
- Reusable structural stack: carrier, lid/clamp, dock plate, service bulkhead test block, window placeholder, gasket witness coupon.
- Wetted path: disposable commercial tubing, connectors, reservoirs, filters, and pump-contact tubing.
- Flow mode: one-condition pressure-limited single-pass flow.
- Recirculation: deferred.
- CO2 incubation: deferred.
- Older 4 x 5 / 20-position CAD: reference only until explicitly ported to the 16-slot first-article geometry and A6 port map.

Locked mechanical values consumed by both generators:

| Interface | Base-body or nominal value | True overall or functional value |
| --- | --- | --- |
| Chip layout | 24.00 x 24.00 mm gutter; 151.76 x 109.48 mm pitch; 583.04 x 413.92 mm array | Centers X = +/-227.64, +/-75.88 mm; Y = +/-164.22, +/-54.74 mm |
| Carrier | 699.04 x 541.92 x 24.00 mm | 699.04 x 541.92 x 31.35 mm |
| Lid/clamp | 717.04 x 559.92 x 10.00 mm | 717.04 x 559.92 x 11.60 mm |
| Lid structure / chip clearance | 4.00 mm continuous underside seal skin plus 6.00 mm upper frame | Upper-only 635.04 x 457.92 mm lightening relief; sixteen 130.16 x 87.88 x 0.50 mm chip-top reliefs; 2.20 mm groove floor |
| Window | 667.04 x 489.92 x 3.00 mm | 667.04 x 489.92 x 4.80 mm |
| Dock | 869.04 x 691.92 x 22.00 mm | 869.04 x 691.92 x 40.00 mm |
| Bulkhead | 789.04 x 34.00 x 76.00 mm | 789.04 x 60.00 x 76.00 mm |
| Seal stack | Per-slot outer/inner 145.76 x 103.48 / 129.76 x 87.48 mm; perimeter outer/inner 629.04 x 459.92 / 605.04 x 435.92 mm | Both lands and all stops are 7.35 mm high and meet the lid underside; 6.00 mm inter-land gap; 1.80 x 3.20 mm lid grooves for a 2.40 mm gasket at 25% nominal squeeze |
| Hard stops / service relief | Nine 4.00 mm-diameter internal stops at the 3 x 3 inter-slot gap intersections; 4.00 mm-wide perimeter stops in the 5.00 mm seal-to-gutter web | No stop intersects a seal; 7.00 mm-high side reliefs remain below the 7.35 mm closure plane |
| Gutter / drain | 651.04 x 481.92 mm outer; 639.04 x 469.92 mm inner; 6.00 mm wide x 3.00 mm deep; 5.00 mm web | Drain diameter 8.00 mm at (315.52, -254.96, 10.00) mm, length 40.00 mm along Y |
| Fasteners / datums | Side X = +/-332.52 mm at slot-center Y; front/rear Y = +/-247.96 mm at slot-center X; all 16 carrier pilot receivers are 3.30 mm | D1-D4 at X = +/-335.52 mm, Y = +/-204.96 mm; only D1 round and D2 relieved locate. D1/D2 use replaceable lid-seated pins (5.80 x 7.35 mm CAD surrogate); D3/D4 have no pins. |
| Identity / sensor | Global 96.00 x 12.00 mm barcode land at (-271.52, -263.96) mm and 118.00 x 10.00 mm text land at (-159.52, -263.96) mm; per-slot labels outside gutter | Front lands clear the receiver row; bulkhead sensor/backplane cut center X = 320.00 mm |

Locked dock interfaces:

| Dock interface | A0 geometry | Build / inspection meaning |
| --- | --- | --- |
| Support and recesses | Datum-A support plane at local Z = +11.00; sixteen 137.76 x 95.48 mm recesses, 5.50 mm deep, at the slot centers | Inspect every recess floor at local Z = +5.50 and verify the carrier seats directly on datum A. |
| Registration | Rear 743.04 x 16.00 x 18.00 rail centered at Y = 278.96 mm; left 16.00 x 587.92 x 18.00 rail centered at X = -357.52 mm; front 743.04 x 10.00 x 10.00 lip centered at Y = -275.96 mm | Preserve the nominal rear/left/front carrier contacts; do not add an unrecorded spacer or offset. |
| Through-deck openings | Five 657.04 x 8.00 mm air bypass cuts at Y = -218.96, -109.48, 0.00, 109.48, 218.96 mm; front drain 799.04 x 10.00 mm at Y = -307.96 mm; right drain 10.00 x 615.92 mm at X = 392.52 mm; visibility opening 58.00 x 38.00 mm at (376.52, -287.96) mm | All cutters are 24.00 mm high through the 22.00 mm base deck. Their XY locations are locked. |
| Top-side lands | Sixteen 24.00 x 10.00 x 3.00 position-token lands; four solid 48.00 x 32.00 x 8.00 logger reservation lands; two 160.00 x 20.00 x 7.00 robot-lift lands; four 32.00 mm-diameter x 3.00 leveling lands | These are additive support/reservation geometry, not pockets, drains, or proof of sensing hardware. |
| Drain bridges | The front lip bridges part of the visibility cut; the two right logger lands bridge the right drain; front-left, front-right, and rear-right leveling lands bridge the named drain openings beneath them | The base-body region below datum A remains void and the additive land above is solid. Inspect both clear spans and bridge sections; do not describe the fit-check as a continuous gutter or validated sump. |
| Mounting | Six 5.40 mm through holes at X = +/-412.52 mm, Y = +/-323.96 mm plus X = 0.00 mm, Y = +/-323.96 mm | Freeze final M5 hardware and tolerances in D5. |

## Build Command And Generated Outputs

Run from the `laminarforge-cad` repo using the MCP build tool:

```text
mcp__agentic-mcp__laminarforge_build
repo: laminarforge-cad
action: run
bin: sixteen_slot_cassette_incubator_first_article
```

The fit-check command is defined to export the following seven STL files. Regenerate them for current-build evidence; historical A8 output does not substitute for a release manifest tied to the current A0 contract.

| Output | Role | A8 disposition |
| --- | --- | --- |
| `output/sixteen_slot_cassette_lower_carrier.stl` | Lower carrier fit-check model | Internal fit-check only; matching STEP draft exists, but D1 still controls RFQ release. |
| `output/sixteen_slot_cassette_lid_clamp.stl` | Lid/clamp fit-check model | Internal fit-check only; needs final captive hardware and drawing notes. |
| `output/sixteen_slot_cassette_window_placeholder.stl` | Retained window placeholder | Internal fit-check only; window material and sheet drawing remain open. |
| `output/sixteen_slot_cassette_gasket_witness_coupon.stl` | Gasket compression/leak witness coupon | Internal fit-check only; coupon drawing must duplicate final material/finish/groove notes. |
| `output/sixteen_slot_incubator_dock_plate.stl` | Dry dock plate with segmented through-deck openings and solid logger reservation lands | Internal fit-check only; drainage and incubation control are not validated by this part. |
| `output/sixteen_slot_service_bulkhead_test_block.stl` | Dry service bulkhead placeholder | Internal fit-check only; bought connector cutouts are not finalized. |
| `output/sixteen_slot_cassette_incubator_first_article_assembly.stl` | Combined assembly view | Assembly review only; not a manufacturing source. |

The `output/` directory is intentionally ignored by git. Regenerate these files from source instead of committing generated STL artifacts.

Run the STEP draft through the same MCP tool with `action: run`,
`bin: sixteen_slot_cassette_first_article_step`, and `features: ["step"]`.
The runner supplies the required OpenCascade CMake policy environment. The
underlying exporter invocation is recorded in the run artifact for
reproducibility but is not the operator workflow.

It creates seven true B-rep draft files under `output/rfq/` and fails if any
required file is missing. Re-import all outputs in a separate OpenCascade
verifier pass and require one closed solid per standalone part plus a readable
stackup reference. Run `sixteen_slot_cassette_step_verify` with the same `step` feature
immediately after export; it performs that re-import, solid-count, topology, and
A0-envelope gate. Both verifiers also probe dock recess floors, clear
through-deck spans, and representative void-below/solid-above bridge sections.
The STEP and STL lid builders use parity-equivalent
construction and subtract the 1.80 x 3.20 mm underside grooves last. D0-D9
drawings still block RFQ release.

## Custom Part Package

| Part | Current draft source | Preferred first-article process | Material direction | Critical notes |
| --- | --- | --- | --- | --- |
| Lower carrier | `output/rfq/sixteen_slot_cassette_lower_carrier.step` plus fit-check STL | CNC machined plate after D1 release | 6061-T651/T6 aluminum, clear Type II anodize option | Dry structural part; preserve A0 datums, 16 pockets, 7.35 mm lands/stops, 3.30 mm receiver pilots, drain, labels, handling lands, and 7.00 mm service reliefs. |
| Lid/clamp | `output/rfq/sixteen_slot_cassette_lid_clamp.step` plus fit-check STL | CNC machined or waterjet rough + secondary machining after D2 release | 6061-T6/T651 aluminum or stainless alternate | Must use controlled compression, captive M4 hardware assumption, imaging keepouts, and torque sequence. |
| Window placeholder | `output/rfq/sixteen_slot_cassette_window_placeholder.step` plus fit-check STL | CNC/laser-cut sheet after D3 release | Polycarbonate for fit-check; borosilicate/glass if imaging requires | Mechanically retained; adhesive-only retention is not the baseline. |
| Gasket witness coupon | `output/rfq/sixteen_slot_cassette_gasket_witness_coupon.step` plus fit-check STL | CNC machined coupon after D4 release | Same seal-band material/finish assumptions as final stack | Must duplicate gasket groove, stop, finish, and hardware strategy for evidence. |
| Dock plate | `output/rfq/sixteen_slot_incubator_dock_plate.step` plus fit-check STL | CNC machined plate after D5 release | 6061-T651/T6 aluminum or stainless alternate | Dry support with locked segmented through-deck openings and solid logger/leveling/handling lands. D5 must section every bridge and dimension every clear span. |
| Service bulkhead test block | `output/rfq/sixteen_slot_service_bulkhead_test_block.step` plus fit-check STL | CNC machined dry block after D6 and connector selection | Aluminum or polycarbonate dry structure | Placeholder holes are not connector drawings and not sterile ports. |

## Bought Component BOM Assumptions

These are first RFQ/test assumptions, not final SKUs.

| Function | Starting assumption | Owner / closeout |
| --- | --- | --- |
| Lid fasteners | Stainless M4 x 0.7 captive panel screws with positive-drive heads | A2/A4/vendor to finalize length, head style, retainers, and torque. |
| Carrier receivers | Sixteen locked 3.30 mm M4 tap-drill/pilot placeholders, finished with stainless threaded inserts, nut plates, or replaceable threaded receiver hardware | A4/vendor to select the final thread/insert SKU; post-install flatness inspection required. |
| Datum pins | Replaceable 6 mm stainless ground dowel family; active CAD surrogate is 5.80 mm diameter x 7.35 mm long in 6.00 mm x 4.00 mm D1/D2 lid seats | A1/A4 to finalize pin SKU and bore/seat tolerances while preserving D1 round, D2 relieved, and D3/D4 no-pin clearance/witness roles. |
| Gasket | Platinum-cured silicone 50-60A first quote; high-purity EPDM 60-70A alternate | A3/A4/vendor to finalize compound, tolerance, compression set, and compatibility. |
| Window | Optically clear polycarbonate for fit-check; borosilicate/glass alternate | Imaging/material review to select final. |
| Labels | Humidity/IPA-wipe compatible 2D code plus human-readable label stock | Build package/vendor to select and test condensation survival. |
| Tubing | Compare weldable TPE/C-Flex-style route and silicone/peristaltic route if budget allows | A6/pump owner/material review to select. |
| Connectors | Bought aseptic connectors or sterile tube welding for live-use planning; low-dead-volume connectors for no-cell only | A6/RFQ to select final connector family and panel cutouts. |
| Waste | Sealed waste bag or bottle with anti-backflow/siphon strategy | A6/A7 validation to prove. |
| Sensors | Pressure sensors at common upstream, four rows, and waste/backpressure; optional flow/bubble sensors | A7 fixture owner to select. |
| Collection | Sixteen vial nests or gravimetric collection positions | A7 fixture owner to select. |
| Dye/leak witness | Nonhazardous visible dye, wicking paper or clear leak tray | A7 validation to select and record. |

## Vendor RFQ Deliverables Still Required

The package is not vendor-ready until these are created:

- Regenerated and reviewed STEP draft manifest tied to the release commit; the STEP exporter exists, but its outputs remain draft support geometry until paired with drawings.
- 2D drawing for each custom part with material, finish, tolerances, flatness, surface finish, datum scheme, inspection table, and no-biological-claim notes.
- Stackup drawing showing chip, carrier, gasket, lid/clamp, window, hard stops, and compression witness locations.
- Hardware table with fastener, insert, dowel, washer/retainer, and torque assumptions.
- Gasket drawing or supplier print with cross-section, compound, splice/join strategy, compression target, and lot traceability.
- Label drawing with global cassette/condition ID land, per-slot labels, slot-1 orientation, and barcode size.
- Connector panel drawings using selected bought connector datasheets, not placeholder hole diameters.
- D5 dock sections that dimension every unbridged drain/visibility span and show
  the front-lip, logger-land, and leveling-land bridges without moving the A0
  opening coordinates.
- No-cell validation fixture drawing or sourced fixture plan for pressure, bubble, flow, dye, waste, and repeat-cycle gates.
- Vendor note that the quote is a mechanical no-cell first article only and makes no sterile, clinical, live-cell, AAV containment, or biological release claim.

## Assembly Order

Use this order for the first physical build. Do not skip metrology gates because later no-cell tests can pass while hidden damage or overcompression remains.

1. Regenerate STL outputs from the active CAD bin and verify the seven expected files exist.
2. Regenerate the implemented true B-rep STEP drafts and create/review D0-D9 drawings; without the drawings, keep the package internal only.
3. Inspect machined carrier, lid, dock, bulkhead, window, and coupon before installing hardware.
4. Verify carrier dimensions: pocket array, datum bores, rear/left/front datum edges, 7.35 mm gasket lands and hard stops, 3.30 mm fastener pilots, 7.00 mm service reliefs, drain, label lands, and optical openings.
5. Verify lid dimensions: 4.00 mm sealing skin, upper-only relief, 0.50 mm chip
   reliefs, 2.20 mm groove floor, outside-gutter fastener holes, D1/D2 pin seats,
   D3/D4 no-pin policy, alignment ears, 1.80 x 3.20 mm underside gasket grooves,
   window retention, and torque sequence marks.
6. Verify dock dimensions: direct datum-A support plane; sixteen exact recess
   floors; rear/left rail inner faces; front lip; clear spans of the front,
   right, visibility, and air-bypass through-cuts; void-below/solid-above bridge
   sections; solid logger reservation lands; carrier support flatness; and
   robot/manual handling keepouts.
7. Install threaded receivers, captive hardware, dowels, and labels only after pre-install metrology passes.
8. Recheck flatness, burrs, datum fits, and label placement after hardware installation.
9. Install surrogate Rev C chips or gauge chips into all 16 pockets and confirm 16/16 seating without forcing.
10. Install gasket or coupon material using the A3 process and verify witness shims.
11. Install retained window placeholder without point-loading or blocking view openings.
12. Close lid/clamp using the A2 paired cross-pattern torque sequence and record driver setting, hardware lot, and witness results.
13. Seat the carrier directly on datum A, contact the nominal rear/left/front edges against the rail inner faces and front lip, and confirm the dock does not twist the carrier or hide drain/witness paths.
14. Connect the A6 disposable harness for dry topology inspection.
15. Run the A7 no-cell sequence before any media-only, live-cell, or AAV planning.

## QA And Inspection Checklist

| Area | Required evidence |
| --- | --- |
| Slot map | 4 x 4 row-major S01-S16 confirmed against A0. |
| Dimensions | Carrier, lid, dock, bulkhead, window, and coupon measured against drawings or CAD fit-check targets. |
| Datums | Direct datum-A seating, nominal rear/left/front edge contact, and D1/D2 locator behavior verified without overconstraint. |
| Pocket fit | 16/16 surrogate chips seat; no burrs, binding, or pocket-floor debris. |
| Gasket compression | 20-30% guard band with 25% target; 1.68-1.92 mm compressed height if using 2.40 mm nominal gasket; land/stop faces contact the lid underside and nominal height is set by the 1.80 mm groove cavity. |
| Fasteners | Captive hardware retained, torque sequence marked, centers remain outside the gutter, and all 3.30 mm carrier pilots/final receiver installations are inspected. |
| Imaging | 16 view openings and carrier optical windows unobstructed by lid, gasket, labels, tubing, or window retention. |
| Drain/leak witness | Carrier gutter/drain and dock clear spans are visible when docked; front-lip, logger-land, and leveling-land bridges match D5 sections; no gravity-flow or condensate-capacity claim is made until no-cell testing passes. |
| Fluid path | G/M/W ports, R1-R4 branches, S01-S16 pigtails, waste paths, caps, and labels traceable to run record. |
| Labels | Cassette ID, condition ID, slot map revision, harness ID, and waste ID readable/scannable after assembly. |
| Cleaning | No residue, sharp chips, blasting media, corrosion, label lift, or dye-trapping crevices after cleaning trial. |
| Documentation | Photos, measurements, material lots, hardware lots, gasket lots, harness lots, and pass/fail gates recorded. |

## No-Cell Acceptance Gates

A8 adopts the A7 acceptance gates as build-release gates:

| Gate | Minimum pass condition |
| --- | --- |
| Fit and datum | 16/16 surrogate chips seat; cassette registers to rear/left datums; lid aligns without forcing. |
| Compression | A3 witness target met, nominally 20-30% squeeze with 25% target. |
| Harness map | Every port, row, slot, and waste path traceable to the run record. |
| Prime | 16/16 paths prime with no visible bubble at chip inlet witnesses. |
| Bubble challenge | Calibrated, preapproved bubble stimulus clears to W1/W3 within the frozen clearing volume/time without reaching chip inlet witnesses; unset stimulus/limits fail. |
| Leak | Installed system has no visible dye leak and <=5% pressure decay over 10 minutes at 1.5x maximum operating pressure; representative isolated gasket loop also passes A3 qualification. |
| Flow balance | Row and slot CV are each <=10% for the characterized nominal surrogate set; pressure drift is within +/-5% after stabilization. |
| Restriction detection | Independently characterized nominal, low, high, blocked, and bypass coupons remain in band and produce signatures separated by at least three combined standard uncertainties. |
| Dead volume | Recovery reconciles within +/-10%; unrecovered volume is below the A6 target with no discretionary waiver. |
| Waste safety | No backflow, siphon, or uncontrolled overflow into cassette or bench. |
| Repeatability | Cycles 1, 5, 10, and 25 pass the same required gates. |

## First-Article Run Record

Every first-article build/run should create a searchable artifact with:

- Ticket ID and artifact ID.
- Commit hash and CAD bin used.
- MCP build/run records, seven-output STL confirmation, seven-output STEP
  confirmation, STL probe report, and separate STEP re-import/solid-count
  report.
- Cassette serial/revision and part revision set.
- Carrier/lid/dock/bulkhead/window/coupon material, finish, supplier, and inspection reports.
- Fastener, insert, dowel, gasket, window, label, tubing, connector, reservoir, and waste lots.
- Slot map revision and G/M/W port map revision.
- Torque driver setting and paired sequence used.
- Compression witness measurements.
- Harness topology photos and scan results.
- A7 prime, bubble, leak, flow, restriction, dye recovery, waste, and repeat-cycle results.
- Deviations, rejected parts, corrective actions, and retest status.

## Open Decisions Blocking RFQ Or Biology

| Decision | Blocks | Required owner action |
| --- | --- | --- |
| D0-D9 drawings and STEP release manifest | Vendor RFQ | The true B-rep STEP exporter is implemented; create the drawings, inspection tables, release manifest, and review record. |
| Final Rev C chip lot dimensions | Carrier drawing release | Measure chip lot and resolve pocket target versus 1.20 mm CAD fit-check clearance. |
| Final pin/bore production detail | Carrier/lid drawing release | Select pin SKU, D2 relief detail, and bore tolerances while preserving the locked D1-round/D2-relieved roles. |
| Final hardware and torque | Physical build release | Select captive M4 hardware, final thread/insert SKU, insert process, washers/retainers, and torque-to-stop value while preserving the locked centers and 3.30 mm carrier pilots. |
| Final gasket compound and supplier | Physical build release | Select silicone/EPDM compound, cross-section tolerance, splice strategy, and coupon plan; the 1.80 x 3.20 mm lid groove and 7.35 mm land/stop stack are already fixed. |
| Final seal-band finish/coating plan | Vendor RFQ | Decide masked anodize, post-machining, or as-machined seal bands. |
| Final window material | Imaging and cleaning tests | Choose polycarbonate or glass and define retention/cleaning compatibility. |
| Final connector and tubing SKUs | Fluid-path RFQ and validation fixture | Select bought connector family, tubing OD/ID, weldability, pressure rating, and panel cutouts. |
| Pump/pressure-control interface | A7 validation | Select pressure-limited pump/control mode and sensor range. |
| Functional A7 fixture, drawings, and sourced hardware | No-cell validation | Replace solid layout tokens with flow-through surrogate/restriction coupons, add the missing dead-volume dye station, freeze bubble/waste stimuli and uncertainties, then build/quote pressure, collection, leak, dye, and waste modules with selected measurement hardware. |
| Media-only aseptic workflow | Live-cell planning | Prove no-cell gates, then media-only/aseptic transfer evidence. |
| IBC/safety workflow for AAV | AAV work | Define containment, waste handling, decontamination, documentation, and approvals. |

## Explicit Blockers

These are not negotiable in the first article:

- Do not send STL files alone as a vendor manufacturing definition.
- Do not treat the carrier, lid, dock, bulkhead, or gasket as a validated sterile wetted path.
- Do not route multiple AAV candidates through one cassette.
- Do not use the 16 slots as separate AAV lanes.
- Do not introduce recirculation until single-pass recovery, sterility, and carryover evidence exists.
- Do not run cells before A7 no-cell gates pass.
- Do not run AAV before media-only/aseptic evidence and safety/IBC workflow exist.
- Do not use older 4 x 5 / 20-position fixtures as proof for the 16-slot cassette.
- Do not silently relocate dock openings or erase additive bridges to make the
  fit-check appear to have continuous drainage; either validate the locked A0
  topology or revise A0 explicitly.

## Next Tickets

Recommended next work after A8:

1. Complete the A9 D0-D9 drawing and release-manifest package. The six-part plus stackup STEP draft exporter is already implemented; see `docs/sixteen_slot_cassette_a9_step_drawing_export_plan.md`.
2. A10 16-slot A7 validation fixture iteration into STEP/drawings or sourced bench fixture package.
3. A11 bought-component downselect for tubing, connectors, pump/pressure-control, sensors, labels, and collection hardware.
4. A12 RFQ packet draft with vendor questions, drawing notes, inspection table, and quote options.
