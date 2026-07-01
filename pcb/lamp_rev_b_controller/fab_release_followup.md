# Rev B Controller Fab Release Follow-Up

This pass does not create an order-ready fab release because the captured schematic is ERC-clean, but the board is still not routed to zero real unconnected items.

Completed through `T-49FD0ECC` and routing follow-up `T-3DFB93CC`:

- Replaced the architecture-shell schematic with generated captured connectivity from `parts.toml`, `placement.toml`, `pin_nets.toml`, and placement test points.
- Added an MCP-runnable ERC gate, `lamp_rev_b_controller_erc_report`; KiCad ERC reports `0` violations.
- Added deterministic local route seeding through `lamp_rev_b_controller_seed_routes_from_drc`; the accepted `routing_seed.toml` preserves physical DRC `0` but leaves `214` real unconnected items.
- Added schema-v2 routing seed support with explicit reviewed vias and named routes; the current durable seed preserves physical DRC `0` and reduces the route-report blocker to `212` real unconnected items.

Completed in routing/source-boundary follow-up `T-352986EA`:

- Added `FB1` as the explicit `+3V3` to `+3V3_ANA` filter/link boundary.
- Added `R55` as the 12 V first-article-only `VIN_PROTECTED` to `VDRV` feed link, with 24 V population blocked unless a regulator path is added.
- Added `J24` as the high-current external cutoff-loop terminal from `VIN_PROTECTED` to `VIN_HEATER`; `J19` remains status/control only.
- Added DRC-clean schema-v2 route records for the FB1 analog rail source tie, R55-to-U6 VDRV route, LED_SUPPLY local feed, and two local GND return ties. Physical DRC remains `0`; real unconnected items are reduced to `206`.
- Added a second DRC-clean schema-v2 route-reduction pass covering duplicated USB data pads, local +3V3/+5V rails, selected `VIN_PROTECTED` branches, the U4 ground pair, and the HEATER0 gate local path. Physical DRC remains `0`; real unconnected items are reduced to `191`.

Concrete follow-up to reach release:

1. Complete the durable `routing_seed.toml` route for the selected P7805/AP63203/LDD-700H source package, including AP63203 switch/bootstrap routing, LDD input/output current paths, LED shunt Kelvin sense, DIM/fault/control signals, heater drive, USB, I2C/SPI, thermistor mux/ADC, and interlock nets.
2. Continue adding reviewed fanout vias and channel routes through schema-v2 `routing_seed.toml`; do not hand-edit `.kicad_pcb` traces and do not replace FB1/R55/J24 with copper shorts.
3. Complete route phases from `routing_plan.toml`, then run `lamp_rev_b_controller_route_report` until physical DRC is zero and real unconnected items are zero.
4. Add `lamp_rev_b_controller_fab_release` by adapting the Rev A release binary only after ERC/DRC/unconnected gates are meaningful and green.
5. Generate vendor Gerbers, drills, BOM, CPL, assembly notes, source snapshot, and review bundle, then attach the final release artifact to `T-49FD0ECC` or its routing follow-up ticket.
