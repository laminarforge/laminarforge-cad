# Lowest practical cost water bench — W2

Alex chose lowest practical cost. Hardware remains unpurchased and unselected.
This supersedes the W1 lab-instrument reference as the cost priority. Keep W1's
interface findings; do not treat its $825 pump as a required purchase.
Sources checked 2026-11-22 UTC. Scope remains water-only engineering.

## Recommended staging

Start with a manually operated source, identified tubing/connections, catch tray,
independently supported outlet and collection vial on a balance. A syringe can
move water for visual path checks; a gravity source is another manual option,
but its output varies with head and resistance. Neither establishes controlled
flow. Begin with the tubing path; only add the coupon after its first-article,
fitting and working-limit review. Do not count a reference coupon as fabricated.

This is a useful lower-cost bring-up stage, not the completed automated bench.
It can establish physical routing, outlet support, gross delivered mass and
visible problems. It cannot establish programmed flow, pump stop behavior or
unattended operation. Keep those as explicit later module requirements.

## Candidate cost ledger

| Item | Quantity | USD | Evidence / status |
| --- | --- | --- | --- |
| U.S. Solid USS-DBS46-1, SKU JFDBS00046-100G | 1 | 96.49 | Manufacturer listed price; 100 g capacity, 0.001 g readability, RS-232; availability needs confirmation [S1] |
| New Era small syringe/plumbing kit P-SYRKIT-SM | 1 | 20.38 | Supplier-listed reference price; includes syringes, barbs and PVC tubing [S3]. Not matched to coupon ports |
| Manual source/outlet supports, shutoff, vial, fasteners and print material | bundle | 30–80 | Planning allowance, not a sourced quote; actual parts still need identification |
| RS-232 USB adapter and correct cable | bundle | 20–40 | Planning allowance; can defer for initial manual readout |
| Listed-price subtotal | | 116.87 | Balance + kit only |
| Planning range with allowances | | 166.87–236.87 | Before shipping/tax; not a complete-system quote |

The range excludes coupon fabrication/bonding/inspection and its selected
fittings, motorized pump, electronics, and any purchased stand needed beyond
the support allowance. Those costs are unknown; do not present this as a $240
finished research instrument. Local delivered prices and reuse may change it.
Do not buy both complete plumbing options below by default.

The balance page shows inconsistent stock text. Confirm actual availability,
regional power connection and supplied items before purchase. It lists a 50 g
calibration weight; included weight does not establish traceability [S1].

## Balance tradeoff and integration

The product page lists ±0.002 g repeatability and ±0.003 g linearity. Its linked
manual lists ±0.003 g and ±0.004 g respectively [S1,S2]. Do not silently choose
the better numbers. Confirm the exact revision and observe real drift,
repeatability and delivered-mass behavior before assigning uncertainty.
The 100 g limit includes vial, nest and liquid even after tare.

The manual shows 9600 baud serial output and a crossed 2/3 signal connection,
with continuous and key-output modes [S2]. That differs from the A&D direct
query/identity contract. Do not reuse A&D commands or its straight-through cable
assumption. Treat unsupported identity/stability flags as unavailable, not
invented. Capture raw bytes, host timestamps and manually recorded label/serial
identity; confirm the actual framing on hardware. USB adapter is RS-232, not TTL.

The generic receiver still is not a fit claim. Its body/pan origin and draft
shield must be measured independently. Use an off-scale adjustable outlet
support, not an enlarged integral frame sized from packaging dimensions.

## Tubing alternatives and new dry guide candidate

The New Era kit specifies 0.125 inch ID × 0.188 inch OD PVC tubing [S3], about
3.175 / 4.775 mm. Adafruit 3659 separately lists silicone tubing 2.5 mm ID /
4.7 mm OD at $3.50 per meter [S4]. These are different parts, not interchangeable
specifications. Neither is matched to the coupon's provisional 2 mm review bore.
Identify the exact barb/fitting and material/working limits before assembly.

`models/water_transfer_dock_4p7mm.toml` creates a separate 4.7 mm OD dry-guide
candidate using the existing generator, with its 0.6 mm diametral saddle
clearance retained. This is for the Adafruit-size alternative only. Do not use
it as proof that the kit's 4.775 mm tubing fits, that a tie will restrain it, or
that either tube is rated for the final path. Preserve the original 4 mm config.
The new candidate must be exported, byte-verified and inspected before delivery.

## Motorized options after the manual lane

Adafruit 1150 is listed at $24.95, 12 V and 200–300 mA, with up to 100 mL/min
flow [S5]. Those facts do not establish low-flow stability, pressure capacity,
pulsation or compatibility with the coupon. It is a possible bulk-transfer
module, not a selected precision-flow pump. Supply, switching/driver and stop
design add cost. Do not buy it merely because the sticker price is low.

The historical custom syringe pump remains a separate redesign candidate;
fix its axis, through-clearance and retention design before fabrication.
Its motor, driver, mechanics, supply, limits, wiring and calibration effort
must all enter the cost comparison. No defensible complete custom-pump quote
exists yet. A purchased programmable pump remains the higher-cost reference.
The first manual stage needs no separate controller board or custom PCB.

## Next actual checkpoints

- Select exact source/tubing/fittings and establish the physical path; retain
  the unresolved coupon fabrication cost rather than hiding it in an allowance.
- Measure balance/pan/draft-shield and vial; adapt an independent outlet support.
- Observe actual mass readings and serial framing if the adapter is acquired.
- Decide whether required flow performance warrants a custom syringe drive,
  purchased pump or other method based on independent measurements.

## Primary sources

- [S1: U.S. Solid USS-DBS46-1 product page](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html)
- [S2: Linked U.S. Solid DBS46 manual](https://file.ussolid.com/content/JFDBS/Manual-USS-DBS46%20Precision%20Balance.pdf)
- [S3: New Era syringe/plumbing supplies](https://www.syringepump.com/syringes%20and%20plumbing%20supplies.php)
- [S4: Adafruit 3659 tubing](https://www.adafruit.com/product/3659)
- [S5: Adafruit 1150 pump](https://www.adafruit.com/product/1150)
