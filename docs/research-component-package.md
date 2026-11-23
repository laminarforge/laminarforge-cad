# LaminarForge component selection and completion checklist

The first instrument uses a commercial syringe pump, replaceable wetted path,
commercial vessel interface and independent delivered-mass observation. The
component register now covers the complete set of purchasing and integration
roles, including the parts missing from the earlier pump recommendation.
**Step 1 remains open.** Fourteen of 21 cost rows now have US reference
prices, including one cable bundled in the starter kit. The priced items
total **$1,588.03**. Seven rows still need prices or a design decision.
The delivered cost in Thailand remains open.
The estimate assumes an existing supported Mac will be available beside the
finished bench. If the bench needs a dedicated computer, its cost is additional.

Use [the itemized cost ledger](research-component-costs.csv) and
[the component register](../models/research_component_package.json) together.
Blank prices mean unquoted, never zero. The pump cable included in the starter
kit has zero *additional* cost and is not a second purchase. Pack prices retain
the full cash outlay rather than dividing it into hypothetical single units.

## Current component decisions

Retain NE-501, its OEM-USB-START kit and bottom cover as the pump candidate.
The manufacturer currently lists those at $575, $42.50 and $40 respectively.
The kit includes the required pump cable. Its unidentified USB converter is
not the basis for Mac compatibility. The cover does not provide a complete
protective enclosure. [Pump and accessory source](https://syringepump.com/oem.php)

Select **FTDI US232R-100-BULK**, one per instrument, at the manufacturer's
listed $26.80 each. This is FTDI's finished FT232R cable with USB-A, male DB9
and 1 m cable length. FTDI also supplies the macOS 26 ARM VCP driver. Using its
own cable removes the earlier third-party support mismatch; actual enumeration
and device communication remain commissioning work. No driver installation
is needed for design.
[Cable and price](https://ftdichip.com/product-category/products/cables/usb-rs232-cable-series/),
[driver table](https://ftdichip.com/drivers/vcp-drivers/).

Price two Apple MW5L3AM/A USB-C-to-USB data adapters at $19 each for the US
reference basket. Apple Thailand lists the local SKU MW5L3ZA/A at ฿690 each.
Two direct connections avoid adding a hub to the initial list. Available ports
and reusable adapters still need physical inventory.
[US adapter price](https://www.apple.com/shop/product/mw5l3am/a/usb-c-to-usb-adapter),
[Thai SKU](https://www.apple.com/th/shop/product/mw5l3za/a/usb-c-to-usb-adapter).

The observed Mac mini is an Apple M4, model Mac16,10, running macOS 26.5.1.
The live inventory exposed only Bluetooth-Incoming-Port and debug-console
serial paths. No instrument port was opened or command sent. The final rig
can use a supported Mac beside the hardware; it need not attach to this Mac
mini during design or use this particular host permanently.

Retain the ibidi I Luer family as the vessel interface reference, 80171 as the
costing example and 10831 as the reference adapter pack. Use one SYR-GL3LL
3 mL standard glass syringe as the exact water-characterization reference,
listed at $32. Its vendor specifies a 43.5 psi component rating and nominal
9.06 mm barrel ID for the NE-500/501 family. Final surface, duration, retention
and tubing remain open. Remove the $20.38 assortment from the current basket;
it does not define a final syringe or a rated assembled path.
[Syringe specification and price](https://www.syringepump.com/syringes%20and%20plumbing%20supplies.php),
[vendor pump/syringe table, catalog page 22](https://www.syringepump.com/download/catalog-1.pdf).
The adapter's 500 mbar specification does not establish the assembled path's
working limit. The [F1 selection](../models/research_fluid_service_selection.json)
retains exact reference prices and component sources.

U.S. Solid's public store feed shows the exact JFDBS00046-100G variant
(46324225769727) available at $96.49 on November 23. Delivery to Thailand and
the previously recorded accuracy discrepancy remain unresolved.
[Seller inventory for this product](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html.js).
Its cable must follow its own instrument pinout;
the pump cable cannot be reused by assumption. Selection still needs a gross
vial/nest/liquid mass budget and measurement uncertainty appropriate to the
required delivery. [Balance basis](water-bench-low-cost.md#balance-tradeoff-and-integration).

## Connection schedule

| Link | Candidate connection | Remaining decision or observation |
| --- | --- | --- |
| Pump data | Mac → adapter A → CBL-PC-PUMP-7 → NE-501 | Physical connector mating, actual identity and state readback |
| Balance data | Mac → adapter B → balance-specific cable → DBS46 | Connector gender/continuity, raw framing, units and actual sample timing |
| Liquid | Syringe → tubing → 10831 → selected slide → outlet tube → collection vial | Exact wetted SKUs, retained volume, materials, assembled pressure limit and protection |
| Measurement | Vial/nest on balance; outlet anchored off scale | Gross mass, free outlet gap and no tube/support load on pan |
| Power | Regional OEM supply → reviewed independent stop → pump | Circuit, ratings, connectors, enclosure and deliberate reset |

The balance manual's printed page 7 specifies balance pin 2 to host pin 3,
balance pin 3 to host pin 2, and ground 5 to 5. It documents 9600 baud, eight
data bits, no parity and one stop bit. SCNM9FF is a crossed DB9 female/female
cable candidate. CDW lists it at $11.99 in the US. StarTech's UK price of
£11.99 including VAT is a separate regional offer. Confirm the
balance connector gender and continuity before ordering. The diagram does not
specify gender, so no gender changer or additional crossover is selected.
[US cable price](https://www.cdw.com/product/startech-10ft-rs232-serial-null-modem-cable-10ft-null-modem-cable/488601),
[UK cable price](https://www.startech.com/en-gb/cables/scnm9ff),
[balance manual](https://file.ussolid.com/content/JFDBS/Manual-USS-DBS46%20Precision%20Balance.pdf).

The power row is a design boundary, not a wiring instruction. A TTL foot switch
commands the pump and does not independently remove power. Stop implementation
must address both motor power and remaining fluid pressure or siphoning.
The manufacturer documents serial timeout and power-return controls; the actual
device must be configured and observed before use. No start command may be
automatically replayed after connection loss.
[NE-500/501 manual](https://syringepump.com/download/NE-500%20OEM%20Syringe%20Pump%20User%20Manual%20V3.9.pdf).

Two commercial products narrow the independent-stop decision. Klinger & Born
documents emergency stop and protection against restart after power returns
on its K3000 starter, item 0098.3006. Tough Leads lists an enclosed UK
plug-and-socket unit at £39.99 and documents that it stays off after power
returns. Its listing leaves the separate stop-latch behavior unspecified.
Electrical review still needs to identify the actual local plug and supply
load and confirm that releasing the stop alone cannot restart motion. The
installed assembly has no quote yet. [K3000](https://www.klibo.com/product-display/starter-k3000-1ph-230v-00983006),
[Tough Leads unit](https://toughleads.co.uk/products/no-volt-release-safety-switch-with-start-stop-and-emergency-stop-button).

## What the priced items cover

| Newly priced US item | Purchasing unit | USD |
| --- | --- | ---: |
| StarTech SCNM9FF, conditional on connector check | One cable | 11.99 |
| Nalgene 2118-0001PK | One pack of 12 jars at regular price | 52.70 |
| EMS 63312-02 | One tray with cover | 70.75 |
| **Added to the previous $1,452.59** | | **135.44** |

Thermo Fisher also displays a $44.81 online offer for the same pack. The
regular price keeps the budget independent of that discount. The jar is
cataloged at 20 g, 36 mm diameter and 47 mm height. The listing does not
specify the lid state for that weight. Actual tare, clearance and collection
capacity remain to be checked. The tray's 27.9 × 15.5 × 8.3 cm external
dimensions leave its usable spill volume and coverage unresolved. EMS lists
Becthai Bangkok as a local distributor. Its delivered price remains open.
[Jar and pack price](https://www.thermofisher.com/order/catalog/product/2118-0001PK),
[tray price](https://www.emsdiasum.com/instrument-tray-2),
[EMS distributor directory](https://www.emsdiasum.com/world-distribution).

### Thailand electronics prices

| Seller and listed part | Quantity | THB each | THB for quantity |
| --- | ---: | ---: | ---: |
| DigiKey Thailand, FTDI US232R-100-BULK | 2 | 893.78 | 1,787.56 |
| DigiKey Thailand, StarTech SCNM9FF (conditional) | 1 | 409.68 | 409.68 |
| Apple Thailand, MW5L3ZA/A | 2 | 690.00 | 1,380.00 |
| **Listed electronics** | | | **3,577.24** |

Using the Thai listings replaces $103.59 of the US estimate. The resulting
estimate is **$1,484.44 plus ฿3,577.24** before outstanding costs. DigiKey
amounts come from indexed regional listings and need a fresh quote before
purchase. The two FTDI cables alone exceed DigiKey's published ฿1,600
standard-shipping threshold. Duty and taxes may still be payable on delivery.
[FTDI listing](https://www.digikey.co.th/th/products/detail/ftdi-future-technology-devices-international-ltd/US232R-100-BULK/2441363),
[StarTech listing](https://www.digikey.co.th/th/products/detail/startech-com/SCNM9FF/25697500),
[Apple Thailand](https://www.apple.com/th/shop/product/mw5l3za/a/usb-c-to-usb-adapter),
[DigiKey shipping terms](https://www.digikey.co.th/en/help-support/payment-and-pricing).

### Costs still open

| Rows | What sets the price |
| --- | --- |
| CP-14, CP-15 | Final tubing and pressure protection depend on the accepted configuration. |
| CP-16 | Local electrical review and a quote for the complete independent stop. |
| CP-19 | Support dimensions, material quantities and a fabrication quote. |
| CP-20 | Required measurement uncertainty, check masses and calibration scope. |
| CP-21 | Qualified research-use requirements for the readout and environment. |
| CP-22 | Destination charges, replacement quantities and labor. Count fabrication only once under CP-19. |

TPA Calibration lists ten-point electronic balance calibration from ฿1,600.
That gives CP-20 a service-price reference. The fee for this balance, applicable
uncertainty, site charges and cost of check masses remain open. The parts
subtotals above exclude this service. [TPA service table](https://tpacal.or.th/services/?4296_paged=2&4719_paged=4).

Keep the low-cost DBS46 balance limited to water-bench engineering evaluation.
Its manual excludes specified regulated, medical and pharmaceutical analyses;
qualified selection must establish the eventual research-use measurement
equipment. The existing accuracy discrepancies also remain unresolved.

The ledger includes distinct rows for both serial adapters, balance cable, host
USB connection, final syringe/tubing, pressure observation/protection, independent
power stop, collection vessel, spill containment, supports/fabrication, references,
readout/environment, shipping/tax and labor. This prevents those costs from
disappearing behind a pump-only total.

Engineering can continue the cable, support and procurement comparison. Final
vessel, pressure-protection and measurement selections need the intended
observable, compatible vessel, materials, environment and timing described in
[RI1](../models/research_readout_interfaces.json). Qualified input must supply
scientific requirements; Alex is not being asked to invent technical limits.
Purchasing and real operation follow the completed configuration and cost review.

## Completion checklist

- [x] Audit existing work and identify reusable CAD and unresolved interfaces.
- [ ] 1. Finish the component-selection package: exact candidates, tubing,
  fittings, measurement equipment, compatibility gaps and full cost ledger.
- [ ] 2. Lock research-use requirements with qualified input.
- [ ] 3. Finish integrated mechanical design around selected components.
- [ ] 4. Finish controls, recording, independent stop and restart behavior.
- [ ] 5. Produce one fabrication/purchase package and assembly instructions.
- [ ] 6. Assemble and commission actual hardware.
- [ ] 7. Establish intended research-use readiness from qualified review and
  actual measurement evidence.

## Resume checkpoint

Recovered from the November 22 Codex conversation beginning “where did we leave
off on design for hsv2 research box thing,” task T-21BC0885. Its latest turn was
interrupted during component consolidation. This pass adds the consolidated
register, explicit connection schedule, adapter support distinction and all
missing cost roles. It does not establish completed component selection.

**Current step 1.** CP3 prices every previously named US part in the basket.
The remaining seven rows are listed above with their dependencies. The next
inputs are the qualified measurement requirements, a reviewed local power-stop
assembly, and fabrication and destination quotes. Hardware will be needed to
verify actual fit and communication.
