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
