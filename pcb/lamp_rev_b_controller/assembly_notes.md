# Rev B Controller — 12 V First-Article Assembly Instructions

Release ticket: `T-E36FA2C2`  
Population: `rev_b_12v_first_article`  
Machine assembly: top side only  
First-article quantity: 5 PCBAs

This document is manufacturer-neutral. It is an assembly authority only when the generated `MANIFEST.json` says `release_ready: true` and the enclosing ZIP checksum matches the release record. No substitution or deviation is authorized by portal defaults.

## Mandatory population rules

- `R25` and `R26` are mandatory DNP. They are not fit options and they are not current shunts. Populating either footprint bypasses its heater MOSFET drain-to-source path and can energize the corresponding heater whenever `VIN_HEATER` is present.
- Populate `R55` with the released 0 ohm 0805 part for this 12 V first article. `R55` directly feeds `VDRV` from `VIN_PROTECTED`; it must be DNP for a 24 V build unless an approved regulated 12 V VDRV path is populated.
- Do not populate the `LDD-1000H` thermal-validation alternate. The released LED-driver population is the selected `LDD-700H` path.
- Do not populate a `P7812-500R` 24 V VDRV option. This release is 12 V only.
- `ADS1115 AIN1` is a physical no-connect. It must remain unselected in firmware.

## Heater cutoff boundary

`J24` is the high-current boundary between `VIN_PROTECTED` and `VIN_HEATER`. Before any heater dummy-load test, connect `J24` through an external normally-closed thermal cutoff or a correctly rated jumper. `J19` / `AUX_IO0_THERMAL_CUTOFF_OK` is status/control only and must not carry heater current. Never bridge `VIN_PROTECTED` to `VIN_HEATER` in board copper.

## Manual/THT operations

The BOM and `dnp.csv` are authoritative for population state. The following references are manual/THT and are excluded from CPL machine placement:

`J2, U7, J4-J11, Q1-Q2, J12-J13, J24, U8, J15-J23`.

- Confirm terminal-block wire-entry direction and connector pin 1 against the top assembly drawing.
- Hand-place `P7805-2000-S`, `LDD-700H`, `Q1`, `Q2`, and all connector/header references unless the assembler explicitly confirms an equivalent controlled THT operation.
- Keep the ESP32 antenna end unobstructed and preserve its board-edge keepout.
- Record any manual-placement rework against board serial number.

## No substitutions

No substitution is allowed for any reference listed in `lamp_rev_b_controller-no-substitution.csv`. A proposed alternate requires written engineering approval, an updated controlled source/BOM/manifest, fresh DRC/connectivity/ERC/BOM/CPL gates, and a new archive checksum. Portal-selected or assembler-selected alternates are not approved.

## Polarity and orientation inspection

Inspect USB-C orientation, IC pin 1, diode/LED polarity, ESP32 antenna edge, terminal/header orientation, and `R25/R26` absence. Reconcile every populated SMT reference with exactly one CPL row. Hold the order if the portal overlay disagrees with the released top assembly drawing or CPL rotation.

