# LaminarForge component selection and completion checklist

The first instrument uses a commercial syringe pump, replaceable wetted path,
commercial vessel interface and independent delivered-mass observation. The
component register now covers the complete set of purchasing and integration
roles, including the parts missing from the earlier pump recommendation.
**Step 1 remains open.** Fourteen of 21 cost rows now have US reference
prices, including one cable bundled in the starter kit. The priced items
total **$1,588.03**. Seven rows still need prices or a design decision.
The delivery destination is Montgomery, Texas, in the Houston area. Adding
the balance seller's lowest returned freight of $141.34 brings the priced
items to **$1,729.37** before tax, other freight, services and the open rows.
This is a pricing exercise using public listings and automated estimates.
Alex confirmed on November 23 that he has purchased no project hardware.
Budget and specification work can proceed now. Physical checks follow receipt.
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
reference basket. Apple publishes free standard US delivery.
Two direct connections avoid adding a hub to the initial list. Available ports
and reusable adapters still need physical inventory.
[US adapter price](https://www.apple.com/shop/product/mw5l3am/a/usb-c-to-usb-adapter),
[Apple US shipping](https://www.apple.com/shop/help/shipping_delivery).

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
(46324225769727) available at $96.49 on November 23. Its Montgomery shipping
estimate appears below. The previously recorded accuracy discrepancy remains open.
[Seller inventory for this product](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html.js).
Its cable must follow its own instrument pinout;
the pump cable cannot be reused by assumption. Selection still needs a gross
vial/nest/liquid mass budget and measurement uncertainty appropriate to the
required delivery. [Balance basis](water-bench-low-cost.md#balance-tradeoff-and-integration).

## Connection schedule

| Link | Candidate connection | Remaining decision or observation |
| --- | --- | --- |
| Pump data | Mac → adapter A → CBL-PC-PUMP-7 → NE-501 | Physical connector mating, actual identity and state readback |
| Balance data | Mac → adapter B → StarTech SCNM9FF → DBS46 | Check delivered connectors, continuity, raw framing, units and sample timing |
| Liquid | Syringe → tubing → 10831 → selected slide → outlet tube → collection vial | Exact wetted SKUs, retained volume, materials, assembled pressure limit and protection |
| Measurement | Vial/nest on balance; outlet anchored off scale | Gross mass, free outlet gap and no tube/support load on pan |
| Power | Regional OEM supply → reviewed independent stop → pump | Circuit, ratings, connectors, enclosure and deliberate reset |

The balance manual's printed page 7 specifies balance pin 2 to host pin 3,
balance pin 3 to host pin 2, and ground 5 to 5. It documents 9600 baud, eight
data bits, no parity and one stop bit. The connector close-up for the exact
balance listing shows nine male pins. FTDI specifies a male host-adapter end.
StarTech's drawing for SCNM9FF shows two female ends and the required three
connections. Select one SCNM9FF. Confirm fit, continuity and real data when
the hardware arrives. CDW lists the cable at $11.99 in the US. The UK listing
of £11.99 including VAT is a separate offer.
[Balance connector photo](https://cdn.shopify.com/s/files/1/0723/8113/7151/files/4__77808.1569302765.1280.1280.jpg?v=1731291328),
[FTDI datasheet](https://ftdichip.com/wp-content/uploads/2023/07/DS_US232R-10_R-100-500.pdf),
[SCNM9FF drawing](https://sgcdn.startech.com/005329/media/sets/SCNM9FF/Diagram/SCNM9FF_Diagram.PDF),
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

Two overseas products serve as functional comparisons for a future US review. Klinger & Born
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
| StarTech SCNM9FF | One cable | 11.99 |
| Nalgene 2118-0001PK | One pack of 12 jars at regular price | 52.70 |
| EMS 63312-02 | One tray with cover | 70.75 |
| **Added to the previous $1,452.59** | | **135.44** |

Thermo Fisher also displays a $44.81 online offer for the same pack. The
regular price keeps the budget independent of that discount. The jar is
cataloged at 20 g, 36 mm diameter and 47 mm height. The listing does not
specify the lid state for that weight. Actual tare, clearance and collection
capacity remain to be checked. The tray's 27.9 × 15.5 × 8.3 cm external
dimensions leave its usable spill volume and coverage unresolved. The delivered price to Texas remains open.
[Jar and pack price](https://www.thermofisher.com/order/catalog/product/2118-0001PK),
[tray price](https://www.emsdiasum.com/instrument-tray-2),
[EMS distributor directory](https://www.emsdiasum.com/world-distribution).

### Houston-area pricing

| Item under consideration | Purpose | Published or estimated US price |
| --- | --- | ---: |
| U.S. Solid USS-DBS46-1 | Weigh collected water during bench evaluation | $96.49 |
| UPS Worldwide Saver for one balance | Lowest rate returned for Montgomery, TX | $141.34 |
| **Balance plus its freight** | Before tax or any other charges | **$237.83** |
| StarTech SCNM9FF, FTDI US232R-100-BULK and Apple USB-C adapter | One of each to read the scale from an existing Mac | $11.99 + $26.80 + $19.00 = **$57.79** |

Two automated estimates for the same SKU and destination returned the same
nine rates on November 23. The second estimate also verified the US storefront,
USD currency and quantity of one. The seller advertised 3.75 lb gross weight
and recorded 5 lb for the exact shipping variant. The origin, duties, tax and
final invoice remain open. An open-box listing for the exact model showed $250
before tax. Its displayed shipping estimate used a different ZIP. A cheaper
new listing with free shipping identified a different part, JFDBS00003-100G.
[Manufacturer listing](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html),
[manufacturer variant](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html.json),
[exact-model open-box listing](https://www.ebay.com/itm/175814390323),
[other model listing](https://www.ebay.com/itm/111988441562).

The Texas state sales tax is 6.25% and the combined rate can reach 8.25%.
Taxable merchandise generally includes its associated seller delivery charges
in the taxable amount. The exact local jurisdiction depends on the street
address. As a budgeting example, applying the statewide maximum to $237.83
gives $19.62 tax and $257.45 total for this balance shipment.
[Texas local sales tax](https://comptroller.texas.gov/taxes/sales/faq/local.php),
[Texas shipping and tax](https://comptroller.texas.gov/taxes/sales/faq/collection.php).

MNM Scales in Cypress publishes $95 for a shop calibration with its in-house
certificate and $395 for its NIST-traceable certificate in the up-to-2.2 kg
capacity tier. An on-site visit adds $295 per visit plus $2 per round-trip
mile. Drop-off is by appointment. Exact service scope and expenses depend on
the eventual selection. As one independent weight comparison, ScalesGalore
lists Mettler Toledo 158657, a 50 g F1 weight with certificate, at $253.30.
No service or weight has been selected. The $1,588.03 parts subtotal excludes
both. [Cypress service options](https://mnmscales.com/services/balance-calibration-houston/),
[50 g weight listing](https://www.scalesgalore.com/product/Mettler-Toledo-158657-OIML-Class-F1-Calibration-Weight-With-Certification-50-g-px59723.cfm).

The earlier Thailand parts and service comparisons remain in the component
register and the prior saved checklist. The active budget uses US prices.

### Costs still open

| Rows | What sets the price |
| --- | --- |
| CP-14, CP-15 | Final tubing and pressure protection depend on the accepted configuration. |
| CP-16 | Local electrical review and a quote for the complete independent stop. |
| CP-19 | Support dimensions, material quantities and a fabrication quote. |
| CP-20 | Required measurement uncertainty, check masses and calibration scope. |
| CP-21 | Qualified research-use requirements for the readout and environment. |
| CP-22 | Destination charges, replacement quantities and labor. Count fabrication only once under CP-19. |

The exact balance listing includes one 50 g weight for external adjustment.
CP-07 already covers its cost. The listing supplies no calibrated mass value
or uncertainty for that individual weight. CP-20 stays open for an independent
reference matched to the approved weighing range and allowable error.
[Balance package contents](https://ussolid.com/products/u-s-solid-100-x-0-001g-analytical-balance-1-mg-digital-lab-precision-scale-html).

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

- [x] Audit existing work
- [ ] 1. Finish component-selection package
  - [x] 1.1. Name and price the initial bench parts.
  - [x] 1.2. Match the balance data cable to both catalog connectors and the published pinout.
  - [x] 1.3. Identify the bundled balance weight and a resolution-matched calibration service.
  - [ ] 1.4. Specify the independent mass reference and required measurement uncertainty.
  - [ ] 1.5. Resolve the remaining parts and their compatibility.
  - [ ] 1.6. Obtain a complete delivered budget for the selected parts and services.
    - [x] 1.6.1. Price the balance and computer accessories for the confirmed delivery region.
    - [ ] 1.6.2. Close remaining US freight, taxes, local services and labor.
- [ ] 2. Lock research-use requirements
  - [x] 2.1. Document which requirements need qualified review.
  - [ ] 2.2. Record qualified requirements and acceptance limits.
- [ ] 3. Finish integrated mechanical design
- [ ] 4. Finish controls and recording
  - [x] 4.1. Recover saved records and check them against their receipts.
  - [ ] 4.2. Capture an actual instrument reading with its identity, units and timing.
    - [x] 4.2.1. Check the Mac for a USB-serial balance adapter.
    - [x] 4.2.2. Implement receive-only capture and verify how it handles a missing adapter.
    - [ ] 4.2.3. Record balance output and match its value and units to the instrument display.
  - [ ] 4.3. Verify independent stop and deliberate restart on the assembled hardware.
- [ ] 5. Produce fabrication and purchase package
- [ ] 6. Assemble and commission
- [ ] 7. Establish intended research-use readiness

## Resume checkpoint

The November 22 conversation and ticket T-21BC0885 hold the original work list.
As of November 23, the next budget item is 1.6.2. One balance shipment and local calibration options now have prices.
The other US freight, exact tax and outstanding labor estimates are open. The first physical
balance reading belongs under 4.2.3 after the equipment arrives.

**Current step 1.4 depends on 2.2.** The measurement reviewer needs to record
the expected gross load including tare, the smallest mass change that matters,
and the allowable error for that change, with units and the source of approval.
Those values determine which independent reference and certificate uncertainty
are adequate. The balance's display resolution alone cannot establish them.
Electrical review and fabrication quotes can proceed as their interfaces are
settled. The recorder includes the substeps in each new
[handoff](water-bench-records.md#recover-a-checklist-and-its-evidence).
