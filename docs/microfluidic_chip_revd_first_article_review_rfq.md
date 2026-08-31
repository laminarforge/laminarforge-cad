# Rev D0 Microfluidic First-Article Review And RFQ

Status: advisor-review draft; not released for fabrication

Ticket: T-8125D858

Date: 2026-08-31

## Decision

Do not quote the current full Rev D model as a build-ready tissue chip. First
buy a paid DFM review, correct the geometry, and fabricate a single-lane
water/dye coupon. The coupon is a manufacturing and flow-learning article, not
an organ-on-chip or biological result.

## Why the current Rev D model is blocked

The source at `src/bin/microfluidic_chip_revd.rs` is useful concept work, but it
contains contradictions that must be resolved before money is spent:

1. Each chamber is 6 mm wide, but its five inlet and five outlet channels are
   placed at X offsets of -18, -9, 0, +9, and +18 mm. Only the center channel
   intersects the chamber; the other four channels on each side terminate in
   solid PMMA.
2. The chambers are cut into the bottom face. The supposed top TEER electrode
   pockets are cut into the opposite top face of the same 14.2 mm PMMA body, so
   they do not contact the chamber fluid.
3. The build notes say Ti/Au and Pt are sputtered on glass, while the generator
   cuts the electrode and RTD pockets into PMMA. The deposited-layer stack and
   electrical routing are not defined consistently.
4. The four chip-stacking vias are not connected to a chamber or channel.
5. No porous membrane or paired tissue channel exists, so the model cannot yet
   support the stated TEER or tissue-barrier use.
6. The 140 um PSA layer is almost as thick as the 150 um shallow channel. A
   vendor must evaluate channel intrusion/collapse and the bonding process.
7. The generator creates STL first and converts it to STEP. A triangulated
   conversion is not an acceptable machining definition unless the vendor
   explicitly accepts it; native B-rep STEP and controlled 2D drawings are the
   target.

These are review blockers, not vendor-change requests. A fabricator must not be
asked to infer or silently repair the design.

## Proposed Rev D0 coupon for redlining

The advisor may change these values when the manufacturing reason is recorded.

| Item | Proposed starting point | State |
| --- | --- | --- |
| Use | Water and dye flow only | Fixed |
| Footprint | 75 x 25 mm slide-format coupon | Proposed |
| Top fluidic body | Clear PMMA, nominal 3 mm thick | Proposed |
| Bottom | Clear PMMA or borosilicate, selected with bond process | Open |
| Flow paths | One inlet, one chamber, one outlet | Fixed |
| Channel | 0.50 mm wide x 0.20 mm deep | Proposed |
| Chamber | 6.0 mm wide x 14.0 mm long x 0.20 mm deep | Proposed |
| Ports | Two through ports matched to bought tubing/fittings | Diameter open |
| Bonding | Vendor-recommended transparent process | Open |
| Quantity | Three bonded coupons plus one unbonded witness | Proposed |
| Sensors | None | Fixed |
| Stacking vias | None | Fixed |
| Cells or biological material | None | Fixed |

## Stage A quote: paid design review

Quote four hours and eight hours separately. The session may be in person or
hybrid, but the reviewing engineer must speak English and be able to discuss
the actual manufacturing process.

Required review outputs:

- Marked-up model/drawing showing every manufacturing change
- Recommended PMMA grade and bottom material
- Recommended bonding method, alignment method, and cure/conditioning steps
- Selected commercial fitting/tubing interface or a short options table
- Achievable channel, chamber, port, outer-profile, and flatness tolerances
- Inspection method for channel width/depth, bond-line intrusion, and leaks
- Native-CAD/STEP requirements and drawing notes expected by the shop
- Written list of remaining risks and decisions
- Stage B fixed-price fabrication quote and lead time

## Stage B quote: no-cell first article

Stage B is authorized only after the Stage A redlines are incorporated and the
vendor confirms the released files are complete.

Requested deliverables:

- Three cleaned and bonded Rev D0 coupons
- One unbonded channel-body witness from the same process/lot
- Material identification for both layers and adhesive/interlayer, if used
- Dimensional inspection results for the agreed critical features
- Bond-process record sufficient to repeat the build
- Photos of each part before and after bonding
- Packaging that prevents scratches and channel contamination
- Written record of substitutions or deviations; none may be made silently

## Acceptance criteria to freeze in Stage A

The quote must state the measurement method and acceptance value for each row.
Blank values are blockers, not permission for vendor defaults.

| Characteristic | Draft target | Stage A output |
| --- | --- | --- |
| Channel width | 0.50 mm nominal | Tolerance and measurement method |
| Channel depth | 0.20 mm nominal | Tolerance and measurement method |
| Chamber width/length/depth | 6.0 x 14.0 x 0.20 mm nominal | Tolerances and method |
| Port location/diameter | Per selected fitting | Tolerances and fit check |
| Bond intrusion | No visible blockage or meaningful section loss | Quantified limit and inspection |
| Optical path | Clear enough to see dye front and bubbles | Lighting/image check |
| Debris/burrs | No loose debris or channel-edge burrs | Visual/magnified check |
| Leak test | Water, pressure and duration selected in review | Pressure, duration, pass rule |
| Flow continuity | Water/dye traverses inlet to outlet without a trapped blockage | Test setup and pass rule |

## Files for Stage A

- Existing concept source: `src/bin/microfluidic_chip_revd.rs`
- This review/RFQ document
- Thailand introduction brief:
  `docs/thailand_prototyping_introduction_brief.md`
- Advisor/vendor shortlist:
  `docs/thailand_advisor_vendor_shortlist.md`

The Stage A packet intentionally does not claim that the current ignored STL or
STEP outputs are released manufacturing files. After review, the corrected
Rev D0 generator, native STEP, PDF drawing, and an output manifest with hashes
must be added before Stage B fabrication is ordered.

## Advisor decisions to record

1. CNC micromilling, laser machining, molding/embossing, or another process?
2. PMMA/PMMA or PMMA/glass for the cheapest transparent first coupon?
3. Thermal, solvent, plasma-assisted, PSA, or another bond method?
4. Which bought port/fitting avoids fragile hand-drilled connections?
5. What minimum land width is required around the channel for a reliable bond?
6. What channel depth and width can the selected shop repeatedly inspect?
7. What water-only leak pressure is useful without pretending the part is
   pressure-rated?
8. What exact CAD and drawing format should LaminarForge release to that shop?
