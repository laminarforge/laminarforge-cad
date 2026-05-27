# Tissue-Chip Reproducibility Controls

This note maps the main tissue-on-chip reproducibility risks to LaminarForge design controls. The goal is not to pretend the biology is easy. The goal is to make every known source of drift measurable, constrained, or explicitly blocked before live-cell work depends on it.

## Summary

The core strategy is:

1. Automate the repetitive handling steps.
2. Keep the sterile fluid path closed or semi-closed.
3. Add sensors where manual observation is too slow or subjective.
4. Validate hardware with no-cell and media-only runs before cells.
5. Move from single-chip biological pilots to multi-chip repeatability only after the engineering data is stable.

## Risk-To-Control Matrix

| Reproducibility risk | Design controls | Validation gates | Open design work |
| --- | --- | --- | --- |
| Temperature drift | CO2 incubator thermal design, independent logger enclosure, cassette-level logger routing, cassette storage/recovery rack, thermal simulation, door-recovery tests | Warmup time, steady-state variation, door-open recovery, overnight hold against independent probe, mapped rack slot gradients | Cassette-level probe mount and thermal mapping fixture |
| CO2 drift | External CO2 service module, rear gas/sample ports, independent CO2 logging, environmental sensor calibration station, controller-vs-logger agreement | 5% CO2 hold, door-open recovery, sensor calibration check, alarm on disagreement | Final 0-20% NDIR sensor mount and gas routing |
| Humidity and evaporation | Humidity tray, RH logging, sealed reservoirs/tubing, minimized open handling | RH hold, evaporation mass-loss test, media osmolality/conductivity drift check | Cassette humidity/evaporation mapping |
| pH drift | CO2 control, sealed media path, planned pH sensing or sampled media loop | Media pH trend against reference, CO2/pH recovery after media exchange | Inline pH/O2 service module |
| Oxygen drift | Planned dissolved oxygen sensing, controlled flow, bubble management, oxygen-aware chip validation, environmental O2 probe checks | DO calibration, hypoxia/recovery test, per-chip oxygen trend where relevant | Sensorized chip or inline DO module |
| Flow instability | Syringe pump, media reservoir, sterile tubing harness, cassette row rails, seeding distribution manifold, inline media QC, pressure/flow sensors | Per-row flow calibration, pressure map, pump command-vs-output map, long-run drift test | Inline sensor service module with pressure/flow ports |
| Bubbles | Tubing harness bubble windows, priming fixture, row manifold visibility, bubble-management module, bubble-clearing validation | Priming time, bubble clearance test, bubble introduction/recovery test | Dedicated bubble trap/debubbler module |
| Leaks | Cassette bench nest leak tray, drain corner, reusable dry carrier with disposable wetted path, fluid-path integrity tester | Pressure hold, pressure/vacuum decay, dyed-water leak test, leak sensor response test | Leak sensor well and electronics |
| Dead volume | Short row/branch paths, replaceable harness, manifold volume accounting | Flush/recovery volume measurement, carryover test, media exchange efficiency | Volume model per cassette and harness revision |
| Shear stress spikes | Controlled pump profiles, pressure sensing, flow calibration before cells | Ramp-rate test, max pressure/flow limits, no-cell shear surrogate test | Flow model linked to chip channel geometry |
| Contamination | Closed/semi-closed fluid path, sterile disposable tubing, bought sterile connectors/filters, aseptic tubing weld/seal prep, cleaning/sanitization validation cart, no routine pipetting, closed shuttle transfer, pre-run integrity tester | Media-only sterility hold, contamination observation, connector handling qualification, tubing weld/seal qualification, cleaning-cycle evidence, shuttle/airlock transfer qualification, pressure-decay pass | SOP and connector selection; facility approval |
| Cell suspension input drift | Closed harvest/passaging source, closed source-to-seeding path, gentle mixing/recirculation, viable-cell concentration adjustment, at-line count/viability QC, standardized tubing/connectors | Homogeneity CV, viable concentration accuracy, viability loss, recovery, dead-volume, bubble, leak, and volume-balance checks | Cell suspension prep/QC module and upstream processor interface |
| Seeding variability | Automation-first seeding plan, cassette datum, chip fixtures, randomized loading tray, 20-way distribution manifold, future sensor/image checks | Cell suspension mixing check, volume accuracy, seeding uniformity imaging/signal check, assignment-to-position reconciliation | Seeding manifold or controlled cell-loading module |
| Chip position effects | Fixed 20-chip cassette geometry, bench nest datum, row/position tracking, environmental mapping surrogate, randomized tray | Position map for temp/flow/pH/DO, randomized chip assignment, edge-position analysis | Per-position calibration metadata and analysis tooling |
| ECM/coating variability | Standardized coating protocol, timed automation, controlled incubation, documented lot IDs, automated ECM/coating QC station with wetness and fluorescent witness coupons | Coating volume/timing check, surface wetting check, fluorescent witness CV, lot-to-lot comparison | Coating witness acceptance thresholds and protocol record |
| Media exchange inconsistency | Sterile tubing harness, pump-controlled media exchange, reservoir tracking, no routine manual pipetting | Delivered volume, residual/carryover, timing repeatability, media recovery check | Closed media exchange recipe and calibration tool |
| Connector topology errors | Physical scan station, connector comb, pump/valve map plate, leak-test port bar, reject/mismatch pocket, EBR connector topology record | Planned-vs-actual connector map reconciliation, duplicate/missing ID fail, wrong-port fail, leak/prime test pass | Connector topology scan station and eBR topology schema |
| Passage number / cell lot drift | Closed harvest/passaging module, run metadata, lot/passaging log, acceptance gates before chip seeding | Passage range lock, viability/count acceptance, harvest recovery, lot comparison controls | Data model and inventory/run tracker |
| Media/reagent lot drift | Physical quarantine/released segregation, closed reagent thaw/equilibration station, run-record material scan station, lot scan, temperature-zone placeholders, QC sampling drawer, cold-chain metadata, and release status | Incoming/released lot reconciliation, thaw/equilibration temperature trace, storage excursion check, COA/sterility record review, opened-use timer, and lot-to-run traceability | Media/reagent quarantine pod, thaw/equilibration station, material scan station, and inventory release records |
| Timing drift | Automated schedules, event logs, controller timestamps, sensor timestamps | Media-change timing accuracy, incubation duration audit, missed-event alarms | Controller software and run manifest |
| Insufficient drift measurement | Multimodal monitoring design: flow/pressure, temp/CO2/RH, TEER/impedance, pH/O2, metabolites, closed sample fraction archive, imaging triage, calibration station, and run-record material scans | Sensor calibration, missing-data fail, sample custody/data traceability, material identity reconciliation, image/sensor disagreement review | Sensor backplane, inline service module, media sampling/archive interface, imaging station, eBR material model |
| Poor repeatability evidence | Staged validation: no-cell, media-only, simple cell line, single-chip pilot, multi-chip pilot, batch/operator study | Predefined acceptance criteria, repeated runs, negative/positive controls, operator/batch records, immutable run/audit records | Formal validation plan, electronic batch-record schema, and analysis scripts |

## Quantitative Acceptance Targets

Use these as initial engineering gates for CAD modules and test fixtures. They are not final biological release criteria, but they keep design reviews tied to measurable behavior.

| Workflow | Initial acceptance target |
| --- | --- |
| Automated seeding | Mean cell density within +/-10% of target, ROI coefficient of variation <=15-20%, edge:center density ratio 0.8-1.2, and at least 90% of lanes passing before scale-up. |
| Closed upstream harvest/passaging | Wash/enzyme/quench/harvest volumes within +/-5-10%, harvest recovery >=70-80% early target, viability loss <=5 percentage points, passage/lot limits enforced, enzyme dwell recorded, and closed wash/concentrate interface volume balance passing. |
| Seeding distribution manifold | All 20 outlets prime, volume CV <=10%, branch pressure/shear pockets read within limits, no downstream bubble events, isolation/prime valves reconcile, and calibration coupons pass before cell loading. |
| First perfusion after seeding | Retained cells >=85-90% of post-attach count after the first ramp, with no dry regions, detached sheets, or cross-lane tracer/cell leakage. |
| ECM/coating | Coating volume within +/-5%, dwell time within +/-2%, fluorescent coating witness CV <=15%, no dry exposure longer than 30 seconds unless validated, and flow resistance shift <=15% from baseline. |
| Automated ECM/coating QC station | All coating cartridge sockets scan to expected lot, row dispense/recirculation/prime lanes pass, 20 wetness witness pockets remain wet for the intended dwell, fluorescent witness coupons read within CV target, bubble/degas waste path clears, and barcode/lot lands reconcile before seeding. |
| Closed fluid path | No visible leaks and pressure decay <=5% over 10 minutes at 2x maximum operating pressure, or an equivalent integrity test for the selected disposable assembly. |
| Sterile fluid-path integrity tester | Every lane isolation valve actuates, pressure/vacuum decay passes, bubble/wetness optics respond to challenge, sterile filter/vent fit checks pass, waste/decon diversion routes correctly, and barcode/lot lands reconcile to the run record. |
| Aseptic tubing weld/seal prep | Bought welder/sealer fit checks pass, selected tubing OD fits staging lanes, weld/seal cycle metadata attaches to lot record, offcuts are segregated, leak-test handoff ports pass, and connector cap staging remains clean/used separated. |
| Closed reagent thaw/equilibration | Frozen bag/vial receiving nests fit, dry thaw/equilibration chambers hold target profiles, sterile connector bulkhead ports and vent/filter placeholders fit, inline temperature pockets read, barcode/lot lands reconcile, leak tray remains dry, and handoff docks connect to media conditioning/seeding/passaging modules. |
| Closed sample fraction archive | All 96 SBS archive positions are addressable, cold block reaches target, sample-loop handoff lanes prime/flush, cap/seal staging reconciles, chain-of-custody slots scan, leak tray remains dry, and archived fraction IDs attach to the run record. |
| Sterility transfer | Media-fill or media-only hold for 7-14 days with no turbidity/CFU growth, waste backflow/siphon tests passing, and microbial-ingress challenge planning for single-use assemblies. |
| Perfusion control | Lane flow within +/-5% in single-lane mode and +/-10% in multiplex mode, wall shear stress within +/-15% of target, and operating pressure <=50% of verified leak/burst threshold. |
| Perfusion bubble management | No bubble detector events at chip inlets during challenge, upstream debubbler clears introduced bubbles, relief bypass opens below validated limits, equal-length datum comb preserves row routing, and waste diversion catches purged fluid. |
| Inline media conditioning/QC | Media temperature reaches 37 +/-0.3 C before cassette handoff, pH/DO/conductivity pockets calibrate, pressure/flow block agrees with reference, filter and degasser fit checks pass, and bypass/waste routing clears challenge fluid. |
| Pressure-driven perfusion | Purchased controller zero/span checks pass, each reservoir headspace path holds pressure, occluding one lane changes remaining lane flow <=10% in multiplex tests, relief valves open below validated hardware limits, and pressure sensors remain within calibration drift limits over a no-cell endurance run. |
| No-cell flow validation | Known restriction coupons produce the expected pressure order, row flow CV <=10%, row pressure-map repeatability within +/-5%, leak-witness response passes, and bubble challenge produces no downstream detector events. |
| Workcell calibration standards | Required flow, pressure/leak, TEER phantom, pH/DO/O2, imaging, and environmental logger standards present, scanned, released, unexpired, in calibration, and recorded with as-found/as-left results. |
| Environmental sensor calibration | Temp/RH/CO2/O2 probe docks fit, reference gas and saturated-salt RH standard checks pass, logger slots and flow/pressure references scan to current certificates, clean/used standards remain segregated, leak tray remains dry, and as-found/as-left results attach to the run record. |
| Cassette storage/recovery rack | Six sealed cassette slots fit, humid-air gaps remain unobstructed, condensate paths drain to the tray, barcode and 120 position lands scan, logger pockets hold references, edge/center mapping targets reconcile, and transfer tray datum repeats before live cassette storage. |
| Closed reagent thaw/equilibration | Frozen bag/vial nests fit, thaw/equilibration blocks reach target temperature profile, eight sterile connector ports route correctly, inline temperature pockets agree with references, barcode/lot lands reconcile, leak tray stays dry, and handoff docks match media conditioning/seeding/passaging receivers. |
| Closed sample fraction archive | All 96 archive positions scan, cold block holds target storage temperature, eight closed sample-loop handoff lanes prime and flush, cap/seal staging remains segregated, chain-of-custody slots reconcile, barcode/lot/time lands attach to the run record, and leak/robot keepouts pass. |
| Waste/decon service packaging | Secondary containment captures worst-case local spill, level/overflow interlocks trip, vent/filter fit checks pass, drain routing holds pressure/leak tests, and waste backflow/siphon tests pass before live media waste. |
| Closed cleaning/sanitization validation | Clean/rinse reservoirs and waste returns route correctly, flush/return ports connect to the intended module, validation coupon drawers scan, VHP/H2O2 service clearances are unobstructed, clean/dirty segregation remains physical, and cycle barcode/sensor evidence attaches to the cleaning record. |
| Incubator/isolator transfer | Vendor sterile boundary/interlock checks pass, sealed tray leak response passes, door interlock logic verified, transfer thermal drop mapped, condensate drain checks pass, and barcode/datum handoff reconciles to the run record. |
| Cassette storage/recovery rack | Six sealed cassette slots fit, humid-air gaps remain open, condensate/drip controls route to the tray, barcode and 20-position lands reconcile per slot, logger pockets and edge/center map targets scan, and transfer-tray/robot keepouts clear. |
| Robotic cassette handling | Gripper pad symmetry verified, force/torque limits set, datum receiver engagement repeatable, leak-tray clearance maintained, scan window readable, strain-relief bend radius preserved, and collision keepout checked. |
| Robot tool changes | Tool ID scan/RFID matches expected recipe, clean and used tool lanes remain segregated, force/torque datum check passes, VHP/cleanability clearances are unobstructed, and collision/service keepouts are verified. |
| Cassette randomization/loading | Twenty assignment positions reconcile to chip/cartridge IDs, clean/used segregation is maintained, rejected-chip pocket handling is recorded, robot pick envelopes clear, and barcode/lot lands scan into the run record. |
| Run-record material scan | Reagent, media, chip, and connector lots scan; released/quarantine lanes remain segregated; calibration standards reconcile; mismatch pocket blocks nonconforming material; load-cell placeholder and camera/illumination scan check pass; and accepted material IDs attach to the batch record. |
| Cleaning/sanitization validation cart | Clean and rinse reservoirs, closed waste returns, validation drawers, VHP/H2O2 clearances, flush/return ports, clean/dirty bulkhead, barcode cycle lands, sensor pockets, leak tray, and robot/service keepouts pass before cleaning evidence is accepted. |
| Environmental cassette mapping | All 20 positions mapped with edge/center labels, temperature/CO2/RH/O2/pressure sensor pockets present, dummy flow channels clear, cable strain relief holds, and position-map metadata attaches to the run record before live chips. |
| Environmental conditioning | Chip temperature 37 +/-0.3 C, pH within +/-0.05-0.10 of target, O2 within the model target band, reservoir mass loss <=2% over 72 hours, gas/humidity panel relief/exhaust checks pass, and no bubbles at chip outlet under challenge flow. |
| Cell suspension prep/QC | Reservoir and first/middle/last dose concentration CV <=10%, final viable-cell concentration within +/-10%, viability loss <=5 percentage points, final viability >=85% unless justified, viable cells/chip within +/-10-15%, count CV <=10%, viability repeatability within +/-5 percentage points, recovery >=70-80% early target, dead volume <=one chip dose or <=10% of formulated volume, and zero bubble detector events downstream. |

## Engineering Gates Before Live Cells

Do not start live-cell tissue-chip experiments until these pass:

1. CAD outputs and fit checks for cassette, sterile harness, and bench nest.
2. Water or media-equivalent flow test across every row and chip branch.
3. Pressure/leak test with dyed fluid.
4. Bubble priming and bubble recovery test.
5. Environmental hold test with independent logger.
6. Media-only sterility hold in the closed path.
7. Full run logging: commands, sensor values, alarms, and event timestamps.
8. Cell suspension prep/QC run with viable concentration, homogeneity, recovery, dead-volume, bubble, and volume-balance gates passing before seeding live chips.

## Biological Scale-Up Gates

After engineering gates pass, scale biology in this order:

1. Non-cell media-only endurance run.
2. Simple robust adherent cell line in one chip.
3. Same cell line across a small subset of cassette positions.
4. Full 20-chip cassette with randomized positions.
5. Repeat across days.
6. Repeat across cell lots and passages.
7. Repeat across operators.
8. Only then move to fragile tissue, iPSC-derived, primary, or complex co-culture work.

## What The Hardware Solves

The current LaminarForge hardware direction reduces manual variability:

- `automated_media_exchange_cassette`: fixed 20-chip geometry and row media rails.
- `closed_cell_harvest_passaging_module`: closed source-vessel dock, wash/enzyme/quench/media/flush/collection inputs, pump/valve bank, dissociation rocker, harvest collection/QC loop, bought wash/concentrate interface, waste/decon diversion, lot labels, and service keepouts for upstream passaging control.
- `cell_suspension_prep_qc_module`: closed bag holder, gentle mixing/recirculation, temperature hold, count/viability QC loop, bubble/dead-volume block, sterile connector panel, prime/waste tray, and row handoff manifold before cassette seeding.
- `automated_cell_seeding_distribution_manifold`: single-use cassette datum, reservoir/mixer interface, equalized 20-way outlets, isolation/prime ports, bubble trap/degas return, pressure/shear pockets, calibration coupons, and robot keepout for measured seeding distribution.
- `automated_seeding_coating_station`: constrained cassette seeding/coating deck with reservoir/mixer placeholders, degassing, priming, pump/valve packaging, pressure/flow/bubble sensing, waste routing, and service clearances.
- `automated_ecm_coating_qc_station`: coating cartridge sockets, row dispense/recirculation/prime lanes, timed wetness witness pockets, fluorescent witness slide/coupon handling, bubble/degas waste path, barcode/lot lands, and robot keepouts for coating evidence before seeding.
- `inline_media_conditioning_qc_module`: water-jacket equilibration, filter holder, membrane degasser placeholder, pH/DO/conductivity pockets, pressure/flow block, sampling loop, calibration ports, bypass/waste path, and five-row cassette handoff.
- `media_conditioning_perfusion_rack`: media bag, warming, degassing, pump, valve/filter, pressure relief, waste, and service-panel packaging for no-cell conditioning/perfusion validation.
- `perfusion_bubble_management_module`: sterile interface manifold, upstream debubbler chambers, optical bubble forks, relief bypass, equal-length tubing comb, low-dead-volume valve area, waste diversion, leak tray, and robot keepouts.
- `pressure_driven_perfusion_panel`: bought pressure-controller packaging, reservoir headspace ports, vacuum reference, sterile gas filters, relief valves, pressure sensors, calibration ports, and strain relief for multiplexed low-shear perfusion.
- `flow_pressure_validation_fixture`: no-cell dummy coupon carrier, row manifold tree, bubble challenge insert, pressure sensor bar, and leak witness tray for pump/manifold/sensor qualification before live culture.
- `sterile_fluid_path_integrity_tester`: 20-lane cassette datum, pressure/vacuum source panel, isolation valve bank, pressure-decay sensor matrix, bubble/wetness optics, sterile filter/vent bank, waste/decon diverter, barcode lands, leak witness tray, and robot keepouts.
- `aseptic_tubing_weld_seal_station`: bought sterile tube welder/sealer envelopes, tubing spool/cassette datum, cut/weld/seal lanes, connector cap staging, leak-test handoff ports, clean/used segregation, offcut/waste trays, lot labels, and cleanability/robot keepouts.
- `closed_reagent_thaw_equilibration_station`: dry thaw/equilibration chambers, frozen bag/vial receiving nest, sterile connector bulkhead, inline temperature pockets, barcode/lot lands, handoff docks, leak tray, and robot/service keepouts.
- `media_sampling_analyzer_interface`: lane selectors, sample loops, flush/waste routing, cold-block fraction collection, analyzer dock, sterile bulkhead, and bubble/dead-volume controls for closed media sampling.
- `closed_sample_fraction_archive_module`: chilled SBS 96-position archive nest, cold block, sterile sample-loop handoff, flush/waste manifold, cap/seal staging, barcode/lot/time lands, chain-of-custody slots, leak tray, and robot keepouts.
- `chip_cassette_position_randomization_tray`: physical randomized assignment slots, clean chip/cartridge staging, barcode/lot label lands, reject pocket, fiducials, robot pick clearances, and clean/used segregation to separate biological effects from cassette position bias.
- `environmental_mapping_cassette_surrogate`: no-cell 4x5 cassette surrogate with sensor pockets, dummy flow channels, cable strain relief, calibration label lands, humidity shields, robot datum features, and edge/center markers for position-effect mapping.
- `cassette_storage_recovery_incubator_rack`: six sealed cassette slots, humid-air spacing, condensate controls, barcode/position lands, environmental logger pockets, edge/center mapping targets, transfer-tray interface, and robot/service keepouts.
- `sterile_tubing_harness`: disposable/replaceable fluid path and branch strain relief.
- `cassette_bench_nest`: repeatable datum, tube clearance, leak tray, drain corner, and fiducials.
- `sealed_culture_module`: closed cassette envelope, gasketed lid, thermal plate, and rear service bulkhead.
- `sealed_module_docking_bay`: repeatable pod-side receiver with leak capture, datum rails, latch/presence features, and standardized service coupler region.
- `culture_module_service_skid`: service-side pump/reservoir/waste/sensor packaging outside the culture module.
- `closed_isolator_workcell`, `aseptic_transfer_hatch`, and `clean_support_pod_shell`: sterile boundary, transfer architecture, pressure/HEPA/VHP placeholders, and support-pod space planning.
- `incubator_cassette_shuttle_airlock`: sealed cassette tray, dual-door interlock envelope, thermal buffer, humidity/condensation drains, HEPA/VHP ports, scan/datum handoff, and robot/service keepouts.
- `sterile_consumable_cartridge_hotel`: segregated clean/used consumable staging, transfer-tray datum, lot/barcode lands, service clearances, and VHP/UV exposure clearance placeholders.
- `media_reagent_quarantine_pod`: receiving shelf, quarantine/released bays, temperature-zone placeholders, barcode/QC station, sampling drawers, thermal buffers, spill/waste tray, and pressure/HEPA/VHP clearances.
- `workcell_calibration_drawer`: physical staging for flow restrictors, pressure/leak adapters, TEER phantoms, pH/DO/O2 standards, imaging targets, logger holders, clean/used segregation, and lot labels.
- `environmental_sensor_calibration_station`: temp/RH/CO2/O2 probe docks, reference gas manifold, saturated-salt RH standard pockets, logger rack, flow/pressure reference panel, thermal block, leak tray, clean/used segregation, and certificate barcode lands.
- `environmental_utility_skid`: external gas, humidification, pressure cascade, thermal loop, drain, and cylinder service routing.
- `gas_humidity_service_panel`: CO2/O2/N2/air channel service panel, regulator/MFC envelopes, culture-box feeds, humidifier cradle, condenser trap, sterile vent filters, relief/exhaust path, sensor/calibration ports, and isolation cover.
- `inline_sensor_service_module`: pressure, flow, bubble, pH/DO, bypass, purge, and cable routing around a sealed module path.
- `run_record_material_scan_station`: barcode/RFID scanner bridge, material lot staging pockets, released/quarantine lanes, calibration standard scan pockets, mismatch reject pocket, load-cell placeholder, camera/illumination bar, and robot keepouts.
- `connector_topology_scan_station`: physical 20-chip connector topology mapping, 80 connector ID slots, 24 pump/valve channel map, 20 leak-test placeholders, scan bridge, and mismatch quarantine pocket.
- `robotic_cassette_gripper_end_effector`: cassette gripper fingers, compliant pads, datum receivers, robot wrist plate, force/torque placeholder, scan window, leak-tray clearance, strain relief, cleanable covers, and collision keepout.
- `robot_tool_change_and_end_effector_rack`: clean tool pockets, used-tool quarantine saddles, barcode/RFID tool ID lands, force/torque datums, VHP/cleanability keepouts, drip tray, and robot collision/service envelopes for contained tool changes.
- Electronic batch-record traceability: immutable run, recipe, material, connector-topology, calibration, sample, image, sensor-stream, alarm, deviation, and audit records are required before scaled repeatability claims.
- `cassette_sensor_backplane`: dry cassette-level electrical access for TEER/impedance without manual probing.
- `automated_culture_imaging_module`: repeatable dark/clean cassette imaging envelope with datum nest, XY/focus placeholders, transmitted/epi illumination, calibration target, and service panels.
- `sterility_validation_challenge_rack`: fixed coupon, media-fill surrogate, settle/contact plate, sensor, transfer-datum, and leak-witness locations for non-cell qualification.
- `waste_decon_service_pod`: secondary containment, liquid waste cassette, vent/filter placeholders, contact-time/neutralization placeholders, solid pass-out drawer, overflow interlocks, drain routing, and service panels.
- `closed_cleaning_sanitization_validation_cart`: clean/rinse reservoirs, closed waste returns, validation drawers, VHP/H2O2 clearances, flush/return ports, clean/dirty bulkhead, barcode cycle lands, sensor pockets, leak tray, and robot/service keepouts.
- `cell_culture_logger_enclosure`: independent validation logger.
- `co2_sensor_service_module`: keeps CO2 measurement/control electronics outside the humid chamber.
- `media_reservoir` and `peltier_reservoir_block`: structured reservoir and media-temperature handling.

## What Still Requires Process Validation

The hardware does not automatically solve:

- Cell-source variability.
- ECM lot variability.
- Seeding biology.
- Long-term phenotype stability.
- Contamination risk from poor technique.
- Biological interpretation of sensor data.
- Cross-lab reproducibility.

Those require SOPs, controls, run records, repeated experiments, and review by someone competent in the cell type and assay.

For upstream cell preparation, the likely product architecture is hybrid: buy validated closed wash/concentrate and count/viability hardware where practical, then build LaminarForge-specific low-volume formulation, gentle mixing, final seeding manifold, trace capture, and acceptance-gate tooling. Open liquid handlers inside a BSC are useful for exploration, but they should not become the target production path for no-manual-liquid-handling culture.

## Design Rule

Every future component should answer at least one of these questions:

- What source of variability does this remove?
- What drift does this measure?
- What failure does this make visible?
- What manual step does this eliminate?
- What validation gate does this enable?

If a part does not answer any of those, it is probably decoration or premature complexity.
