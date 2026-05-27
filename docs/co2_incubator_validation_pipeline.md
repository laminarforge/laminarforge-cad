# CO2 Incubator Validation Pipeline

This is the validation path for the starter CO2 incubator before any public website release. It is intentionally staged: cheap deterministic checks first, CFD/FEA only after the basic model and CAD stop moving.

## Validation Layers

| Layer | Tool | Purpose | Status |
| --- | --- | --- | --- |
| CAD generation | `co2_incubator` | Generate printable STL outputs | Active |
| CAD output verification | `co2_incubator_verify` | Fail if STL files disappear, corrupt, or dimensionally drift | Active |
| Lumped thermal/CO2 simulation | `co2_incubator_sim` | First-order heater, heat-loss, CO2 dosing, leak, and door-open recovery model | Active |
| CFD/CHT | OpenFOAM first, with Gmsh/OpenFOAM meshing | Airflow, CO2 mixing, heater diffuser, thermal gradients, condensation-risk regions | Planned in `docs/cell_culture_cfd_validation_plan.md` |
| Physical validation | sensor logs | Empty-chamber and loaded-chamber data against acceptance gates | Planned |

## Current Lumped Simulation

Run:

```text
cargo run --release --bin co2_incubator_sim
```

The model simulates:

- chamber volume from the CAD interior: 300 x 250 x 250 mm, or 18.75 L
- chamber heat-transfer area from the interior dimensions
- 25 mm PIR-style insulation
- parasitic heat loss for window, seams, ports, and leakage
- 80 W maximum heater
- PI heater control
- CO2 injection from a 100% CO2 source through a normally closed solenoid
- low steady leak/exchange
- one 60-second door-open event at hour 2

Current default output:

| Metric | Result |
| --- | ---: |
| Chamber volume | 18.75 L |
| Heat transfer coefficient | 0.574 W/K |
| Warmup to 36.5 C | 18.15 min |
| Max temperature | 37.09 C |
| Final temperature | 37.00 C |
| Max CO2 | 4.94% |
| CO2 minimum after warmup | 2.29% |
| CO2 recovery after door close | 1.95 min |
| Final CO2 | 4.94% |
| Heater energy over 8 h | 79.69 Wh |
| CO2 used over 8 h | 2.31 L |
| Gate result | pass |

## Acceptance Gates

The simulator fails if:

- warmup to 36.5 C takes more than 45 minutes
- temperature overshoots above 38.0 C
- final temperature is outside 36.8-37.2 C
- CO2 overshoots above 5.6%
- final CO2 is outside 4.8-5.2%
- CO2 does not recover to 4.8-5.2% within 20 minutes after door close
- CO2 use exceeds 10 L over the 8-hour simulated run

These are engineering gates only. They do not prove biological readiness.

## What CFD Should Answer Later

Only run CFD/CHT after the sensor/gas/service-bay architecture is stable. The CFD case should answer:

- Does the fan/diffuser create dead zones?
- Does CO2 injection short-circuit from inlet to relief/sample outlet?
- Are there shelf-level CO2 gradients that matter for plates/flasks?
- Are heater diffuser surfaces too warm near cultureware?
- Where is condensation likely to collect?
- Does door geometry or window heat loss create local cold zones?

The preferred first solver path is OpenFOAM `buoyantPimpleFoam` for transient buoyant airflow and heat transfer, plus passive scalar transport for CO2. Move to `chtMultiRegionFoam` only after the air-volume case is useful and the wall/door/insulation model is stable.

## Physical Validation Mapping

The physical test harness should log:

- at least four temperature probes: lower shelf, middle shelf, upper shelf, near door
- independent 0-20% CO2 logger
- RH logger
- optional pressure sensor across the chamber
- heater PWM/duty
- solenoid state
- door-open events

The first physical run should reproduce the same events as the simulator:

1. empty chamber from ambient
2. warm to 37 C
3. stabilize CO2 at 5%
4. open door for 60 seconds at hour 2
5. run for at least 8 hours

The simulator should then be calibrated against the logged heat loss, effective thermal mass, leak rate, and CO2 injection rate. CFD is useful after this calibration, not before.
