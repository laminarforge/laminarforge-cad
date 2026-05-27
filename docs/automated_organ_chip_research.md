# Automated Organ-on-Chip and Multi-Chip Media Exchange Research

This note checks whether the LaminarForge automated cell-culture cassette is genuinely novel or already solved by existing organ-on-chip platforms.

## Short Answer

The technology area exists, but it is still fragmented and mostly closed. Commercial systems provide integrated organ-on-chip platforms, and academic groups have demonstrated automated perfusion, robotic coupling, fluidic circuit boards, and high-throughput chip arrays. The open gap is a buildable, modular, low-cost, multi-chip cassette/shelf system that combines:

- High chip density.
- Robot-addressable mechanical registration.
- Closed or semi-closed sterile fluid handling.
- Pump/reservoir/tubing manifolds that can be inspected and serviced.
- Standard lab footprint compatibility.
- Open CAD and validation gates.

That makes the LaminarForge direction plausible and research-aligned, but not trivial.

## Existing Reference Systems

| Source | What exists | Scale signal | Relevance to LaminarForge |
| --- | --- | --- | --- |
| Wyss / Nature Biomedical Engineering robotic Interrogator | Liquid-handling robotics, custom software, mobile microscope, automated culture/perfusion/media addition/sample collection/in situ imaging inside a standard tissue-culture incubator | Up to 10 organ chips; eight vascularized two-channel chips maintained for 3 weeks | Proves robotic fluidic coupling is real. Also shows the bar: calibration, software, microscopy, incubator integration, and long-run culture validation. |
| University of Twente TOP / FCB work | Fluidic circuit board integrating organ chips, valves, sensors, and pumps; standardized modular microfluidic building blocks | Three 64-chamber MFBBs in one FCB paper; 192 controlled culture chambers discussed in STARTER context | Strongest architectural match. LaminarForge should borrow the FCB/manifold mindset instead of loose tubing everywhere. |
| STARTER platform | Open-design modular organ-on-chip platform with swappable pump, reservoir, sensor, routing, and OoC modules inside an ANSI/SLAS footprint | Demonstrates multiple OoC devices, inline sensing, pump modules, routing block, and open-source design direction | Confirms that open modularity is the right direction. LaminarForge should align with ANSI/SLAS/ISO-style interfaces where practical. |
| Pressure-driven perfusion / recirculation systems | Multiplexed pressure-driven flow and medium recirculation for organs-on-chips | Built to address high-throughput limits and expensive media use | Validates recirculation and multiplexing as design goals, but pressure systems can add complexity and hardware cost. |
| MIMETAS OrganoFlow / OrganoPlate | Gravity-driven, pump-free perfusion for many chips in parallel inside standard incubators | Claims up to about one thousand chips in parallel across OrganoPlate platforms | Shows extreme scaling is possible when chip format and rocker are tightly co-designed. It is closed/commercial and platform-specific. |
| CN Bio PhysioMimix | Commercial closed-loop fluidics, multi-chip plates, integrated pumps, COC consumables, incubator-compatible docking station/controller | Multi-chip Liver-48 and up to 288 replicates/run cited in product material | Confirms multi-chip, closed-loop, plate-based scaling is market-relevant. Also shows why disposable consumable design matters. |
| 96-well perfusion / vascularized micro-organ platforms | 96-well format perfused microphysiological systems, some without external pumps/valves | 96-well / large-scale drug screening framing | Supports the move away from single-chip fixtures toward plate/shelf/cassette scale. |

## Design Implications

1. Do not design around one chip as the main unit. Use a multi-chip cassette or shelf as the minimum automation module.
2. Keep Rev C chip compatibility, but treat the cassette as a fluidic/mechanical carrier, not just a holder.
3. Borrow from FCB architecture: row/column manifolds, standardized connector locations, routing blocks, and modular pump/reservoir/sensor interfaces.
4. Keep robot interface features: gripper ears, fiducials, latching, and repeatable deck datum.
5. Use closed or semi-closed disposable wetted paths where possible. Printed parts should provide structure, not become unvalidated sterile fluid contact surfaces.
6. Add explicit validation gates: flow-rate mapping, cross-channel uniformity, leak test, bubble clearance, dead-volume measurement, priming time, media recovery/sampling accuracy, and contamination hold tests.

## Current CAD Response

The initial `automated_media_exchange_cassette` is being shaped as a 20-chip cassette/shelf:

- 4 columns x 5 rows of Rev C chips.
- Row media rails for inlet/outlet routing.
- Robot gripper ears and fiducial targets.
- Bubble-view gutters.
- Leak/drain features.
- Compatibility with the existing `media_reservoir`, `chip_priming_tubing_fixture`, and higher-density chip-farm direction.

The next iteration should add a more explicit manifold/FCB layer: either a reusable dry mechanical cassette with disposable sterile tubing snapped in, or a dedicated sterilizable fluidic plate with gasketed chip interfaces.

## Sources

- Robotic fluidic coupling and interrogation of multiple vascularized organ chips, Nature Biomedical Engineering: https://www.nature.com/articles/s41551-019-0497-x
- Pressure-Driven Perfusion System to Control, Multiplex and Recirculate Cell Culture Medium for Organs-on-Chips: https://pmc.ncbi.nlm.nih.gov/articles/PMC9416133/
- Modular operation of microfluidic chips for highly parallelized cell culture and liquid dosing via a fluidic circuit board: https://pmc.ncbi.nlm.nih.gov/articles/PMC8433198/
- Fluidic circuit board with modular sensor and valves enables stand-alone, tubeless microfluidic flow control in organs-on-chips: https://pmc.ncbi.nlm.nih.gov/articles/PMC8922413/
- STARTER open modular organ-on-chip platform: https://pmc.ncbi.nlm.nih.gov/articles/PMC12834090/
- STARTER Kit dataset: https://data.4tu.nl/datasets/3d95f25c-bbb9-4169-8c21-3fc75ca4056e
- MIMETAS OrganoFlow: https://mimetas.com/organoflow
- CN Bio PhysioMimix organ-on-chip technology: https://cn-bio.com/organ-on-a-chip-technology/
- A vascularized and perfused organ-on-a-chip platform for large-scale drug screening applications: https://pmc.ncbi.nlm.nih.gov/articles/PMC6995340/
- Microfluidics-enabled 96-well perfusion system for high-throughput tissue engineering and long-term all-optical electrophysiology: https://pmc.ncbi.nlm.nih.gov/articles/PMC7680692/
