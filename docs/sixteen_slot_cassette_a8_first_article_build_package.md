# 16-Slot Cassette A8 First-Article Build Package

Ticket: T-E0187ED3

This A8 package consolidates A0-A7 into the first practical build package for the LaminarForge 16-slot AAV cassette. It defines what can be built now, what must be generated or selected before vendor RFQ, how the first article should be assembled, and which acceptance gates block cells or AAV.

This is not a final vendor drawing set, sterile-barrier validation, biological protocol, AAV containment claim, or live-cell release criterion. It is a first-article engineering package for a dry reusable cassette/dock stack plus a disposable one-condition fluid path.

## Package Status

| Lane | Status | Meaning |
| --- | --- | --- |
| Internal CAD fit-check package | Ready as v0 | The active Rust CAD bin builds and exports seven STL fit-check files. |
| Mechanical RFQ package | Not ready | Vendor manufacturing requires STEP files and 2D drawings, not STL as the manufacturing definition. |
| Disposable fluid-path package | Architecture ready, SKUs open | A6 defines topology and port roles, but final tubing, connectors, pump interface, and harness vendor are not selected. |
| No-cell validation package | First-pass CAD fixture available | A7 defines gates and fixture modules; `sixteen_slot_cassette_no_cell_validation_fixture` now exports a dry bench fixture package, while STEP/drawings and sourced hardware remain follow-up. |
| Media-only planning | Blocked until A7 gates pass | No-cell fit, leak, flow, bubble, dye recovery, and waste tests must pass first. |
| Live-cell/AAV work | Blocked | Requires A7 no-cell pass, media-only/aseptic evidence, safety/IBC workflow, and material compatibility data. |

## Controlling Inputs

| Input | A8 use |
| --- | --- |
| `docs/sixteen_slot_cassette_a0_interface_spec.md` | Controls slot map, dimensions, datums, one-condition semantics, dry structural policy, and downstream contracts. |
| `docs/sixteen_slot_cassette_a1_carrier_spec.md` | Controls carrier pocket, datum, drain, label, material, and handling assumptions. |
| `docs/sixteen_slot_cassette_a2_lid_clamp_spec.md` | Controls lid/clamp, imaging windows, captive fasteners, torque sequence, and stiffness requirements. |
| `docs/sixteen_slot_cassette_a3_gasket_spec.md` | Controls gasket material candidates, 2.40 mm cross-section, 25% target squeeze, grooves, hard stops, and leak/burst/reconnection logic. |
| `docs/sixteen_slot_cassette_a4_dfm_spec.md` | Controls manufacturing process, material, tolerance, finish, flatness, inspection, and RFQ requirements. |
| `src/bin/sixteen_slot_cassette_incubator_first_article.rs` | Active A5 integrated CAD generator and export source. |
| `docs/sixteen_slot_cassette_a6_disposable_fluid_path_spec.md` | Controls disposable one-condition single-pass harness, G/M/W port roles, prime/debubble path, waste path, and run record. |
| `docs/sixteen_slot_cassette_a7_no_cell_validation_fixture_spec.md` | Controls no-cell fixture architecture, test sequence, acceptance gates, and blockers before biology. |

## Locked Build Decisions

- First build format: 16 slots in a 4 x 4 cassette.
- Experimental unit: one cassette equals one AAV capsid/promoter/payload/dose/timing/media condition.
- Slot role: cell-type and technical readouts under the same exposure, not separate AAV candidates.
- Active CAD bin: `sixteen_slot_cassette_incubator_first_article`.
- Reusable structural stack: carrier, lid/clamp, dock plate, service bulkhead test block, window placeholder, gasket witness coupon.
- Wetted path: disposable commercial tubing, connectors, reservoirs, filters, and pump-contact tubing.
- Flow mode: one-condition pressure-limited single-pass flow.
- Recirculation: deferred.
- CO2 incubation: deferred.
- Older 4 x 5 / 20-position CAD: reference only until explicitly ported to the 16-slot first-article geometry and A6 port map.

## Build Command And Generated Outputs

Run from the `laminarforge-cad` repo using the MCP build tool:

```text
mcp__agentic-mcp__laminarforge_build
repo: laminarforge-cad
action: run
bin: sixteen_slot_cassette_incubator_first_article
```

Verified during A8 in the ticket worktree: the MCP build completed successfully and exported seven STL files.

| Output | Role | A8 disposition |
| --- | --- | --- |
| `output/sixteen_slot_cassette_lower_carrier.stl` | Lower carrier fit-check model | Internal fit-check only; convert to STEP/drawing before RFQ. |
| `output/sixteen_slot_cassette_lid_clamp.stl` | Lid/clamp fit-check model | Internal fit-check only; needs final captive hardware and drawing notes. |
| `output/sixteen_slot_cassette_window_placeholder.stl` | Retained window placeholder | Internal fit-check only; window material and sheet drawing remain open. |
| `output/sixteen_slot_cassette_gasket_witness_coupon.stl` | Gasket compression/leak witness coupon | Internal fit-check only; coupon drawing must duplicate final material/finish/groove notes. |
| `output/sixteen_slot_incubator_dock_plate.stl` | Dry dock plate and drain/logger support | Internal fit-check only; incubation control is not solved by this part. |
| `output/sixteen_slot_service_bulkhead_test_block.stl` | Dry service bulkhead placeholder | Internal fit-check only; bought connector cutouts are not finalized. |
| `output/sixteen_slot_cassette_incubator_first_article_assembly.stl` | Combined assembly view | Assembly review only; not a manufacturing source. |

The `output/` directory is intentionally ignored by git. Regenerate these files from source instead of committing generated STL artifacts.

## Custom Part Package

| Part | Current source | Preferred first-article process | Material direction | Critical notes |
| --- | --- | --- | --- | --- |
| Lower carrier | `sixteen_slot_cassette_lower_carrier.stl` | CNC machined plate after STEP/drawing generation | 6061-T651/T6 aluminum, clear Type II anodize option | Dry structural part; preserve A0 datums, 16 pockets, gasket lands, drain, labels, handling lands. |
| Lid/clamp | `sixteen_slot_cassette_lid_clamp.stl` | CNC machined or waterjet rough + secondary machining | 6061-T6/T651 aluminum or stainless alternate | Must use controlled compression, captive M4 hardware assumption, imaging keepouts, and torque sequence. |
| Window placeholder | `sixteen_slot_cassette_window_placeholder.stl` | CNC/laser-cut sheet after sheet drawing | Polycarbonate for fit-check; borosilicate/glass if imaging requires | Mechanically retained; adhesive-only retention is not the baseline. |
| Gasket witness coupon | `sixteen_slot_cassette_gasket_witness_coupon.stl` | CNC machined coupon | Same seal-band material/finish assumptions as final stack | Must duplicate gasket groove, stop, finish, and hardware strategy for evidence. |
| Dock plate | `sixteen_slot_incubator_dock_plate.stl` | CNC machined plate | 6061-T651/T6 aluminum or stainless alternate | Dry support, rail, drain, logger, and module-reservation hardware only. |
| Service bulkhead test block | `sixteen_slot_service_bulkhead_test_block.stl` | CNC machined dry block after connector selection | Aluminum or polycarbonate dry structure | Placeholder holes are not connector drawings and not sterile ports. |

## Bought Component BOM Assumptions

These are first RFQ/test assumptions, not final SKUs.

| Function | Starting assumption | Owner / closeout |
| --- | --- | --- |
| Lid fasteners | Stainless M4 x 0.7 captive panel screws with positive-drive heads | A2/A4/vendor to finalize length, head style, retainers, and torque. |
| Carrier receivers | Stainless threaded inserts, nut plates, or replaceable threaded receiver hardware | A4/vendor to select; post-install flatness inspection required. |
| Datum pins | 6 mm stainless ground dowel family | A1/A4 to finalize round/slotted locator strategy and bore tolerances. |
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

- STEP export for lower carrier, lid/clamp, window placeholder or sheet part, dock plate, service bulkhead, and gasket witness coupon.
- 2D drawing for each custom part with material, finish, tolerances, flatness, surface finish, datum scheme, inspection table, and no-biological-claim notes.
- Stackup drawing showing chip, carrier, gasket, lid/clamp, window, hard stops, and compression witness locations.
- Hardware table with fastener, insert, dowel, washer/retainer, and torque assumptions.
- Gasket drawing or supplier print with cross-section, compound, splice/join strategy, compression target, and lot traceability.
- Label drawing with global cassette/condition ID land, per-slot labels, slot-1 orientation, and barcode size.
- Connector panel drawings using selected bought connector datasheets, not placeholder hole diameters.
- No-cell validation fixture drawing or sourced fixture plan for pressure, bubble, flow, dye, waste, and repeat-cycle gates.
- Vendor note that the quote is a mechanical no-cell first article only and makes no sterile, clinical, live-cell, AAV containment, or biological release claim.

## Assembly Order

Use this order for the first physical build. Do not skip metrology gates because later no-cell tests can pass while hidden damage or overcompression remains.

1. Regenerate STL outputs from the active CAD bin and verify the seven expected files exist.
2. Convert custom parts to STEP and generate drawings, or keep the run as internal fit-check only.
3. Inspect machined carrier, lid, dock, bulkhead, window, and coupon before installing hardware.
4. Verify carrier dimensions: pocket array, datum bores, rear/left datum edges, gasket lands, drain, label lands, and optical openings.
5. Verify lid dimensions: view openings, fastener holes, alignment ears, gasket grooves/stops, window retention, and torque sequence marks.
6. Verify dock dimensions: rear/left rails, drain gutters, sump visibility, logger pockets, carrier support flatness, and robot/manual handling keepouts.
7. Install threaded receivers, captive hardware, dowels, and labels only after pre-install metrology passes.
8. Recheck flatness, burrs, datum fits, and label placement after hardware installation.
9. Install surrogate Rev C chips or gauge chips into all 16 pockets and confirm 16/16 seating without forcing.
10. Install gasket or coupon material using the A3 process and verify witness shims.
11. Install retained window placeholder without point-loading or blocking view openings.
12. Close lid/clamp using the A2 paired cross-pattern torque sequence and record driver setting, hardware lot, and witness results.
13. Dock cassette on rear/left rails and confirm the dock does not twist the carrier or hide drain/witness paths.
14. Connect the A6 disposable harness for dry topology inspection.
15. Run the A7 no-cell sequence before any media-only, live-cell, or AAV planning.

## QA And Inspection Checklist

| Area | Required evidence |
| --- | --- |
| Slot map | 4 x 4 row-major S01-S16 confirmed against A0. |
| Dimensions | Carrier, lid, dock, bulkhead, window, and coupon measured against drawings or CAD fit-check targets. |
| Datums | Rear/left rail registration and D1/D2 locator behavior verified without overconstraint. |
| Pocket fit | 16/16 surrogate chips seat; no burrs, binding, or pocket-floor debris. |
| Gasket compression | 20-30% guard band with 25% target; 1.68-1.92 mm compressed height if using 2.40 mm nominal gasket. |
| Fasteners | Captive hardware retained, torque sequence marked, receiver installation inspected. |
| Imaging | 16 view openings and carrier optical windows unobstructed by lid, gasket, labels, tubing, or window retention. |
| Drain/leak witness | Gutter, drain port, sump, leak tray, and witness-paper paths visible when docked. |
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
| Bubble challenge | Introduced bubble clears to W1/W3 without reaching chip inlet witnesses. |
| Leak | No visible dye leak and <=5% pressure decay over 10 minutes at the selected no-cell test pressure. |
| Flow balance | Row CV <=10%; slot CV <=10-15%; pressure drift within +/-5% after stabilization. |
| Restriction detection | Nominal, low, high, blocked, and bypass coupons produce expected pressure/flow signatures. |
| Dead volume | Recovery reconciles within +/-10%; unrecovered volume below A6 target or explicitly accepted. |
| Waste safety | No backflow, siphon, or uncontrolled overflow into cassette or bench. |
| Repeatability | Cycles 1, 5, 10, and 25 pass the same required gates. |

## First-Article Run Record

Every first-article build/run should create a searchable artifact with:

- Ticket ID and artifact ID.
- Commit hash and CAD bin used.
- Build command and seven-output export confirmation.
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
| STEP export and 2D drawings | Vendor RFQ | Create STEP/drawing package from the active CAD baseline. |
| Final Rev C chip lot dimensions | Carrier drawing release | Measure chip lot and resolve pocket target versus 1.20 mm CAD fit-check clearance. |
| Final dowel/bore strategy | Carrier/lid drawing release | Select D1/D2 round/slotted locator implementation and bore tolerances. |
| Final hardware and torque | Physical build release | Select captive M4 hardware, receiver type, insert process, washers/retainers, and torque-to-stop value. |
| Final gasket compound and supplier | Physical build release | Select silicone/EPDM compound, cross-section tolerance, splice strategy, and coupon plan. |
| Final seal-band finish/coating plan | Vendor RFQ | Decide masked anodize, post-machining, or as-machined seal bands. |
| Final window material | Imaging and cleaning tests | Choose polycarbonate or glass and define retention/cleaning compatibility. |
| Final connector and tubing SKUs | Fluid-path RFQ and validation fixture | Select bought connector family, tubing OD/ID, weldability, pressure rating, and panel cutouts. |
| Pump/pressure-control interface | A7 validation | Select pressure-limited pump/control mode and sensor range. |
| 16-slot A7 fixture CAD or sourced fixture | No-cell validation | Build/quote fixture modules for pressure, collection, bubble, leak, dye, and waste tests. |
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

## Next Tickets

Recommended next work after A8:

1. A9 STEP/drawing export plan for the six custom RFQ parts plus stackup drawing: see `docs/sixteen_slot_cassette_a9_step_drawing_export_plan.md`.
2. A10 16-slot A7 validation fixture iteration into STEP/drawings or sourced bench fixture package.
3. A11 bought-component downselect for tubing, connectors, pump/pressure-control, sensors, labels, and collection hardware.
4. A12 RFQ packet draft with vendor questions, drawing notes, inspection table, and quote options.
