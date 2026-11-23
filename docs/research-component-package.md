# LaminarForge component selection and completion checklist

The first instrument uses a commercial syringe pump, replaceable wetted path,
commercial vessel interface and independent delivered-mass observation. The
component register now covers the complete set of purchasing and integration
roles, including the parts missing from the earlier pump recommendation.
**Step 1 remains incomplete:** several components depend on the vessel/readout
requirements, assembled pressure limits and electrical design. The known
reference outlay is $1,452.59 USD, plus a separate £11.99 conditional cable
reference; the complete delivered cost is unknown. Ten USD rows remain
unquoted, down from fourteen in CP1.

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
reference basket. The Thai SKU is MW5L3ZA/A; obtain its local price separately.
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

Retain USS-DBS46-1 as a balance candidate with the previously recorded stock
and accuracy discrepancies. Its cable must follow its own instrument pinout;
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
cable candidate; the UK manufacturer lists £11.99 including VAT. Confirm the
balance connector gender and continuity before ordering. The diagram does not
specify gender, so no gender changer or additional crossover is selected.
[Cable](https://www.startech.com/en-gb/cables/scnm9ff),
[balance manual](https://file.ussolid.com/content/JFDBS/Manual-USS-DBS46%20Precision%20Balance.pdf).

The power row is a design boundary, not a wiring instruction. A TTL foot switch
commands the pump and does not independently remove power. Stop implementation
must address both motor power and remaining fluid pressure or siphoning.
The manufacturer documents serial timeout and power-return controls; the actual
device must be configured and observed before use. No start command may be
automatically replayed after connection loss.
[NE-500/501 manual](https://syringepump.com/download/NE-500%20OEM%20Syringe%20Pump%20User%20Manual%20V3.9.pdf).

## Remaining component and price decisions

The added host connections contribute $91.60: two FTDI cables ($53.60) and
two USB-C adapters ($38). The £11.99 cable reference stays in GBP; no currency
conversion or shipping/tax estimate is hidden in the USD subtotal. Replacing
the $20.38 assortment with the $32 syringe adds $11.62; together this round
adds $103.22 to the earlier $1,349.37 reference basket.

Use Nalgene 2118-0001PK as the collection-container candidate: a pack of twelve
30 mL wide-mouth PPCO jars. Its empty mass, nest fit and collection capacity
must match the eventual run; it is not a sealed transport vessel. The pack
price is unquoted. EMS 63312-02 is the polypropylene drip-tray candidate, with
nominal external dimensions 27.9×15.5×8.3 cm. Its usable containment volume and
route coverage need review; multiplying outer dimensions would overstate it.
[Jar](https://www.thermofisher.com/order/catalog/product/2118-0001PK),
[tray](https://www.emsdiasum.com/instrument-trays-with-cover).

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

Current step: **1**. CP2 closes the named adapter support basis and prices the
USB host connections. It establishes the balance signal map and identifies
collection/tray candidates. Next, resolve the vessel/readout requirements and
assembled working limits, finish the independent stop design, and obtain the
remaining prices. Hardware acquisition is needed for actual communication,
fit and commissioning; it is not a prerequisite for continuing design.
