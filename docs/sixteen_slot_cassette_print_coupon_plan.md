# 16-Slot Cassette Printable Coupon Plan

Ticket: T-5C29EBD8

Architecture status: active mechanical-learning precursors for the 16-slot,
4 x 4 first physical cassette. These local coupons retire fit, gasket, datum,
and connector risks before a full-size carrier is fabricated. They do not by
themselves satisfy the integrated first-article or vendor-release gates.

This document defines the first dry, desktop-printable validation coupons for the LaminarForge 16-slot cassette. These prints are for mechanical learning only. They are not sterile parts, live-cell parts, AAV-contact parts, or vendor-release geometry.

The coupons do not change experimental semantics: one cassette remains one AAV
capsid/promoter/payload/dose/timing/media condition, and the 16 slots are
same-condition readouts rather than separate candidate lanes.

## Why Coupons First

The active carrier alone is 699.04 x 541.92 mm and the dock base is 869.04 x
691.92 mm before the service bulkhead and separate coupon are included, so the
integrated hardware is too large for normal desktop printers. Printing the
whole thing would also hide the most important early questions. The first useful checks are local:

- Does a Rev C chip physically seat in the pocket with the current 1.20 mm/side CAD clearance?
- Can the gasket groove and hard-stop concept be printed and inspected?
- Does the rear/left dock datum scheme make physical sense?
- Are the bulkhead port spacing, label strip, and tubing strain-relief comb ergonomic?

## Generator

`sixteen_slot_cassette_print_coupons` consumes the shared machine-readable A0
contract for chip, seal, groove, stop, datum, and dock-interface values. Coupon
envelopes and fixture-only offsets may remain local, but the generator must not
duplicate or override cassette constants.

Run:

```text
mcp__agentic_mcp.laminarforge_build action=run repo=laminarforge-cad bin=sixteen_slot_cassette_print_coupons
```

Output directory:

```text
output/print_coupons/
```

## Coupon Outputs

| Output | Approx envelope | Checks |
| --- | ---: | --- |
| `sixteen_slot_chip_pocket_fit_coupon.stl` | 168 x 126 x approximately 31.35 mm | Rev C chip seating, 1.20 mm/side CAD pocket clearance, pocket depth, optical through-window, 7.35 mm gasket land, and representative portions of the shared 4.00 mm inter-slot hard-stop pattern without a seal intersection. These are coupon representations of array intersections, not a per-slot corner-stop definition. Slot-label land is intentionally omitted because it is not needed for pocket-fit evidence. |
| `sixteen_slot_gasket_compression_coupon.stl` | 190 x 88 x 14 mm | 3.20 mm groove width, 1.80 mm groove depth, 20/25/30% compression guard heights, and printability of small gasket features. |
| `sixteen_slot_dock_datum_rail_coupon.stl` | 198 x 178 x 40 mm | Direct datum-A carrier support, rear primary and left secondary rail inner-face contact, front low-lip contact at the nominal carrier edge, visible leak gutter, and mounting holes. |
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
| Raised pocket features | Confirm the 7.35 mm gasket land and representative 4.00 mm shared inter-slot stops are fused to the base, remain separate from the seal path, and are not interpreted as per-slot corner stops. |
| Pocket depth | Printed pocket depth at four corners and center. |
| Optical window | Whether the center opening leaves enough support and does not warp. |
| Gasket groove | Printed groove width/depth, corner cleanup, whether gasket seats without twisting. |
| Compression stops | Stop height at each squeeze lane and whether a straightedge rocks. |
| Dock datums | Whether the carrier surrogate seats directly on datum A and repeats against the rear/left rail inner faces and front lip at its nominal edges without wedging. |
| Bulkhead | Finger access, connector spacing, label visibility, tube bend/comb behavior. |

## Hard Boundaries

- Do not put cells, media, AAV, or sterile tubing through printed structural coupons.
- Do not treat a printed coupon pass as vendor drawing release.
- Do not reduce the official chip pocket clearance from 1.20 mm/side to 0.80 mm/side until real chip lot measurements and printed/ machined evidence exist.
- Do not freeze connector SKUs from the bulkhead mockup; it only checks spacing and ergonomics.

## Next After Printing

After the first prints, capture measured deviations and decide which CAD assumptions need revision:

- Pocket clearance and corner/radius policy.
- The locked 1.80 x 3.20 mm gasket groove, 7.35 mm land/stop height, and absence
  of seal/stop intersections.
- Datum-A seating plus rear/left rail and front-lip nominal-contact fit and tolerance behavior.
- Bulkhead connector spacing, label strip, and strain-relief comb geometry.
