# Lowest practical cost water bench — W3

Alex chose lowest practical cost. Hardware remains unpurchased. The F1 section
below makes a preferred commercial integration recommendation, subject to its
compatibility gates; no complete hardware configuration is released.
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


## F1 — path toward the actual instrument (2026-11-22)

The [consolidated component package](research-component-package.md) carries the
current complete role ledger and seven-step progress checklist. F1's subtotal
below remains a reference basket, with additional component costs unresolved.

Prefer a commercial serial-controlled syringe pump with replaceable wetted parts,
a commercial vessel interface and independent mass observation. Manual initial
loading is allowed by A-12A0063A; finite operation does not establish continuous
supply. This is the integration direction. Research vessel suitability remains
an unresolved scientific interface, rather than a decision inferred from a water
coupon. The machine-readable selection and ordered gates are in
`models/research_fluid_service_selection.json`.

### Preferred pump and complete-package comparison

Carry NE-501 as the preferred integration candidate: $575, plus OEM-USB-START
$42.50 and bottom cover $40 = **$657.50** [P1,P2]. It retains stall detection for
$55 more than NE-500. The bottom cover closes the open base; it is not a full
motion or spill enclosure. NE-1000 plus its $27.50 USB cable kit is $852.50
[P2,P3], a $195 listed-package difference. Its local keypad may justify the
premium, but no recovered requirement demands it. These are engineering choices,
not an instruction to purchase. Requote regional power and included accessories.

Stall sensing is not pressure measurement or the normal stop method. The family
supports serial timeout/alarm behavior [P4]; actual firmware, configuration and
loss-of-link stop must be observed. Do not depend on the Windows vendor GUI.
The serial protocol is usable independently, but the bundled converter's actual
Apple Silicon support is unverified. Do not count host communication as complete.

### Vessel and connector compatibility

Keep the ibidi I Luer family as the commercial interface reference. Published
footprint is 75.5×25.5 mm with female Luer ports [V1]. It does not fit by assuming
the existing 75×25 coupon or Rev C payload is equivalent. Preserve A0; derive a
separate adapter only after exact vessel/readout requirements are established.
No particular channel height, surface or biological application is selected.
SKU 80171 below is an exact cost example, not a scientific recommendation.

The 10831 adapter set connects that port family to external flexible tubing;
its external barb is specified for 3.2 mm ID tube and it lists 500 mbar [V2].
The existing New Era kit has 3.175 mm ID PVC [F1]. That is only a nominal match:
tolerances, retention, material compatibility and each component's rated working
limit remain unresolved. **500 mbar is the adapter rating, not an assembled-system
operating limit.** Do not energize the candidate path from these catalog facts.

### Known cost and missing cost

| Reference item | Purchase quantity | Listed USD |
| --- | --- | ---: |
| NE-501 + USB/power starter + bottom cover | one package assembled from three SKUs | 657.50 |
| ibidi 80171 costing example | one box, 15 slides | 360.00 |
| ibidi 10831 adapter set | one box, 12 pieces / six pairs | 215.00 |
| P-SYRKIT-SM water-characterization assortment | one kit | 20.38 |
| USS-DBS46-1 balance candidate | one | 96.49 |
| Known reference subtotal | | **1,349.37** |

Sources P1/P2/V3/F1/M1. The slide price comes from the US manufacturer price list;
web variants may select different SKUs, bulk quantities and currencies. Adapter
packaging is a convenient documented reference, not proof of cheapest procurement.
Pack-level outlay cannot be replaced by amortized per-slide prices.

**The complete total is unknown.** Missing costs include pressure observation and
protection, independent stop/wiring/enclosure, balance-specific cable and adapter,
possible host USB hub, collection/containment, mounts/fasteners/fabrication,
calibration and labor, environment and scientific readout, tax and shipping.
Retain the balance accuracy/stock discrepancies recorded above. Do not describe
$1,349.37 as a complete box, purchase-ready BOM or guaranteed budget.

### Alternative disposition

Existing custom motorized geometry remains an alternative, with no further
fixture families admitted before cost/performance justify them. Fluigent remains
unselected: its public SDK23 table lists macOS x64, not native macOS ARM64; no
emulation success or complete pressure/flow/gas-source package is established
[A1]. Golander BT100S-1 lists $575 without a head and $820 with YZ15T [A2]. Do
not treat the drive price as a usable pump or import a different manufacturer's
BT100S specifications. Rotor-speed accuracy is not delivered-flow accuracy.

### Ordered completion gates

1. G01: reconcile qualified vessel, measurement access, materials and environment
   requirements. Engineering owns the synthesis; Alex need not invent limits.
2. G02: close every exact wetted connection, working rating and protection choice.
3. G05: prepare adapter/drawings from the chosen vessel's documented interfaces;
   final dimensional acceptance waits for actual parts.
4. G06: finish every support-item price and delivered procurement/assembly package
   before spending authorization and acquisition.
5. G03: on acquired hardware, demonstrate host identity/readback, timeout stop,
   independent stop and deliberate restart behavior.
6. G04: on acquired hardware, establish independent mass observations and uncertainty.
7. G07: finish actual dimensional acceptance, commission the assembled hardware
   and obtain qualified intended-use evidence.

G01/G02 can advance together. G03/G04 follow acquisition; their commissioning
results may require revisions to the provisional drawings. These are
completion gates, not seven additional printed parts. None is marked passed by
this catalog review. No purchase, supplier outreach, print or hardware run occurred.

### F1 primary references

- [P1](https://syringepump.com/oem.php)
- [P2](https://syringepump.com/accessories.php)
- [P3](https://www.syringepump.com/NE-1000.php)
- [P4](https://www.syringepump.com/detailedfeatures.php)
- [V1](https://ibidi.com/channel-slides/50-5399--slide-i-luer.html)
- [V2](https://ibidi.com/flow-accessories/75-tube-adapter-set.html)
- [V3](https://ibidi.com/img/cms/downloads/pl/ibidi_Price_List_USA_150dpi.pdf)
- [F1](https://www.syringepump.com/syringes%20and%20plumbing%20supplies.php)
- [M1](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html)
- [A1](https://github.com/Fluigent/fgt-SDK/blob/master/README.md)
- [A2](https://www.golanderlab.com/product/bt100s-1-variable-speed-peristaltic-pump/)

## Primary sources

- [S1: U.S. Solid USS-DBS46-1 product page](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html)
- [S2: Linked U.S. Solid DBS46 manual](https://file.ussolid.com/content/JFDBS/Manual-USS-DBS46%20Precision%20Balance.pdf)
- [S3: New Era syringe/plumbing supplies](https://www.syringepump.com/syringes%20and%20plumbing%20supplies.php)
- [S4: Adafruit 3659 tubing](https://www.adafruit.com/product/3659)
- [S5: Adafruit 1150 pump](https://www.adafruit.com/product/1150)
