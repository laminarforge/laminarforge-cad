# LAMP Rev B Controller PCBA

Ticket: `T-4C206871`

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
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_materialize_board"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_route_report"}
```

```json
{"repo":"laminarforge-cad","action":"run","bin":"lamp_rev_b_controller_fab_preview"}
```

## Current Release State

This implementation pass creates an updated source package and fab preview with the Rev B regulator and excitation-driver selections represented, not an order-ready fab release.

Known release blockers:

- The schematic is an architecture shell, not full ERC-clean schematic capture.
- The PCB is a materialized placement/test-pad seed with copper zones and no routed signal/power completion.
- The selected `P7805-2000-S`, `AP63203WU-7`, `74439346068`, `LDD-700H`, `SN74LVC1G08DBVR`, `INA180A1IDBVR`, and LED shunt path are represented, but the board is not routed to zero unconnected items.
- The optional embedded camera connector remains SPI/FIFO-module oriented and debug-populated until a camera module is selected.

Do not send this package to a PCBA vendor as a fabrication release until `lamp_rev_b_controller_route_report` shows zero physical DRC and zero real unconnected items, schematic ERC is complete on captured connectivity, and manual/DNP/no-substitution assembly notes are reviewed against the selected vendor.
