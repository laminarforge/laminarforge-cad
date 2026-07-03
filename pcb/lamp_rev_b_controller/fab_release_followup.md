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
- Added a third DRC-clean schema-v2 route-reduction pass covering local I2C header/test branches, camera/interlock test branches, USB/VBUS local feeds, AP63203/P7805 local returns, and selected +3V3/+5V bridges. Physical DRC remains `0`; real unconnected items are reduced to `172`.
- Added a fourth DRC-clean schema-v2 route-reduction pass covering reviewed B.Cu escapes for selected +3V3, GND, I2C_SDA, VIN_PROTECTED, USB ground, and optics/interlock header branches. Physical DRC remains `0`; real unconnected items are reduced to `159`.
- Added a fifth DRC-clean schema-v2 route-reduction pass covering the USB-C VBUS duplicated-pad/feed tie, ESP32 module ground tie, RUN LED test branch, lid/cartridge interlock test branches, and local HEATER0/HEATER1 low-side terminal/test/shunt branches. Physical DRC remains `0`; real unconnected items are reduced to `149`.
- Added a sixth DRC-clean schema-v2 route-reduction pass covering the AP63203/P7805 input-capacitor ground return bridge, U5-to-J9 thermistor mux ground return, I2C_SCL bottom-layer MCU branch, ESP32 +3V3 module supply tie, and +3V3 J21 header branch. Physical DRC remains `0`; real unconnected items are reduced to `144`.
- Added a seventh DRC-clean schema-v2 route-reduction pass covering ESP_EN pullup/test/MCU/programming-header branches and the STATUS_FAULT_LED local/MCU chain. Physical DRC remains `0`; real unconnected items are reduced to `138`.
- Added an eighth DRC-clean schema-v2 route-reduction pass covering the protected-VIN input-spine B.Cu join, USB D+/D- test-point B.Cu hops, the fan tach local test branch, and the USB VBUS sense-divider link. Physical DRC remains `0`; real unconnected items are reduced to `133`.
- Added a ninth DRC-clean schema-v2 route-reduction pass covering the HEATER0 PWM test/resistor branch, VIN_12_24 input/test branch, USB VBUS test branch, ADS_AIN1 test branch, thermistor mux OUT/EN/S0 local branches, FAN_PWM local branch, and LED_DIM_GATE local branch. Physical DRC remains `0`; real unconnected items are reduced to `123`.
- Added a tenth DRC-clean schema-v2 route-reduction pass covering HEATER1 PWM/gate local B.Cu jumps and ADS_AIN2/ADS_AIN3 ADC test-point B.Cu branches. Physical DRC remains `0`; real unconnected items are reduced to `119`.

Concrete follow-up to reach release:

1. Complete the durable `routing_seed.toml` route for the selected P7805/AP63203/LDD-700H source package, including AP63203 switch/bootstrap routing, LDD input/output current paths, LED shunt Kelvin sense, DIM/fault/control signals, heater drive, USB, I2C/SPI, thermistor mux/ADC, and interlock nets.
2. Continue adding reviewed fanout vias and channel routes through schema-v2 `routing_seed.toml`; do not hand-edit `.kicad_pcb` traces and do not replace FB1/R55/J24 with copper shorts.
3. Complete route phases from `routing_plan.toml`, then run `lamp_rev_b_controller_route_report` until physical DRC is zero and real unconnected items are zero.
4. Add `lamp_rev_b_controller_fab_release` by adapting the Rev A release binary only after ERC/DRC/unconnected gates are meaningful and green.
5. Generate vendor Gerbers, drills, BOM, CPL, assembly notes, source snapshot, and review bundle, then attach the final release artifact to `T-49FD0ECC` or its routing follow-up ticket.
