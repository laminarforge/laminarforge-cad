# LAMP Rev A Optical Mode Decision

Rev A is locked to red-light turbidimetry.

## Decision

- Mode: closed-tube turbidity measurement.
- Target optical wavelength: 650 nm red illumination.
- Geometry: LED through the reaction tube to a detector channel.
- Sampling: one optical channel at a time through the analog mux.
- Front end: eight photodiodes into the mux, one shared transimpedance stage,
  then the ADS1115 ADC input.

## Rationale

Published 8-tube real-time RT-LAMP turbidimeter work used 650 nm red LED
illumination through 0.2 ml tubes, measured magnesium pyrophosphate turbidity,
and sampled once per second during a 65 C LAMP run.

Colorimetric camera readout remains a later option, but it couples the board to
dye chemistry, imaging optics, enclosure lighting, camera module availability,
and image-processing firmware. Rev A needs the lowest-risk PCBA path that can
prove heating plus optical time-to-positive curves.

## Rev A Gate

Before fabrication release, bench-test one optical channel with negative control
and positive control reactions to verify:

- no detector saturation at the chosen LED current;
- enough signal slope to call positive reactions;
- stable negative-control baseline;
- acceptable channel-to-channel variation after mechanical alignment.
