# Manufacturing notes - LF-CAS-V0 Rev A

## Quote scope

One water-test prototype. Supply aluminum parts, guard sheets, shim kit, one-piece silicone gasket and cut guide tape. Quote commercial hardware separately if providing assembly. No pressure test, cosmetic color matching, cleanroom process, motor or rocker.

Quantities: one housing, rear cover, drawer, front flange and nest; one left and one right rail; two risers; eight identical spacers; two identical guards; one moving and one fixed cable bracket; two identical jaws. Six tape profiles, three foam frames including spares, and the specified shim kit. The shim STEP shows a nominal 0.60 mm stack; cut its common outline from each required stock thickness.

## Datums and anodizing

STEP uses assembly coordinates: X left/right, Y rearward, Z up. Housing closing-stop face is Y=0. All dimensions are FINISHED after coating. The shop compensates precoat sizes using its finishing process. Do not subtract 50 micrometers uniformly from the CAD.

6061-T6, Type III undyed hard anodize, nominal 50 micrometers; acceptable coating process range 40-60 micrometers away from masked lands. Mask threads and specified heater/sensor bond lands. Protect bare lands for shipment. Type II clear is acceptable on thin guards. No coating on shim stock or tape. Oxide is not the electrical-insulation system.

General dimensions +/-0.10 mm, clearance bores +0.10/0, angles +/-0.5 degree unless overridden. Critical guide dimensions and drawer/rail thickness +/-0.03. Housing mounting faces, plate contact surface and front mating faces: 0.05 mm flatness. Seal/contact/bond lands Ra 1.6 micrometers max; groove floor Ra 3.2 max. Inspect critical faces after coating.

Deburr 0.2-0.4 mm without rolling burrs into guides/seal. Cable entries R0.5 minimum. Do not round datum faces or change gasket geometry. Unfunctional internal edge radii up to 0.5 mm may not reduce specified clearance envelopes. Nest cutter reliefs are modeled.

## Threads

STEP shows tap-drill bores, not helical threads. Tap M2x0.4, M3x0.5 and M4x0.7 to 6H. Verify engagement and no bottoming. Flat-ended pilot geometry defines cylindrical depth; drill-point/runout beyond it must leave adequate wall. Nest taps must not break through the drawer underside.

Housing rail taps: M3, 8 mm full thread minimum from Z=6, 11 mm drill depth. Rear taps: M3, 8 full thread/11 drill. Stop receiver: M2, 4.2 full thread/8 drill from guide roof. Closure M4 and shim-ear M2 through bezel. Guard/thermostat roof taps M3 through roof.

Drawer front: M3, 8 full thread/11 drill. Nest: M2, 3.5 full thread/4.5 drill from top; 1.5 mm nominal bottom wall. Guard, thermostat and cable-bracket M3 through drawer. Rear-cover cable mounts: M3, 6 full thread/9 drill from bottom. Jaw taps: M2, 4.5 full thread/7 drill from split face. See STEP centerlines and individual sheets.

## Guides

Use adhesive-backed igus A160-T-010-0500-G. CAD thickness 1.225 mm is the nominal INSTALLED envelope. Accept measured thickness 1.10-1.35; reject material outside it rather than force the slide. The tolerance stack permits approximately 0.19-0.81 mm vertical clearance before flatness. Assembled acceptance: 0.10-0.90 mm vertical play, 0.15-0.60 mm side clearance per side, smooth travel and no metal rubbing, cold and hot. No precision robotic repeatability claim.

Left upper tape starts at Y=22 and retains 29 mm engagement. A screening 20 N reaction on only this 5 x 29 mm patch gives 0.138 MPa bearing pressure. This is not a wear-life rating. Limit plate/liquid payload to 0.25 kg; support the drawer while servicing and do not lean on it. Verify loaded hot travel.

## Gasket and shims

Rogers BISCO BF-1000 white, unbacked 3/32 inch nominal 2.38 mm, cut as one CLOSED ring. Width 3.00 mm, rounded outline per DXF. No splice or bottom gap. 3M9731 attaches foam to metal shim: silicone adhesive toward foam, acrylic toward metal. No PSA on retaining ears.

Groove width 3.50, depth 2.50 +/-0.05 including connected ear pockets. Supply 302 stainless full-frame shims at 0.05/0.10/0.15/0.20/0.25/0.30/0.40/0.50 mm, with a second 0.50 frame. Precision Brand 22999 is a stock-source option, not a cut gasket kit. The modeled 0.60 metal + 0.14 typical adhesive gives about 26% nominal compression.

Measure groove depth d, installed adhesive a and low-force foam thickness t_min/t_max around the ring. Select metal S within [d-a-0.80*t_min, d-a-0.70*t_max], using available sheet increments. This yields 20-30% compression at all measured points. If the interval is empty, cut a more uniform foam piece. Never set compression from broad stock thickness tolerance alone.

M2x4 screws retain shim ears; flange pockets clear the heads. M4 knobs close against metal stops, avoiding gasket overcompression. For the approximately 64 mm-high frame, 16.5 kPa typical compression stress implies about 22 N force. The broad published stress range implies approximately 9-47 N; measure actual effort. This is a thermal draft seal.

## Process and inspection

Housing: establish datum faces; mill open-bottom cavity/guides; front setup for opening, seal and holes; rear for taps; top/bottom drilling as needed. Drawer/flat parts: face, profile/drill, then edge-tap. Brackets use accessible open-side grooves. Avoid cosmetic setups and welded assemblies.

Inspect and report critical dimensions after anodizing. Deliver dry clean parts, protected contact faces and chip-free blind holes.

## Sources

Foam: https://www.rogerscorp.com/-/media/project/rogerscorp/documents/elastomeric-material-solutions/bisco/english/data-sheets/180-048-bf-1000-extra-soft-cellular-silicone.pdf

PSA: https://multimedia.3m.com/mws/media/2366081O/3m-double-coated-tape-9731.pdf

Tape: https://www.igus.com/product/iglidur_T_A160

Shims: https://www.precisionbrand.com/product/8-piece-metric-stainless-steel-shim-stock-asst-150mm-x-300mm-sheets/
