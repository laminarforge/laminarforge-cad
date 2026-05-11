# LAMP Rev A Optical Mode Decision

The Rev A board must not lock an emitter wavelength until the detection mode is
selected. LAMP can be monitored several ways, and the PCB choices change with
that decision.

## Candidate Modes

- Turbidimetry: measures magnesium pyrophosphate precipitation during LAMP. This
  is compatible with LED plus photodiode readout, but the wavelength, optical
  path, and transmission/scatter geometry must be selected together.
- Colorimetric imaging: measures dye color change such as phenol red or HNB.
  This may be better served by camera/RGB analysis or multi-wavelength
  photometry than by a single fixed emitter/detector pair.
- Fluorescence: uses excitation/emission dyes or probes. This needs optical
  filtering and a different emitter/detector design than turbidity.

## Rev A Gate

Before schematic capture of the optical channels, decide:

- assay chemistry and readout mode;
- emitter wavelength and package;
- detector type and spectral response;
- tube/chip optical path geometry;
- whether the analog front end needs a transimpedance amplifier before mux/ADC.

Until those are selected, optical emitters and photodiodes remain
fab-blocking gaps in `parts.toml`.
