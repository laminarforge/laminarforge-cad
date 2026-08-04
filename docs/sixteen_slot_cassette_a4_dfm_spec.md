# 16-Slot Cassette A4 DFM And Manufacturing Spec

This A4 spec defines design-for-manufacturing constraints for the first
physical LaminarForge 16-slot AAV cassette build. It is subordinate to
`docs/sixteen_slot_cassette_a0_interface_spec.md`, which remains the
controlling interface contract. Manufacturing context comes from
`docs/cassette_incubator_manufacturing_readiness.md`.

This is not a final production drawing, biological validation claim, sterile
barrier claim, or detailed carrier/lid/gasket design. It defines the material,
process, tolerance, inspection, assembly-risk, and vendor-quote constraints
that the A1 carrier, A2 lid/clamp, A3 gasket, fluid-path, validation, and build
package agents must respect.

## Controlling Baseline

Locked A0 interface values remain unchanged:

| Interface | A0 baseline |
| --- | ---: |
| Cassette format | 16 slots, 4 x 4 |
| Experimental unit | One AAV condition per cassette |
| Active CAD baseline | `sixteen_slot_cassette_incubator_first_article` |
| Structural wetted-path policy | Dry reusable structural parts only |
| Fluid path | Bought disposable tubing/connectors unless separately validated |
| Carrier base body / true overall | 699.04 x 541.92 x 24.00 mm / 699.04 x 541.92 x 31.35 mm |
| Lid/clamp base body / true overall | 717.04 x 559.92 x 10.00 mm / 717.04 x 559.92 x 11.60 mm |
| Lid construction | One machined part: 4.00 mm continuous underside sealing skin plus 6.00 mm upper frame; the 635.04 x 457.92 mm lightening relief is upper-frame-only. |
| Window base panel / true overall | 667.04 x 489.92 x 3.00 mm / 667.04 x 489.92 x 4.80 mm |
| Dock plate base body / true overall | 869.04 x 691.92 x 22.00 mm / 869.04 x 691.92 x 40.00 mm |
| Service bulkhead base body / true overall | 789.04 x 34.00 x 76.00 mm / 789.04 x 60.00 x 76.00 mm |
| Chip gutter / slot pitch | 24.00 x 24.00 mm / 151.76 mm X, 109.48 mm Y |
| Slot array / centers | 583.04 x 413.92 mm; X = +/-227.64, +/-75.88 mm and Y = +/-164.22, +/-54.74 mm |
| Seal interfaces | Sixteen independent 145.76 x 103.48 mm outer / 129.76 x 87.48 mm inner lands, 8.00 mm wide, with 6.00 mm inter-land clearance; 629.04 x 459.92 mm outer / 605.04 x 435.92 mm inner perimeter land, 12.00 mm wide. Both are 7.35 mm high and meet the lid underside at closure. |
| Nominal closure plane | 7.35 mm above carrier top, matching nominal chip protrusion. |
| Gasket grooves / hard stops | Lid grooves are 1.80 mm deep x 3.20 mm wide for a 2.40 mm gasket at 25% nominal squeeze. Nine internal stops are 4.00 mm diameter at the 3 x 3 inter-slot intersections; perimeter stops are 4.00 mm wide in the 5.00 mm seal-to-gutter web. All stops top out at 7.35 mm and clear every seal. |
| Leak gutter / drain | 651.04 x 481.92 mm outer, 639.04 x 469.92 mm inner, 6.00 mm wide x 3.00 mm deep, with 5.00 mm web; drain diameter 8.00 mm at (315.52, -254.96, 10.00) mm, length 40.00 mm along Y. |
| Fasteners / datum bosses | Side fasteners X = +/-332.52 mm at slot-center Y; front/rear fasteners Y = +/-247.96 mm at slot-center X. Matching carrier receivers are 3.30 mm M4 tap-drill/pilot placeholders; final thread/insert SKU remains open. D1-D4 centers use X = +/-335.52 mm, Y = +/-204.96 mm; only D1 round and D2 relieved locate. D1/D2 use replaceable pins seated in the lid; integral pins are prohibited. |
| Labels / service cut | Global barcode land is 96.00 x 12.00 mm at (-271.52, -263.96) mm; adjacent text land is 118.00 x 10.00 mm at (-159.52, -263.96) mm. Global and per-slot labels stay outside the gutter and clear the front receiver row. Bulkhead sensor/backplane cut center X = 320.00 mm. |
| Side service relief | 7.00 mm high above carrier top, below the 7.35 mm lid closure plane. |
| Current pocket clearance | 1.20 mm per side |
| Current gasket squeeze assumption | 20-30 percent guard band, 25 percent nominal witness target |

The manufacturing readiness note proposes 0.5-0.8 mm chip pocket clearance per
side after real chip lot tolerance is measured. Until that measurement exists,
the DFM package must preserve the A0 1.20 mm per-side baseline and mark tighter
clearance as a pre-drawing decision, not a silent CAD change.

## Research Basis

The targeted Exa pass was limited to material/process/tolerance and
manufacturability decisions. It did not revisit biological workflow or cassette
architecture.

| Source | DFM finding used here |
| --- | --- |
| Xometry CNC Machining Design Guide, https://xometry.pro/wp-content/uploads/2023/07/TR-EN-CNC-Machining-Design-Guide.pdf | General CNC tolerances are commonly around +/-0.1 mm or ISO 2768 medium unless tighter features are called out. As-machined finish is commonly Ra 3.2 um or better. Tight tolerances increase fixturing, inspection, cost, and scrap, so they should be limited to critical features. Internal radii must exist because milling tools are round; radii should be greater than 0.5 mm and as large as possible. |
| Protolabs / Hubs sharp-corner machining note, https://www.hubs.com/knowledge-base/sharp-corners-in-cnc-machining/ | CNC pockets cannot have true sharp internal corners with standard end mills. Corner radius should scale with pocket depth; square internal corners require EDM or very small tools and should be avoided unless functionally necessary. |
| Protolabs DFM Guidelines for CNC Machining, https://www.protolabs.com/resources/design-for-machining-toolkit/ | Typical general CNC tolerance is +/-0.005 in. (+/-0.13 mm), with tighter network tolerances available only where justified. Holes deeper than 6 diameters, deep narrow pockets, very thin walls, and inaccessible features should be treated as DFM risks. |
| Parker O-Ring Handbook face seal chart, https://www.parker.com/content/dam/Parker-com/Literature/O-Ring-Division-Literature/O-Ring-ehandbook-pdfs/Design-chart-4-3-for-O-ring-face-seal-glands.pdf | Static face seal design depends on selected cross-section, gland depth, width, squeeze, corner radii, and pressure direction. Seal surface finish guidance is 32 RMS for liquids and 16 RMS for gases/vacuum. |
| Allorings static face seal guide, https://www.allorings.com/o-ring-groove-design-static-face-seal | Face seals need specified gland dimensions and surface finish; recommended finish is 16 Ra max for gases and 32 Ra max for fluids. |
| Apple Rubber seal and gland design, https://applerubber.s3.us-east-1.amazonaws.com/sdg/pdf/SealTypesGlandDesign_Section1.pdf | Static axial seals are easier to tolerance than radial seals; rectangular straight-sided grooves are preferred, sharp corners should be broken by at least 0.005 in., and static gland finish of 32 RMS is preferred with 16 RMS for gases. |
| RivCut 6061-T6 material guide, https://www.rivcut.com/materials/aluminum-6061 | 6061-T6 is the default machinable aluminum for structural prototypes; 6061-T651 is better for large flat parts where warping matters. |
| CNC surface finish guide, https://thesupplier.in/cnc-milling-surface-finish-guide-2/ | Typical fine CNC aluminum finish is Ra 1.6-3.2 um; low Ra 0.8-1.6 um should be limited to sealing/contact bands. Clear anodize changes dimensions and perceived finish, while bead blasting can raise Ra and should be masked away from seal bands. |
| CDC disinfection and sterilization guideline, https://stacks.cdc.gov/view/cdc/252996 | Cleaning/disinfection effectiveness depends on removal of organic/inorganic soil and correct disinfectant, concentration, contact time, and material compatibility. |
| Medical device anodization / hydrogen peroxide compatibility note, https://aqrdm.org/wp-content/uploads/2020/04/29-120951-190815-Medical-Device-Anodization-Compatibility-with-Hydrogen-Peroxide-Sterilization-Reformat_A4Poster_01_pub.pdf | Anodized aluminum, especially dyed organic-color finishes, can be affected by oxidizing sterilants. Clear or inorganic/electrocolored finishes and vendor compatibility review are preferred when oxidizing decontamination is possible. |

## Manufacturing Strategy

### First-Article Process Selection

Use CNC machining for the reusable carrier, lid/clamp, dock plate, and service
bulkhead first article. Add waterjet/laser/sheet processes only for non-critical
flat window placeholders, shields, or rough blanks that receive secondary
machining at datum and seal interfaces.

Do not use FDM, SLA, SLS, MJF, or printed polymer parts for final seal lands,
datum faces, dowel bores, chip pocket floors, or any vendor-quoted first article
claiming gasket compression control. Printed parts remain fit-check and fixture
prototypes only unless the integrator opens a separate validation path.

### Preferred Material Stack

| Part class | Preferred first quote | Alternate quote | DFM notes |
| --- | --- | --- | --- |
| Lower carrier | 6061-T651 aluminum plate, CNC machined, clear Type II anodize with masked seal/datum bands as needed | 6061-T6 if vendor can demonstrate flatness; 316 stainless only if cleaning chemistry demands it | T651 is preferred for the 699 x 542 mm class base body because stress-relieved stock reduces warp risk. Aluminum is acceptable because the carrier is dry structural hardware. |
| Lid/clamp | 6061-T651 or 6061-T6 aluminum, clear anodize, machined compression faces | 304/316 stainless for thinner clamp members if deflection is too high | Match material and finish to carrier when possible to reduce galvanic and thermal-expansion surprises. |
| Dock plate | 6061-T651 aluminum, CNC machined, clear anodize or hardcoat only after fit review | 304 stainless where cleaning durability dominates | Dock is dry support and drain/witness hardware. Require flatness evidence over the cassette support area. |
| Service bulkhead test block | 6061 aluminum or polycarbonate dry structure, CNC machined | 316 stainless if selected connectors require it | Do not custom-machine sterile/wetted connector geometry for the first article. Use bought connector cutouts. |
| Optical window placeholder | Borosilicate/glass or polycarbonate sheet, retained mechanically | Acrylic only for non-incubation fit checks | Avoid adhesive-only retention. Polycarbonate must be reviewed for disinfectant and stress-cracking compatibility. |
| Gasket witness coupons | Same material/finish as carrier seal bands | Separate coupon material only for screening | Coupons must duplicate seal-land finish, groove depth, coating, and cleaning exposure of production-intent parts. |

### Material Rejection Rules

Reject or escalate before quote if a proposed material/process depends on any
of the following:

- Machined structural parts acting as validated sterile wetted surfaces.
- Dyed cosmetic anodize on seal lands, datum bores, or barcode-critical zones
  exposed to oxidizing cleaners.
- Bead-blasted or media-tumbled gasket lands unless the vendor can preserve the
  required seal finish and flatness after finishing.
- Threaded plastic holes for repeated clamp loading without inserts or a
  documented life test.
- Large printed plates as the basis for flat gasket compression.
- Solvent-welded or adhesive-only optical windows where mechanical retention is
  required.

## Machining Constraints

### Setup And Datum Control

Machine the carrier chip-pocket floors, gasket lands, datum pin bores, and lid
alignment features in the minimum practical number of setups. The quote request
must ask whether the vendor recommends matched machining of the carrier and lid.

Critical datum policy:

- Datum A is the carrier bottom plane seated directly on the dock support plane;
  no intermediate spacer or recess offset is part of the active stack.
- Datum B is the dock rear-rail inner face contacting the nominal carrier rear
  edge.
- Datum C is the dock left-rail inner face contacting the nominal carrier left
  edge.
- The front low retention lip contacts the nominal carrier front edge and
  prevents gross lift/walkout but is not a competing datum.
- Only D1 at (-335.52, -204.96) is the round functional locator and D2 at
  (335.52, -204.96) is the relieved functional locator. D3/D4 at the rear
  coordinates are clearance/witness features, not hard locators.
- Dowel/pin bores must be drilled undersize and reamed after the pin diameter is
  selected.
- Machine 6.00 mm-diameter x 4.00 mm-deep replaceable-pin seats in the lid at
  D1/D2 only. The 5.80 mm x 7.35 mm CAD pin is a fit-check surrogate, not a
  production pin specification; D3/D4 receive no seats.
- Do not accept as-machined interpolated holes as final precision datum holes
  unless the vendor inspection report proves the required positional tolerance.

### Tool Access

The final drawing set must avoid features that require nonstandard tooling unless
the feature is explicitly called out as critical.

| Feature class | Constraint |
| --- | --- |
| Chip pockets | Use vertical walls and flat floors where possible. Avoid undercuts, draft-dependent pockets, or toolpaths requiring deep slender cutters. |
| Gasket lands/grooves | Keep carrier land faces accessible from the carrier top setup and lid groove walls accessible from the lid underside setup. Avoid dovetail grooves unless the gasket-retention need is proven and the vendor agrees to quote them. |
| Optical windows | Keep through-cuts large enough for standard end mills or sheet cutting. Protect view opening edges with chamfers or deburr notes. |
| Drain/gutter features | Avoid blind horizontal side-drilled features unless the drawing clearly defines drill entry, deburr access, and cleaning inspection. |
| Bulkhead cutouts | Dimension around selected bought connector panel geometry. Do not define custom sterile ports from placeholder diameters alone. |
| Label lands | Prefer shallow engraved or recessed features only where they do not trap residue or interfere with label adhesion. |

### Internal Radii

No square internal milled corners are allowed in manufacturer drawings unless
the drawing explicitly calls for EDM and explains why.

Minimum first-article radii:

| Feature | Minimum radius | Preferred radius | Notes |
| --- | ---: | ---: | --- |
| General internal pocket corners | R1.0 mm | R2.0-R3.0 mm | Use larger radii wherever chip fit and gasket geometry allow. |
| Chip pocket internal corners | R1.0 mm | R2.0 mm | Must not interfere with Rev C chip seating. If chip corners require clearance, use relief pockets rather than forcing sharp corners. |
| Gasket groove internal corners | Per selected gasket vendor | R1.0 mm or vendor-approved | Groove radii must not cut/nick gasket edges or create leak paths. |
| Drain/gutter internal corners | R1.5 mm | R3.0 mm | Rounded corners reduce machining time and cleaning traps. |
| Service bulkhead connector cutouts | Per connector drawing | R1.0 mm minimum where not connector-controlled | Connector manufacturer geometry controls. |
| External exposed edges | 0.25-0.50 mm chamfer or equivalent break | 0.50 mm chamfer | Larger chamfers may affect gasket compression and chip capture. |

Use dogbone or T-bone reliefs only when a square-cornered mating commercial part
must fit. Dogbone reliefs must be visible in the drawing and must not intrude
into gasket compression bands, optical keepouts, chip support lands, or label
areas.

## Tolerance Framework

General untoleranced machining should use ISO 2768-m or a vendor-equivalent
general CNC tolerance such as +/-0.13 mm, whichever the selected vendor uses.
Tight tolerances must be attached only to critical fit, datum, seal, and optical
interfaces.

Starting tolerance targets before vendor review:

| Feature | Starting target | Reason |
| --- | ---: | --- |
| Slot pitch, local adjacent pockets | +/-0.10 mm | Preserves the 151.76 x 109.48 mm 4 x 4 pitch without over-constraining the full 699 mm carrier. |
| Full-array pocket position relative to A/B datums | +/-0.25 mm | Keeps array compatible with dock/imaging references over the large plate. |
| Chip pocket X/Y size | +0.20 / -0.00 mm from selected clearance target | Pocket should not undersize and bind chips. |
| Pocket depth | +/-0.10 mm | Prevents chip seating and lid interference issues. |
| Pocket floor parallelism to carrier bottom | 0.10 mm over each pocket | Supports repeatable chip height and imaging stackup. |
| Optical through-window X/Y size | +/-0.20 mm | Preserve line of sight; avoid tight cosmetic tolerance. |
| Lid view opening X/Y size | +/-0.20 mm | Preserve optical keepout and deburr. |
| Lid chip-top relief | 130.16 x 87.88 mm, 0.50 mm deep, +/-0.05 mm depth starting target | Prevent chip/lid load while preserving 2.20 mm to the gasket-groove inner edge. |
| Dowel/registration bores | Reamed H7 or vendor-equivalent after pin selection | Final diameter depends on dowel family. |
| Dowel bore true position | 0.05-0.10 mm relative to datums | Critical for carrier-to-lid and dock repeatability. |
| M4/M5 clearance holes | ISO 2768-m unless hardware demands tighter | Clearance holes should not drive cost. |
| Threaded/insert features | Preserve the 3.30 mm carrier pilot diameter and locked centers; vendor standard thread class and inspected thread gauge after final hardware selection. | Final thread/insert SKU is not frozen. |
| Gasket land width | +/-0.05 mm where compression-critical | Supports the locked 2.40 mm cross-section and 3.20 mm groove; confirm against supplier tolerance and groove-fill review. |
| Lid gasket groove depth | 1.80 mm nominal, +/-0.03 mm after finish | Squeeze control is one of the highest-risk dimensions. |
| Lid groove floor | 2.20 mm nominal, 2.00 mm minimum after finish | The upper lightening relief must not penetrate the 4.00 mm seal skin. |
| Gasket land and hard-stop height | 7.35 mm nominal, +/-0.03 mm | The land and stop faces meet the lid underside and prevent overcompression and chip/lid load variation. |
| Service bulkhead placeholder holes | +/-0.20 mm until connector selected | Do not overtolerance placeholder geometry. |

Vendor review may relax noncritical dimensions. Vendor review must not relax
datum bores, gasket compression depth/stop height, local pocket pitch, or seal
band finish without an explicit design revision.

## Flatness And Parallelism

Flatness is a first-order risk because the carrier is a large, thin-ish plate
with repeated gasket compression and optical height requirements.

Starting targets:

| Surface | Starting target | Inspection note |
| --- | ---: | --- |
| Carrier bottom support face | 0.25 mm total flatness over full carrier | Vendor to state whether stress-relieved stock, rough/finish machining, or stress relief is needed. |
| Carrier 7.35 mm gasket/stop plane | 0.15 mm total flatness over full perimeter gasket plane | Must be measured after final coating/finish. |
| Per-slot gasket land plane | 0.05 mm flatness over each slot land | Critical for local seal squeeze. |
| Relative height between 16 slot gasket lands | 0.10 mm total | Prevents edge/center compression imbalance. |
| Lid/clamp compression face | 0.15 mm total flatness over full clamp face | Pair with carrier measurement. |
| Dock cassette support lands | 0.25 mm total over cassette support region | Dock must not twist the cassette. |
| Window insert seating plane | 0.10 mm local | Prevents stress concentration in glass/polycarbonate. |

Ask vendors to quote the flatness targets separately from dimensional tolerance.
If these targets drive high cost, request a DFM proposal with predicted flatness,
inspection method, and design changes such as thicker stock, ribbing, segmented
clamp bars, matched machining, or local compression stops.

## Surface Finish

Surface finish must be specified by function, not by whole-part cosmetic finish.

| Area | Required first-article finish | Finish cautions |
| --- | --- | --- |
| Gasket seal bands and groove floors | Ra 0.8 um target; Ra 1.6 um hard maximum | A gasket vendor may require smoother, never rougher. Do not bead blast. Mask or post-machine after anodize if needed. |
| Chip pocket floors and support lands | Ra 1.6-3.2 um | Avoid sharp burrs and high tool marks that change seating height. |
| Datum edges and bores | Machined, deburred, no coating buildup that changes fit unless accounted for | Bore fit must be measured after finish if anodized. |
| Optical window edges/openings | Deburred, no raised burrs or flakes | No cosmetic requirement unless imaging glare demands it. |
| Drain/gutter surfaces | Ra 1.6-3.2 um or smoother, fully deburred | Avoid rough traps that retain residue or dye. |
| External noncritical faces | As-machined, clear anodized, or cosmetic finish | Cosmetic bead-blast must not reach seal/datum bands. |
| Label lands | Smooth, clean, compatible with label adhesive | No oily residue, blast dust, or heavy texture. |

All machined parts must be deburred. Deburr must not round over gasket stops,
reduce gasket land width, enlarge chip pockets beyond tolerance, or alter datum
edges used by the dock.

## Coating And Dimensional Effects

Clear Type II anodize is the preferred aluminum finish for the first quote
because it improves corrosion resistance while keeping the part visually
inspectable. Hardcoat anodize should be quoted only as an option because its
greater thickness can disturb fits, pocket size, groove depth, and dowel bores.

Coating rules:

- Drawings must specify whether dimensions apply before or after coating.
- Precision bores must be masked, post-machined, or inspected after coating.
- Gasket groove depth and compression-stop height must be controlled after
  coating or protected from coating buildup.
- Seal bands must not receive bead blast if the blast raises roughness above
  the seal finish target.
- Dyed cosmetic anodize is not preferred for the first article. Use clear or
  non-dyed finishes unless labeling/visual coding is required outside critical
  surfaces.

## Cleaning And Compatibility

The first article is reusable dry hardware around a disposable fluid path. The
cleaning spec is therefore a material-compatibility and residue-removal
requirement, not a sterile release claim.

Required cleaning compatibility review:

| Material/feature | Compatible direction | Risks to verify |
| --- | --- | --- |
| Clear anodized 6061 aluminum | Mild detergent, water rinse, 70 percent IPA wipe, humid 37 degrees C incubation exposure | Strong oxidizers, chlorine/bleach, peroxide sterilants, and repeated alkaline exposure may discolor or attack finishes; vendor must advise. |
| 316 stainless | Broadest chemical durability option | Higher cost, mass, machining time, and possible galling with stainless hardware. |
| Polycarbonate window | Humid incubation and gentle detergent cleaning if stress is controlled | Solvent and disinfectant stress cracking; compatibility with IPA, peroxide, bleach, and quats must be reviewed before use. |
| Borosilicate/glass window | Strong chemical durability and imaging stability | Edge chips, retention stress, breakage handling, and seal-to-glass interface. |
| Silicone gasket | Good first screening material for compression witness work | Swell/compression set under cleaner, incubation humidity, and clamp dwell. |
| EPDM gasket | Often strong against water/peroxide exposure | Compatibility with media residues, oils, and selected disinfectants must be checked. |
| Barcode/RFID label | Humidity- and wipe-compatible label stock | Condensation, IPA wipes, adhesive creep, and anodized surface adhesion. |

First-article acceptance must include post-cleaning visual inspection of gasket
lands, chip pockets, drain gutters, datum bores, and label lands. Any visible
residue, corrosion product, dye retention in a crevice, label lift, or gasket
surface damage is a DFM failure until the part geometry, finish, or cleaning
method is revised.

## Assembly Order And Risk Controls

Manufacturing drawings must preserve an assembly order that avoids hidden
damage, trapped residue, and uncontrolled gasket compression.

Recommended dry assembly order for fit checks:

1. Inspect and clean machined carrier, lid, dock, service block, and window
   placeholder.
2. Verify datum bores, pocket dimensions, gasket-land finish, and flatness
   before installing inserts, labels, or gaskets.
3. Install threaded inserts or captive hardware if selected, then re-inspect
   nearby flatness and burrs.
4. Install barcode/RFID labels on approved clean label lands.
5. Place dummy Rev C chips or gauges into chip pockets and verify no binding.
6. Install gasket or gasket witness coupons using the A3-defined process.
7. Install retained window placeholder if applicable.
8. Close lid/clamp in defined torque sequence using compression witnesses.
9. Seat datum A directly on the dock support plane, contact the nominal rear,
   left, and front carrier edges against the corresponding rail inner faces and
   front lip, and confirm the dock does not twist the carrier.
10. Run no-cell leak, dye, drain, and cleaning checks only after dry metrology
    passes.

Assembly order risks:

| Risk | DFM control |
| --- | --- |
| Insert installation distorts large carrier | Specify insert process, location, and post-install flatness check. Prefer through-bolts or metal inserts only where repeated use demands them. |
| Lid torque overcompresses gasket or chips | Require compression stops, witness coupons, and torque sequence before live-use claims. |
| Burrs cut gaskets or scratch windows | Require deburr inspection of all seal/window edges before gasket/window install. |
| Coating buildup changes dowel or groove dimensions | Inspect critical fits after finish; mask or post-machine if needed. |
| Label installation blocks cleaning or datum contact | Keep labels on dedicated lands outside gasket, datum, and rail interfaces. |
| Dock rails introduce twist | Verify cassette flatness free-state and docked. Add local supports or relax rail preload if needed. |
| Window retention loads glass/polycarbonate unevenly | Use compliant retention and local flatness checks. Avoid point load at sheet edges. |
| Gasket grooves trap cleaning residue | Prefer simple rectangular accessible grooves. Avoid dovetails unless retention need is proven. |

## Vendor Quote Package Requirements

The RFQ package must not send STL files as the manufacturing definition.
Vendor-ready quote packages require STEP files and drawings.

Minimum RFQ deliverables:

- `cassette_lower_carrier.step` and drawing.
- `cassette_lid_clamp.step` and drawing.
- `cassette_window.step` or sheet drawing.
- `incubator_slot_dock.step` and drawing.
- `service_bulkhead_test_block.step` and drawing.
- Gasket witness coupon drawing with same material/finish notes as the real
  seal interface.
- Preliminary BOM with hardware, inserts, gasket stock, label stock, window
  stock, and bought connectors marked as placeholders where not selected.
- Inspection table listing datum bores, pocket array, gasket lands, flatness,
  surface finish, coating thickness, and deburr.
- Explicit note that the first quote is a mechanical no-cell first article and
  does not make sterile, clinical, live-cell, AAV containment, or biological
  release claims.

Vendor quote questions:

1. Can you hold the requested flatness on a 699.04 x 541.92 x 24.00 mm base-body carrier
   after pocketing, gasket features, and anodize?
2. Do you recommend 6061-T651, 6061-T6, 316 stainless, or another material for
   this large gasketed dry fixture?
3. What stock thickness, roughing/finishing order, stress relief, or post-finish
   machining do you recommend to reduce warp?
4. What minimum internal radii should be applied to chip pockets, gasket
   grooves, drain gutters, and bulkhead cutouts for your standard tools?
5. Can you machine carrier gasket lands, pocket floors, and datum bores in a
   shared setup or otherwise provide relative inspection data?
6. Should the carrier and lid be machined or inspected as a matched pair?
7. Can you provide CMM or optical inspection for the 4 x 4 pocket array, datum
   bores, and gasket land heights?
8. Can you measure and report surface finish on seal bands separately from
   cosmetic faces?
9. Can you mask seal bands and precision bores during anodize, or should those
   surfaces be post-machined?
10. Can you source/install threaded inserts or captive hardware, and how will
    installation affect flatness?
11. What cleaning agents and wipe/disinfection products are compatible with
    your proposed material and finish over repeated humid incubation exposure?
12. Can you quote one prototype set, three prototype sets, and a pilot quantity
    with DFM comments separated from price?

Quote options to request:

| Option | Purpose |
| --- | --- |
| 6061-T651 clear anodized baseline | Preferred first article for cost, machinability, and flatness risk reduction. |
| 6061-T651 no anodize / chem-film / masked finish option | Determines whether coating complexity is driving cost or fit risk. |
| 316 stainless carrier/lid/dock option | Cleaning-durability comparison where chemistry forces it. |
| Matched carrier/lid machining option | Evaluates compression uniformity improvement. |
| CMM inspection add-on | Separates fabrication cost from measurement cost. |
| Flatness-relaxed DFM option | Captures vendor's practical lower-cost alternative before redesign. |

## Drawing Notes To Carry Forward

Use these as starting drawing notes. Revise only after selected vendor and
gasket/hardware choices are known.

- Material: 6061-T651 aluminum plate unless otherwise specified.
- Finish: clear Type II anodize, natural, with critical seal bands and precision
  bores controlled after finish or masked per drawing.
- General tolerance: ISO 2768-m unless otherwise specified.
- Break all noncritical sharp edges 0.25-0.50 mm.
- Do not break, polish, or blend gasket compression stops beyond specified
  tolerance.
- Seal bands: Ra 0.8 um target and Ra 1.6 um hard maximum; a gasket vendor may require smoother, never rougher.
- General machined faces: Ra 3.2 um max unless otherwise specified.
- No burrs, loose media, sharp chips, or embedded blasting residue.
- Dimensions for gasket groove depth, compression stop height, precision bores,
  and pocket size apply after coating unless explicitly marked pre-coat.
- Vendor to provide inspection report for critical dimensions listed on drawing.
- Part is dry reusable structural hardware for no-cell first-article testing;
  sterile/wetted path is not claimed by this drawing.

## Acceptance Criteria

The A4 manufacturing package is acceptable when all of these are true:

- The 16-slot A0 interface geometry is preserved, including slot grid,
  base-body dimensions, true overall bounds, datum intent, imaging keepouts,
  corrected nonoverlapping seal geometry, and reserved dry service interfaces.
- The drawing set identifies which dimensions are general tolerance and which
  are critical to gasket compression, chip fit, optical access, dock
  registration, or assembly repeatability.
- Gasket-land finish, groove/stop height, and flatness are specified and
  inspectable after finish.
- Internal corners are machineable with called-out radii or documented reliefs.
- The selected material/process does not make structural parts into unvalidated
  sterile wetted-path components.
- Cleaning compatibility is reviewed for aluminum finish, window material,
  gasket material, labels, inserts, and hardware.
- Vendor quote notes request DFM feedback, material/process options, flatness
  risk comments, inspection reports, and separate cost for tight tolerances.
- Assembly order includes metrology before gasket installation and no-cell
  validation before any biological use.
- Any deviation from A0 dimensions, pocket clearance, dowel strategy, or gasket
  squeeze range is promoted to an interface revision rather than hidden in a
  manufacturing drawing.

## Open Manufacturing Decisions

The following remain unresolved and must be closed by subsystem owners or the
cassette integrator before release drawings:

- Final chip pocket clearance after measuring real Rev C chip lot tolerance.
- Final pin SKU, bore fit, bore positional tolerance, and reaming process while
  preserving the locked D1-round/D2-relieved locator roles.
- Final screw family, torque sequence, captive hardware, and thread/insert SKU.
  The 16 fastener centers, 3.30 mm carrier pilots, and hard-stop geometry are
  fixed.
- Final gasket compound/vendor and leak-test pressure; the shared A0/A3 draft
  already fixes the 2.40 mm free height, 1.80 x 3.20 mm lid groove, 25%
  squeeze target, 7.35 mm common land/stop height, and 7.35 mm closure plane.
- Whether gasket seal bands are masked during anodize, post-machined, or left
  as-machined.
- Final optical window material, thickness, retention method, and allowable
  cleaning agents.
- Final barcode/RFID label material and wipe/condensation survivability test.
- Vendor-specific minimum radii, fixture strategy, flatness capability, and
  inspection method.
