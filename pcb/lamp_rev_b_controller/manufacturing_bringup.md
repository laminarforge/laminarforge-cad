# Rev B Controller First-Article Bring-Up

Build target: 5 assembled prototype controller PCBAs, plus 5-10 bare boards if the incremental cost is low.

Board class: 4-layer FR-4, 1.6 mm, ENIG preferred, top-side SMT first article. Use no blind/buried vias and no via-in-pad unless separately quoted and reviewed.

## Vendor Package Requirements

- Schematic PDF with revision, date, power tree, connector pinouts, DNP/DNI marks, and net labels.
- Gerber ZIP for copper, mask, paste, silkscreen, and Edge.Cuts layers.
- Excellon drills and drill map/report.
- BOM CSV with designator, value, package, footprint, manufacturer/source, supplier/catalog ID, assembly side, and DNP/DNI.
- CPL CSV with designator, x, y, rotation, side, and footprint/package.
- Assembly drawing or marked board plot showing polarity, pin 1, connector orientation, module antenna side, and DNP parts.
- Assembly notes: no substitutions for ESP32 module, regulator, ADC, mux, LED driver path, MOSFET/gate driver, USB-C connector, camera connector, heater connectors, and safety parts without approval.

## Incoming Electrical Bring-Up

1. Visual inspection: polarity, USB-C orientation, ESP32 antenna keepout, FFC/header orientation, MOSFET and IC pin 1, DNP/manual parts.
2. Continuity: no rail shorts from `VIN_12_24`, `VBUS`, `+5V`, `+3V3`, `+3V3_ANA`, `VIN_HEATER`, or `VDRV` to `GND`.
3. USB-only logic power: confirm `+3V3`, boot/reset, UART/native USB logs, and heater/LED rails disabled.
4. External supply current-limited bring-up: confirm `VIN_PROTECTED`, `+5V`, `+3V3`, and `+3V3_ANA` before fitting loads.
5. Thermistor/ADC check: connect known 10K resistance and verify mux settling, ADS1115 reads, open/short fault classification.
6. Interlock check: lid, cartridge, latch, and cutoff inputs all force heater/LED unsafe state when open.
7. LED dummy-load check: confirm `LED_EXC_EN` default off, PWM/dim control, current sense, and `LED_SYNC_OUT` timing before connecting optical head.
8. Heater dummy-load check: confirm `HEATER_ARM` default off, gate pulldowns, TC4427 output, low-frequency PWM, current sense, and cutoff loop before connecting heater pad.
9. Camera sync/log check: verify `FRAME_TRIG_OUT`, `EXPOSURE_ACTIVE_IN`, `LED_SYNC_OUT`, and UART/native USB logs align with host frame capture.
10. Record board serial, lot, rail currents, firmware hash, rework notes, and pass/fail result.

## Manual Or Connectorized Items

- Heater pads, spreader, insulation, and thermal cutoff are off-board.
- Optical LED head, heat sink, filters, lens, camera module, and dark baffles are off-board.
- Cartridge wet path and final cartridge bay mechanics are off-board.
- Header/connector family substitutions require schematic, BOM, CPL, and harness update before order release.
