# Rev B Controller First-Article Bring-Up

Build target: 5 assembled prototype controller PCBAs, plus 5-10 bare boards if the incremental cost is low.

Known board baseline: 4-layer FR-4, 1.6 mm nominal, 1 oz outer / 0.5 oz inner copper, top-side SMT first article. `ENIG preferred` is historical preference, not a released finish specification. The handoff generator must remain blocked until the selected order profile records the physical stackup, tolerances/class, final finish specification, colors, and impedance-policy decision.

Panelization is fabricator-owned from the released 118 x 94 mm single-up board. Engineering does not release panel Gerbers. Any proposed rails, tooling, fiducials, coupons, or depanelization are accepted only through the portal-preview approval gate.

## Vendor Package Requirements

- Schematic PDF with revision, date, power tree, connector pinouts, DNP/DNI marks, and net labels.
- Gerber ZIP for copper, mask, paste, silkscreen, and Edge.Cuts layers.
- Excellon drills and drill map/report.
- BOM CSV with quantity, designator, value, package, footprint, explicit manufacturer and MPN, explicit supplier and supplier part number, assembly side, variant, and DNP/DNI.
- CPL CSV with designator, x, y, rotation, side, and footprint/package.
- Assembly drawing or marked board plot showing polarity, pin 1, connector orientation, module antenna side, and DNP parts.
- Assembly notes: no substitutions for ESP32 module, regulator, ADC, mux, LED driver path, MOSFET/gate driver, USB-C connector, camera connector, heater connectors, and safety parts without approval.
- Manual/THT notes: hand-place `P7805-2000-S`, `LDD-700H`, and the connectorized heater cutoff loop terminal `J24` unless the assembler explicitly confirms through-hole module assembly. Keep `LDD-1000H` and any `P7812-500R` 24 V VDRV option DNP for first articles; populate `R55` only for the 12 V first-article VDRV feed. `R25` and `R26` are mandatory DNP: populating either directly bypasses its heater MOSFET and can energize the heater whenever `VIN_HEATER` is present; neither footprint measures current.

## Incoming Electrical Bring-Up

1. Visual inspection: polarity, USB-C orientation, ESP32 antenna keepout, FFC/header orientation, MOSFET and IC pin 1, DNP/manual parts.
2. Continuity: no rail shorts from `VIN_12_24`, `VBUS`, `+5V`, `+3V3`, `+3V3_ANA`, `VIN_HEATER`, or `VDRV` to `GND`.
3. USB-only logic power: confirm `+3V3`, boot/reset, UART/native USB logs, and heater/LED rails disabled.
4. External supply current-limited bring-up: confirm `VIN_PROTECTED`, P7805 `+5V`, AP63203 `+3V3`, `FB1` continuity/noise on `+3V3_ANA`, and `R55` 12 V-only VDRV population before fitting loads.
5. Analog-front-end check: confirm `THERM_MUX_OUT -> ADS1115 AIN0`, physical no-connect on AIN1, `LED_CURRENT_SENSE -> AIN2`, and `AUX_ANALOG_IN -> AIN3`. Use the +/-4.096 V PGA range; connect known 10K thermistor resistances and verify mux settling/open-short classification, verify about 1.4 V at AIN2 with the 700 mA LED dummy load, and exercise AIN3 only from a bounded 0-3.3 V, <=10 kohm source.
6. Interlock check: lid, cartridge, latch, and cutoff inputs all force heater/LED unsafe state when open.
7. LED dummy-load check: confirm `LED_EXC_EN` default off, `LED_EXC_PWM AND LED_EXC_EN` DIM gating, LDD-700H current near 700 mA, INA180 `LED_CURRENT_SENSE` scaling, pulled-up/no-native-fault `LED_FAULT_N`, and `LED_SYNC_OUT` timing before connecting optical head.
8. Heater dummy-load check: verify `R25`/`R26` are unpopulated, confirm `J24` is closed through the external normally-closed thermal cutoff or rated jumper, then confirm `HEATER_ARM` default off, gate pulldowns, TC4427 output, low-frequency PWM, externally measured load current, and cutoff-loop opening before connecting a heater pad. The Rev B first article has no heater-rail or heater-current telemetry.
9. Camera sync/log check: verify `FRAME_TRIG_OUT`, `EXPOSURE_ACTIVE_IN`, `LED_SYNC_OUT`, and UART/native USB logs align with host frame capture.
10. Record board serial, lot, rail currents, firmware hash, rework notes, and pass/fail result.

## Manual Or Connectorized Items

- Heater pads, spreader, insulation, and thermal cutoff are off-board.
- `J24` is the high-current heater cutoff-loop boundary between `VIN_PROTECTED` and `VIN_HEATER`; `J19`/`AUX_IO0_THERMAL_CUTOFF_OK` is status/control only and must not carry heater current.
- Optical LED head, heat sink, filters, lens, camera module, and dark baffles are off-board.
- 470-490 nm high-power excitation is a blue-light/eye hazard. Do not operate an exposed emitter outside a covered heatsunk optical head with interlocks and appropriate lab eyewear.
- Cartridge wet path and final cartridge bay mechanics are off-board.
- Header/connector family substitutions require schematic, BOM, CPL, and harness update before order release.

## Order-release gate

Do not upload or order unless the generated `MANIFEST.json` says `release_ready: true`, `SHA256SUMS` verifies, the two-build reproducibility gate passes, and the portal layer/drill/BOM/CPL/panel previews are approved and recorded. The provisional checklist in `A-D87AC12B` is not current vendor-cited evidence.
