# 16-Slot Cassette A7 No-Cell Validation Fixture Spec

Ticket: T-1C88E651

This A7 spec defines the no-cell validation fixture and bench acceptance plan for the first physical LaminarForge 16-slot AAV cassette build. It is subordinate to `docs/sixteen_slot_cassette_a0_interface_spec.md` and validates the A5 mechanical stack plus the A6 disposable fluid path before any cells, AAV, or sterile-use claims.

This is not a biological protocol, live-cell release criterion, sterile-barrier validation, AAV containment claim, or final vendor drawing. It is the controlled bench gate that proves the cassette can be assembled, sealed, primed, flowed, challenged with bubbles, drained, and measured repeatably using water, dye, or media-equivalent non-biological fluids.

## A7 Baseline

| Item | A7 requirement |
| --- | --- |
| First-build format | 16 slots in a 4 x 4 cassette. |
| Experimental unit | One cassette equals one AAV capsid/promoter/payload/dose/timing/media condition. |
| Validation material | DI water, dyed water, viscosity-adjusted surrogate, or media-equivalent non-biological fluid only. |
| Chip substitute | No-cell surrogate chips or restriction coupons matching the Rev C external footprint and inlet/outlet interface. |
| Flow path | A6 disposable one-condition single-pass harness. |
| Recirculation | Not validated by A7. Any recirculating future design needs its own validation ticket. |
| Mixed-AAV routing | Blocked. The fixture must not normalize using one cassette as a multi-AAV test article. |
| Incubation control | Out of scope except for dry fit, condensation/drain observation, and module keepout preservation. |

The active first-pass CAD source for this fixture is `src/bin/sixteen_slot_cassette_no_cell_validation_fixture.rs`. It exports the 4 x 4 cassette nest, surrogate/restriction coupon set, pressure sensor bar, flow collection deck, bubble challenge station, leak witness tray, waste/backflow station, run-record plate, and review assembly under `output/no_cell_fixture/`.

The older `flow_pressure_validation_fixture`, `closed_chip_inlet_outlet_dead_volume_dye_recovery_station`, and related 20-position CAD generators are reference concepts only. They do not validate the 16-slot first article until explicitly ported to the 4 x 4 geometry, A5 datums, and A6 port map.

## Validation Questions

A7 answers these questions before the cassette is allowed into media-only or live-cell planning:

1. Can the A5 carrier/lid/gasket stack be assembled to the A3 compression target without damaging chips, blocking imaging windows, or losing datum registration?
2. Can the A6 harness be connected according to the port map without ambiguous routing, wrong-condition assembly, or unrestrained tubing movement?
3. Can all 16 no-cell surrogate paths prime without visible bubbles at the chip inlet witnesses?
4. Are row and slot flow rates balanced enough for one-condition AAV exposure screening?
5. Does pressure decay, visible dye inspection, and leak-tray capture show the stack is sealed at the selected test pressure?
6. Can the system detect known restriction, bypass, occlusion, bubble, waste backflow, and dead-volume failure modes?
7. Does repeated assembly remain within the same acceptance limits at the defined cycle checkpoints?

Failing any of these questions blocks live cells and AAV. Do not work around a failed bench gate with operator judgment.

## Fixture Architecture

The A7 fixture is a bench validation deck that holds the real A5 mechanical stack, the A6 disposable harness, and a controlled set of no-cell surrogate chips. It should be physically separate from the biological workflow until the sterile/aseptic path is validated.

Required fixture modules:

| Module | Function | Requirement |
| --- | --- | --- |
| Cassette nest | Locates the A5 carrier/lid/gasket stack | Uses the A0 rear/left datum scheme, preserves drain visibility, and exposes slot labels. |
| Surrogate-chip set | Replaces live chips for hydraulic tests | Sixteen footprint-matched coupons with known restriction and visible inlet/outlet witness regions. |
| Restriction coupon set | Exercises pressure/flow detection | Nominal, low-resistance, high-resistance, blocked, and bypass coupons must produce a predictable pressure order. |
| Pressure sensor bar | Measures system and branch pressure | At minimum: common upstream, four row branches, waste/backpressure point, and optional selected slot outlets. |
| Flow collection deck | Captures slot or row outlet volumes | Sixteen labeled vial nests or gravimetric collection positions aligned to S01-S16. |
| Bubble challenge station | Introduces known bubbles upstream and by row | Includes a visible challenge inlet and a controlled path to W1/W3 waste during prime/debubble mode. |
| Leak witness tray | Detects visible dyed leaks | Wicking paper, clear leak moat, or dye-visible tray under gasket, connector, and waste handoff regions. |
| Waste/backflow station | Challenges siphon, overflow, and reverse pressure | Allows waste container high/low head tests without exposing the bench or cassette. |
| Dead-volume station | Measures dye recovery and flush volume | Includes dye injection, flush/recovery collection, and optical or photographic color reference lands. |
| Run-record plate | Links physical setup to data | Shows cassette ID, harness ID, condition ID, slot map revision, operator, date/time, and pass/fail state. |

The fixture should make the wrong assembly obvious. Ports, rows, slots, and collection nests must use the same S01-S16 and G/M/W naming as A0/A6.

## Port And Measurement Map

A7 uses the A6 logical port assignments without freezing final connector SKUs.

| A6 port | A7 use |
| --- | --- |
| G0 | Reservoir/bag vent or controlled vent during no-cell pressure checks. |
| G1 | Regulated pressure input when pressure-driven tests are used. |
| G2 | Relief/exhaust or pressure-test vent to safe capture. |
| G3 | Reserved; cap unless a module-specific test needs it. |
| M0 | Main condition/surrogate feed inlet. |
| M1 | Rinse or media-equivalent feed inlet. |
| M2 | Prime/debubble bypass inlet. |
| M3 | Pressure/leak-test and dye-test inlet. |
| M4 | Sample/QC loop inlet if installed. |
| M5 | Inline sensor/cartridge service inlet if installed. |
| M6 | Spare; cap during first A7 testing. |
| W0 | Main chip outlet waste. |
| W1 | Prime/debubble purge waste. |
| W2 | Sample/QC waste if installed. |
| W3 | Leak-test/dye-test waste. |
| W4 | Relief/overflow/emergency waste. |

Minimum measurement points:

- Common upstream pressure before the branch split.
- Row branch pressure for R1, R2, R3, and R4.
- Waste/backpressure measurement near W0/W3.
- Sixteen collected outlet volumes or masses for S01-S16.
- Prime volume and time to bubble-clear state.
- Dye input, recovered slot output, recovered waste, and residual/unrecovered estimate.
- Leak-tray wet/dry or dye-visible result for the cassette, bulkhead, and waste handoff.

Preferred optional measurements:

- Inline flow sensor before the branch split.
- Row-level or selected-slot inline flow sensors.
- Upstream and downstream bubble-event sensor, if available.
- Temperature of the fluid and bench during long pressure/flow runs.
- Photographic record of gasket witness, bubble windows, leak tray, collection deck, and final dye state.

## No-Cell Test Sequence

Run the sequence in order. A later pass does not override an earlier failure unless the root cause is corrected and the affected test is repeated.

### 1. Incoming Inspection

- Confirm cassette revision, carrier/lid/gasket revision, harness revision, surrogate-chip set revision, and fixture revision.
- Confirm all sixteen surrogate chips fit their pockets without forcing.
- Confirm slot labels, row labels, G/M/W port labels, and collection positions match the run record.
- Confirm unused ports are capped and documented.

Acceptance:

- 16/16 surrogate chips seat without interference.
- No visible chip-pocket obstruction, datum damage, gasket tear, clogged port, or mislabel.
- Any untracked harness or surrogate lot fails the run.

### 2. Dry Assembly And Compression Witness

- Assemble carrier, gasket, surrogate chips, lid/clamp, and fasteners using the A2/A3 torque sequence.
- Verify alignment through datum bosses and lid openings.
- Check gasket witness points and hard stops.
- Confirm imaging windows remain unobstructed.

Acceptance:

- Gasket witness height is within the A3 20-30% compression guard band, with the nominal target at 25%.
- If measured directly from the 2.40 mm nominal gasket assumption, compressed height should be 1.68-1.92 mm.
- No window, port, label, drain, or witness path is blocked by clamp hardware or tubing.

### 3. Harness Topology Check

- Connect the A6 disposable harness to the fixture without fluid.
- Trace reservoir, pump/pressure interface, prime bypass, four row feeds, sixteen slot inlets, sixteen slot outlets, row waste branches, and common waste.
- Scan or record cassette ID, harness ID, condition ID, slot map revision, and port map revision.

Acceptance:

- Every pigtail maps to the correct S01-S16 position.
- No tube crosses imaging openings, gasket lands, hard stops, datum features, or drain inspection regions.
- Wrong-port ambiguity is grounds for fixture redesign, not operator training alone.

### 4. Prime-To-Waste Validation

- Fill the upstream path with water or dye-compatible surrogate fluid.
- Prime M0/M2 to W1 until the common upstream line, bubble window, and prime bypass are bubble-free.
- Prime row branches one row at a time.
- Prime each slot through the surrogate chip and collect outlet fluid.

Acceptance:

- 16/16 slot paths prime.
- No visible bubbles remain at common upstream, row high points, chip inlet witnesses, or outlet witnesses.
- Prime volume, time, pressure, and recovered waste are recorded.
- Any persistent bubble that requires manual tapping or tube manipulation is a design issue to log.

### 5. Bubble Challenge

- Introduce a known upstream bubble or air slug using the bubble challenge station.
- Run prime/debubble mode to W1/W3.
- Repeat by row if the harness has row-level high points.
- Observe chip inlet witnesses and outlet/bubble sensors if installed.

Acceptance:

- The introduced bubble clears to the prime/debubble waste path.
- No bubble reaches a chip inlet witness during the validated prime/debubble mode.
- No new trapped bubble remains at a row branch high point after the specified clearing volume.
- The clearing volume and time are recorded for the volume ledger.

### 6. Pressure Decay And Visible Leak Test

- Fill the isolated loop with dyed liquid or the selected leak-test fluid.
- Stabilize the fixture for thermal and compliance effects.
- Pressurize to the selected no-cell leak-test pressure.
- Hold for 10 minutes while logging pressure and inspecting leak-witness regions.

Acceptance:

- No visible dyed leak at chip interfaces, gasket perimeter, bulkhead, connector handoffs, waste handoff, or drain paths.
- Pressure decay is <=5% over 10 minutes after stabilization.
- Test pressure is 2x selected operating pressure or A3's 35 kPa isolated-loop target, whichever is more appropriate and still below the weakest component limit.
- Routine planned operation must remain <=50% of the weakest validated leak/burst/connector limit.

If the fixture uses a known artificial leak to verify the detection method, that leak coupon must be physically isolated from release testing and clearly marked as a detector-verification tool.

### 7. Flow Balance Test

- Run the selected pump or pressure-control mode through all sixteen nominal surrogate chips.
- Collect outlet fluid from each slot for a fixed interval or fixed input volume.
- Weigh or measure each S01-S16 collection position.
- Repeat after the system reaches pressure and compliance steady state.

Acceptance:

- Row collected-volume coefficient of variation is <=10%.
- Slot collected-volume coefficient of variation is <=10-15% until actual chip restriction data supports a tighter limit.
- Pressure drift is within +/-5% after stabilization.
- Any systematic row or corner bias must be corrected or explicitly carried into the next design revision.

Gravimetric collection is the preferred early measurement because low-flow sensors can have substantial uncertainty at microfluidic flow rates.

### 8. Restriction, Occlusion, And Bypass Challenge

- Replace selected nominal coupons with known low-resistance, high-resistance, blocked, and bypass coupons.
- Run the pressure/flow profile used for nominal testing.
- Confirm the pressure sensor bar and collection deck detect the expected ordering.

Acceptance:

- Blocked coupon shows high upstream/row pressure and low/no collection at the expected slot.
- Bypass or low-resistance coupon shows low local restriction and abnormal collection relative to nominal.
- High-resistance coupon produces pressure and flow values between nominal and blocked states.
- The fixture identifies the affected row/slot without relying on visual inspection alone.

### 9. Dead-Volume And Dye-Recovery Test

- Inject a known dye volume through M3 or the selected dye-test inlet.
- Flush using the A6 planned prime/rinse path.
- Collect S01-S16 outlet volume, W0/W3 waste volume, and residual visible dye state.
- Compare recovered mass/volume against input and the A6 dead-volume ledger.

Acceptance:

- Input, slot output, waste output, and residual estimate reconcile within +/-10% for first article.
- Total unrecovered dead volume remains less than one chip dose or less than 10% of the formulated cassette condition volume, whichever is more conservative, unless the integrator explicitly accepts the reagent loss.
- Persistent dye retention in connectors, filters, bubble elements, or dead legs blocks expensive vector work until the loss is understood.

### 10. Waste Backflow, Siphon, And Overflow Challenge

- Run normal flow into the selected sealed waste bag or bottle.
- Challenge waste placement below and above cassette height within the planned bench/module envelope.
- Apply controlled reverse head or backpressure using the waste/backflow station.
- Trigger the overflow/relief path if W4 is installed.

Acceptance:

- No waste backflow reaches chip outlets or row branches.
- No siphon drains the cassette outside commanded flow.
- Relief/overflow routes to secondary containment without wetting dry cassette structures.
- Waste identity and condition ID remain linked to the cassette run.

### 11. Repeat Assembly And Reconnection Cycling

- Repeat dry assembly, prime, leak, and flow checks at cycles 1, 5, 10, and 25.
- Include connector reconnection cycles when the selected bought connector or tube-welding workflow allows repeat handling.
- Record gasket, fastener, connector, surrogate-chip, and harness changes.

Acceptance:

- Cycle checkpoints meet the same prime, leak, and flow targets as cycle 1.
- Any loss of compression witness, cracked surrogate, thread damage, stripped insert, connector loosening, or route-label degradation blocks the affected design revision.

## Acceptance Gate Summary

The first-article cassette can move from A7 into media-only planning only when all required gates pass:

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

No single aggregate pass/fail score is allowed. Each gate must be recorded independently because different downstream fixes own different failures.

## Run Record Fields

Each A7 run should produce a searchable markdown artifact or lab notebook entry with:

- Ticket ID and artifact ID.
- Cassette serial/revision.
- Carrier/lid/gasket revision.
- Surrogate-chip and restriction-coupon revision.
- Harness revision, lot, tubing OD/ID, connector family, and sterilization/clean state if relevant.
- Fluid type, viscosity assumption if used, dye type/concentration, and temperature.
- Pump or pressure-controller model, settings, pressure limit, and calibration state.
- Slot map, row map, and G/M/W port map revision.
- Torque sequence, torque value, compression witness result, and operator.
- Prime volume/time, bubble challenge volume/time, leak-test pressure, hold time, pressure trace, and visible leak result.
- Row and slot collected volumes or masses.
- Restriction/occlusion/bypass challenge results.
- Dead-volume and dye-recovery mass balance.
- Waste/backflow challenge result.
- Cycle count and any failed part replacements.
- Photographs of setup, witness regions, collection deck, leak tray, bubble windows, and final dye state.

## Research Basis

External sources checked for A7:

| Source | Useful finding | A7 implication |
| --- | --- | --- |
| Keiser et al., "Overcoming technological barriers in microfluidics: Leakage testing", Frontiers in Bioengineering and Biotechnology, 2022 | Microfluidic leaks are common, difficult to detect at small volume/flow scales, and can occur at connectors, interfaces, channels, materials, or upstream/downstream components. The review emphasizes quantitative, application-specific leakage testing and notes the lack of universal standards. | A7 cannot rely on visual confidence alone. It combines pressure decay, dyed liquid inspection, leak-witness capture, connector checks, and run-recorded acceptance thresholds. |
| Bozorgnezhad et al., "Development of a Tool for Verifying Leakage Detection in Microfluidic Systems", Micromachines, 2025 | Leakage-detection systems can be verified with controlled hydrodynamic-resistance elements and target leakage rates. The paper also notes low-flow sensor uncertainty and the usefulness of gravimetric measurement. | A7 includes artificial leak/restriction coupons for detector verification and uses gravimetric slot collection as the preferred early flow-balance measurement. |
| de Graaf et al., "Pressure-Driven Perfusion System to Control, Multiplex and Recirculate Cell Culture Medium for Organs-on-Chips", Micromachines, 2022 | Controlled pressure-driven perfusion can support multiplexed organ-chip flow; compliant tubing/devices and parallel branches need pressure/flow validation to avoid unstable or unequal flow. | A7 requires upstream, row, and waste pressure measurements plus row/slot flow CV gates before using one cassette as a controlled exposure panel. |
| Vegunta et al., "Orientation-independent bubble trap with internal partition for robust operation of microfluidic systems", Lab on a Chip, 2025 | Bubbles can block channels, damage cells, and distort measurements; robust systems test bubble removal with flow sensors before and after traps over long operation. | A7 treats bubble challenge as a required bench gate and records whether bubbles clear to waste without reaching chip inlet witnesses. |

Internal source anchors:

- A0 freezes the 16-slot geometry, slot map, datums, condition semantics, and disposable wetted-path policy.
- A3 provides the starting gasket compression guard band and leak/burst coupon targets.
- A6 provides the disposable harness topology, port map, prime/debubble path, waste/backflow requirement, pressure/flow targets, and dead-volume ledger.
- `docs/tissue_chip_reproducibility_controls.md` identifies no-cell flow, pressure mapping, leak-witness, and bubble challenge as required engineering gates before live cells.

## Handoffs

A7 hands these decisions to the first-article build package:

- Iterate the new `sixteen_slot_cassette_no_cell_validation_fixture` CAD outputs into either STEP/drawing release files or sourced bench fixture selections.
- Select actual pressure sensors, flow sensors if used, scales, gauges, fittings, tubing, connector SKUs, collection vials, dye, and leak-witness materials.
- Define the final operating pressure and test pressure once pump, tubing, connector, surrogate-chip, and gasket coupon data are known.
- Add drawing notes that the older 20-position fixture CAD is not the active first-build validation fixture.
- Create acceptance-test forms or scripts for run-record capture.

A7 blocks:

- Live cells before all required no-cell gates pass.
- AAV before media-only and sterility/aseptic workflow evidence exists.
- Mixed-AAV routing in one cassette.
- Recirculation until a separate recirculation validation fixture and recovery/carryover evidence exist.
- Any use of the older 4 x 5 / 20-position validation fixtures as proof for the 16-slot first article.
