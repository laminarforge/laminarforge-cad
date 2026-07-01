# Rev B Controller Fab Release Follow-Up

This pass does not create an order-ready fab release because the captured schematic is ERC-clean, but the board is still not routed to zero real unconnected items.

Completed in `T-49FD0ECC`:

- Replaced the architecture-shell schematic with generated captured connectivity from `parts.toml`, `placement.toml`, `pin_nets.toml`, and placement test points.
- Added an MCP-runnable ERC gate, `lamp_rev_b_controller_erc_report`; KiCad ERC reports `0` violations.
- Added deterministic local route seeding through `lamp_rev_b_controller_seed_routes_from_drc`; the accepted `routing_seed.toml` preserves physical DRC `0` but leaves `214` real unconnected items.

Concrete follow-up to reach release:

1. Complete the durable `routing_seed.toml` route for the selected P7805/AP63203/LDD-700H source package, including AP63203 switch/bootstrap routing, LDD input/output current paths, LED shunt Kelvin sense, DIM/fault/control signals, heater drive, USB, I2C/SPI, thermistor mux/ADC, and interlock nets.
2. Extend the route seed/materializer as needed for reviewed fanout vias and channel routing that avoid pad-center via violations; do not hand-edit `.kicad_pcb` traces.
3. Complete route phases from `routing_plan.toml`, then run `lamp_rev_b_controller_route_report` until physical DRC is zero and real unconnected items are zero.
4. Add `lamp_rev_b_controller_fab_release` by adapting the Rev A release binary only after ERC/DRC/unconnected gates are meaningful and green.
5. Generate vendor Gerbers, drills, BOM, CPL, assembly notes, source snapshot, and review bundle, then attach the final release artifact to `T-49FD0ECC` or its routing follow-up ticket.
