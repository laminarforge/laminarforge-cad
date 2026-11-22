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

All hardware dimensions remain unselected. Do not infer them from historical
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
