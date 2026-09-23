# Rev D cost and verification review

Rev D replaces the integral housing with four bolted plates. This is a quotation
candidate for a water-only prototype, not evidence of successful biological use.

## Cost rationale

The former housing's bounding blank is 184.76 x 162.73 x 75.225 mm. The new frame
uses a roof blank 179.26 x 156.73 x 6.35, a bezel blank 197.26 x 75.225 x 6.35,
and two side blanks 156.73 x 41.225 x 19.05 mm. These are nominal stock comparison
dimensions before saw kerf and external machining allowance, not a cut list.
The comparison reduces rectangular starting-stock volume by approximately 77%.
It also replaces deep cavity removal with accessible plate profiles. Additional
joint holes and assembly labor offset some savings. Only supplier quotes can
establish the actual price reduction.

The roof and bezel finish to 6.00 mm; sides finish to 18.50 mm. The shop must
confirm its nominal stock provides sufficient flatness and cleanup allowance.
Body width increases from 166.76 to 179.26 mm; flange width from 184.76 to
197.26 mm. Drawer width, travel, nest, lid-height allowance, and guide clearances
are retained. Rear cover, rails, risers and flange profiles follow the wider frame.
Do not mix previous revisions of these parts with Rev D.

Type II clear sealed anodizing replaces the original Type III requirement.
Finishing is quoted separately because the previous $1,500–2,000 supplier
estimate was for anodizing alone. No complete Rev D machining quote exists yet.
The earlier electrical BOM has a $357.18 priced subtotal for only part of the
electronics; it excludes sensors and several necessary items. That historical
subtotal is not a current complete-build quote. CNC price excludes owner-sourced
electronics, tape, foam, shims and ordinary hardware.

## Joint review

Roof: six M3x14 screws through 6 mm plate give 8 mm nominal engagement in sides.
Front: four M3x10 screws through counterbored bezel give 7.2 mm nominal engagement.
Roof joints require 9 mm minimum full thread to accommodate screw-length tolerances; front joints require 8 mm. Both use 11 mm cylindrical drill depth.
D6 x 3.20 mm front counterbores recess a 3 mm high screw head by 0.20 mm nominal.
Minimum nominal land between a front counterbore and the gasket groove is 0.75 mm.
That land is not an allowance to move holes or widen the groove.

Six D3 m6 x10 locating pins insert 6 mm into the sides. Roof and front plate
slip bores are D3.15 +0.02/0; joint coordinates are +/-0.025 mm. Side H7/m6 fits
are transitional and may need retaining compound. Pins end 2 mm below the
outer roof/front surfaces nominally. Do not substitute 12 mm front pins.

Source dimensions: [Accu M3x14 socket cap](https://www.accu.co.uk/metric-cap-head-screws/3821-SSCF-M3-14-A2)
specifies 5.5 mm maximum head diameter and 3 mm maximum head height.
[Accu HDP-3-10-A1](https://www.accu.co.uk/dowel-pins/72638-HDP-3-10-A1)
specifies 3 mm m6 diameter (+0.002/+0.008) and 10 mm length. Specification pages
were checked September 23, 2026; live stock and delivered prices need confirmation.

## Evidence boundary

The generator checks closed meshes, rigid part overlap, five sampled drawer
positions, joint hardware against non-parent parts, and analytic STEP reimport
volume/bounds. Packaging performs a separate importer check. Threaded parent
engagement is excluded from collision checks because STEP uses tap-drill bores.
These checks do not establish screw preload, fatigue, supplier process capability,
continuous motion clearance under deformation, or the accuracy of purchased parts.

The reduced thermal model calculates metal-sheet sensitivity cases, not liquid
temperature. Bolted-joint thermal contact resistance is not resolved. Some assumed
loss cases exceed 0.5 C metal span; do not describe the model as passing the well
uniformity requirement. Physical acceptance remains the water-plate mapping,
opening recovery, hot sliding, seal inspection and electrical fault tests in
validation.md. Measure the actual Greiner plate/lid height before assembly.
