# Cell Culture Equipment Design Matrix

This matrix expands the starter plan from a single CO2 incubator into the full minimum equipment set needed for responsible mammalian cell-culture workflow development. The rule is conservative: DIY hardware can support training, fixtures, environmental testing, and microfluidic hardware validation, but real culture work needs certified or facility-approved sterile handling, storage, disposal, and measurement infrastructure.

## Equipment Coverage

| Area | Required equipment | Decision | Current CAD coverage | Validation gate |
| --- | --- | --- | --- | --- |
| Sterile handling | Certified Class II biosafety cabinet or lab-provided hood | Buy/access | None | Current certification, facility approval, decontamination SOP |
| Sterile handling practice | Still-air enclosure | Prototype only | `still_air_box_corner`, `still_air_box_rail`, `still_air_box_accessories`, `arm_hole_grommet`, `workstation_enclosure` | Fit/cleanability check; nonhazardous practice only |
| Incubation | Validated commercial/accessed CO2 incubator | Buy/access | None | Independent 37 C / 5% CO2 / humidity / recovery logs |
| Incubation prototype | Starter DIY CO2 cabinet | Prototype only | `co2_incubator`, `co2_incubator_verify`, `co2_incubator_sim` | CAD verifier, lumped sim, empty-chamber commissioning, independent logger agreement |
| Warming | 37 C media water bath or dry warming block | Build or buy | `water_bath`, `heating_block` | Independent temperature probe, leak/electrical safety test |
| Observation | Inverted phase-contrast microscope | Buy/access | `optical_mount` support only | Culture vessel/chip morphology and contamination inspection |
| Manual liquid handling | P20/P200/P1000, filtered tips, serological pipettes | Buy | `tube_holder`, `column_rack` support only | Calibration status and sterile consumable availability |
| Aspiration/waste | Vacuum aspirator, trap, disinfectant, biohazard waste path | Buy/access | `wash_station`, `media_reservoir` support only | Facility-approved disposal and splash/aerosol control |
| Centrifugation | Benchtop centrifuge with correct rotor/buckets | Buy | `centrifuge_adapter` organization only | Manufacturer-rated rotor; no printed safety-critical rotor substitution |
| Cold storage | 4 C refrigerator and -20 C freezer | Buy/access | `sample_cold_block`, `peltier_reservoir_block` bench support only | Temperature logging and lab-only storage segregation |
| Cryostorage | LN2 or approved cryostorage access | Access | None | Inventory, PPE, controlled-rate freezing, facility approval |
| Mixing | Orbital shaker, rack rocker, magnetic stirrer | Build or buy | `orbital_shaker`, `rack_rocker`, `rack_rocker_2axis`, `magnetic_stirrer` | RPM/tilt verification, spill containment, sterility plan |
| Microfluidics | Syringe pump, chip fixtures, reservoirs, tubing management | Build | `syringe_pump_standalone`, `chip_adapter_plate`, `chip_stack_rack`, `media_reservoir`, `pbmc_flow_cell_mount` | Flow calibration, leak/bubble/dead-volume test, disposable sterile fluid path |
| Environmental logging | Independent temp/CO2/RH/power logging | Buy/build support | `controller_enclosure` | Logger agreement with controller over warmup, steady state, recovery, overnight hold |
| Safety and waste | PPE, disinfectant, spill kit, sharps, biohazard bags | Buy/access | None | Facility SOP and disposal chain before live work |
| Automation scale-up | High-density chip incubator and chip farm | Defer | `chip_incubator_v3`, `chip_farm_assembly`, `chip_farm_assembly_v2`, `lh_interface` | Starter incubator and manual chip culture workflow pass first |

## Build Order

1. Secure or buy the non-negotiables for real culture: certified sterile handling, validated CO2 incubation, microscope access, cold/cryostorage, centrifuge, pipettes, consumables, waste, PPE, and facility SOPs.
2. Build support hardware with low biosafety risk: water bath, tube/tip organization, still-air practice enclosure, environmental logging, and chip fixtures.
3. Continue the DIY CO2 incubator as an engineering prototype: CAD verification, thermal/CO2 simulation, empty-chamber commissioning, then independent sensor comparison.
4. Build microfluidic support: syringe pump, reservoirs, tubing clips, bubble traps, chip holders, and flow calibration fixtures.
5. Defer high-density automation until manual culture and single-chip workflows are stable.

## Design Backlog

| Priority | Design output | Reason |
| --- | --- | --- |
| P0 | CO2 sensor/regulator mounting after exact sensor selection | The current incubator has ports, but no final sensor package mount. |
| P0 | Independent logger enclosure and probe routing | Validation must not depend on the controller's own sensors. |
| P0 | Aspirator trap/waste bottle holder | Waste handling is required before real culture and should be mechanically stable. |
| P1 | Pipette/tip/conical organization module | Reduces workflow errors and improves clean bench ergonomics. |
| P1 | Chip priming fixture with tubing clips and bubble observation | Needed before real microfluidic culture trials. |
| P1 | Water bath safety revision with probe clamp and bottle rack | Media warming needs repeatable placement and safe cable routing. |
| P2 | Still-air practice pass-through tray | Good for training and nonhazardous dry runs, not for real culture. |
| P2 | Microscope chip-stage adapter | Lets LaminarForge chips be inspected repeatably on an inverted microscope. |
| P3 | `chip_incubator_v3` scale-up review | Only after the starter incubator validation data is credible. |

## Machine-Readable Manifest

Run:

```sh
cargo run --release --bin cell_culture_equipment_manifest
```

The manifest emits JSON rows for each equipment item, including disposition, readiness, culture use, CAD bins, validation gate, and notes. Keep this aligned with this document before any website publication or BOM export.

CFD and conjugate heat-transfer work is tracked separately in `docs/cell_culture_cfd_validation_plan.md`. Treat it as an engineering validation layer for airflow and thermal behavior, not as sterility or biosafety evidence.

## Boundaries

The LaminarForge CAD set can design fixtures, enclosures, support hardware, prototype environmental chambers, and validation tooling. It should not imply that DIY equipment is certified for biosafety, clinical, GMP, iPSC, primary human material, or viral workflows. Those uses require facility review, certified equipment, documented SOPs, and independent validation.
