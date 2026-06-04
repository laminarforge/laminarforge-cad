# 16-Slot Cassette A3 Gasket Subsystem Spec

Ticket: T-3ABDE2C9

This is the gasket subsystem specification for the first physical LaminarForge 16-slot AAV cassette build. It is subordinate to `docs/sixteen_slot_cassette_a0_interface_spec.md`; if this document conflicts with A0, A0 controls until the cassette integrator updates the interface contract.

This is not a sterile-barrier claim, biological release criterion, or final production drawing. It defines the first-article gasket geometry, material candidates, compression evidence, and no-cell validation gates needed before manufacturer drawings and live-cell/AAV work.

## Scope Boundary

A3 owns:

- Per-slot and perimeter gasket material direction.
- Gasket cross-section and provisional gland dimensions.
- Compression target, compression stops, and witness shims/coupons.
- Gasket land surface finish, flatness, and cleaning-inspection assumptions.
- No-cell leak, coupon burst, and repeated reconnection validation logic.
- Gasket-specific open decisions for integrator, A2 lid/clamp, A4 DFM, and validation agents.

A3 does not own carrier architecture, lid/clamp layout, disposable tubing route, port family, incubation control, or final CAD generator edits except where those features directly set gasket compression, retention, or test interfaces.

## A0 Interface Inputs

The gasket design uses these A0 locked inputs:

| Interface | A0 value |
| --- | ---: |
| Cassette format | 16 slots, 4 x 4 |
| Active CAD baseline | `sixteen_slot_cassette_incubator_first_article` |
| Per-slot gasket land outer | 141.76 x 99.48 mm |
| Per-slot gasket land wall width | 8.00 mm |
| Per-slot gasket land height | 3.00 mm above carrier top |
| Perimeter gasket land outer | 604.04 x 428.92 mm |
| Perimeter gasket land wall width | 12.00 mm |
| Perimeter gasket land height | 4.00 mm above carrier top |
| Nominal gasket thickness | 2.40 mm |
| Target squeeze | 25% |
| Squeeze guard band | 20-30% |
| Witness steps | 20%, 25%, 30% |

First-article wetted-path policy remains unchanged: disposable commercial tubing/connectors are the culture-fluid boundary. Structural cassette parts and A3 gaskets are dry structural/environmental seals unless separately validated as wetted, cell-facing, and AAV-compatible interfaces.

## Target Seal Architecture

Use controlled-compression axial face seals, not adhesive-only sealing.

The first-article gasket should be a replaceable continuous elastomer seal seated in machined grooves on the lid/clamp compression side, compressed against the raised carrier gasket lands. This keeps the carrier land surfaces simple to inspect and lets the lid carry gasket retention features. If A2 chooses the opposite layout, the same compressed height, groove fill, corner radius, and witness requirements still apply.

Recommended first-article architecture:

| Seal loop | Function | Direction |
| --- | --- | --- |
| 16 per-slot loops | Local chip-pocket/environmental isolation and visible leak segregation by slot | One continuous loop per slot, seated over the A0 per-slot raised land. |
| Perimeter loop | Outer condensate/leak containment and environmental boundary | One continuous loop on the A0 perimeter land; do not treat this as a high-pressure chamber boundary until structural proof is complete. |
| Witness coupon loops | Process and material evidence | Same cross-section, gland depth, finish, and compression stops as production loops. |

Avoid discrete corner pieces, butt joints, hand-cut square gasket corners, or RTV-in-place sealing for the first article. If spliced cord is used, splice joints must be vendor-made or made with a documented fixture and placed on straight segments away from ports, drains, and high-observation zones.

## Material Candidates

The first material downselect is between high-purity silicone and high-purity EPDM. TPE/SEBS remains a research-backed option for integrated organ-chip microgaskets, not the default reusable cassette face seal until vendor review.

| Rank | Material | Use case | Advantages | Risks / limits | A3 disposition |
| ---: | --- | --- | --- | --- | --- |
| 1 | Platinum-cured silicone, 50-60 Shore A, USP Class VI / ISO 10993 supplier data preferred | First dry/environmental cassette gasket and witness coupons | Common medical/bioprocess gasket material, tolerant of 37 C humid incubation, available as molded seals or cord, low closure force versus harder elastomers | Gas permeability; possible absorption/adsorption if used as wetted drug/vector-contact surface; tear risk at thin sections | Baseline candidate for first quote and no-cell validation. Not approved for AAV/culture-fluid contact without extractables/adsorption review. |
| 2 | High-purity EPDM, 60-70 Shore A, USP Class VI supplier data preferred | Alternate for humid/water/cleaning exposure and higher tear margin | Good water/steam/humidity compatibility, lower gas permeability than silicone in many applications, robust reusable seal option | Less transparent; compatibility depends on cleaning chemistry; not for oils/hydrocarbons; higher closure force at 70A | Quote as alternate material and run same witness coupon tests. |
| 3 | Oil-free SEBS/TPE microgasket material | Future disposable chip-interface or integrated microgasket layer | Organ-chip literature supports reversible compression interfaces and lower small-molecule absorption than PDMS in some TPE systems | Supplier-specific; large reusable face-seal behavior and compression set must be proven; fabrication path differs from simple machined gland | Escalated to future disposable interface work, not first carrier/lid baseline. |
| 4 | FKM / FFKM | Specialty chemical-resistance gasket if cleaning chemistry forces it | Strong chemical resistance and low gas permeability options | Higher cost and closure force; cell-facing/bioprocess data must be compound-specific | Do not use for first article unless DFM/vendor review rejects silicone and EPDM. |
| Rejected baseline | PDMS | Common organ-chip elastomer | Familiar and easy to prototype | Strong small-molecule absorption risk and poor fit for AAV condition-interface assumptions | Not the first-article gasket baseline. |

Material certification required for quote package:

- Supplier, compound name, durometer, lot traceability, and sheet/cord dimensional tolerance.
- Compression set data at 37 C in humid conditions or nearest available test.
- Cleaning exposure notes for the actual planned cleaning agents.
- Extractables/biocompatibility data if any part of the gasket later becomes wetted or cell-facing.

## Cross-Section And Groove Dimensions

Baseline cross-section: 2.40 mm nominal round cord/O-ring cross-section, used as a controlled axial face seal.

This keeps A0's 2.40 mm gasket thickness and 20-30% witness range intact while using standard static face-seal gland logic. Larger cross-sections are more tolerant of flatness error but would increase clamp load and consume the current gasket lands; smaller cross-sections reduce tolerance margin. Do not change the 2.40 mm baseline without updating A0 and the witness-coupon steps.

First-article gland dimensions:

| Parameter | Baseline value | Notes |
| --- | ---: | --- |
| Gasket cross-section / free height | 2.40 mm nominal | Supplier tolerance target +/-0.08 mm or better. |
| Target compressed height | 1.80 mm | 25% squeeze. |
| Acceptable compressed-height band | 1.68-1.92 mm | 30-20% squeeze guard band. |
| Groove depth | 1.82 mm | Starting value for 2.40 mm axial liquid face seal. Update if supplier cross-section tolerance differs. |
| Groove width | 3.20 mm | Starting liquid-service width; leaves expansion volume and fits inside A0 land widths. |
| Groove side finish | 1.6 um Ra max | Deburred; no tool marks that cut the seal. |
| Seal land top/bottom finish | 0.8 um Ra max | Liquid/no-cell leak validation. Use 0.4 um Ra max if gas/vacuum retention becomes a requirement. |
| Groove entry radius / break | 0.20 mm min | No sharp edges at gasket entry. |
| Seal-loop corner radius | 3.0 mm min centerline radius | Larger preferred; no square elastomer corners. Confirm with gasket vendor. |
| Groove fill check | Vendor-calculated before drawings | Must leave expansion volume at temperature and after media/cleaning exposure. |

Per-slot fit on the 8.00 mm A0 land:

- 3.20 mm groove/seal width centered on the land.
- Minimum 2.0 mm nominal machined land margin on each side after groove placement.
- No groove overlap into optical keepout, chip pocket edge, label land, drain feature, or fastener clearance.

Perimeter fit on the 12.00 mm A0 land:

- Same 2.40 mm cross-section and 3.20 mm groove width unless vendor review recommends a larger perimeter seal.
- Keep at least 3.0 mm nominal land margin on each side.
- Treat the perimeter loop as a containment/environmental seal; do not design validation around pressurizing the full cassette footprint.

If A2/A4 choose a custom molded rectangular gasket instead of round cord, the substitute must preserve:

- Free height 2.40 mm nominal.
- Compressed height 1.80 mm nominal.
- 20-30% verified squeeze range.
- Groove or pocket geometry with volume expansion room.
- Rounded molded corners and no hand-cut stress risers.
- Same witness-coupon compression and leak evidence.

## Compression Stops

Compression must be set by hard stops and verified by witnesses. Fastener torque is only the method used to bring the lid down to the stops; torque alone is not an acceptable compression-control strategy.

Required stop logic:

| Stop item | Requirement |
| --- | --- |
| Nominal stop gap | 1.80 mm from gasket land sealing face to mating compression face for a 2.40 mm gasket. |
| Guard-band verification | Assembly must reject stop stacks that produce less than 1.68 mm or more than 1.92 mm compressed height at any measured witness location. |
| Per-slot stops | At least four local stops or equivalent stiff clamp features around each slot loop, placed outside imaging keepouts. |
| Perimeter stops | Distributed hard stops around the perimeter loop; spacing selected by A2/A4 after lid stiffness analysis. |
| Datum relation | Stop heights must reference the same machined setup as the gasket land/groove where possible. |
| Shim access | Witness shim insertion/removal must be possible without disturbing the gasket or chip pocket. |

Because A0 has per-slot lands at 3.00 mm above carrier top and the perimeter land at 4.00 mm above carrier top, A2 must either use stepped lid compression faces or separate stop stacks for per-slot and perimeter loops. A3 does not allow a single flat lid underside to compress both seal elevations unless the integrator changes the land heights.

## Witness Shims And Coupons

Witness evidence is required before the cassette is trusted for no-cell liquid testing.

### Assembly Witness Shims

Provide three go/no-go references tied to A0:

| Witness label | Compressed height | Equivalent squeeze |
| --- | ---: | ---: |
| 20% | 1.92 mm | Minimum acceptable squeeze |
| 25% | 1.80 mm | Nominal target |
| 30% | 1.68 mm | Maximum acceptable squeeze |

Use shims at the front-left, front-right, rear-left, rear-right, and center-zone witness locations for both the per-slot field and the perimeter loop. If access to all 16 slot loops is impractical, use a mapped sampling plan that includes all four corners, all four rows, all four columns, and at least four interior slots.

### Gasket Compression Coupons

Create separate coupons using the same material, groove depth, groove width, land finish, stop gap, and fastener style as the cassette.

Minimum coupon set:

| Coupon | Purpose | Acceptance evidence |
| --- | --- | --- |
| 20/25/30 squeeze ladder | Confirms visual and dimensional witness behavior | Measured compressed height agrees with shim labels. |
| Leak coupon | Isolated small-volume liquid pressure check | No visible dye leak and stable pressure within the defined threshold. |
| Burst coupon | Destructive margin test away from full cassette | Failure pressure recorded; failure mode is gasket extrusion, tear, or hardware lift, not ambiguous seepage. |
| Reconnection coupon | Repeated assembly damage screen | Leak test passes at defined cycle counts with inspected gasket surface. |
| Soak coupon | Material and compression-set screen | Post-soak dimensions and visual condition recorded after planned incubation/cleaning exposure. |

Witness coupons must be ticketed/artifacted with material lot, fastener torque-to-stop notes, photos/measurements, and test results. A passed CAD build alone is not gasket validation evidence.

## Surface Finish, Flatness, And Handling Assumptions

Machined metal or suitable engineering plastic is required for first-article seal lands. Printed layer lines are not acceptable on production-intent seal surfaces.

Starting drawing callouts:

| Feature | First-article callout |
| --- | --- |
| Sealing land top/bottom | 0.8 um Ra max for liquid validation; 0.4 um Ra max if gas/vacuum retention is later required. |
| Groove sidewalls | 1.6 um Ra max. |
| Local per-slot land flatness | 0.05 mm target across each seal loop after machining and deburr. |
| Perimeter land flatness | 0.15 mm target around the full perimeter loop and 0.05 mm per 100 mm segment. |
| Burrs | No raised burrs, rolled edges, or anodize buildup that changes squeeze. |
| Machining lay | Avoid continuous radial/cross-groove tool marks that create leak paths across the seal. |
| Coatings | Clear anodize or passivation must be reviewed for seal-land buildup; mask or post-finish lap if needed. |

Cleaning and handling:

- Gasket lands must be inspectable under white light before assembly.
- Do not scrape seal lands with metal tools.
- Replace gaskets after visible cuts, compression cracking, permanent flattening outside supplier limits, contamination, or failed leak evidence.
- Lubricant is not part of the baseline because culture-facing contamination risk is unresolved. If assembly damage requires lubricant, select a documented, biocompatibility-reviewed lubricant and repeat leak/coupon tests.

## Leak, Burst, And Reconnection Test Logic

All tests below are no-cell, nonhazardous-liquid validation gates. They do not certify sterility, AAV containment, or live-cell readiness.

### Test Order

1. Dimensional inspection of groove depth, groove width, land finish, stop gap, and gasket free height.
2. Dry closure to hard stops using witness shims.
3. Isolated leak-coupon test at nominal squeeze.
4. Isolated per-slot loop leak tests on the cassette.
5. Perimeter containment leak test without pressurizing the full cassette footprint.
6. Coupon burst test.
7. Reconnection cycle test.
8. Humid 37 C soak/conditioning followed by repeat leak test.

### Leak Test

Use isolated liquid tests with the smallest practical trapped volume. Do not run a full-envelope pneumatic pressure test on the 600 mm class perimeter loop.

Starting per-loop leak target:

| Parameter | First-article target |
| --- | --- |
| Test medium | DI water with visible dye or equivalent nonhazardous tracer. |
| Test pressure | 35 kPa gauge (5 psi) or 1.5x the selected maximum expected cassette fluid-interface pressure, whichever is greater, only for isolated small-volume loops. |
| Hold time | 10 minutes after pressure stabilization. |
| Pressure-decay acceptance | <=5% decay after thermal stabilization, unless fixture compliance characterization sets a tighter threshold. |
| Visual acceptance | No visible dye in adjacent slot, optical window area, drain gutter, perimeter gutter, or underside witness paper. |
| Documentation | Record loop ID, gasket lot, squeeze witness, start/end pressure, hold time, visible result, and operator. |

If the fluid-path agent later selects a lower allowable pressure due to chip or tubing limits, use a separate gasket coupon to prove gasket margin and keep cassette loop tests below the weakest installed component limit.

### Burst Test

Burst validation belongs on coupons or instrumented surrogate loops before the cassette is challenged.

Starting coupon burst target:

- Pressurize a representative coupon loop in liquid to at least 3x the leak-test pressure or 100 kPa gauge (15 psi), whichever is greater.
- Record failure pressure and failure mode.
- Accept the first-article gasket direction only if failure occurs above the target without lid lift, fastener damage, or uncontrolled extrusion at the normal 25% squeeze stop.

Do not use the full cassette perimeter loop as the first burst article. The clamp load from large-area pressure can dominate the result and damage the lid/carrier before the gasket failure mode is understood.

### Reconnection Test

Reconnection evidence is required because the cassette is reusable and organ-chip literature treats reversible assembly as a first-class interface risk.

Starting reconnection plan:

| Cycle point | Action |
| ---: | --- |
| 1 | Leak test, inspect gasket and lands. |
| 5 | Leak test, inspect for cuts, flattening, twist, and debris. |
| 10 | Leak test and dimensional spot-check of compressed witness marks. |
| 25 | Leak test, inspect, and decide whether 100-cycle durability is needed before live-cell work. |

The 25-cycle first-article gate is a minimum. Escalate to 100 cycles if the cassette workflow requires frequent chip swaps, if the selected material has uncertain compression-set behavior, or if supplier data is weak.

## Resolved Decisions And Escalations

| Item | A3 disposition | Owner for next action |
| --- | --- | --- |
| First gasket form | Resolve to replaceable continuous axial face seals, 2.40 mm nominal round cord/O-ring baseline. | A2/A4 to implement in drawings; vendor to review. |
| First material baseline | Resolve to quote platinum-cured silicone first, high-purity EPDM alternate. | A4/vendor; validation coupons compare both if budget allows. |
| Compression target | Keep A0 25% nominal with 20-30% allowed band. | A2 hard stops and A4 tolerance stack. |
| Groove dimensions | Start with 1.82 mm depth, 3.20 mm width, 0.20 mm entry radius for 2.40 mm cross-section. | A4/vendor to confirm fill, tolerance, and corner radius before release drawing. |
| Stop strategy | Resolve that hard stops are mandatory; torque alone is not acceptable. | A2 lid/clamp and A4 DFM. |
| Land elevation mismatch | Escalate: A0 per-slot land height is 3.00 mm, perimeter is 4.00 mm. Compression faces/stops must be stepped or A0 must change. | Integrator/A2. |
| Surface finish | Resolve 0.8 um Ra liquid seal-land target; 0.4 um Ra if gas/vacuum claim is added. | A4 drawing package/vendor. |
| Leak-test pressure | Resolve first-article isolated-loop liquid target at 35 kPa gauge (5 psi) or 1.5x selected maximum expected pressure, whichever is greater. | Validation agent to fixture and characterize compliance. |
| Burst validation | Resolve to coupon/surrogate burst first; no full-cassette perimeter burst before structural review. | Validation agent/A2. |
| Reconnection durability | Resolve 25-cycle first gate, escalate to 100 cycles when workflow or material uncertainty requires it. | Validation agent/integrator. |
| Wetted gasket use | Escalate: no gasket is approved as culture-fluid/AAV-contacting until material absorption, extractables, and cleaning compatibility are reviewed. | Integrator/fluid-path/biology owner. |
| Lubrication | Escalate: baseline is dry assembly. Any lubricant requires compatibility review and repeated coupon tests. | A4/vendor/validation. |

## Targeted Exa Research Pass

The A3 research pass was intentionally limited to face seals, elastomer selection, organ-chip gasket interfaces, and leak/burst validation.

| Source | Finding used by A3 |
| --- | --- |
| Apple Rubber, Seal Design Guide, https://www.applerubber.com/src/pdf/seal-design-guide.pdf | Seal design must be application-verified; material, gland, environment, and failure mode cannot be inferred from nominal gasket size alone. |
| Sealing Devices, O-ring face seal guide, https://sealingdevices.com/blog/o-ring-face-seal-design-guide/ | Axial face seals depend on groove depth/width, plate gap, surface quality, flatness, tolerance stack, deburr/radius, torque pattern, and early pressure/media/temperature validation. |
| Ace Seal static axial gland guide, https://www.aceseal.com/gland-design-static-axial-application | 2.40 mm static axial liquid face seal starting dimensions align with roughly 1.82 mm gland depth and 3.20 mm liquid groove width; liquid land finish target is 32 microinch / 0.8 um Ra and gas/vacuum is finer. |
| KEF America medical elastomer tradeoff note, https://www.kef-america.com/material-tradeoffs-silicone-vs-fluoroelastomers-vs-epdm-for-medical-devices/ | Silicone, EPDM, and fluoroelastomers have different biocompatibility, chemical, permeability, and compression-force tradeoffs; compound-specific data matters. |
| James Walker Elast-O-Pure EP75, https://www.jameswalker.biz/our-solutions/our-products/elastomers/materials-for-biopharmaceutical-applications/elast-o-pure-ep75 | High-purity USP Class VI EPDM-class materials exist for biopharmaceutical sealing and are credible alternates to silicone. |
| Minahan et al., modular reusable perfusion-ready MPS cassette, https://pmc.ncbi.nlm.nih.gov/articles/PMC12914553/ | Reusable cassette hardware with elastomeric inserts and clamping can support reversible leak-tight organ-chip assembly while preserving imaging access. |
| Sun et al., reusable standardized universal interface module, https://www.mdpi.com/2072-666X/10/12/849 | Reusable organ-chip interfaces emphasize standardized alignment, reversible clamping, and leakage-free assembly. |
| Integrated micro-gasket multi-organ chip connections, https://www.mdpi.com/2072-666X/16/11/1251 | TPE/SEBS integrated microgaskets are relevant for reversible organ-chip connections; burst, durability, repeated detachment/reconnection, and incubation exposure are appropriate validation categories. |
| Gong et al., high-density reversible microfluidic interconnects, https://pmc.ncbi.nlm.nih.gov/articles/PMC5811381/ | Microgasket validation can include pressure monitoring, visible liquid leak observation, high-pressure margin testing, and repeated reconnection cycles; reported systems demonstrate 50 psi and 100 reconnections for their specific microgasket architecture, not as direct LaminarForge acceptance limits. |

## Manufacturer Drawing Notes To Carry Forward

Add these notes to the A4 drawing package when the gasket geometry is promoted from spec to drawings:

- Gasket glands are controlled-compression axial face seals; do not substitute adhesive gasket or RTV without engineering approval.
- Seal surfaces must be free of burrs, tool marks crossing the seal path, coating buildup, and printed layer texture.
- Supplier must confirm gasket compound, durometer, dimensional tolerance, compression set, and cleaning/incubation compatibility.
- Vendor should review groove fill, squeeze stack, corner radii, and splice/mold strategy before machining.
- Compression stops and witness coupons are part of the deliverable, not optional inspection extras.
