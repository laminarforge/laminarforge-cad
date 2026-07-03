# LAMP Rev B Controller PCBA

Source-package ticket: `T-4C206871`

Implementation tickets: `T-49FD0ECC`, `T-352986EA`

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

This implementation pass replaces the architecture-shell schematic with generated captured connectivity and keeps the Rev B regulator and excitation-driver selections represented, but it is still not an order-ready fab release.

Current validation state:

- KiCad ERC on the captured schematic: `0` violations.
- KiCad physical DRC on the materialized board: `0` violations.
- Real unconnected items from `lamp_rev_b_controller_route_report`: `105`.
- `routing_seed.toml` contains the deterministic DRC-clean local F.Cu segments accepted by `lamp_rev_b_controller_seed_routes_from_drc`, plus schema-v2 reviewed `[[vias]]` and named `[[routes]]` records.
- Source-boundary components are explicit in the Rev B source model: `FB1` sources `+3V3_ANA` from `+3V3`, `R55` sources `VDRV` from `VIN_PROTECTED` for the 12 V first article only, and `J24` carries the external high-current cutoff loop from `VIN_PROTECTED` to `VIN_HEATER`.

Known release blockers:

- The PCB is still a partial materialized route seed; it is not routed to zero real unconnected items.
- The schema-v2 seed now includes `94` named routes and `41` explicit vias and can capture DRC-clean local top-layer routes, reviewed bottom-layer escapes, and named channel routes, but the remaining nets require deliberate fanout/channel routing, especially `GND` (`10`), `+3V3_ANA` (`3`), `I2C_SCL` (`3`), `I2C_SDA` (`3`), regulator switch/bootstrap, thermistor mux/ADC, heater, camera/debug, USB, and interlock nets.
- The selected `P7805-2000-S`, `AP63203WU-7`, `74439346068`, `LDD-700H`, `SN74LVC1G08DBVR`, `INA180A1IDBVR`, and LED shunt path are represented, but the board is not routed to zero unconnected items.
- `R55` is a 12 V first-article-only VDRV feed and must be DNP for any 24 V build unless an approved 12 V regulator path is added.
- `J24` requires an external normally-closed thermal cutoff or rated jumper before heater dummy-load tests; do not bridge `VIN_PROTECTED` to `VIN_HEATER` in copper.
- The optional embedded camera connector remains SPI/FIFO-module oriented and debug-populated until a camera module is selected.

Do not send this package to a PCBA vendor as a fabrication release until `lamp_rev_b_controller_erc_report` and `lamp_rev_b_controller_route_report` both pass with ERC `0`, physical DRC `0`, and real unconnected items `0`, and manual/DNP/no-substitution assembly notes are reviewed against the selected vendor.
