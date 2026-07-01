# Rev B Controller Fab Release Follow-Up

This pass does not create an order-ready fab release because the board is still a source-package and placement/materialization seed after landing the selected regulator and excitation-driver hardware.

Concrete follow-up to reach release:

1. Replace the architecture-shell schematic with full hierarchical schematic capture and run KiCad ERC to zero errors.
2. Complete the durable `routing_seed.toml` route for the selected P7805/AP63203/LDD-700H source package, including AP63203 switch/bootstrap routing, LDD input/output current paths, LED shunt Kelvin sense, and DIM/fault/control signals.
3. Complete route phases from `routing_plan.toml`, then run `lamp_rev_b_controller_route_report` until physical DRC is zero and real unconnected items are zero.
4. Add `lamp_rev_b_controller_fab_release` by adapting the Rev A release binary after ERC/DRC gates are meaningful for this project.
5. Generate vendor Gerbers, drills, BOM, CPL, assembly notes, source snapshot, and review bundle, then attach the final release artifact to `T-727698C1` or a follow-up ticket.
