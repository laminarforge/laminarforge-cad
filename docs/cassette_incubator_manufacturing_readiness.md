# 16-Slot Cassette/Incubator Manufacturing Readiness Package

This is the first manufacturable direction for the LaminarForge cell-culture equipment stack. It narrows the work from broad equipment concepts to one buildable subsystem: a 16-slot closed cassette carrier that can dock into an incubated module/rack.

## Current Decision

Use a 16-slot, 4 x 4 cassette/module as the first hardware anchor. This is now the committed first-build format for the AAV cassette work.

One cassette remains one AAV capsid/promoter/payload/dose/timing/media
condition. The 16 slots are same-condition readouts, not separate AAV
candidates. This package is mechanical manufacturing readiness only; it does
not establish sterility, containment, live-cell readiness, or biological
validity.

The immediate work is the cassette first article: reusable lower carrier, reusable lid/clamp, gasket/witness system, disposable sterile tubing harness assumptions, barcode/condition ID, no-cell validation fixture, and dock/module interfaces reserved for later controlled incubation. Do not spend the next build cycle on CO2 incubator control; design the cassette so it can dock into a cassette-level controlled module later.

The shared mechanical/interface baseline for downstream agents is
`docs/sixteen_slot_cassette_a0_interface_spec.md`. The compact
`docs/standard_footprint_aav_condition_module_a0.md` direction is a supplemental
module/local-coupon study, not a replacement for the active 16-slot first build.

Both the STL fit-check and true B-rep STEP draft generators now consume `src/sixteen_slot_cassette_a0.rs`. The contract corrected the predecessor geometry's overlapping per-slot seal loops, unequal seal-land heights, undersized margins, gutter/label conflicts, corner-stop layout, and fasteners inside the gutter. Both lid builders complete additive geometry before subtracting the shared underside grooves so STEP and STL remain parity-equivalent. Use the following nomenclature and values in RFQs and drawings:

| Interface | Base-body or nominal dimensions | True overall or functional dimensions |
| --- | --- | --- |
| Chip layout | 24.00 x 24.00 mm gutter; 151.76 x 109.48 mm pitch; 583.04 x 413.92 mm array | Centers X = +/-227.64, +/-75.88 mm; Y = +/-164.22, +/-54.74 mm |
| Carrier | 699.04 x 541.92 x 24.00 mm | 699.04 x 541.92 x 31.35 mm |
| Lid/clamp | 717.04 x 559.92 x 10.00 mm | 717.04 x 559.92 x 11.60 mm |
| Window | 667.04 x 489.92 x 3.00 mm | 667.04 x 489.92 x 4.80 mm |
| Dock | 869.04 x 691.92 x 22.00 mm | 869.04 x 691.92 x 40.00 mm |
| Dock support / recesses | Datum-A support plane at local Z = +11.00; sixteen 137.76 x 95.48 mm recesses at slot centers | 5.50 mm recess depth; exact floor local Z = +5.50 |
| Dock through openings | Five 657.04 x 8.00 mm air bypasses; 799.04 x 10.00 mm front drain; 10.00 x 615.92 mm right drain; 58.00 x 38.00 mm visibility opening | All use 24.00 mm-high cutters through the 22.00 mm deck at the locked A0 coordinates |
| Dock top-side lands / bridges | Sixteen position-token lands; four solid 48.00 x 32.00 x 8.00 logger reservation lands; two robot-lift lands; four 32.00 mm-diameter leveling lands | Additive lands deliberately bridge portions of the visibility, front-drain, and right-drain openings while leaving base-body void below datum A |
| Service bulkhead | 789.04 x 34.00 x 76.00 mm | 789.04 x 60.00 x 76.00 mm |
| Per-slot seal lands | 145.76 x 103.48 mm outer; 129.76 x 87.48 mm inner; 8.00 mm width | 7.35 mm height; 6.00 mm clear gap; 16 independent, nonoverlapping loops; land face meets lid underside |
| Perimeter seal land | 629.04 x 459.92 mm outer; 605.04 x 435.92 mm inner; 12.00 mm width | 7.35 mm height on the common lid underside closure plane |
| Lid grooves / hard stops | 1.80 mm deep x 3.20 mm wide grooves for 2.40 mm gasket | Nine 4.00 mm-diameter inter-slot stops plus 4.00 mm-wide perimeter stops in the 5.00 mm web; all top at 7.35 mm and clear every seal |
| Leak gutter and drain | 651.04 x 481.92 mm outer; 639.04 x 469.92 mm inner; 6.00 mm width x 3.00 mm depth; 5.00 mm separating web | Drain diameter 8.00 mm, center (315.52, -254.96, 10.00) mm, length 40.00 mm along Y |

Datum A is the carrier bottom plane seated directly on the dock support plane;
the rear/left rail inner faces and front lip contact the nominal rear, left, and
front carrier edges. D1-D4 sit at X = +/-335.52 mm and Y = +/-204.96 mm, but
only D1 round and D2 relieved are functional locators. The 16 lid fasteners sit
outside the gutter at side X = +/-332.52 mm on the slot-center Y values and
front/rear Y = +/-247.96 mm on the slot-center X values. All 16 matching carrier
M4 tap-drill/pilot receiver placeholders are 3.30 mm; final thread/insert SKU
remains open. The global 96.00 x 12.00 mm barcode land at (-271.52, -263.96)
mm and 118.00 x 10.00 mm text land at (-159.52, -263.96) mm stay outside the
gutter and clear the front receiver row; per-slot labels also stay outside the
gutter. The 7.00 mm-high side service reliefs stay below closure, and the service
sensor/backplane cut stays centered at X = 320.00 mm.

The dock openings are not continuous gutter/sump claims. The front lip bridges
part of the visibility opening, both right-side logger reservation lands bridge
the right drain, and the front-left, front-right, and rear-right leveling lands
bridge the drain openings beneath them. The A0 generators intentionally retain
void below datum A and solid additive land above at these overlaps. D5 must show
each bridge in section and dimension the remaining clear spans; moving an
opening to hide the overlap requires an explicit A0 revision. No-cell drainage,
condensate capacity, and cleaning evidence remain required.

The repo contains two competing cassette directions:

| Direction | Status | Decision |
| --- | --- | --- |
| 20-chip cassette/shelf | Exists in `automated_media_exchange_cassette`, `cassette_storage_recovery_incubator_rack`, `sealed_culture_module`, and older chip-farm models. Good density concept, but too spread out for first manufacturing. | Defer. |
| 16-slot closed cassette | Exists across gasket install, attachment/settle, media exchange, evaporation balance, environmental mapping, and seeding validation fixtures. Better aligned with the user's remembered 16-slot board and easier to harden first. | Use as first build package baseline. |

## Fresh Research Check

Exa research was restored and run for this package. The design direction is supported by current organ-on-chip and incubator references:

| Source | Relevant finding | Design implication |
| --- | --- | --- |
| Minahan et al., modular reusable perfusion-ready MPS platform, PMC12914553 | A reusable acrylic cassette can clamp interchangeable elastomeric inserts, forming leak-tight channels without irreversible bonding; the platform emphasizes imaging access, sterilizable/reusable hardware, and parallel insert seeding before perfusion. | Keep the LaminarForge carrier reusable and structural; make the cell-contact/fluid-contact layer disposable or separately validated. Use screw/clamp compression and visible gasket witnesses rather than adhesive-only sealing. |
| Modular fluidic circuit board for parallelized cell culture, Microsystems & Nanoengineering | A standardized plug-and-play fluidic circuit board can operate multiple microfluidic building blocks and reduce the external tubing/footprint problem. It references standardized MFBB/FCB interface formats and 1.5 mm inlet grids. | Treat the cassette as a mechanical/fluidic interface board, not just a tray. Freeze port locations, datum scheme, and routing interface early. |
| AKITA scalable OOC/TEER platform, PMC11583584 | Standard 96-well/384-well plate layouts improve automation compatibility; TEER and automated liquid handling are integrated as repeatable readouts. | Preserve automation-compatible spacing where possible and reserve sensor/TEER/readout features in the cassette/dock stackup. |
| Baker humidity/condensation white paper | Common culture targets are 37 C, 5% CO2, and about 95% RH; poor RH feedback and condensation can drive evaporation, contamination, and irreproducibility, especially in small volumes. | The incubator module needs per-slot evaporation/RH evidence and condensation management before live-cell claims. |
| PHCbi incubator selection guide | Commercial incubators specify 0-20% CO2 control, temperature uniformity, RH behavior, access ports, stainless/copper alloy interiors, shelves, and decontamination options. | Use commercial incubator specs as benchmark targets and buy critical sensors/control elements rather than inventing all of them. |
| Thermo Fisher incubator care guide | Incubator placement, leveling, ventilation clearance, sterile distilled water, gas regulation, leak checks, and independent calibration are routine requirements. | Add leveling, independent logging/calibration, gas leak checks, and water-quality assumptions to the first-article test plan. |
| Parker O-Ring Handbook | O-ring/gland design is application-specific and requires material compatibility, pressure, temperature, tolerance, and failure-mode analysis. | Keep the fixed 2.40 mm gasket, 1.80 x 3.20 mm nominal lid groove, and 20-30% squeeze band; selected compound, dimensional tolerance, groove fill, and validation evidence still require vendor review. |

## Baseline CAD Evidence

The current repo has manufacturable-reference geometry, but not final production drawings.

| Model | Evidence from current CAD/build | Manufacturing relevance |
| --- | --- | --- |
| `sixteen_slot_cassette_incubator_first_article` plus `sixteen_slot_cassette_first_article_step` | The active generators share the machine-readable A0 contract and emit seven STL fit-check files plus seven true B-rep STEP drafts. | Controlling first-article mechanical source. STEP remains draft support geometry until D0-D9 drawings are released. |
| `closed_cassette_gasket_install_torque_compression_station` | Historical 4 x 4 fixture with torque stages, leak-test ports, and compression witnesses. Its pre-contract dimensions and seal layout do not control the first article. | Reference only for test-fixture ideas; do not copy its geometry into drawings. |
| `closed_cassette_cell_attachment_settle_uniformity_witness_station` | MCP build exports 13 STLs. Reports 4 x 4 no-flow slots on a 663 mm x 492 mm nest, timed perfusion gates, humidity witnesses, edge/center markers. | Best source for slot layout, attachment dwell, level/tilt, and imaging access requirements. |
| `closed_incubator_slot_to_slot_media_evaporation_balance_station` | MCP build exports 12 STLs. Reports 16 media surrogates, 16 RH logger pockets, 4 dewpoint references, 16 condensate witness lands, 16 matched humidity ports/restrictor coupons. | Best source for incubator rack slot-to-slot reproducibility requirements. |
| `closed_media_exchange_shear_pulse_carryover_validation_station` | MCP build exports 13 STLs. Reports 16 feed, 16 flush, and 16 harvest route legs, pressure/flow docks, bubble/dead-volume windows, and carryover wells. | Best source for fluid routing and per-slot media-exchange validation requirements. |
| `sealed_culture_module` | MCP build exports 5 STLs. Reports 892 mm x 788 mm footprint with gasketed lid frame, service bulkhead, and thermal plate interface. | Useful enclosure/docking concept, but it is based on the older 20-chip cassette geometry and must be resized around the 16-slot baseline. |

## Manufacturer-Ready Scope For First Quote

Do not quote the whole incubator/cell-culture machine yet. Quote the mechanical cassette and dock test article first.

### Custom Parts To Quote First

| Part | Starting CAD source | Preferred process | Material direction | Notes |
| --- | --- | --- | --- | --- |
| 16-slot cassette lower carrier | `output/rfq/sixteen_slot_cassette_lower_carrier.step` draft from the active A0 contract | CNC machining after D1 drawing release | 6061-T651/T6 aluminum, clear anodized, or alternate after material review | Structural carrier only. Keep wetted sterile path disposable or separately validated. |
| Cassette lid/clamp frame | `output/rfq/sixteen_slot_cassette_lid_clamp.step` draft | CNC machining or waterjet + secondary machining after D2 release | 6061-T651/T6 aluminum or stainless 304/316 | Needs controlled gasket compression, captive fasteners, and inspection window cutout. |
| Optical/imaging window insert | `output/rfq/sixteen_slot_cassette_window_placeholder.step` draft or controlled sheet drawing | Laser/CNC cut sheet after D3 release | Polycarbonate or borosilicate/glass depending imaging need | Must be mechanically retained; adhesive-only sealing is not enough for first article. |
| Gasket groove/witness coupon | `output/rfq/sixteen_slot_cassette_gasket_witness_coupon.step` draft | CNC machining after D4 release | Same seal-band material/finish as carrier plus selected gasket material | Use witness shims and compression coupons before trusting the real cassette. |
| Incubator slot dock plate | `output/rfq/sixteen_slot_incubator_dock_plate.step` draft | CNC machining or sheet metal + machined inserts after D5 release | 6061-T651/T6 aluminum or stainless 304 | Preserve the locked segmented through-deck openings, solid logger reservation lands, bridge sections, slot repeatability, and robot/finger access. |
| Service bulkhead test block | `output/rfq/sixteen_slot_service_bulkhead_test_block.step` draft | CNC machining or printed fit-check only after D6 release | Aluminum/PC for dry structure; commercial connectors for sterile/fluid boundary | Preserve the A6 G/M/W role map; bought connector data controls final cutouts. |

### Off-The-Shelf Parts To Select

| Function | Buy item class | Reason |
| --- | --- | --- |
| Fastening | Stainless M3/M4/M5 screws, dowel pins, heat-set/captive inserts where appropriate | Repeatable assembly and registration. |
| Seal | Silicone or EPDM gasket cord/sheet, selected after chemical/sterilization review | Gasket compression is a core risk. |
| Fluid transfer | Sterile disposable tubing, luer/dry-break/aseptic connectors, check valves | Avoid custom wetted printed/machined paths for first culture-facing prototype. |
| Incubator sensing | Independent temp/RH logger, external incubator-range 0-20% CO2 sensor/analyzer | Controller sensor alone is not validation evidence. |
| Slot monitoring | RH/dewpoint logger hardware mounted from the solid reservation lands; optional optical O2 spots/sensors later | Needed to prove slot-to-slot variability; active dock geometry does not contain logger pockets. |
| Thermal interface | Silicone heater pad or external thermal plate, thermal cutoff, insulation | Keep heaters serviceable and isolated from wet leaks. |
| Identification | Barcode/RFID labels/tags compatible with humidity/condensation | Run identity must survive incubation. |

## What Remains Before RFQ

The STL files remain fit-check only, and the implemented STEP files remain drafts. Drawings still block a vendor package. Remaining items:

- D0-D9 2D drawings with critical tolerances, inspection tables, and release index.
- Reviewed STEP/drawing manifest tied to the release commit.
- Final material callouts and surface finish.
- Selected gasket compound/vendor/tolerance and tested fastener torque-to-stop value. The 2.40 mm free height, 1.80 x 3.20 mm lid groove, 25% compression target, and 7.35 mm common land/stop/closure plane are already defined.
- Drawing implementation of the locked A/B/C datum scheme and D1-round/D2-relieved locator roles.
- Fluid connector family and panel cutout dimensions.
- Stackup drawing showing chip, gasket, lid, carrier, window, and clamp height.
- Leak-test pressure method and allowable deformation.
- Cleaning/sterilization assumptions for each material.
- DFM review for minimum wall thickness, tool reach, internal corners, and part warpage.
- D5 sections and inspection dimensions for every clear drain/visibility span
  and every front-lip, logger-land, and leveling-land bridge, without relocating
  the A0 opening coordinates.

## First Build Package Requirements

Before sending to a manufacturer, regenerate and review these implemented drafts from the 16-slot baseline and create their controlling drawings:

| Deliverable | Required content |
| --- | --- |
| `output/rfq/sixteen_slot_cassette_lower_carrier.step` | Implemented draft: 16 chip pockets, direct A/B/C datum interfaces, 7.35 mm gasket lands/stops, 3.30 mm receiver pilots, 7.00 mm service reliefs, gutter/drain, and labels. Requires D1. |
| `output/rfq/sixteen_slot_cassette_lid_clamp.step` | Implemented draft: imaging openings, 1.80 x 3.20 mm underside grooves, outside-gutter fastener pattern, and alignment features. Groove operation order is parity-equivalent to STL. Requires D2. |
| `output/rfq/sixteen_slot_cassette_window_placeholder.step` | Implemented retained-window draft with base thickness and raised witness/retention features. Requires D3 or controlled sheet drawing. |
| `output/rfq/sixteen_slot_incubator_dock_plate.step` | Implemented dock draft with exact recess floors, rails/lip, segmented through-deck openings, solid logger/handling/leveling lands, and intentional void-below/solid-above bridges. Requires D5 sections and no-cell drainage evidence. |
| `output/rfq/sixteen_slot_service_bulkhead_test_block.step` | Implemented dry receiver draft with the preserved A6 gas/media/waste map and sensor placeholder. Requires D6 and bought-connector data. |
| Drawing set | One PDF per machined part with dimensions, tolerances, material, finish, and inspection notes. |
| BOM | Custom parts, hardware, gasket, sensors, tubing/connectors, labels, and consumables. |
| Vendor notes | Prototype purpose, no biological validation claim, request for DFM feedback, and quote options by material/process. |

## Proposed First-Article Tolerances

These are starting engineering targets, not final release specs.

| Feature | Starting target |
| --- | --- |
| Chip pocket XY clearance | 0.5-0.8 mm per side until chip lot tolerance is measured. |
| Datum pin holes | Reamed or precision drilled after choosing pin diameter; avoid relying on printed-hole tolerance. |
| Gasket compression | Design around 20-30% squeeze with a nominal 25% witness target. |
| Flatness under cassette | Call out after material/process selection; do not assume large printed parts are flat enough. |
| Surface finish near gasket | Smooth machined finish; no layer lines on seal lands. |
| Internal corners | Add machineable radii; avoid square internal pockets unless EDM is intended. |
| Wet path | Disposable commercial tubing/connectors for first article. Structural parts should not be treated as validated wetted surfaces. |

## Incubator/Dock Control Targets

Use these as engineering targets for the cassette dock test article. They are not biological release claims.

| Control | First-article target |
| --- | --- |
| Temperature | 37 C nominal, mapped at all 16 slot positions and independently logged. Benchmark against commercial incubator uniformity expectations. |
| CO2 | 5% nominal for bicarbonate-buffered media, measured with an incubator-range 0-20% sensor/analyzer. |
| Humidity | High RH target with logged slot-to-slot evaporation; do not assume an open-loop water pan proves humidity control. |
| Condensation | No uncontrolled dripping onto cassette/chips. Test collection through the locked clear spans and around the intentional bridges; do not assume the fit-check geometry proves drainage. |
| Oxygen | Reserve sensor/coupon windows, but do not make O2-control claims until gas mixing and sensor calibration are designed. |
| Fluid exchange | Dye-only first: per-slot flow, carryover, bubble, dead-volume, and pressure-spike checks before live-cell use. |
| Sterility boundary | Closed/disposable fluid path first; structural cassette is not automatically a sterile wetted component. |

## Vendor Questions

Send these with the RFQ:

1. Can you machine the 699.04 x 541.92 x 24.00 mm base-body carrier flat enough for gasket compression without stress relief or post-machining warp?
2. What material/process gives the best first article: 6061-T6 clear anodized, stainless, polycarbonate, or another autoclavable plastic?
3. What minimum internal radius should we apply to pocket and gasket-land corners for your tooling?
4. Can you provide inspection reports for datum hole positions, pocket spacing, flatness, and gasket land width?
5. Do you recommend machining the lid and carrier as matched parts?
6. Can you source and install threaded inserts/captive hardware, or should hardware installation stay in-house?
7. What finish is compatible with repeated humid 37 C incubation and cleaning agents?
8. Can you quote one prototype set, three prototype sets, and a small pilot run?
9. Can you support CMM or optical inspection for the chip-pocket pattern and gasket land flatness?
10. Do you see any issue machining the gasket groove and chip pockets in the same setup to preserve relative alignment?

## Build/Test Sequence

1. Fit-check with dummy Rev C chips or gauge blocks.
2. Verify cassette lid closes without chip interference.
3. Measure gasket compression using witness shims/coupons before using live-cell hardware.
4. Leak-test the closed cassette with no cells and nonhazardous liquid.
5. Level the incubator/dock, inspect every through-deck clear span and
   void-below/solid-above bridge against D5, and document utility clearances
   before environmental tests.
6. Run 24-hour dry incubation with temp/RH/CO2 logging.
7. Run slot-to-slot evaporation balance with sterile water/media surrogate.
8. Run media exchange using dye to measure carryover, dead volume, bubbles, and per-slot uniformity.
9. Verify cleaning access and residue removal on gasket lands, pockets, dock
   clear spans, and additive bridge sections.
10. Only after those gates, consider live-cell compatibility testing in an appropriate lab environment.

## Research State

Existing repo research and the fresh Exa check support the direction: automated organ-chip systems exist, multi-chip fluidic circuit-board architectures are the right analogy, reusable clamped cassettes are credible, and a standardized cassette/dock is more realistic than single-chip handling. The saved baseline research notes are in `docs/automated_organ_chip_research.md` and `docs/cell_culture_starter_equipment.md`.

Fresh sources used:

- Modular reusable perfusion-ready MPS cassette: https://pmc.ncbi.nlm.nih.gov/articles/PMC12914553/
- Fluidic circuit board for parallelized cell culture: https://preview-www.nature.com/articles/s41378-020-00216-z
- Scalable standardized OOC platform with TEER: https://pmc.ncbi.nlm.nih.gov/articles/PMC11583584/
- Baker humidity/condensation white paper: https://bakerco.com/wp-content/uploads/dlm_uploads/2024/11/Controlling-Relative-Humidity-and-Condensation-in-a-ReCO2ver-Incubator-White-Paper.pdf
- PHCbi incubator selection guide: https://www.phchd.com/global/-/media/biomedical/global/WhitePaper/MCO/07_Choosing-Your-Cell-Culture-Incubator_web.pdf
- Thermo Fisher incubator care guide: https://www.thermofisher.com/TFS-Assets/LPD/Handbooks/incubator-care-and-maintenance.pdf
- Parker O-Ring Handbook: https://www.valin.com/sites/default/files/2023/asset/document/parker-o-ring-handbook.pdf

## Immediate Release Work Next

The dedicated `sixteen_slot_cassette_incubator_first_article.rs` STL fit-check generator and `sixteen_slot_cassette_first_article_step.rs` true B-rep STEP draft generator are implemented from the shared A0 contract. The immediate release work is to produce and review D0-D9 drawings, inspection tables, and the file/commit manifest, then select bought hardware and connector details. Do not send the STEP drafts for RFQ until those drawings exist.
