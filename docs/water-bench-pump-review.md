# Water bench pump interface review

M01 remains a concept pending selected hardware. The frozen historical
`src/bin/syringe_pump_standalone.rs` is preserved as evidence, not released for
manufacture. This review concerns water-only bench engineering.

## Findings in the historical source

- Rod and bearing axes run along X. The pusher nut pocket and screw clearance
  remain cylinders along Z; they do not align with the horizontal drive axis.
- The 40 mm pusher contains 24.5 mm long centered bearing pockets without
  full-length rod clearances. That leaves 7.75 mm end material at each end
  blocking rod passage.
- The 6 mm base has 8.1 mm rod channels centered at its midplane. Those channels
  open through the top and bottom; they are not closed retaining bores.
- The source calls an 8.1 mm bore for an 8 mm rod a press fit. Fit and retention
  have not been established by measurement or fabrication.

These are source geometry findings. No physical pump has been inspected here.

## Required interfaces for the replacement

| Interface | Required measured or selected information |
|---|---|
| Syringe | Manufacturer/model, barrel OD, flange envelope/thickness, plunger head envelope, useful stroke, outlet fitting |
| Linear guidance | Rod diameter/length/spacing, bearing OD/length, fit and axial retention |
| Drive | Motor mounting/boss/shaft, coupling envelope, screw diameter/lead, nut mounting and axial load retention |
| Mechanical limits | Home and travel stops, limit switch locations, clearance over full stroke, accessible manual release |
| Controller | Board/driver model, supply ratings, connectors, supported command/acknowledgment and stop behavior |
| Measurement | Scale model/pan, actual readout interface, independent delivered-mass observation and calibration record |

## Replacement design sequence

Use X for syringe travel, rod and lead-screw axes. Establish the measured
syringe seating plane and plunger contact reference before sizing travel.
Separate rod retention, bearing retention and lead-nut retention; each needs a
real fastening or capture feature. Keep rod passages open through the carriage.
Route tubing without loading the scale or obstructing full travel. Check the
actual assembled travel envelope, then observe manual motion, stop behavior and
water delivery before treating a command acknowledgment as a physical result.

Purchased hardware remains unselected and unmeasured. Do not infer them from historical
constants or claim print, motion or delivery verification from this review.

## Separate runtime carriage review

`water_transfer_dock --module carriage` with
`models/water_bench_carriage.toml` creates a separate dry carriage review.
The historical frozen pump entry point remains untouched. The default body is
40×76×36 mm, with rod axes along X at Y±24 mm, Z18 mm. Full-length Ø8.6 mm
clearances continue beyond the rear-loaded Ø15.2×24.5 mm bearing pockets.
The separate 3 mm end plate has matching rod and lead-screw passages. Four
nominal M3 through-bolt paths align with the body. The 48 mm body variant is a
separate adjustable candidate, not a chosen travel/frame dimension.

A rectangular 20.4×20.4 mm nut pocket opens from the same rear face and ends at
X16.4 mm. Its flats provide a nominal anti-rotation envelope and its front
shoulder/end plate provide nominal axial lands. Actual nut geometry, clearance,
endplay, fastening, shimming and load capacity remain unresolved. It is not the
round historical T8 nut, a selected commercial square nut or a printable thread.
The displayed reference nut has an unthreaded visualization bore only.

Body and cap review meshes are rotated to pocket-up/flat orientations; assembly
and REF parts remain in the unrotated X-axis engineering frame. The manifest
records those transformations and actual mesh checkpoints for through passages,
bearing insertion, shoulder and cap lands. Every carriage mesh is DRY_FIT_REVIEW
or REF. These geometry checks do not qualify a powered pump or establish
alignment, torque, force, backlash, travel, motion or water delivery.

The bearing envelope is consistent with the nominal 8/15/24 mm dimensions in
[THK's LM8UU product record](https://store-na.thk.com/Product/lm8uu), but no actual
bearing or shaft is purchased/measured and fit tolerances are not inferred from
that nominal size. [ServoCity's square-nut product](https://www.servocity.com/3506-series-lead-screw-square-nut-8mm-lead-4-start-16mm-length/)
illustrates a commercial non-round restraint approach; its M12 attachment and
full external geometry have NOT been adopted into this provisional pocket.

Next integrate only after choosing measured nut/shaft/bearing interfaces and
reviewing the motor/frame, guide alignment, full stroke, syringe flange/plunger
restraint, hard stops/limits, electrical stop and independent delivered mass.
Do not fabricate or energize a working pump from the old source or this
standalone carriage review.


## Unpowered guide-frame review

`water_transfer_dock --module frame` reads `models/water_bench_frame.toml`.
The nested carriage parameters generate the actual carriage in two static poses;
rod pitch, diameter and height derive from that same carriage definition. The
frame adds a review base, two lower supports and two separate caps with aligned
vertical M3 clearance paths. The central Ø12 mm opening is clearance only;
it does not support a rotating screw or react thrust.

The default clear span is 200 mm; the review base is 260×114×6 mm. Nominal Ø8 mm
rods are 240 mm long, at Y±24 mm and Z28 mm. The carriage bottom is 4 mm above
the base. Body start positions X8 and X148 produce 140 mm configured geometric
travel, with a 5 mm minimum rear gap and 12 mm front gap. The short configuration
uses a 160 mm clear span, 220 mm base, 200 mm rods and 100 mm geometric travel.
Neither travel value is an adopted syringe stroke.

Individual lower supports are shown at Z0. Separate caps rotate 180 degrees
about X and translate to Z0, leaving the flat face down and saddles upward.
Actual STL preflight identified downward saddle overhangs in the prior export;
this orientation is a review improvement, not print qualification. Their assembled
split plane lies on the rod axis. The 8.2 mm nominal saddle and fixed seam do
not establish clamping preload or rod retention. Fasteners, nut access,
under-base clearance, shims, material, plate stiffness and alignment remain
unresolved. The default base may exceed the selected printer bed; no printer
or fabrication method has been selected. Do not infer print qualification.

These files add no motor, coupler, screw thrust support, actual nut, syringe
flange/plunger restraint, hard stop, limit switch or electrical controls. Static
endpoint views and envelope checks do not establish motion or water delivery.

The supplier STEP archive for ServoCity 3506-0804-0016 was retrieved from
https://www.servocity.com/content/step_files/3506-0804-0016.zip . Inspection of
its product images confirms an external threaded shoulder unlike the current
rectangular reference block. The original generic pocket remains provisional. A separate supplier-referenced
engineering candidate is described below; no physical fit is claimed. No hardware
was purchased. Supplier source and reference files are retained on ORICO.


## Supplier-referenced nut candidate

`models/water_bench_carriage_servocity.toml` and
`models/water_bench_frame_servocity.toml` add an engineering reference for
[ServoCity 3506-0804-0016](https://www.servocity.com/3506-series-lead-screw-square-nut-8mm-lead-4-start-16mm-length/),
listed at $7.99 when checked. This is not a purchase or a selected complete pump.
The product specifies an 8 mm lead, four-start screw interface and an external
M12 attachment. Screw/motor performance and required load are still unresolved.

The retrieved supplier STEP has SHA-256
`184a31dd9036a85bae2cc1c604921d0fab78edbb214b72ec170c28f16b150599`.
Open CASCADE via `cadquery-ocp-novtk 7.8.1.1.post1` successfully transferred it
as one valid solid. Its analytic bounds are 11.7×16×11.7 mm in the supplier's
X/Y/Z placement. Planar faces show an 11.7 mm square base from supplier Y32
through Y40; the remaining shoulder extends through Y48. These are CAD facts,
not a controlled dimensional tolerance specification or measured hardware.

The review reference uses an 11.7×11.7×8 mm square base and a conservative
Ø12×8 mm shoulder envelope. Neither the external nor internal thread is modeled;
the displayed Ø8 mm hole is visualization only. The rectangular rear-entry
pocket is 12.3×12.3×16.4 mm, allowing the shoulder to enter without a printed
M12 thread. The pocket's flats surround the square base; actual rotational play,
backlash, endplay, shimming and retention remain unqualified.

The cap's Ø8.8 mm screw passage leaves only 1.45 mm nominal radial overlap with
the square base. This candidate explicitly uses a 1.2 mm *review* end-land
minimum, separate from the unchanged 3 mm structural body-web minimum. That
reduced land is an unresolved load-retention issue, not a strength approval.
The generic nut configurations retain their original 3 mm review minimum.
The compact frame retains 100 mm geometric travel and a 220×114 mm base.

The runtime contract now requires `nut_square_length_mm`,
`nut_round_shoulder_diameter_mm` and `nut_minimum_end_land_mm` in every carriage
configuration, including nested frame configurations. A zero shoulder is valid
only for a full-length square reference. Existing exported packages retain their
original exact configurations and receipts; do not mix those old configurations
with this newer executable without migrating the required fields.


Actual supplier-CAD comparison against the first exported nut candidate found
zero supplier BRep volume outside the configured pocket. The rigid transform
rotates supplier geometry -90 degrees about Z and translates its original Y32
start to carriage X0.2, with the axis at Z18. Nominal CAD clearances are 0.2 mm
at both axial ends and 0.3 mm on each side of the square envelope. This checks
CAD containment only; tolerances, printed dimensions and physical fit remain
unverified. The original source and all inspection records are on ORICO.


## Motor mounting and separated-base drive layout

`water_transfer_dock --module motor` with `models/water_bench_motor.toml`
adds a dry motor bracket and posed motor/guide-frame layout. The 8 mm face-plate
variant is `models/water_bench_motor_thick.toml`. These are optional custom-drive
engineering references; the lower-cost manual bring-up remains the first lane.
No motor, coupler or complete custom pump has been bought or qualified.

[Adafruit 324](https://www.adafruit.com/product/324) lists a 31 mm square M3
pattern and a Ø5×24 mm shaft. Its linked drawing was retrieved from
https://www.adafruit.com/images/product-files/324/C140-A%20datasheet.jpg and
visually inspected. The drawing gives 42.3 mm maximum face width, 34±0.5 mm body
length, 24±1 mm shaft projection, Ø22 mm pilot and M3 holes 4.5 mm minimum deep.
The pilot's projection is not dimensioned; its height is not invented in CAD.
The block reference uses the 42.3 mm face and 34.5 mm maximum body envelope,
with a round nominal shaft; the flat, boss projection, wires and mounting
threads are omitted. Motor-specific corner chamfers are not represented.

The product page and drawing disagree on some electrical/torque values
(35 versus 34 ohms, and page holding torque versus the drawing's 1.6 kg-cm).
No drive circuit, current setting, torque margin, pump force or achievable flow
is inferred here. The drawing's axial-force figure does not qualify the motor
as the pump's thrust support.

[Adafruit 1176](https://www.adafruit.com/product/1176) is a 5-to-8 mm coupler
with a published Ø18×25 mm outer envelope. The reference shows that outer
cylinder only. Internal transition, insertion depths and set-screw positions
are not published dimensions adopted by this model. Motor and screw overlap
with the outer envelope is only a layout observation; it does not prove grip,
shaft engagement or compatibility with an externally threaded screw.

The bracket is 96×76×56 mm, with an upright 64 mm wide face plate, four Ø3.4 mm
motor clearances on the 31 mm pattern, a full Ø22.4 mm pilot clearance and four
Ø4.5 mm bench anchor paths. Its axis at Z28 matches the actual nested guide
configuration. At maximum reference motor width, the body is 0.85 mm above the
base. Face-plate thickness is 6 mm or 8 mm. Screw lengths, washers, engagement,
thread-bottoming, anchoring, alignment and bracket strength remain unresolved.

In the combined view the motor face is at guide X−80. The independent bracket
and guide bases have a 6 mm gap on a shared bench plane; there is no hidden
structural connection. The coupler starts 15 mm from the motor face. Nominal
motor-shaft overlap with its outer cylinder is 9 mm; worst reference shaft-end
gap to the smooth screw is 5 mm. The reference screw starts 30 mm from the face
and is a generic Ø8×250 mm smooth envelope, not selected threaded hardware.
The long exposed tail and access envelope require later guarding review.

No radial or bidirectional axial screw support, syringe flange/plunger restraint,
hard stops, switches, guard, driver, supply or electrical stop is included.
A flexible coupler does not supply the missing thrust load path. Do not energize
this assembly. The bracket is exported in its upright engineering orientation;
large horizontal pilot clearance may need supports or a fabrication redesign.
Run actual mesh and slicer review before any physical manufacture.
