# 16-Slot Cassette A7 No-Cell Validation Fixture Spec

Ticket: T-1C88E651

This A7 spec defines the no-cell validation fixture and bench acceptance plan for the first physical LaminarForge 16-slot AAV cassette build. It is subordinate to `docs/sixteen_slot_cassette_a0_interface_spec.md` and validates the A5 mechanical stack plus the A6 disposable fluid path before any cells, AAV, or sterile-use claims.

This is not a biological protocol, live-cell release criterion, sterile-barrier validation, AAV containment claim, or final vendor drawing. It specifies the controlled bench gate that must eventually prove the cassette can be assembled, sealed, primed, flowed, challenged with bubbles, drained, and measured repeatably using water, dye, or media-equivalent non-biological fluids.

Implementation status: the current Rust output is a dry layout/mockup package,
not an executable hydraulic validation fixture. Its surrogate chips and
restriction tokens are solid geometry, and it does not yet contain functional
flow passages, characterized resistance coupons, or the required dead-volume
dye station. Therefore it proves packaging and measurement-position intent
only. It cannot pass A7, unblock media-only planning, or support live-cell/AAV
claims.

The controlled operator procedure and blank execution forms are now defined in
`docs/sixteen_slot_cassette_no_cell_bench_validation_protocol.md` and
`docs/sixteen_slot_cassette_no_cell_bench_validation_data_sheets.md`. Their
release does not remove the functional-fixture, selected-hardware,
characterized-coupon, calibration, or frozen-stimulus blockers above.

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
| Active mechanical contract | 699.04 x 541.92 x 24.00 mm carrier base body with 31.35 mm carrier true overall Z and 41.35 mm closed carrier/closure/lid stack; 151.76 x 109.48 mm slot pitch and 583.04 x 413.92 mm array. |
| Seal/closure contract | Sixteen independent, nonoverlapping per-slot lands, the perimeter land, and all hard stops top out at 7.35 mm and meet the lid underside; the 2.40 mm gasket compresses in 1.80 x 3.20 mm lid grooves. |

The active first-pass layout source for this fixture is `src/bin/sixteen_slot_cassette_no_cell_validation_fixture.rs`. It consumes the shared machine-readable A0 contract for cassette dimensions and exports the 4 x 4 cassette nest, solid surrogate/restriction layout set, pressure-sensor placement bar, flow-collection layout deck, bubble-station envelope, leak-witness tray, waste/backflow envelope, run-record plate, and review assembly under `output/no_cell_fixture/`. Local fixture dimensions may describe the surrounding bench deck, but they must not duplicate or override cassette geometry constants. Functional passages, sourced hardware, calibration, and characterized challenge articles remain blocked work.

The older `flow_pressure_validation_fixture`, `closed_chip_inlet_outlet_dead_volume_dye_recovery_station`, and related 20-position CAD generators are reference concepts only. They do not validate the 16-slot first article until explicitly ported to the 4 x 4 geometry, A5 datums, and A6 port map.

The shared machine-readable A0 contract supersedes the earlier overlapping seal-loop and unequal-height geometry. A7 fixtures use the corrected 24.00 x 24.00 mm inter-chip gutter, 6.00 mm inter-land clearance, common 7.35 mm seal-land/stop elevation, and 651.04 x 481.92 mm outer / 639.04 x 469.92 mm inner leak gutter rather than reproducing the defective predecessor geometry. The gutter is 6.00 mm wide x 3.00 mm deep with a 5.00 mm separating web; its 8.00 mm drain is centered at (315.52, -254.96, 10.00) mm and runs 40.00 mm along Y. The fixture must also preserve the nine 4.00 mm-diameter inter-slot stops, the 4.00 mm-wide perimeter web stops, the outside-gutter fasteners at X = +/-332.52 mm or Y = +/-247.96 mm, their 3.30 mm carrier pilots, and the 7.00 mm service relief below closure.

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
| Cassette nest | Locates the A5 carrier/lid/gasket stack | Seats the carrier bottom directly on datum A; contacts nominal rear, left, and front carrier edges with the rear/left rail inner faces and front lip; checks the D1 round and D2 relieved functional locators without loading D3/D4; preserves drain visibility; and exposes labels outside the gutter. |
| Surrogate-chip set | Replaces live chips for hydraulic tests | Sixteen footprint-matched coupons with known restriction and visible inlet/outlet witness regions. |
| External fault-insertion manifold and coupon set | Exercises pressure/flow detection without disturbing closure | Nominal, low-resistance, high-resistance, blocked, and bypass coupons must be independently characterized and selectable at frozen reference nodes without opening/retorquing the cassette, disturbing the harness/source condition, or revealing the blinded key. |
| Pressure sensor bar | Measures system and branch pressure | At minimum: common upstream, four row branches, waste/backpressure point, and optional selected slot outlets. |
| Flow collection deck | Captures slot or row outlet volumes | Sixteen labeled vial nests or gravimetric collection positions aligned to S01-S16. |
| Bubble challenge station | Introduces known bubbles upstream and by row | Includes a visible challenge inlet and a controlled path to W1/W3 waste during prime/debubble mode. |
| Leak witness tray | Detects visible dyed leaks | Wicking paper, clear leak moat, or dye-visible tray under gasket, connector, and waste handoff regions. |
| Waste/backflow station | Challenges siphon, overflow, and reverse pressure | Allows waste container high/low head tests without exposing the bench or cassette. |
| Dye/hold-up station | Measures tracer recovery and physical fluid hold-up | Includes quantitative dye injection/recovery, mutually exclusive fraction collection, and an independent volumetric hold-up ledger. |
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
- Dye input, mutually exclusive recovered fractions, signed mass bias, tracer-equivalent unresolved loss, and separate physical hold-up upper bound.
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
- Confirm the global 96.00 x 12.00 mm barcode land and 118.00 x 10.00 mm text
  land remain at Y = -263.96 mm outside the leak gutter and clear of the front
  receiver row; confirm the bulkhead sensor/backplane cut remains centered at
  X = 320.00 mm without conflicting with W4.
- Confirm the 16 fastener centers clear the gutter, all 16 carrier receiver
  placeholders measure 3.30 mm before final threading/inserts, and the nine
  internal plus perimeter hard stops do not intersect a seal.
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
- The 7.35 mm per-slot lands, perimeter land, and hard stops contact the lid
  underside on one closure plane matching nominal chip protrusion. The nominal
  1.80 mm compressed gasket height is contained by the lid groove cavity; it is
  not a gap above a shorter land.
- No window, port, label, drain, or witness path is blocked by clamp hardware or tubing.

### 3. Harness Topology Check

- Connect the A6 disposable harness to the fixture without fluid.
- Trace reservoir, pump/pressure interface, prime bypass, four row feeds, sixteen slot inlets, sixteen slot outlets, row waste branches, and common waste.
- Scan or record cassette ID, harness ID, condition ID, slot map revision, and port map revision.

Acceptance:

- Every pigtail maps to the correct S01-S16 position.
- No tube crosses imaging openings, gasket lands, lid grooves, either hard-stop
  family, datum features, or drain inspection regions.
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

- Introduce a calibrated upstream bubble or air slug using the bubble challenge station. Freeze its volume, injection uncertainty, location, flow condition, and photographic/optical evidence method in the approved run record before testing; an unset stimulus is an automatic gate failure.
- Run prime/debubble mode to W1/W3.
- Repeat by row if the harness has row-level high points.
- Observe chip inlet witnesses and outlet/bubble sensors if installed.

Acceptance:

- The introduced bubble clears to the prime/debubble waste path.
- No bubble reaches a chip inlet witness during the validated prime/debubble mode.
- No new trapped bubble remains at a row branch high point after the preapproved maximum clearing volume and time. An unset clearing limit is an automatic gate failure.
- The clearing volume and time are recorded for the volume ledger.

### 6. Pressure Decay And Visible Leak Test

- Fill the isolated loop with dyed liquid or the selected leak-test fluid.
- Stabilize the fixture for thermal and compliance effects.
- Pressurize to the selected no-cell leak-test pressure.
- After source isolation, acquire at least 661 seconds while inspecting
  leak-witness regions. Use the protocol-defined mean differential-pressure
  windows at t = 0-60 seconds and t = 600-660 seconds; their centers are exactly
  10 minutes apart.

Acceptance:

- No visible dyed leak at chip interfaces, gasket perimeter, bulkhead, connector handoffs, waste handoff, or drain paths.
- Pressure decay calculated from the two protocol-defined window means is <=5%.
- The installed system is tested at 1.5x selected maximum operating pressure using liquid and below every installed component's qualified proof-pressure limit. A selected part that cannot support that pressure fails selection.
- A separate representative gasket coupon or isolated gasket loop passes A3's max(35 kPa gauge, 1.5x maximum operating pressure) qualification gate.
- Routine planned operation must remain <=50% of the lowest credible temperature-derated failure/burst/connector limit after approved engineering margin and must not exceed a supplier-rated working pressure. The 1.5x installed proof checkpoint and a lone coupon result are margin evidence, not operating ratings.

If the fixture uses a known artificial leak to verify the detection method, that leak coupon must be physically isolated from release testing and clearly marked as a detector-verification tool.

### 7. Flow Balance Test

- Run the selected pump or pressure-control mode through all sixteen nominal surrogate chips.
- Collect outlet fluid from each slot for a fixed interval or target input volume while independently measuring actual input with a traceable source balance, calibrated displacement, or reference-flow method.
- Weigh or measure each S01-S16 collection position with matched evaporation blanks.
- Repeat after the system reaches pressure and compliance steady state.

Acceptance:

- Row collected-volume coefficient of variation is <=10%.
- Slot collected-volume coefficient of variation is <=10% for the characterized nominal no-cell surrogate set. Actual-chip limits require a separate evidence-backed gate.
- Every lane, row, mean, total, and collected-versus-measured-input recovery result remains in its frozen delivery band after uncertainty; CV alone cannot pass a uniformly starved system.
- Pressure drift is within +/-5% after stabilization for positive references; near-zero/negative gauge channels use a frozen absolute-kPa rule.
- Any systematic row or corner bias must pass the frozen spatial test or remain on HOLD for design review.

Gravimetric collection is the preferred early measurement because low-flow sensors can have substantial uncertainty at microfluidic flow rates.

### 8. Restriction, Occlusion, And Bypass Challenge

- Use a hydraulically equivalent, externally accessible fault-insertion manifold to select known low-resistance, high-resistance, blocked, and bypass challenges at the frozen nodes without replacing an in-pocket surrogate or disturbing the closure/harness/source condition. The current in-pocket solid layout tokens do not satisfy this requirement.
- Characterize each functional coupon independently before the cassette challenge. Freeze a finite resistance band and uncertainty for flowing coupons; for blocked coupons, freeze a one-sided maximum-flow/resistance lower bound and pressure signature because R = delta P / Q is not finite below LOQ. The current solid layout tokens do not satisfy this prerequisite.
- Run the pressure/flow profile used for nominal testing.
- Confirm the pressure sensor bar and collection deck detect the expected ordering.

Acceptance:

- Each flowing coupon remains within its preapproved finite resistance band; each blocked coupon satisfies its one-sided flow/resistance and pressure-signature limits.
- Blocked, bypass/low-resistance, high-resistance, and nominal signatures are separated by at least three combined standard uncertainties at the selected flow condition.
- The fixture identifies the affected row/slot without relying on visual inspection alone.

### 9. Dye-Recovery And Physical Hold-Up Test

- Inject a known dye volume through M3 or the selected dye-test inlet.
- Flush using the A6 planned prime/rinse path.
- Collect mutually exclusive S01-S16, W0/W3, rinse, leak/drain, and residual fractions so no fluid is counted twice.
- Compare corrected tracer mass recovery against input and use a separate volumetric inlet/output/rinse/residual ledger for physical hold-up.

Acceptance:

- Corrected tracer mass reconciles within +/-10% for first article.
- The one-sided upper bound on physical retained/hold-up volume remains below one chip dose or 10% of the formulated cassette condition volume, whichever is more conservative. Tracer non-closure alone is not labeled hydraulic dead volume. This gate cannot be waived.
- Dye above the validated location-specific detection threshold in connectors, filters, bubble elements, dead legs, gasket interfaces, or dry structure blocks expensive vector work until the loss is understood.

### 10. Waste Backflow, Siphon, And Overflow Challenge

- Run normal flow into the selected sealed waste bag or bottle.
- Challenge waste placement below and above cassette height within the planned bench/module envelope.
- Apply the preapproved high/low waste elevations and reverse head/backpressure using the waste/backflow station. The run record must state exact elevations, pressure, dwell, uncertainty, and secondary-containment capacity; unset challenge values are an automatic gate failure.
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
| Bubble challenge | Calibrated, preapproved bubble stimulus clears to W1/W3 within the frozen clearing volume/time without reaching chip inlet witnesses; unset stimulus/limits fail. |
| Leak | Installed system has no visible dye leak and guard-banded decay `100 x (P0 - P10) / P0 <= 5%` at 1.5x maximum operating pressure, using no active makeup, at least 661 s acquisition, P0 mean over t = 0-60 s, and P10 mean over t = 600-660 s; representative isolated gasket loop also passes A3 qualification. |
| Flow balance | Row and slot CV are each <=10%; every lane/row/mean/total and measured-input recovery stays within frozen bands; pressure drift passes its relative or near-zero absolute rule. |
| Restriction detection | Independently characterized flowing coupons remain in finite bands, blocked coupons meet one-sided limits, and predictive signatures meet the frozen three-standard-uncertainty separation rule. |
| Recovery/hold-up | Tracer recovery reconciles within +/-10%; the separate physical hold-up upper bound is below the A6 target with no discretionary waiver. |
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
- Dye-recovery mass closure and physical hold-up ledger.
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

- The machine-readable A0 contract freezes the corrected 16-slot geometry, slot map, A/B/C datum scheme, D1/D2 locator roles, condition semantics, and disposable wetted-path policy.
- A3 provides the starting gasket compression guard band and leak/burst coupon targets.
- A6 provides the disposable harness topology, port map, prime/debubble path, waste/backflow requirement, pressure/flow targets, and dead-volume ledger.
- `docs/tissue_chip_reproducibility_controls.md` identifies no-cell flow, pressure mapping, leak-witness, and bubble challenge as required engineering gates before live cells.

## Handoffs

A7 hands these decisions to the first-article build package:

- Iterate the shared-contract `sixteen_slot_cassette_no_cell_validation_fixture`
  CAD outputs into either STEP/drawing release files or sourced bench fixture
  selections without reintroducing local cassette constants.
- Replace the solid layout tokens with functional flow-through surrogate and
  restriction coupons, and add the missing quantitative dye/hold-up station before any
  A7 execution claim.
- Select actual pressure sensors, flow sensors if used, scales, gauges, fittings, tubing, connector SKUs, collection vials, dye, and leak-witness materials.
- Define the final operating pressure and test pressure once pump, tubing, connector, surrogate-chip, and gasket coupon data are known.
- Add drawing notes that the older 20-position fixture CAD is not the active first-build validation fixture.
- Commission the functional bench and execute the controlled protocol/forms;
  the current forms define the record but are not physical validation evidence.

A7 blocks:

- Live cells before all required no-cell gates pass.
- AAV before media-only and sterility/aseptic workflow evidence exists.
- Mixed-AAV routing in one cassette.
- Recirculation until a separate recirculation validation fixture and recovery/carryover evidence exist.
- Any use of the older 4 x 5 / 20-position validation fixtures as proof for the 16-slot first article.
