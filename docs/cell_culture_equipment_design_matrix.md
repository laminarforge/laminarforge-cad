# Cell Culture Equipment Design Matrix

This matrix expands the starter plan from a single CO2 incubator into the full minimum equipment set needed for responsible mammalian cell-culture workflow development. The design boundary is not a stop sign: it is broken down in `docs/cell_culture_equipment_boundary_breakdown.md` into CAD prototype, engineering prototype, biological pilot, internal production tool, and certified/compliant equipment gates. The monitoring architecture is documented in `docs/cell_culture_multimodal_monitoring_design.md`: visual inspection is useful, but culture automation should primarily combine fluidic, electrical, chemical, metabolic, and environmental signals. The reproducibility controls are mapped in `docs/tissue_chip_reproducibility_controls.md`.

## Equipment Coverage

| Area | Required equipment | Decision | Current CAD coverage | Validation gate |
| --- | --- | --- | --- | --- |
| Sterile handling | Certified Class II biosafety cabinet or lab-provided hood | Buy/access | None | Current certification, facility approval, decontamination SOP |
| Sterile handling automation | Closed Grade A / ISO 5 isolator workcell inside support pod/background | Buy/access/qualify, CAD for architecture only | `closed_isolator_workcell`, `aseptic_transfer_hatch`, `clean_support_pod_shell` | HEPA leak test, smoke study, pressure cascade logging, particle monitoring, VHP/decon validation, RTP/transfer qualification |
| Sterile handling practice | Still-air enclosure | Prototype only | `still_air_box_corner`, `still_air_box_rail`, `still_air_box_accessories`, `arm_hole_grommet`, `workstation_enclosure` | Fit/cleanability check; nonhazardous practice only |
| Incubation | Validated commercial/accessed CO2 incubator | Buy/access | None | Independent 37 C / 5% CO2 / humidity / recovery logs |
| Environmental utilities | External gas, humidification, pressure, and thermal-loop utility skid | Build for hardware validation; qualify/buy critical gas hardware | `environmental_utility_skid` | Regulator safety check, sterile filtration/backflow checks, dewpoint/RH calibration, pressure cascade logging, thermal-loop leak test |
| Incubation prototype | Starter DIY CO2 cabinet | Prototype only | `co2_incubator`, `co2_incubator_verify`, `co2_incubator_sim` | CAD verifier, lumped sim, empty-chamber commissioning, independent logger agreement |
| Warming | 37 C media water bath or dry warming block | Build or buy | `water_bath`, `water_bath_safety_kit`, `heating_block` | Independent temperature probe, leak/electrical safety test |
| Observation | Inverted phase-contrast microscope | Buy/access | `optical_mount` support only | Culture vessel/chip morphology and contamination inspection |
| Automated liquid handling | Closed fluid path with pumps, reservoirs, sterile tubing, chip fixtures, and validated connectors | Build/buy/access | `syringe_pump_standalone`, `media_reservoir`, `chip_priming_tubing_fixture`, `automated_media_exchange_cassette`, `sterile_tubing_harness`, `cassette_bench_nest`, `sealed_culture_module`, `sealed_module_docking_bay`, `culture_module_service_skid`, `inline_sensor_service_module` | Flow calibration, sterile disposable path, leak/bubble/dead-volume checks, no routine manual transfers |
| Pipettes for validation/debug only | P20/P200/P1000, filtered tips, serological pipettes | Buy, not production workflow | `tube_holder`, `column_rack`, `pipette_tip_organizer` support only | Calibration/setup/rescue use only; not part of intended culture process |
| Aspiration/waste | Closed vacuum aspirator, trap, disinfectant, biohazard waste path | Buy/access | `wash_station`, `media_reservoir`, `aspirator_waste_trap_holder` support only | Facility-approved disposal and splash/aerosol control |
| Centrifugation | Benchtop centrifuge with correct rotor/buckets | Buy | `centrifuge_adapter` organization only | Manufacturer-rated rotor; no printed safety-critical rotor substitution |
| Cold storage | 4 C refrigerator and -20 C freezer | Buy/access | `sample_cold_block`, `peltier_reservoir_block` bench support only | Temperature logging and lab-only storage segregation |
| Cryostorage | LN2 or approved cryostorage access | Access | None | Inventory, PPE, controlled-rate freezing, facility approval |
| Mixing | Orbital shaker, rack rocker, magnetic stirrer | Build or buy | `orbital_shaker`, `rack_rocker`, `rack_rocker_2axis`, `magnetic_stirrer` | RPM/tilt verification, spill containment, sterility plan |
| Microfluidics | Syringe pump, chip fixtures, reservoirs, tubing management | Build | `syringe_pump_standalone`, `chip_adapter_plate`, `chip_stack_rack`, `chip_priming_tubing_fixture`, `automated_media_exchange_cassette`, `sterile_tubing_harness`, `cassette_bench_nest`, `sealed_culture_module`, `sealed_module_docking_bay`, `culture_module_service_skid`, `inline_sensor_service_module`, `media_reservoir`, `pbmc_flow_cell_mount` | Flow calibration, leak/bubble/dead-volume test, disposable sterile fluid path |
| Environmental logging | Independent temp/CO2/RH/power logging | Buy/build support | `controller_enclosure`, `cell_culture_logger_enclosure`, `co2_sensor_service_module` | Logger agreement with controller over warmup, steady state, recovery, overnight hold |
| Multimodal culture monitoring | Pressure/flow, TEER/impedance, pH/O2, metabolites, imaging triage | Build/buy/access | `inline_sensor_service_module`, `cassette_sensor_backplane`; planned: `cassette_imaging_station`, `sensorized_chip_revision` | Sensor calibration, no-cell endurance run, image/sensor disagreement review, no single-signal biological claims |
| Safety and waste | PPE, disinfectant, spill kit, sharps, biohazard bags | Buy/access | None | Facility SOP and disposal chain before live work |
| Automation scale-up | High-density chip incubator and chip farm | Defer | `closed_isolator_workcell`, `aseptic_transfer_hatch`, `clean_support_pod_shell`, `sealed_culture_module`, `sealed_module_docking_bay`, `culture_module_service_skid`, `inline_sensor_service_module`, `cassette_sensor_backplane`, `environmental_utility_skid`, `chip_incubator_v3`, `chip_farm_assembly`, `chip_farm_assembly_v2`, `lh_interface` | Starter incubator and single-chip automated workflow pass first |

## Build Order

1. Secure or buy the non-negotiables for real culture: certified sterile handling, validated CO2 incubation, microscope access, cold/cryostorage, centrifuge, sterile consumables/connectors, waste, PPE, and facility SOPs.
2. Build support hardware with low biosafety risk: water bath, closed-fluid-path fixtures, still-air practice enclosure, environmental logging, and chip fixtures.
3. Continue the DIY CO2 incubator as an engineering prototype: CAD verification, thermal/CO2 simulation, empty-chamber commissioning, then independent sensor comparison.
4. Build microfluidic support: syringe pump, reservoirs, tubing clips, bubble traps, chip holders, and flow calibration fixtures.
5. Defer high-density automation until the single-chip automated fluid workflow is stable.

## Design Backlog

| Priority | Design output | Reason |
| --- | --- | --- |
| P0 | CO2 sensor/regulator mounting after exact sensor selection | The current incubator has ports, but no final sensor package mount. |
| P0 | Independent logger enclosure and probe routing | Initial CAD exists in `cell_culture_logger_enclosure`; validation must not depend on the controller's own sensors. |
| P0 | External CO2 service module | Initial CAD exists in `co2_sensor_service_module`; final dimensions depend on selected 0-20% NDIR sensor, pump, filters, and fittings. |
| P0 | Aspirator trap/waste bottle holder | Initial CAD exists in `aspirator_waste_trap_holder`; final use still depends on facility-approved disinfectant, filter, trap, and disposal SOP. |
| P1 | Disposable sterile tubing harness for the 20-chip cassette | Initial CAD exists in `sterile_tubing_harness`; replaceable manifold insert, branch comb, and keyed pump couplers keep media exchange automation-first and avoid routine manual pipetting. |
| P1 | Cassette bench/deck nest | Initial CAD exists in `cassette_bench_nest`; gives the 20-chip cassette a repeatable datum, leak tray, tube clearance, latch posts, and robot fiducials for bench validation. |
| P1 | Sealed culture module | Initial CAD exists in `sealed_culture_module`; wraps the cassette/nest in a dockable process module with gasketed lid, service bulkhead, thermal plate, and sensor/service clearances. |
| P1 | Sealed module docking bay | Initial CAD exists in `sealed_module_docking_bay`; gives the sealed module a repeatable pod-side receiver with leak capture, datum rails, latch/presence features, and standardized rear service couplers. |
| P1 | Culture module service skid | Initial CAD exists in `culture_module_service_skid`; keeps pumps, reservoirs, waste, gas, thermal loop, backplane, and strain relief outside the sealed module while preserving a dockable service footprint. |
| P1 | Closed isolator workcell | Initial CAD exists in `closed_isolator_workcell`; reflects the current research direction that exposed aseptic operations belong in a closed isolator, with RTP/VHP transfer features, HEPA plenum, service penetration panel, and rear incubated module bay. |
| P1 | Aseptic transfer hatch | Initial CAD exists in `aseptic_transfer_hatch`; models double-door interlock geometry, 190 mm and 270 mm RTP alpha placeholders, VHP inlet/exhaust/catalyst ports, gasket lands, tray rails, and latch/sensor blocks. |
| P1 | Clean support pod shell | Initial CAD exists in `clean_support_pod_shell`; models the support pod around the isolator with modular panels, personnel/material ante zones, DP stations, HEPA placeholders, utility trench, VHP/RTP wall, isolator footprint, and service keepouts. |
| P1 | Inline sensor service module | Initial CAD exists in `inline_sensor_service_module`; packages pressure taps, flow cartridge, bubble optical fork, pH/DO reader bay, bypass/purge valve block, and cable relief around one sealed module service path. |
| P1 | Cassette sensor backplane | Initial CAD exists in `cassette_sensor_backplane`; gives the 20-chip cassette a dry 4x5 spring-pin/contact-pad carrier with TEER/impedance connector zone, insulation standoffs, cable exit, and fiducials. |
| P1 | Environmental utility skid | Initial CAD exists in `environmental_utility_skid`; makes gas mixing, sterile filters, humidification, pressure cascade sensing, thermal-loop service, condensate/drain routing, cylinder restraints, and rear utility bulkheads explicit outside the sterile boundary. |
| P1 | Closed-fluid-path staging and tubing organization | Initial CAD exists in `automated_media_exchange_cassette`; 20-chip cassette/shelf replaces routine manual transfer with repeatable pump/reservoir/tubing layout. |
| P1 | Chip priming fixture with tubing clips and bubble observation | Initial CAD exists in `chip_priming_tubing_fixture`; needed before real microfluidic culture trials. |
| P1 | Water bath safety revision with probe clamp and bottle rack | Initial CAD exists in `water_bath_safety_kit`; media warming needs repeatable placement and safe cable routing. |
| P2 | Pipette and sterile-tip staging organizer | Initial CAD exists in `pipette_tip_organizer`; keep as calibration/debug/rescue support only, not as intended process equipment. |
| P2 | Still-air practice pass-through tray | Good for training and nonhazardous dry runs, not for real culture. |
| P2 | Microscope chip-stage adapter | Lets LaminarForge chips be inspected repeatably on an inverted microscope. |
| P3 | `chip_incubator_v3` scale-up review | Only after the starter incubator validation data is credible. |

## Machine-Readable Manifest

Run:

```sh
cargo run --release --bin cell_culture_equipment_manifest
```

The manifest emits JSON rows for each equipment item, including disposition, readiness, culture use, CAD bins, validation gate, and notes. Keep this aligned with this document before any website publication or BOM export.

Starter sourcing research is tracked in `docs/cell_culture_starter_bom_research.md`. Treat prices and example vendors as planning inputs, not locked purchasing decisions.

Run the starter equipment STL gate after generating the listed CAD outputs:

```sh
cargo run --release --bin cell_culture_equipment_verify
```

CFD and conjugate heat-transfer work is tracked separately in `docs/cell_culture_cfd_validation_plan.md`. Treat it as an engineering validation layer for airflow and thermal behavior, not as sterility or biosafety evidence.

## Boundaries

The LaminarForge CAD set should design fixtures, enclosures, support hardware, prototype environmental chambers, and validation tooling wherever useful. What it should not do is imply that a CAD prototype is already certified for biosafety, clinical, GMP, iPSC, primary human material, or viral workflows. Those uses require higher evidence levels, documented in `docs/cell_culture_equipment_boundary_breakdown.md`.
