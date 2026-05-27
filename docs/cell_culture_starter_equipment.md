# LaminarForge Starter Cell Culture Equipment Plan

This plan is for a small, self-contained starter setup that can support practice cell-culture workflows, early chip seeding dry runs, media-change workflow development, and incubator control experiments before the larger 100-chip automation stack is needed.

The starter CO2 incubator is not ready for public website publication. Treat it as an internal engineering prototype until the research gates in `docs/co2_incubator_research_audit.md` are satisfied.

## Build Strategy

Use three tiers:

1. Buy anything that directly affects sterility, biosafety, or measurement confidence.
2. Build low-risk support equipment where failure is visible and recoverable.
3. Defer automation-specific hardware until baseline manual culture is working.

The current CAD repo already has printable or reference models for the first build wave:

| Need | Existing model | Role |
| --- | --- | --- |
| Starter CO2 incubator cabinet | `co2_incubator` | Internal prototype for humidified 37 C / 5% CO2 control validation |
| Still-air handling enclosure | `still_air_box_rail`, `still_air_box_corner`, `arm_hole_grommet` | Low-cost practice sterile enclosure |
| Media warming | `water_bath` | Warm media/reagents to 37 C |
| Gentle mixing | `orbital_shaker`, `magnetic_stirrer` | Reagent/culture mixing |
| Microfluidic flow | `syringe_pump_standalone` | Low-rate perfusion and priming |
| Chip fixtures | `chip_adapter_plate`, `chip_stack_rack` | Hold LaminarForge chips during handling |
| Larger automation incubator | `chip_incubator_v3` | Defer until 100-chip rocker/imaging stack |

## Minimum Viable Cell Culture Setup

| Category | Item | Source | Priority | Notes |
| --- | --- | --- | --- | --- |
| Workspace | Access to BSL-2 wet-lab bench | Buy/rent | Required | Required for primary human material, iPSC work, viral work, and serious mammalian culture. |
| Sterile handling | Certified biosafety cabinet or lab-provided hood | Buy/rent | Required | Do not rely on DIY airflow for real culture work. Still-air box is for practice and nonhazardous prep only. |
| Incubation | Certified/accessed CO2 incubator or used commercial CO2 incubator | Buy/access | Required | Use this for serious mammalian culture while the DIY cabinet is validated. |
| Incubation prototype | Starter DIY CO2 cabinet | Build | Prototype only | Use for control development, empty-chamber validation, and chip hardware shakeout before real culture use. |
| Microscopy | Inverted phase-contrast microscope | Buy/access | Required | Must confirm confluence, morphology, contamination, and chip seeding. |
| Cell storage | Cryostorage access | Buy/access | Required before banking | Dewar or facility liquid nitrogen storage. |
| Cold storage | 4 C lab refrigerator | Buy/access | Required | Media and short-term reagent storage. |
| Cold storage | -20 C freezer | Buy/access | Required | Common reagents, aliquots, enzymes where applicable. |
| Water | Sterile DI/distilled water | Buy | Required | For humidification tray and reagent prep where specified. |
| Pipetting | P20, P200, P1000 pipettes | Buy | Required | Buy reliable used or new; accuracy matters. |
| Pipetting | Sterile filtered tips | Buy | Required | 10 uL, 200 uL, 1000 uL. |
| Vessels | T25/T75 flasks, 6/12/24/96-well plates | Buy | Required | Match practice cell line and chip workflow. |
| Consumables | Serological pipettes, conical tubes, microcentrifuge tubes | Buy | Required | 5/10/25 mL pipettes, 15/50 mL tubes, 1.5/2.0 mL tubes. |
| PPE | Gloves, lab coat, eye protection | Buy | Required | Facility rules override this list. |
| Waste | Biohazard bags, sharps container, disinfectant | Buy/access | Required | Use facility-approved disposal. |
| Warm handling | 37 C water bath | Build or buy | Early build | Existing `water_bath` is fine for media warming if leak-tested and temperature-validated. |
| Basic mixing | Orbital shaker | Build or buy | Early build | Useful for staining/washes and bacterial prep; not central for sterile culture. |
| Microfluidics | Syringe pump | Build | Early build | Needed for chip priming/perfusion tests before full automation. |
| Environmental logging | Temperature, CO2, humidity logger | Build/buy | Early build | Independent logger is mandatory for trusting the DIY incubator. |
| Centrifugation | Benchtop centrifuge | Buy used | Next | Needed for passaging workflows, PBMCs, protein work, and many cell protocols. |

## Starter CO2 Incubator Cabinet

Use `src/bin/co2_incubator.rs` as the first cabinet prototype. It is intentionally smaller than `chip_incubator_v3`; the goal is to validate chamber sealing, heat control, CO2 injection, humidity behavior, and workflow ergonomics before scaling up.

Target operating envelope:

| Parameter | Target |
| --- | --- |
| Temperature | 37.0 C, validate to +/- 0.5 C before use |
| CO2 | 5% target for bicarbonate-buffered media, validated with an independent incubator-range meter |
| Humidity | Passive tray, high humidity, no direct misting over cultures |
| Interior | 300 x 250 x 250 mm |
| Construction | PETG inner liner, PIR/foam insulation, PETG shell, removable service bay |
| Controls | ESP32-S3, heater SSR/MOSFET, fan PWM, normally closed CO2 solenoid |
| Safety | Thermal fuse or bimetal cutoff, software over-temp fault, manual power switch |

Buildable subassemblies:

| Subassembly | CAD output | Notes |
| --- | --- | --- |
| Inner chamber | `co2_incubator_chamber.stl` | Smooth PETG liner with shelf rails and ports. |
| Outer shell | `co2_incubator_shell.stl` | Holds insulation around the chamber. |
| Door | `co2_incubator_door.stl` | Gasket channel, acrylic window, latch holes. |
| Shelf | `co2_incubator_shelf.stl` | Ventilated tray support. Print two. |
| Humidity tray | `co2_incubator_water_tray.stl` | Use sterile water; clean often. |
| Electronics bay | `co2_incubator_service_bay.stl` | Keeps controller, MOSFET/SSR, terminals, and wiring outside humid chamber. |
| Gas/sensor manifold | `co2_incubator_service_manifold.stl` | Rear printed bulkhead for CO2 inlet, probe pass-through, cable gland, and heater leads. |
| Heater diffuser | `co2_incubator_heater_diffuser.stl` | Keeps cultures away from direct heater airflow and spreads warm air. |

Current CAD limitation: the CO2 sensor recess is a placeholder. Do not cut final hardware around it until the exact incubator-range CO2 sensor is selected and measured.

## Incubator Parts List

| Function | Part | Quantity | Notes |
| --- | --- | ---: | --- |
| Controller | ESP32-S3 dev board | 1 | Reuse `incubator-v2` firmware architecture where practical. |
| Temperature | DS18B20 probe or SHT40 | 1-2 | Use one independent validation sensor during commissioning. |
| CO2 sensing | 0-20% NDIR CO2 sensor | 1 | Required for 5% CO2 control. Do not use MH-Z19B-class 0-5000 ppm room-air sensors. |
| Humidity sensing | SHT40/SHT31 | 1 | Monitor-only in v1. |
| Heating | 12 V silicone heater pad, 40-80 W | 1 | Size after thermal test; do not overspec without cutoff. |
| Air mixing | 40 mm or 60 mm fan | 1 | Low speed, continuous circulation. |
| CO2 dosing | Normally closed 12 V solenoid valve | 1 | Fail-closed. Put regulator upstream. |
| CO2 supply | Small CO2 cylinder + low-pressure regulator, or validated dry-ice generator | 1 | Needs relief path, regulator/pressure limit, leak checks, and documented operating procedure. |
| Power | 12 V DC supply | 1 | Current rating sized for heater plus fan and valve. |
| Switching | MOSFET module or SSR | 1-2 | Keep outside chamber in service bay. |
| Safety cutoff | Thermal fuse or 55 C bimetal cutoff | 1 | Wire to remove heater power independent of firmware. |
| Door seal | Silicone gasket cord/strip | 1 | Fit to printed gasket channel. |
| Window | Acrylic or polycarbonate sheet | 1 | Mechanically retained and sealed. |
| Insulation | PIR/foam board | As needed | 25 mm target around chamber. |
| Fasteners | M3 screws, heat-set inserts, magnets/latches | As needed | Use stainless where exposed to humidity. |
| Cable sealing | Cable glands/grommets | As needed | Avoid open wire holes into chamber. |

## What To Design Next

1. Incubator cabinet revision: add panelized service bay and manifold details, then print a fit-check set.
2. Sensor selection: choose a specific 0-20% CO2 sensor and revise the sensor mount around its datasheet dimensions.
3. Commissioning protocol: empty-chamber thermal rise, overshoot, door-open recovery, 24-hour stability, CO2 leak/decay test.
4. Still-air-box revision: make the practice enclosure easier to disinfect and add a pass-through tray.
5. Syringe pump fluid path holders: add disposable tubing clips and chip priming fixture.
6. Larger incubator decision: only move to `chip_incubator_v3` after the small cabinet validates control and sealing assumptions.

## Hard Boundaries

This equipment plan does not replace a certified biosafety cabinet, institutional biosafety rules, sterilization validation, or training. DIY incubation should be treated as prototype infrastructure until independently validated with logged temperature, CO2, humidity, contamination outcomes, and cell health.
