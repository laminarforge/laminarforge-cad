# 16-Slot Cassette/Incubator Manufacturing Readiness Package

This is the first manufacturable direction for the LaminarForge cell-culture equipment stack. It narrows the work from broad equipment concepts to one buildable subsystem: a 16-slot closed cassette carrier that can dock into an incubated module/rack.

## Current Decision

Use a 16-slot, 4 x 4 cassette/module as the first hardware anchor. This is now the committed first-build format for the AAV cassette work.

The immediate work is the cassette first article: reusable lower carrier, reusable lid/clamp, gasket/witness system, disposable sterile tubing harness assumptions, barcode/condition ID, no-cell validation fixture, and dock/module interfaces reserved for later controlled incubation. Do not spend the next build cycle on CO2 incubator control; design the cassette so it can dock into a cassette-level controlled module later.

The shared mechanical/interface baseline for downstream agents is `docs/sixteen_slot_cassette_a0_interface_spec.md`.

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
| Parker O-Ring Handbook | O-ring/gland design is application-specific and requires material compatibility, pressure, temperature, tolerance, and failure-mode analysis. | Keep the 20-30% gasket squeeze target as preliminary only; final groove dimensions require selected gasket cross-section/material and vendor review. |

## Baseline CAD Evidence

The current repo has manufacturable-reference geometry, but not final production drawings.

| Model | Evidence from current CAD/build | Manufacturing relevance |
| --- | --- | --- |
| `closed_cassette_gasket_install_torque_compression_station` | MCP build exports 12 STLs. Reports 4 x 4 chip cassette carrier, 16 slots, 648.0 mm x 466.9 mm carrier envelope, 8 torque stages, 4 leak-test ports, compression witness shims. | Best source for the first gasketed 16-slot cassette datum and seal-install requirements. |
| `closed_cassette_cell_attachment_settle_uniformity_witness_station` | MCP build exports 13 STLs. Reports 4 x 4 no-flow slots on a 663 mm x 492 mm nest, timed perfusion gates, humidity witnesses, edge/center markers. | Best source for slot layout, attachment dwell, level/tilt, and imaging access requirements. |
| `closed_incubator_slot_to_slot_media_evaporation_balance_station` | MCP build exports 12 STLs. Reports 16 media surrogates, 16 RH logger pockets, 4 dewpoint references, 16 condensate witness lands, 16 matched humidity ports/restrictor coupons. | Best source for incubator rack slot-to-slot reproducibility requirements. |
| `closed_media_exchange_shear_pulse_carryover_validation_station` | MCP build exports 13 STLs. Reports 16 feed, 16 flush, and 16 harvest route legs, pressure/flow docks, bubble/dead-volume windows, and carryover wells. | Best source for fluid routing and per-slot media-exchange validation requirements. |
| `sealed_culture_module` | MCP build exports 5 STLs. Reports 892 mm x 788 mm footprint with gasketed lid frame, service bulkhead, and thermal plate interface. | Useful enclosure/docking concept, but it is based on the older 20-chip cassette geometry and must be resized around the 16-slot baseline. |

## Manufacturer-Ready Scope For First Quote

Do not quote the whole incubator/cell-culture machine yet. Quote the mechanical cassette and dock test article first.

### Custom Parts To Quote First

| Part | Starting CAD source | Preferred process | Material direction | Notes |
| --- | --- | --- | --- | --- |
| 16-slot cassette lower carrier | `closed_cassette_gasket_install_torque_compression_station_cassette_datum_nest_16_slot.stl` as geometry reference | CNC machining | 6061-T6 aluminum, clear anodized, or autoclavable engineering plastic after material review | Structural carrier only. Keep wetted sterile path disposable or separately validated. |
| Cassette lid/clamp frame | Derive from gasket-install and settle-uniformity geometry | CNC machining or waterjet + secondary machining | 6061-T6 aluminum or stainless 304/316 | Needs controlled gasket compression, captive fasteners, and inspection window cutout. |
| Optical/imaging window insert | Derived from slot window requirements | Laser/CNC cut sheet | Polycarbonate or borosilicate/glass depending imaging need | Must be mechanically retained; adhesive-only sealing is not enough for first article. |
| Gasket groove/witness coupons | Gasket-install fixture | CNC machining | Same as carrier plus silicone gasket material | Use witness shims and compression coupons before trusting the real cassette. |
| Incubator slot rack/dock plate | `closed_incubator_slot_to_slot_media_evaporation_balance_station_sixteen_slot_balance_rack.stl` as reference | CNC machining or sheet metal + machined inserts | 6061-T6 aluminum or stainless 304 | Needs slot repeatability, drain/leak capture, logger pocket positions, and robot/finger access. |
| Service bulkhead test block | Derived from `sealed_culture_module_service_bulkhead` and media-exchange routing | CNC machining or printed fit-check only | Aluminum/PC for dry structure; commercial connectors for sterile/fluid boundary | Do not custom-machine final sterile fluid connectors until connector family is selected. |

### Off-The-Shelf Parts To Select

| Function | Buy item class | Reason |
| --- | --- | --- |
| Fastening | Stainless M3/M4/M5 screws, dowel pins, heat-set/captive inserts where appropriate | Repeatable assembly and registration. |
| Seal | Silicone or EPDM gasket cord/sheet, selected after chemical/sterilization review | Gasket compression is a core risk. |
| Fluid transfer | Sterile disposable tubing, luer/dry-break/aseptic connectors, check valves | Avoid custom wetted printed/machined paths for first culture-facing prototype. |
| Incubator sensing | Independent temp/RH logger, external incubator-range 0-20% CO2 sensor/analyzer | Controller sensor alone is not validation evidence. |
| Slot monitoring | RH/dewpoint logger pockets, optional optical O2 spots/sensors later | Needed to prove slot-to-slot variability. |
| Thermal interface | Silicone heater pad or external thermal plate, thermal cutoff, insulation | Keep heaters serviceable and isolated from wet leaks. |
| Identification | Barcode/RFID labels/tags compatible with humidity/condensation | Run identity must survive incubation. |

## What Is Not Ready To Send Yet

The current STL outputs are not a vendor package. Missing items:

- STEP exports for the selected cassette, lid, dock plate, and bulkhead.
- 2D drawings with critical tolerances.
- Final material callouts and surface finish.
- Gasket cross-section, groove dimensions, compression target, and fastener torque sequence.
- Datum scheme: primary/secondary/tertiary datums for chip pockets, lid, dock, and robot handling.
- Fluid connector family and panel cutout dimensions.
- Stackup drawing showing chip, gasket, lid, carrier, window, and clamp height.
- Leak-test pressure method and allowable deformation.
- Cleaning/sterilization assumptions for each material.
- DFM review for minimum wall thickness, tool reach, internal corners, and part warpage.

## First Build Package Requirements

Before sending to a manufacturer, create these files from the 16-slot baseline:

| Deliverable | Required content |
| --- | --- |
| `cassette_lower_carrier.step` | 16 chip pockets, datum pins, gasket land, drain/leak features, label lands, fastener holes. |
| `cassette_lid_clamp.step` | Clear imaging openings, gasket compression surface, captive fastener pattern, alignment features. |
| `cassette_window.step` | Separate retained window or window placeholder with sheet thickness. |
| `incubator_slot_dock.step` | Rack/dock plate with 16-slot support, logger pockets, condensate shields, drain/leak capture. |
| `service_bulkhead_test_block.step` | Dry service receiver with gas/media/sensor connector placeholders. |
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
| Condensation | No uncontrolled dripping onto cassette/chips; route condensate to visible gutters/drain witness features. |
| Oxygen | Reserve sensor/coupon windows, but do not make O2-control claims until gas mixing and sensor calibration are designed. |
| Fluid exchange | Dye-only first: per-slot flow, carryover, bubble, dead-volume, and pressure-spike checks before live-cell use. |
| Sterility boundary | Closed/disposable fluid path first; structural cassette is not automatically a sterile wetted component. |

## Vendor Questions

Send these with the RFQ:

1. Can you machine the 648 mm x 467 mm class carrier flat enough for gasket compression without stress relief or post-machining warp?
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
5. Level the incubator/dock and document utility clearances before environmental tests.
6. Run 24-hour dry incubation with temp/RH/CO2 logging.
7. Run slot-to-slot evaporation balance with sterile water/media surrogate.
8. Run media exchange using dye to measure carryover, dead volume, bubbles, and per-slot uniformity.
9. Verify cleaning access and residue removal on gasket lands, pockets, docks, and drains.
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

## Immediate CAD Work Next

Create a dedicated production-intent CAD generator named `sixteen_slot_cassette_incubator_first_article.rs` instead of continuing to adapt validation fixtures. It should emit the exact first-article custom parts:

- lower carrier
- lid/clamp frame
- window insert placeholder
- gasket witness coupon
- incubator dock plate
- service bulkhead test block
- full assembly

The generator should remove station-only evidence boards and keep only manufacturable part geometry.
