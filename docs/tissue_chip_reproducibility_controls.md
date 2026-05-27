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
| Temperature drift | CO2 incubator thermal design, independent logger enclosure, cassette-level logger routing, thermal simulation, door-recovery tests | Warmup time, steady-state variation, door-open recovery, overnight hold against independent probe | Cassette-level probe mount and thermal mapping fixture |
| CO2 drift | External CO2 service module, rear gas/sample ports, independent CO2 logging, controller-vs-logger agreement | 5% CO2 hold, door-open recovery, sensor calibration check, alarm on disagreement | Final 0-20% NDIR sensor mount and gas routing |
| Humidity and evaporation | Humidity tray, RH logging, sealed reservoirs/tubing, minimized open handling | RH hold, evaporation mass-loss test, media osmolality/conductivity drift check | Cassette humidity/evaporation mapping |
| pH drift | CO2 control, sealed media path, planned pH sensing or sampled media loop | Media pH trend against reference, CO2/pH recovery after media exchange | Inline pH/O2 service module |
| Oxygen drift | Planned dissolved oxygen sensing, controlled flow, bubble management, oxygen-aware chip validation | DO calibration, hypoxia/recovery test, per-chip oxygen trend where relevant | Sensorized chip or inline DO module |
| Flow instability | Syringe pump, media reservoir, sterile tubing harness, cassette row rails, future pressure/flow sensors | Per-row flow calibration, pressure map, pump command-vs-output map, long-run drift test | Inline sensor service module with pressure/flow ports |
| Bubbles | Tubing harness bubble windows, priming fixture, row manifold visibility, bubble-clearing validation | Priming time, bubble clearance test, bubble introduction/recovery test | Dedicated bubble trap/debubbler module |
| Leaks | Cassette bench nest leak tray, drain corner, reusable dry carrier with disposable wetted path | Pressure hold, dyed-water leak test, leak sensor response test | Leak sensor well and electronics |
| Dead volume | Short row/branch paths, replaceable harness, manifold volume accounting | Flush/recovery volume measurement, carryover test, media exchange efficiency | Volume model per cassette and harness revision |
| Shear stress spikes | Controlled pump profiles, pressure sensing, flow calibration before cells | Ramp-rate test, max pressure/flow limits, no-cell shear surrogate test | Flow model linked to chip channel geometry |
| Contamination | Closed/semi-closed fluid path, sterile disposable tubing, bought sterile connectors/filters, no routine pipetting | Media-only sterility hold, contamination observation, connector handling qualification | SOP and connector selection; facility approval |
| Seeding variability | Automation-first seeding plan, cassette datum, chip fixtures, future sensor/image checks | Cell suspension mixing check, volume accuracy, seeding uniformity imaging/signal check | Seeding manifold or controlled cell-loading module |
| Chip position effects | Fixed 20-chip cassette geometry, bench nest datum, row/position tracking, environmental mapping | Position map for temp/flow/pH/DO, randomized chip assignment, edge-position analysis | Per-position calibration metadata and analysis tooling |
| ECM/coating variability | Standardized coating protocol, timed automation, controlled incubation, documented lot IDs | Coating volume/timing check, surface wetting check, lot-to-lot comparison | Coating/priming fixture and protocol record |
| Media exchange inconsistency | Sterile tubing harness, pump-controlled media exchange, reservoir tracking, no routine manual pipetting | Delivered volume, residual/carryover, timing repeatability, media recovery check | Closed media exchange recipe and calibration tool |
| Passage number / cell lot drift | Run metadata, lot/passaging log, acceptance gates before chip seeding | Passage range lock, viability/count acceptance, lot comparison controls | Data model and inventory/run tracker |
| Timing drift | Automated schedules, event logs, controller timestamps, sensor timestamps | Media-change timing accuracy, incubation duration audit, missed-event alarms | Controller software and run manifest |
| Insufficient drift measurement | Multimodal monitoring design: flow/pressure, temp/CO2/RH, TEER/impedance, pH/O2, metabolites, imaging triage | Sensor calibration, missing-data fail, image/sensor disagreement review | Sensor backplane, inline service module, imaging station |
| Poor repeatability evidence | Staged validation: no-cell, media-only, simple cell line, single-chip pilot, multi-chip pilot, batch/operator study | Predefined acceptance criteria, repeated runs, negative/positive controls, operator/batch records | Formal validation plan and analysis scripts |

## Engineering Gates Before Live Cells

Do not start live-cell tissue-chip experiments until these pass:

1. CAD outputs and fit checks for cassette, sterile harness, and bench nest.
2. Water or media-equivalent flow test across every row and chip branch.
3. Pressure/leak test with dyed fluid.
4. Bubble priming and bubble recovery test.
5. Environmental hold test with independent logger.
6. Media-only sterility hold in the closed path.
7. Full run logging: commands, sensor values, alarms, and event timestamps.

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
- `sterile_tubing_harness`: disposable/replaceable fluid path and branch strain relief.
- `cassette_bench_nest`: repeatable datum, tube clearance, leak tray, drain corner, and fiducials.
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

## Design Rule

Every future component should answer at least one of these questions:

- What source of variability does this remove?
- What drift does this measure?
- What failure does this make visible?
- What manual step does this eliminate?
- What validation gate does this enable?

If a part does not answer any of those, it is probably decoration or premature complexity.
