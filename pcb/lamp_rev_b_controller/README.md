# LAMP Rev B Controller PCBA

Source-package ticket: `T-4C206871`

Implementation tickets: `T-49FD0ECC`, `T-352986EA`, `T-C7BBBEA3`

This package is the Rev B prototype controller carrier source package. It is not a final diagnostic-product board and not an external validation plan.

The board keeps heater pads, thermal cutoff hardware, optics, camera module, filters, lens, cartridge wet path, and final enclosure mechanics connectorized. The controller owns ESP32-S3 control, USB programming/logging, external power entry, heater drive, thermistor/ADC/mux, fluorescence LED control, camera/debug/logging connectors, interlocks, switches, and test points.

## Source Files

- `contract.toml`: board envelope, stackup, modules, rails, nets, GPIO map, test points, and manufacturing policy.
- `parts.toml`: selected part groups, footprints, source fields, external safety parts, and release-blocking selection gaps.
- `placement.toml`: first-pass placement and test pad locations for materialization.
- `pin_nets.toml`: locked footprint pad-to-net assignments.
- `power_architecture.toml`: Rev B power, heater, thermistor, and safety assumptions.
- `optical_interface.toml`: excitation LED, external camera sync, optional SPI/FIFO camera, and analog diagnostic interface.
- `firmware_handoff.toml`: firmware-facing logical peripherals and boot-safe defaults.
- `electrical_validation.toml`: incoming first-article bring-up gates.
- `routing_plan.toml`, `routing_seed.toml`, `copper_zones.toml`: route policy and materialized starter geometry.
- `fab_release.toml`: preview-only fab configuration and release blockers.

## Rust Workflow

Run through the LaminarForge MCP build tool:

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_check"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_materialize_schematic"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_erc_report"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_materialize_board"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_seed_routes_from_drc"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_route_report"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_fab_preview"}
```

## Current Release State

The canonical board is fully routed and physically DRC-clean, but this package is still not an order-ready fab release because one reviewed electrical-source issue remains.

Current validation state:

- Fresh KiCad ERC on the captured schematic: `157` raw findings: `155` generated embedded-symbol library annotations, `1` explicitly justified pulled-up `LED_FAULT_N` singleton, and `1` blocking electrical-source finding.
- KiCad physical DRC on the materialized board: `0` violations.
- Real unconnected items from `lamp_rev_b_controller_route_report`: `0` (`3` raw identical-self-zone entries are ignored by the established filter).
- `U1` pad 41 and all nine replicated exposed-pad lands are assigned directly to system `GND`; the existing exposed-pad spokes are also `GND`, and the preserved autoroute adds only the reviewed three-segment, one-via GND connection produced by the stock Freerouting workflow.
- `routing_seed.toml` remains the durable generated-board seed; the canonical promoted board preserves the clean autorouted copper and its reviewed minimal U1 GND addition.
- Source-boundary components are explicit in the Rev B source model: `FB1` sources `+3V3_ANA` from `+3V3`, `R55` sources `VDRV` from `VIN_PROTECTED` for the 12 V first article only, and `J24` carries the external high-current cutoff loop from `VIN_PROTECTED` to `VIN_HEATER`.

Known release blockers:

- `HEATER_SUPPLY_SENSE` currently terminates only at `TP_HEATER_SENSE`. Human review must either remove the unsupported firmware/contract interface or approve a concrete sensing circuit and ADC mapping; the present package must not imply working heater-supply telemetry.
- `R55` is a 12 V first-article-only VDRV feed and must be DNP for any 24 V build unless an approved 12 V regulator path is added.
- `J24` requires an external normally-closed thermal cutoff or rated jumper before heater dummy-load tests; do not bridge `VIN_PROTECTED` to `VIN_HEATER` in copper.
- The `LED_FAULT_N` ERC singleton is reviewed and intentional for this population: the selected LDD-700H has no native fault output, so R54 only reserves a pulled-up future-fault node.

Do not send this package to a PCBA vendor as a fabrication release until the remaining blocking electrical-source finding is resolved, `lamp_rev_b_controller_erc_report` passes after reviewed exceptions, fresh physical DRC and real unconnected counts remain zero, the MCP fab-readiness gate passes, and manual/DNP/no-substitution assembly notes are reviewed against the selected vendor.
