# 16-Slot Cassette Printable Coupon Plan

Ticket: T-5C29EBD8

This document defines the first dry, desktop-printable validation coupons for the LaminarForge 16-slot cassette. These prints are for mechanical learning only. They are not sterile parts, live-cell parts, AAV-contact parts, or vendor-release geometry.

## Why Coupons First

The full cassette/dock assembly is roughly 818 mm x 617 mm, so it is too large for normal desktop printers. Printing the whole thing would also hide the most important early questions. The first useful checks are local:

- Does a Rev C chip physically seat in the pocket with the current 1.20 mm/side CAD clearance?
- Can the gasket groove and hard-stop concept be printed and inspected?
- Does the rear/left dock datum scheme make physical sense?
- Are the bulkhead port spacing, label strip, and tubing strain-relief comb ergonomic?

## Generator

Run:

```text
mcp__agentic_mcp.laminarforge_build action=run repo=laminarforge-cad bin=sixteen_slot_cassette_print_coupons
```

Equivalent local command:

```text
cargo run --bin sixteen_slot_cassette_print_coupons
```

Output directory:

```text
output/print_coupons/
```

## Coupon Outputs

| Output | Approx envelope | Checks |
| --- | ---: | --- |
| `sixteen_slot_chip_pocket_fit_coupon.stl` | 168 x 126 x 29 mm | Rev C chip seating, 1.20 mm/side CAD pocket clearance, pocket depth, optical through-window, gasket land, hard stops, label land. |
| `sixteen_slot_gasket_compression_coupon.stl` | 190 x 88 x 14 mm | 3.20 mm groove width, 1.82 mm groove depth, 20/25/30% compression guard heights, printability of small gasket features. |
| `sixteen_slot_dock_datum_rail_coupon.stl` | 198 x 178 x 40 mm | Rear primary datum rail, left secondary datum rail, front low retention lip, visible leak gutter, mounting holes. |
| `sixteen_slot_carrier_corner_surrogate.stl` | 156 x 132 x 39 mm | Mating corner piece for the dock datum rail coupon; checks registration and handling/orientation features. |
| `sixteen_slot_bulkhead_connector_mockup.stl` | 232 x 76 x 76 mm | M0-M6 media port spacing, partial waste-port spacing, label strip, mounting access, and tubing comb ergonomics. |

## Printer Guidance

Use the Bambu for the larger dry-fit coupons first. PLA or PETG is enough for geometry checks. Use resin only for small-feature checks where surface detail matters, especially gasket groove inspection.

Local Bambu Studio inspection on the Mac found one configured Bambu device access-code entry but no saved user machine preset naming the exact connected model. The installed Bambu machine profiles show the common Bambu 0.4 mm nozzle family, with the shared `fdm_bbl_3dp_001_common` profile defining a 256 mm x 256 mm printable area and the A1 0.4 nozzle profile defining `printable_height = 256`. Treat 256 mm x 256 mm x 256 mm as the conservative Bambu target until Alex confirms a larger H2D/H2S-class machine.

All current coupons fit that conservative Bambu target. They do not fit an A1 mini target without scaling or splitting; do not scale the chip pocket coupon because it must remain 1:1 for fit evidence.

Suggested first print order:

1. `sixteen_slot_chip_pocket_fit_coupon.stl`
2. `sixteen_slot_gasket_compression_coupon.stl`
3. `sixteen_slot_dock_datum_rail_coupon.stl` plus `sixteen_slot_carrier_corner_surrogate.stl`
4. `sixteen_slot_bulkhead_connector_mockup.stl`

## What To Measure

Record these before changing the cassette baseline:

| Check | Measurement |
| --- | --- |
| Pocket fit | Actual chip length/width, printed pocket length/width, subjective insertion force, rocking, visible binding, burr/contact points. |
| Raised pocket features | Confirm the gasket land, label land, and four hard stops are fused to the base, not separate slicer islands. |
| Pocket depth | Printed pocket depth at four corners and center. |
| Optical window | Whether the center opening leaves enough support and does not warp. |
| Gasket groove | Printed groove width/depth, corner cleanup, whether gasket seats without twisting. |
| Compression stops | Stop height at each squeeze lane and whether a straightedge rocks. |
| Dock datums | Whether the carrier surrogate repeats against rear/left rails without wedging. |
| Bulkhead | Finger access, connector spacing, label visibility, tube bend/comb behavior. |

## Hard Boundaries

- Do not put cells, media, AAV, or sterile tubing through printed structural coupons.
- Do not treat a printed coupon pass as vendor drawing release.
- Do not reduce the official chip pocket clearance from 1.20 mm/side to 0.80 mm/side until real chip lot measurements and printed/ machined evidence exist.
- Do not freeze connector SKUs from the bulkhead mockup; it only checks spacing and ergonomics.

## Next After Printing

After the first prints, capture measured deviations and decide which CAD assumptions need revision:

- Pocket clearance and corner/radius policy.
- Gasket groove and hard-stop dimensions.
- Datum rail clearances and front retention lip height.
- Bulkhead connector spacing, label strip, and strain-relief comb geometry.
