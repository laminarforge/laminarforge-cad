# Manufacturing notes - LF-CAS-V0 Rev D

## Quote scope

One water-test prototype. Base quote: supply the aluminum parts and two guard sheets listed in manufacturing-bom.csv. Quote machining and Type II finishing separately. Foam, shim backing, guide tape, commercial hardware, electronics and assembly are owner-sourced; quote those only as separately labeled options. No pressure test, cosmetic color matching, cleanroom process, motor or rocker.

Quantities: one each roof, left side, right side, bezel, rear cover, drawer, front flange and nest; one left and one right rail; two risers; eight identical spacers; two identical guards; one moving and one fixed cable bracket; two identical jaws. The BOM groups identical pieces and points to one representative STEP each. Spacers may be cut and faced from plain aluminum tube: OD6, bore3.4, length8.00 +/-0.10; no anodize required. The optional-materials folder contains six tape profiles and a nominal shim reference, excluded from the base machining quote.

## Datums and anodizing

STEP uses assembly coordinates: X left/right, Y rearward, Z up. Housing closing-stop face is Y=0. All dimensions are FINISHED after coating. The shop compensates precoat sizes using its finishing process. Coating thickness is not equal to outward growth; confirm allowance with the finisher.

6061-T6, Type II clear sealed sulfuric anodize, 5-15 micrometers. Type III hardcoat is not required for this water-test prototype. Mask threads and specified heater/sensor bond lands. Protect bare lands for shipment. Guards use the same Type II finish. Spacers may remain uncoated. No coating on shim stock or tape. Oxide is not the electrical-insulation system.

General dimensions +/-0.10 mm, clearance bores +0.10/0, angles +/-0.5 degree unless overridden. Guide separation from actual rail-seat plane to guide roof: 8.950 +/-0.05. Drawer/rail thickness: 6.00 +/-0.05. Guide relief width and drawer width: +/-0.10. Guide seating/sliding faces: 0.05 mm flatness; plate-support and front stop faces: 0.10 mm flatness. General surface roughness Ra 3.2 micrometers max; ONLY labeled heater/RTD adhesive lands require Ra 1.6 max. Inspect critical fit dimensions after coating.

Deburr 0.2-0.4 mm without rolling burrs into guides/seal. Cable entries R0.5 minimum. Do not round datum faces or change gasket geometry. The old blanket R0.5 internal-radius restriction is removed. For larger cutter radii, supply a marked-up deviation identifying each changed corner; do not reduce drawer travel, plate clearance, gasket support or fastener seating. Do not assume sharp CAD inside corners can be machined with a round end mill. Nest cutter reliefs are modeled.

## Threads

STEP shows tap-drill bores, not helical threads. Tap M2x0.4, M3x0.5, M4x0.7 and the probe port M5x0.8 to 6H. Verify engagement and no bottoming. Flat-ended pilot geometry defines cylindrical depth; drill-point/runout beyond it must leave adequate wall. Nest taps must not break through the drawer underside.

Housing rail taps: M3, 8 mm full thread minimum from Z=6, 11 mm drill depth. Rear taps: M3, 8 full thread/11 drill. Stop receiver: M2, 4.2 full thread/8 drill from guide roof. Closure M4 and shim-ear M2 through bezel. Guard/thermostat roof taps M3 through roof.

Drawer front: M3, 8 full thread/11 drill. Nest: M2, 3.5 full thread/4.5 drill from top; 1.5 mm nominal bottom wall. Guard, thermostat and cable-bracket M3 through drawer. Rear-cover cable mounts: M3, 6 full thread/9 drill from bottom. Jaw taps: M2, 4.5 full thread/7 drill from split face. Use the labeled hole schedules on the individual sheets for positions, thread sizes and depths.

Joint coordinates: +/-0.025 mm; D3 H7 side locating bores 6.00 +0.10/0 deep, finished after coating. Mating slip bores D3.15 +0.02/0 THRU. All six locating pins are D3 m6 x10, seated 6 mm into sides. Transition fits may need retaining compound at the side end. Roof screws M3x14 (8 mm nominal engagement, 9 mm minimum full tapped thread); front M3x10 (7.2 mm engagement), D6 counterbores 3.20 +0.05/0 deep. Roof and side top mating lands flatness 0.05.

## Guides

Use adhesive-backed igus A160-T-010-0500-G. CAD thickness 1.225 mm is the nominal INSTALLED envelope. Accept measured thickness 1.10-1.35; reject material outside it rather than force the slide. The tolerance stack permits approximately 0.15-0.85 mm vertical clearance before flatness. Assembled acceptance: 0.10-0.90 mm vertical play, 0.30-1.20 mm total lateral play (0.15-0.60 per side only when centered), smooth travel and no metal rubbing, cold and hot. No precision robotic repeatability claim.

Left upper tape starts at Y=22 and retains 29 mm engagement. A screening 20 N reaction on only this 5 x 29 mm patch gives 0.138 MPa bearing pressure. This is not a wear-life rating. Limit plate/liquid payload to 0.25 kg; support the drawer while servicing and do not lean on it. Verify loaded hot travel.

## Gasket and shims

Rogers BISCO BF-1000 white, unbacked 3/32 inch nominal 2.38 mm, cut as one CLOSED ring. Width 3.00 mm, rounded outline per DXF. No splice or bottom gap. 3M9731 attaches foam to metal shim: silicone adhesive toward foam, acrylic toward metal. No PSA on retaining ears.

Groove width 3.50, depth 2.50 +/-0.05 including connected ear pockets. OWNER SOURCING ONLY: optional adjustment stock consists of 302 stainless full-frame shims at 0.05/0.10/0.15/0.20/0.25/0.30/0.40/0.50 mm, with a second 0.50 frame. Do not order all nine cut frames from the CNC shop. Measure the purchased foam and adhesive first, then source only the full-frame thickness combination the formula requires. Precision Brand 22999 is a stock-source option, not a cut gasket kit. The modeled 0.60 metal + 0.14 typical adhesive gives about 26% nominal compression.

Measure groove depth d, installed adhesive a and low-force foam thickness t_min/t_max around the ring. Select metal S within [d-a-0.80*t_min, d-a-0.70*t_max], using available sheet increments. This yields 20-30% compression at all measured points. If the interval is empty, cut a more uniform foam piece. Never set compression from broad stock thickness tolerance alone.

M2x4 screws retain shim ears; flange pockets clear the heads. M4 knobs close against metal stops, avoiding gasket overcompression. For the approximately 64 mm-high frame, 16.5 kPa typical compression stress implies about 22 N force. The broad published stress range implies approximately 9-47 N; measure actual effort. This is a thermal draft seal.

## Quote alternatives and inspection scope

Return one-set and five-set prices by BOM line. Show per-part prices, anodizing minimum-lot charge, lead time, shipping and any exclusions. State whether masking is included or extra; operation-by-operation costing is not required. No cosmetic matching or independent metrology certification is requested. A shop dimensional report covering the guide gap, drawer thickness/width, groove depth and thread fit is sufficient.

Rev D uses a bolted roof/side/front assembly. Finish roof and bezel to 6 mm from nominal 1/4-inch plate; sides to 18.50 mm from nominal 3/4-inch plate, subject to stock cleanup allowance. Quote saw/waterjet blanks plus machining of functional surfaces. Do not weld.

## Process and inspection

Frame: finish roof and bezel from flat stock. Machine each side plate as an open L profile with accessible rail relief; drill/ream joint bores from exposed ends. Machine bezel opening and gasket groove in a face setup. Drawer/flat parts: face, profile/drill, then edge-tap. Brackets use accessible open-side grooves. Avoid cosmetic setups and welded assemblies.

Inspect and report critical dimensions after anodizing. Deliver dry clean parts, protected contact faces and chip-free blind holes.

## Sources

Foam: https://www.rogerscorp.com/-/media/project/rogerscorp/documents/elastomeric-material-solutions/bisco/english/data-sheets/180-048-bf-1000-extra-soft-cellular-silicone.pdf

PSA: https://multimedia.3m.com/mws/media/2366081O/3m-double-coated-tape-9731.pdf

Tape: https://www.igus.com/product/iglidur_T_A160

Shims: https://www.precisionbrand.com/product/8-piece-metric-stainless-steel-shim-stock-asst-150mm-x-300mm-sheets/

DFM sources checked 2026-09-17:
https://www.hubs.com/knowledge-base/how-prepare-technical-drawing-cnc-machining/
https://www.hubs.com/knowledge-base/how-design-parts-cnc-machining/
https://www.anoplate.com/finishes/sulfuric-anodize
https://www.anoplate.com/news-and-events/the-impact-of-anodize-on-dimensions/
