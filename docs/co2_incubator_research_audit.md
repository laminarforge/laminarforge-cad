# CO2 Incubator Research Audit

Status: not website-ready.

This audit records what needs to be true before the LaminarForge starter CO2 incubator should be treated as a public build instead of an internal engineering prototype.

## Research Findings

### Published DIY incubators can work, but they validate heavily

The PrintrLab incubator paper describes a low-cost open-source CO2 incubator that held 37 C and 5% CO2 for multi-day runs. Its important design choices differ from our first-pass CAD:

| Area | PrintrLab approach | Impact on LaminarForge |
| --- | --- | --- |
| CO2 sensor | ExplorIR-W 20% CO2 sensor | Our prior MH-Z19B-class assumption is not acceptable for 5% CO2 control. |
| Chamber | Leak-proof food container or Styrofoam chamber | The chamber was chosen for sealing first, not printability first. |
| Heating | 3D printer heated bed plus ambient thermistor | It validated chamber temperature away from the heater, not just heater temperature. |
| Gas delivery | Dry ice CO2 source, normally closed solenoid, passive relief | Positive pressure and relief behavior need explicit design, not just an inlet hole. |
| Validation | Multi-day temperature/CO2 logging and biological comparison | We need logs and acceptance criteria before public release. |

Source: Arumugam et al., "PrintrLab incubator: A portable and low-cost CO2 incubator based on an open-source 3D printer architecture," PLOS ONE, 2021. https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0251812

### Modular incubator architecture is the right direction

The 2024 HardwareX modular incubator work supports separating the control unit from interchangeable incubation chambers. That maps well to LaminarForge: the electronics/service bay should be reusable, while the chamber can evolve from starter cabinet to chip incubator.

Source: Duru et al., "A modular and flexible open source cell incubator system for mobile and stationary use," HardwareX, 2024. https://pmc.ncbi.nlm.nih.gov/articles/PMC11639333/

### Cell culture environment control is not just nominal setpoints

The best-practices literature emphasizes that nominal incubator settings are not enough. Medium pH, dissolved CO2/O2, culture geometry, door openings, and handling history affect reproducibility. A public build needs telemetry and reporting expectations, not just a CAD model.

Source: "Toward Best Practices for Controlling Mammalian Cell Culture Environments." https://pmc.ncbi.nlm.nih.gov/articles/PMC8900666/

### MH-Z19B is the wrong default CO2 sensor for this job

The MH-Z19B datasheet describes a 0-2000 ppm or 0-5000 ppm sensor. A 5% CO2 incubator target is 50,000 ppm. That is an order of magnitude above this sensor's documented range. It may be useful for room-air monitoring, not incubator control.

Source: Winsen MH-Z19B datasheet. https://www.winsen-sensor.com/d/files/infrared-gas-sensor/mh-z19b-co2-ver1_0.pdf

### Culture requirements depend on medium and cell type

Thermo Fisher's cell culture environment guidance gives the normal mammalian range as 36-37 C, notes 4-10% CO2 is common, and explains that bicarbonate-buffered media require the correct CO2 tension to maintain pH. The incubator target should therefore be tied to the medium/cell line, not hardcoded as a universal biological truth.

Source: Thermo Fisher, "Cell Culture Environment." https://www.thermofisher.com/us/en/home/references/gibco-cell-culture-basics/cell-culture-environment.html

## Corrections To Current Plan

| Prior assumption | Correction |
| --- | --- |
| MH-Z19B-class CO2 sensor is acceptable | Use an incubator-range NDIR sensor, likely 0-20% CO2. |
| Printed chamber can be the primary sealing strategy | Treat sealing as a design risk; validate printed PETG, coatings, gasket, and door leakage. |
| Passive humidity tray is enough | Keep it as v0, but add condensation management and humidity logging. |
| 40-80 W heater pad is automatically suitable | Size heater from thermal test; include independent cutoff and overtemp validation. |
| Model can go public after STL generation | Public release waits for sensor selection, safety review, and commissioning data. |

## Design Gates Before Website Publication

1. Sensor gate: choose exact CO2 sensor with 0-20% range, humidity tolerance, calibration method, and physical dimensions.
2. Gas gate: define CO2 source, regulator, solenoid, relief path, check valve/filter plan, and leak-test method.
3. Thermal gate: run empty-chamber warmup, overshoot, steady-state, and door-open recovery tests.
4. Humidity gate: log RH and condensation location for at least 24 hours.
5. Sterility gate: decide whether this is only for engineering shakeout or whether a contamination test protocol is in scope.
6. Materials gate: decide whether printed PETG is acceptable as a liner or whether a food-storage/polycarbonate/stainless liner is better.
7. Documentation gate: publish CAD only with validation limits, not as a replacement for a certified incubator or biosafety cabinet.

## Current Recommendation

Do not put the incubator models on the LaminarForge website yet.

Keep the starter cabinet as an internal prototype for:

- fit checks
- service-bay layout
- heater/fan/CO2 control development
- logging and commissioning workflow
- chip hardware shakeout without valuable cultures

For serious cell culture, prioritize access to a certified lab incubator or a used commercial CO2 incubator while this design matures.
