# Rev B Controller Fab Release Follow-Up

The canonical board is now at physical DRC `0` and active unconnected `0`. The package remains blocked because the corrected KiCad 10 ERC reader identifies two real electrical-source issues after separating generated-library annotation noise and one justified population exception.

Reconciled in `T-C7BBBEA3`:

- Corrected generated schematic pin attachment for KiCad's library-symbol Y transform. The previous generator could swap nets on multi-pin symbols and left six edge pins disconnected.
- Snapped generated symbols and connectivity to KiCad's 1.27 mm grid, removing `503` endpoint-off-grid warnings and the six associated unconnected-wire findings.
- Updated `lamp_rev_b_controller_erc_report` for KiCad 10's sheet-nested JSON schema and explicit, item-scoped reviewed exceptions.
- Fresh ERC now reports `158` raw findings: `155` embedded `LF_CAPTURE` library annotations, one intentional pulled-up `LED_FAULT_N` singleton, and two blocking source findings: isolated `GND_EP` and source-less `HEATER_SUPPLY_SENSE`.
- Replaced the obsolete zero-unconnected routing blocker with the two actual electrical blockers. No board copper or routing source was changed in this reconciliation.

Completed through `T-49FD0ECC` and routing follow-up `T-3DFB93CC`:

- Replaced the architecture-shell schematic with generated captured connectivity from `parts.toml`, `placement.toml`, `pin_nets.toml`, and placement test points.
- Added the original MCP-runnable ERC gate, `lamp_rev_b_controller_erc_report`; its pre-KiCad-10 flat-schema parser reported `0` before the `T-C7BBBEA3` reconciliation corrected that false-clean result.
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
- Added an eleventh DRC-clean schema-v2 route-reduction pass covering LED excitation PWM, LED sync, and frame-trigger local optics branches. Physical DRC remains `0`; real unconnected items are reduced to `116`.
- Added a twelfth DRC-clean schema-v2 route-reduction pass covering the local optics GND return chain, AUX_ANALOG_IN J23 branch, and RUN LED D2-to-R30 bottom-layer hop. Physical DRC remains `0`; real unconnected items are reduced to `111`.
- Added a thirteenth DRC-clean schema-v2 route-reduction pass covering THERM_MUX_S1/S2 local branches, THERM_CH_0/1/4 bottom-layer thermistor branches, and the USB_CC1 receptacle-to-pulldown tie. Physical DRC remains `0`; real unconnected items are reduced to `105`.
- Added a fourteenth DRC-clean schema-v2 route-reduction pass covering bottom-layer SPI MISO/MOSI/SCLK and UART0 TX/RX local debug branches with reviewed return vias. Physical DRC remains `0`; real unconnected items are reduced to `100`.
- Added a fifteenth DRC-clean schema-v2 route-reduction pass covering the C5 analog-decoupler ground return, ADS_AIN0 ADC test branch, SD_CS_N pullup/test/MCU spine, and a compact 3V3_SW U3/L1/C9 local star. Physical DRC remains `0`; real unconnected items are reduced to `94`.
- Added a sixteenth DRC-clean schema-v2 route-reduction pass covering CAM_FRAME_INT and CAM_RESET_N camera header/test branches plus the local THERM_CH_5 U5-to-J9 branch. Physical DRC remains `0`; real unconnected items are reduced to `91`.
- Added a seventeenth DRC-clean schema-v2 route-reduction pass covering the U4/C5 analog GND return tie and the filtered `+3V3_ANA` branch from FB1/C5/TP_3V3A to U5 and J23. Physical DRC remains `0`; real unconnected items are reduced to `88`.
- Added an eighteenth DRC-clean schema-v2 route-reduction pass covering the `ESP_GPIO0_BOOT` R49-to-MCU top-layer dogleg. Physical DRC remains `0`; real unconnected items are reduced to `87`.
- Added a nineteenth DRC-clean schema-v2 route-reduction pass covering the `FRAME_TRIG_OUT` U1-to-TP/J17 bottom-layer tail. Physical DRC remains `0`; real unconnected items are reduced to `86`.
- Added a twentieth DRC-clean schema-v2 route-reduction pass covering the `LED_EXC_PWM` U1-to-existing-U9/TP bottom-layer tail with a short F.Cu bridge over the `ESP_EN` programming-header branch. Physical DRC remains `0`; real unconnected items are reduced to `85`.

Concrete follow-up to reach release:

1. Decide whether `U1` exposed pad 41 must join system `GND` directly or through a reviewed net-tie strategy, then update the electrical source and revalidate without hand-editing copper.
2. Human review must either remove `HEATER_SUPPLY_SENSE` from the promised interface or approve a concrete sensing topology, selected parts, and ADC mapping before any board/net update.
3. Re-run generated schematic/board validation, fresh KiCad ERC/DRC, BOM/CPL checks, and the MCP fab-readiness gate after those decisions land.
4. Add `lamp_rev_b_controller_fab_release` only after those gates are meaningful and green, then generate vendor Gerbers, drills, BOM, CPL, assembly notes, source snapshot, and review bundle.
