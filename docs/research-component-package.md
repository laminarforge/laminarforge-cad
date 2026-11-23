# LaminarForge component selection and completion checklist

The first instrument uses a commercial syringe pump, replaceable wetted path,
commercial vessel interface and independent delivered-mass observation. The
component register now covers the complete set of purchasing and integration
roles, including the parts missing from the earlier pump recommendation.
**Step 1 remains incomplete:** several components depend on the vessel/readout
requirements, assembled pressure limits and electrical design. The known
reference outlay is $1,349.37; the complete delivered cost is unknown.

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

Use **StarTech ICUSB2321F** as the named USB-to-RS-232 candidate, with a separate
adapter for each instrument. Its manufacturer identifies an FTDI chipset and
documents the DB9 pinout. ICUSB232V2 is a different, Prolific-based product;
do not substitute it while retaining the FTDI support claim. The ICUSB2321F
guide lists macOS through 15; FTDI separately lists an ARM driver for macOS 26.
That provides a specific support question, not proof that the third-party
adapter works on this Mac. No driver installation is part of this package.
[Adapter](https://www.startech.com/en-us/cards-adapters/icusb2321f),
[manufacturer guide](https://sgcdn.startech.com/005329/media/sets/ICUSB232xF_Manual/ICUSB232xF_QSG.pdf),
[FTDI driver table](https://ftdichip.com/drivers/vcp-drivers/).

Retain the ibidi I Luer family as the vessel interface reference, 80171 as the
costing example, 10831 as the reference adapter pack and P-SYRKIT-SM as a water
characterization assortment. Final surface, syringe and tubing remain open.
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
| Pump data | Mac → adapter A → CBL-PC-PUMP-7 → NE-501 | Product-specific OS support, physical connectors, identity and state readback |
| Balance data | Mac → adapter B → balance-specific cable → DBS46 | Exact pinout/gender, raw framing, units and actual sample timing |
| Liquid | Syringe → tubing → 10831 → selected slide → outlet tube → collection vial | Exact wetted SKUs, retained volume, materials, assembled pressure limit and protection |
| Measurement | Vial/nest on balance; outlet anchored off scale | Gross mass, free outlet gap and no tube/support load on pan |
| Power | Regional OEM supply → reviewed independent stop → pump | Circuit, ratings, connectors, enclosure and deliberate reset |

The power row is a design boundary, not a wiring instruction. A TTL foot switch
commands the pump and does not independently remove power. Stop implementation
must address both motor power and remaining fluid pressure or siphoning.
The manufacturer documents serial timeout and power-return controls; the actual
device must be configured and observed before use. No start command may be
automatically replayed after connection loss.
[NE-500/501 manual](https://syringepump.com/download/NE-500%20OEM%20Syringe%20Pump%20User%20Manual%20V3.9.pdf).

## Remaining component and price decisions

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

Current step: **1**. Next, close product-specific serial support and the balance
cable, then resolve the vessel/readout and assembled-rating requirements that
control the remaining selections. Keep this checklist in each round's report.
Hardware acquisition and qualified review remain dependencies for later steps.
