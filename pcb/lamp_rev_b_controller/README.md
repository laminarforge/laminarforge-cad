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

The deterministic Rev B analog-front-end source is reconciled as a first-article release candidate. Fresh promoted-board ERC, DRC, connectivity, BOM/CPL, and fab-readiness results must remain green before ordering.

Current validation state:

- `THERM_MUX_OUT` directly drives ADS1115 AIN0, `LED_CURRENT_SENSE` directly drives AIN2, and bounded `AUX_ANALOG_IN` directly drives AIN3. The redundant synthetic `ADS_AIN0/2/3` nets and their implied but absent conditioning boundaries are retired.
- ADS1115 AIN1 is an explicit schematic no-connect and un-netted board pad for the first article; firmware must never select it.
- The unsupported heater-supply telemetry interface and its test point are retired. Rev B safety remains based on the physical `J24` cutoff loop, thermistor plausibility/over-temperature checks, and `AUX_IO0_THERMAL_CUTOFF_OK`.
- `U1` pad 41 and all nine replicated exposed-pad lands are assigned directly to system `GND`; the existing exposed-pad spokes are also `GND`, and the preserved autoroute adds only the reviewed three-segment, one-via GND connection produced by the stock Freerouting workflow.
- `routing_seed.toml` remains the durable generated-board seed; the canonical promoted board preserves the clean autorouted copper and its reviewed minimal U1 GND addition.
- Source-boundary components are explicit in the Rev B source model: `FB1` sources `+3V3_ANA` from `+3V3`, `R55` sources `VDRV` from `VIN_PROTECTED` for the 12 V first article only, and `J24` carries the external high-current cutoff loop from `VIN_PROTECTED` to `VIN_HEATER`.

Mandatory population and bring-up constraints:

- `R25` and `R26` remain DNP. Each is placed directly across its heater MOSFET drain-source path; populating either can energize the heater whenever `VIN_HEATER` is present and does not create current sensing.
- `R55` is a 12 V first-article-only VDRV feed and must be DNP for any 24 V build unless an approved 12 V regulator path is added.
- `J24` requires an external normally-closed thermal cutoff or rated jumper before heater dummy-load tests; do not bridge `VIN_PROTECTED` to `VIN_HEATER` in copper.
- The `LED_FAULT_N` ERC singleton is reviewed and intentional for this population: the selected LDD-700H has no native fault output, so R54 only reserves a pulled-up future-fault node.

Do not send this package to a PCBA vendor until `lamp_rev_b_controller_erc_report` passes after item-scoped reviewed exceptions, fresh physical DRC and real unconnected counts remain zero, the MCP fab-readiness gate passes, and manual/DNP/no-substitution assembly notes are reviewed against the selected vendor.
