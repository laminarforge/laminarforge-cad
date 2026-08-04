# 16-Slot Cassette A6 Disposable Fluid-Path Spec

Ticket: T-253E1123

This A6 spec defines the disposable fluid-path architecture for the first physical LaminarForge 16-slot AAV cassette build. It is subordinate to `docs/sixteen_slot_cassette_a0_interface_spec.md` and must mate with the A1 carrier, A2 lid/clamp, A3 gasket, A4 DFM, and A5 integrated mechanical-stack decisions.

This is not a biological protocol, sterile-barrier validation, AAV containment claim, live-cell release criterion, or final vendor drawing. It defines the first-article tubing harness, routing, bought connector assumptions, prime path, waste path, bubble visibility, and validation handoffs.

## A6 Baseline

| Item | A6 requirement |
| --- | --- |
| First-build format | 16 slots in a 4 x 4 cassette. |
| Experimental unit | One cassette equals one AAV capsid/promoter/payload/dose/timing/media condition. |
| Slot role | Slots are cell-type and technical readouts under the same exposure, not separate AAV candidates. |
| Wetted path | Disposable commercial tubing, connectors, reservoirs, filters, and pump-contact tubing. |
| Reusable hardware role | Carrier, lid, dock, bulkhead, and structural cassette parts remain dry fixtures. |
| First-article flow mode | Pressure-limited single-pass feed from a common condition reservoir through matched branches to sealed waste. |
| Recirculation | Deferred. Do not recirculate AAV/media in the first article until recovery, sterility, and carryover evidence justify it. |
| Mixed-AAV routing | Blocked. Do not route multiple AAV candidates through one cassette or shared waste/manifold. |
| Incubation control | Deferred to module work. A6 only reserves gas/vent/service interfaces. |
| Active carrier | 699.04 x 541.92 x 24.00 mm base body; 699.04 x 541.92 x 31.35 mm true overall. |
| Clamp keepout | 7.35 mm land/stop/lid closure plane; nine internal and distributed perimeter stops clear every seal; 16 fasteners and 3.30 mm carrier pilots remain outside the gutter at X = +/-332.52 mm or Y = +/-247.96 mm. |
| Dock registration | Carrier bottom seats directly on datum A; rear/left rail inner faces and front lip contact nominal carrier edges. |
| Service bulkhead base body / true overall | 789.04 x 34.00 x 76.00 mm / 789.04 x 60.00 x 76.00 mm. |
| Sensor/backplane cut | Center X = 320.00 mm, center Z = 18.00 mm; preserve clearance from W4. |
| Carrier routing relief | Left/right service relief bands are 7.00 mm high above carrier top and remain below the 7.35 mm lid underside closure plane. |

## System Boundary

A6 owns:

- Disposable tubing harness architecture.
- Bulkhead port assignment assumptions.
- Reservoir, pump, filter, connector, and waste interfaces as bought-component placeholders.
- Prime, purge, leak-test, waste, and bubble-visible routes.
- Dead-volume accounting and dose-recovery evidence requirements.
- Connector topology and run-record fields needed to prevent wrong-condition assembly.

A6 does not own:

- Chip culture protocol, AAV dose selection, or cell-type media formulation.
- Final sterile validation, IBC release, or live-cell readiness.
- Pump-controller hardware selection beyond interface assumptions.
- Final vendor drawings for the carrier/lid/gasket stack.
- Final no-cell validation fixture design, except for acceptance data A7 must collect.

## Target Architecture

The first article uses one disposable harness per cassette run.

Fluid path:

1. Common condition reservoir or bag.
2. Optional inline sterile filter only if the selected media/AAV workflow can tolerate the filter and recovery loss.
3. Pump or pressure-control interface with pressure limit.
4. Upstream bubble/debubble section with clear visual window.
5. Common supply trunk.
6. Four row branches with equalized routing.
7. Four slot branches per row, one inlet pigtail per chip.
8. One outlet pigtail per chip to a matched row waste branch.
9. Common sealed waste bag or waste bottle with anti-backflow protection.

The first article should be single-pass, not recirculating. Recirculation saves reagent, but it also couples all chips through a shared return volume, increases contamination consequence, and complicates dose-recovery/carryover analysis. A6 therefore treats recirculation as a future controlled module after no-cell flow, bubble, and recovery data exist.

## Condition Semantics

All wetted parts in one cassette harness are assigned to one and only one condition:

- Cassette ID.
- Harness ID.
- AAV condition ID.
- Capsid ID.
- Promoter ID.
- Payload ID.
- Dose/MOI.
- Media recipe.
- Pump/flow recipe.
- Slot map revision.
- Connector topology revision.

If a future experiment needs different media recipes for different cell types, that requirement conflicts with the first A6 shared-condition harness. The acceptable first-article options are to choose a common assay/exposure medium for the cassette window, run separate cassettes, or open a new isolated-lane design ticket. Do not silently use the 16 slots as separate media/AAV lanes.

## Bulkhead Port Assignment

The shared A0 contract implements a dry service bulkhead placeholder with four gas ports, seven media ports, five waste ports, a sensor/backplane cut centered at X = 320.00 mm, and twelve tubing strain-relief comb teeth. Its 789.04 x 34.00 x 76.00 mm base body has 789.04 x 60.00 x 76.00 mm true overall bounds after the added label/strain-relief features. A6 assigns the following first-article logical roles without freezing final connector geometry.

### Gas/Pressure Ports

| Port | Current CAD basis | A6 logical role | First-article note |
| --- | --- | --- | --- |
| G0 | 8.00 mm diameter cut at X = -240.00, Z = 18.00 | Sterile reservoir vent or bag headspace vent path | Use bought sterile vent/filter if open-to-atmosphere venting is needed. |
| G1 | 8.00 mm diameter cut at X = -210.00, Z = 18.00 | Pressure controller or regulated gas input | Pressure limit must be below the weakest validated component. |
| G2 | 8.00 mm diameter cut at X = -180.00, Z = 18.00 | Relief/exhaust or pressure-test vent | Route to safe exhaust or waste capture during no-cell tests. |
| G3 | 8.00 mm diameter cut at X = -150.00, Z = 18.00 | Reserved gas sample / future CO2 module interface | Do not depend on this for first-article incubation control. |

### Media/Condition Ports

| Port | Current CAD basis | A6 logical role | First-article note |
| --- | --- | --- | --- |
| M0 | 6.40 mm diameter cut at X = -78.00 | Common condition feed inlet | Main cassette feed for media/AAV condition. |
| M1 | 6.40 mm diameter cut at X = -52.00 | Media/rinse feed inlet | No-cell/media-only validation or preflush; not a second AAV candidate. |
| M2 | 6.40 mm diameter cut at X = -26.00 | Prime/debubble bypass inlet | Allows harness prime to waste before exposing chip branches. |
| M3 | 6.40 mm diameter cut at X = 0.00 | Pressure/leak-test inlet | Used by A7 validation fixture and dye tests. |
| M4 | 6.40 mm diameter cut at X = 26.00 | Sample/QC loop inlet | Optional closed sampling path; not required for first flow test. |
| M5 | 6.40 mm diameter cut at X = 52.00 | Inline sensor/cartridge service inlet | Reserved for pH/DO/conductivity module work. |
| M6 | 6.40 mm diameter cut at X = 78.00 | Spare / vendor-selected connector alternate | Keep capped in first article if unused. |

### Waste/Return Ports

| Port | Current CAD basis | A6 logical role | First-article note |
| --- | --- | --- | --- |
| W0 | 6.40 mm diameter cut at X = 150.00 | Common chip outlet waste | Main sealed waste path. |
| W1 | 6.40 mm diameter cut at X = 176.00 | Prime/debubble purge waste | Used until upstream and branch bubble windows are clear. |
| W2 | 6.40 mm diameter cut at X = 202.00 | Sample/QC waste | Optional sample-loop waste; keep capped if unused. |
| W3 | 6.40 mm diameter cut at X = 228.00 | Leak-test/dye-test waste | A7 fixture can isolate this path from biological waste. |
| W4 | 6.40 mm diameter cut at X = 254.00 | Relief/overflow/emergency waste | Must include backflow/siphon control if connected. |

The placeholder diameters are not final connector drawings. A4 remains correct: vendor drawings must dimension bought connector panel geometry, not generic holes.

## Tubing Harness Layout

Use a preassembled, labeled disposable harness with a repeatable physical topology.

Required harness zones:

| Zone | Function | A6 requirement |
| --- | --- | --- |
| Reservoir lead | Links bag/reservoir to pump interface | Short, labeled, replaceable, compatible with selected sterile connector or tube welding method. |
| Pump segment | Interfaces with syringe, peristaltic, or pressure controller | Selected by pump agent; must be separately calibrated and replaceable. |
| Upstream bubble window | Visible high point before the cassette branches | Clear tubing or clear inline chamber; must be inspectable when docked. |
| Common supply trunk | One condition feed to all 16 branches | No branch may receive a different AAV candidate. |
| Row branches | Four row feeds | Route as matched lengths where practical; label R1-R4. |
| Slot inlet pigtails | Sixteen chip inlet connections | Label S01-S16; length and bend radius recorded. |
| Slot outlet pigtails | Sixteen chip outlet connections | Route to row waste branches; no open-drain exposure. |
| Waste trunk | Common outlet to sealed waste | Include anti-backflow/siphon control and waste identity label. |
| Prime bypass | Lets pump prime to waste before opening chip branches | Must be visible and pressure-limited. |

Use tubing combs, clips, or harness trays so assembly does not rely on memory. The twelve A5 bulkhead comb teeth are strain-relief placeholders, not a complete topology-control system; the build package must add a physical route map or keyed tray for all chip pigtails.

## Routing Keepouts

Tubing, connectors, clips, filters, and labels must stay out of:

- All 16 lid view openings.
- Carrier optical through-windows.
- Gasket lands, lid grooves, the nine 4.00 mm-diameter internal hard stops, the
  4.00 mm-wide perimeter web stops, and dedicated witness access. No route may
  cross or load a seal or stop.
- Datum bosses, the direct datum-A dock support, rear/left rail inner faces, and
  front retention lip where they contact the nominal carrier edges.
- Carrier global condition ID land and slot-1 orientation marker.
- All global and per-slot label lands, which the shared A0 contract places outside the leak gutter.
- Dock logger pockets unless assigned by the sensor/module agent.
- Robot lift lands and carrier handling lands.
- Leak gutter and visible drain/sump inspection paths.

Preferred routing is from the rear service bulkhead into the 7.00 mm-high side service relief bands, then into controlled row/slot pigtails. The reliefs remain below the 7.35 mm closure plane and cannot be used to prop the lid open. Any tube crossing the top of a slot must be rejected unless the integrator explicitly updates the imaging strategy.

## Bought Connector Strategy

A6 resolves the first article to bought sterile/aseptic connection technology, not custom machined sterile ports.

Acceptable first-article options:

| Option | Use | Requirement |
| --- | --- | --- |
| Preassembled single-use harness | Best early path if a vendor can provide sterile or clean disposable sets | Harness is released as a kit with lot, route map, caps, and leak-test evidence. |
| Sterile tube welding | Useful for compatible PVC/TPE/C-Flex-style tubing | Weld cycle metadata, tubing material, OD/ID, and weld coupons must be recorded. |
| Single-use aseptic connectors | Useful for reservoir and module handoffs | Connector family, termination size, pressure rating, temperature rating, sterilization mode, and extractables/compatibility data must be captured. |
| Non-sterile low-dead-volume microfluidic connectors for no-cell tests only | Useful for early water/dye validation | Must be clearly blocked from live-cell/AAV use until aseptic and material compatibility evidence exists. |

Do not custom-machine sterile connector geometry into the aluminum carrier, lid, dock, or service bulkhead for first article. Those parts can hold, key, strain-relieve, or label bought components; they cannot become the validated sterile boundary by assertion.

## Tubing Material Direction

No final AAV-contact tubing material is frozen by A6. The first quote/test set should compare a weldable TPE/PVC-compatible route against a silicone/peristaltic-pump route if budget allows.

Starting material assumptions:

- Prefer weldable TPE/C-Flex-style tubing where sterile tube welding is part of the selected workflow.
- Use silicone or platinum-cured silicone pump tubing only where pump mechanics or connector availability require it.
- Minimize total wetted surface area and dead legs because AAV, proteins, growth factors, and small additives may adsorb to tubing/filter/connector surfaces.
- Treat any inline filter, debubbler membrane, or connector seal as a recovery-risk item until dye and biological surrogate recovery tests pass.
- Record tubing OD, ID, wall thickness, lot, supplier, sterilization state, and expiration/retest status.

## Prime And Debubble Path

Priming must be possible before chips are exposed.

Required sequence for no-cell validation:

1. Connect reservoir, pump interface, bubble window, prime bypass, and waste.
2. Prime common upstream line to W1 until visible bubbles clear.
3. Prime row branches one row at a time to waste or collection wells.
4. Prime each slot pigtail through a no-cell surrogate chip or restriction coupon.
5. Verify no visible bubble remains at chip inlet, row branch high point, or outlet.
6. Record prime volume, time, pressure, flow setting, and waste recovery.
7. Only after priming evidence passes may the harness be used for media-only or live-cell planning.

The harness must include bubble-visible sections at:

- Upstream of the cassette branch split.
- Each row branch high point or row manifold outlet.
- At least one outlet witness path before common waste.
- Any vertical high point introduced by the dock or module.

If bubbles cannot be seen while the cassette is docked, the design fails A6.

## Waste And Backflow Control

Waste is a closed or semi-closed disposable path, not an open drain.

Required waste controls:

- Main waste route W0 to sealed waste bag/bottle.
- Separate prime/purge waste W1 during setup.
- Dye/leak-test waste W3 separable from biological waste assumptions.
- Anti-backflow or anti-siphon strategy before any live-cell plan.
- Waste bag/bottle identity label linked to cassette ID and condition ID.
- Secondary containment or leak tray under waste handoff.
- No shared waste container between different AAV candidates unless carryover/containment is separately validated.

Waste backflow/siphon testing is handed to A7 but must be physically possible from the A6 harness.

## Physical Hold-Up And Dose-Recovery Model

Every disposable harness revision needs a volume ledger before biological use.

Record calculated and measured volume for:

- Reservoir outlet to pump inlet.
- Pump segment.
- Pump outlet to upstream bubble window.
- Bubble/debubble element.
- Common supply trunk.
- Each row branch.
- Each slot inlet pigtail.
- Each chip/surrogate internal path.
- Each slot outlet pigtail.
- Row waste branches.
- Common waste trunk.
- Prime bypass.
- Sample/QC loop if present.

Initial engineering target: the one-sided upper bound on physical retained/
hold-up volume, including recovery and measurement uncertainty, must be less
than one chip dose or 10% of the formulated cassette condition volume,
whichever is more conservative. Tracer mass non-closure is reported separately
and cannot alone be relabeled hydraulic dead volume because adsorption and
analytical loss are confounded. Missing or failed recovery evidence blocks
expensive vector work; operator or integrator judgment cannot waive this gate.

## Flow Balance And Pressure Limits

A6 does not freeze a pump. It freezes what the pump/service module must prove.

Starting no-cell targets before live-cell planning:

| Metric | Initial target |
| --- | --- |
| Row flow balance | Row collected-volume CV <= 10% during no-cell multiplex flow. |
| Slot flow balance | Slot collected-volume CV <=10% for the characterized nominal no-cell surrogate set. Actual-chip limits require a new evidence-backed gate after chip restriction data exists. |
| Pressure drift | Stable within +/-5% after thermal and compliance stabilization. |
| Pressure ceiling | Routine operation <=50% of the lowest credible temperature-derated failure/burst/connector limit after approved engineering margin, and no higher than any supplier-rated working pressure. The 1.5x installed proof checkpoint and a lone coupon result are not operating ratings. |
| Installed-system leak test | No visible leak and guard-banded decay `100 x (P0 - P10) / P0 <= 5%` at 1.5x selected maximum operating pressure, using liquid, no active pressure makeup, at least 661 s acquisition, P0 mean over t = 0-60 s, and P10 mean over t = 600-660 s. Remain below every installed component's qualified proof-pressure limit; a selected component that cannot support it fails selection. |
| Isolated gasket qualification | A separate coupon or isolated gasket loop passes A3's max(35 kPa gauge, 1.5x maximum operating pressure) liquid gate. |
| Bubble challenge | Introduced upstream bubble clears to waste without reaching a chip inlet witness during validated prime/debubble mode. Bubble volume and maximum clearing volume/time must be frozen in A7 before this gate is executable. |
| Recovery | Dye or surrogate recovery reconciles input/output/waste within the A7-defined mass-balance tolerance. |

A7 owns the final validation fixture and acceptance thresholds. A6 requires the physical ports and harness topology to support these measurements.

## Connector Topology And Run Record

Wrong-port assembly is a first-order failure mode.

The build package must include:

- Harness revision ID.
- Port map from G/M/W bulkhead positions to connector labels.
- S01-S16 pigtail labels.
- R1-R4 branch labels.
- Reservoir, waste, filter, pump segment, connector, and tubing lot IDs.
- Cap/plug status for unused ports.
- Pre-run scan or photo evidence of the assembled topology.
- Pass/fail status for prime, leak, bubble, and volume-balance checks.

Any physical connector pair that can be swapped must be keyed, color coded, scan checked, or physically segregated. Labels alone are not enough for repeated work.

## Research Basis

The A6 research pass was limited to organ-chip fluid interfaces, world-to-chip connector burden, parallelized fluidic boards, and bought aseptic connection technology.

| Source | Finding used by A6 |
| --- | --- |
| Minahan et al., modular reusable perfusion-ready MPS cassette, https://pmc.ncbi.nlm.nih.gov/articles/PMC12914553/ | Reusable cassette hardware can clamp disposable/interchangeable fluidic layers while preserving imaging and separating reusable structure from consumable interfaces. |
| TapeTech microfluidic connectors, https://pubs.rsc.org/ak/content/articlehtml/2025/lc/d4lc00970c | World-to-chip connections are a major setup burden; simplified connectors can reduce pressure surges, improve priming, improve multiplexing, and reduce setup errors. |
| Pressure-driven perfusion system for organs-on-chips, https://pmc.ncbi.nlm.nih.gov/articles/PMC9416133/ | Pressure-driven perfusion and fluidic circuit board strategies can multiplex and recirculate OoC media, but throughput requires controlled routing and software/fixture support. |
| Vollertsen et al., modular FCB for parallelized cell culture, https://preview-www.nature.com/articles/s41378-020-00216-z | Standardized fluidic circuit boards can operate multiple microfluidic building blocks with parallelized liquid dosing and controlled interfaces. |
| Sterile fluid transfer for cell therapy manufacturing, https://pmc.ncbi.nlm.nih.gov/articles/PMC8931546/ | Closed sterile transfer commonly relies on single-use sterile connectors and tube welders; frequent small-volume transfers require device and workflow selection, not improvised open handling. |
| CPC AseptiQuik C product information, https://www.cpc-bio.com/products-overview/connectors/technologies/aseptic-sterile-connectors/aseptiquik-c | Bought aseptic connectors include pressure, temperature, sterilization, material, and termination constraints; supplier warns users must test suitability in their application. |
| Sartorius Biowelder S product information, https://www.sartorius.com/en/products/fluid-management/aseptic-connectors/sterile-connection-device | Sterile tube welding is a bought-device workflow for compatible PVC/TPE tubing and can provide weld metadata; tubing material compatibility is central. |
| STARTER modular OoC platform, https://pubs.rsc.org/en/content/articlehtml/2026/lc/d5lc00756a | Modular open OoC platforms use standardized footprints and swappable pumping/sensing/OoC modules; LaminarForge should preserve service interfaces for later modules. |

## A0 Interface Decisions Implemented By A6

| Interface decision | A6 resolution |
| --- | --- |
| Connector family | Do not freeze one final connector family. Use bought aseptic connectors or sterile tube welding for live-use planning; use low-dead-volume microfluidic connectors only for no-cell tests until validated. |
| Tube OD/ID | Not frozen. Record OD/ID and quote weldable TPE/C-Flex-style and silicone/peristaltic options. |
| Bend radius | Must follow supplier minimum bend radius and be preserved by combs/trays; no kinked pigtails or memory-dependent routing. |
| Port assignment | Assign G0-G3, M0-M6, and W0-W4 logical roles as above while preserving placeholder CAD geometry. |
| Valve/pump interface | Pump not frozen; interface must support pressure-limited single-pass flow, prime-to-waste, branch isolation/testing, and flow/pressure logging. |
| Sterile connection method | Bought sterile connectors or tube welding only; no custom machined sterile boundary in structural hardware. |
| Bubble visibility | Required at upstream split, row branches, outlet/waste witness, and any vertical high point. |
| Dead-volume measurability | Required volume ledger and dye/surrogate recovery before AAV work. |

## A0 Decisions Escalated

| Decision | Escalation path |
| --- | --- |
| Final connector SKU and panel geometry | A8 build package/vendor RFQ after tubing/pump selection. |
| Final tubing material for AAV contact | Biology/material compatibility review plus recovery testing. |
| Pump type and control profile | Pump/service-module owner. |
| Branch equalization geometry and actual pressure-flow model | A7 validation fixture with no-cell restriction coupons. |
| Sterility/aseptic process claim | Media-fill and facility/IBC workflow; not CAD alone. |
| Waste containment and AAV decontamination SOP | Safety/IBC and validation owner. |
| Recirculation | Future module after single-pass evidence. |

## CAD Handoff Notes

The STL and true B-rep STEP draft generators now consume the shared machine-readable A0 geometry contract. The geometric update corrected the earlier carrier/seal/gutter conflicts, moved the 16 clamp fasteners and their 3.30 mm carrier pilots outside the gutter, and locked the service relief below closure while preserving the A6 role map exactly: G0-G3, M0-M6, and W0-W4 retain the assignments above. The service bulkhead remains a placeholder for bought connector cutouts and strain relief. Future CAD changes should add:

- Keyed connector nests based on selected connector datasheets.
- A physical harness route map or tray.
- Bubble-window keepouts visible when docked.
- Cap/plug parks for unused ports.
- Waste/prime segregation labels.
- Gauge-chip/no-cell validation fixture interfaces for pressure, dye, and gravimetric collection.

Any future CAD change must preserve A0 slot count, one-condition semantics, dry structural hardware policy, imaging keepouts, gasket compression access, and service-bulkhead traceability unless the cassette integrator revises the A0 contract.
