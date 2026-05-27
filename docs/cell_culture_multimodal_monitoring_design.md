# Cell Culture Multimodal Monitoring Design

This note records the monitoring architecture for the LaminarForge automated cell-culture stack. Visual inspection is useful, but it must not be the only way the system understands culture state. The intended design is multimodal: fluidic, electrical, chemical, metabolic, environmental, and visual signals are combined into a time-series state model.

## Design Position

The culture automation stack should treat microscopy as a confirmation and triage layer, not as the sole measurement path. A healthy system needs direct instrument signals for:

- Whether fluid is moving correctly.
- Whether the chip is leaking, blocked, bubbled, or drying out.
- Whether the incubator and cassette environment are valid.
- Whether the culture microenvironment is drifting.
- Whether cells or barriers are changing electrically.
- Whether morphology agrees with the non-visual data.

No single signal should make a biological claim by itself. The system should flag anomalies and require validation/human review before any claim about viability, identity, contamination, differentiation, or release quality.

## Signal Stack

| Signal layer | Primary signals | What it detects | First hardware direction |
| --- | --- | --- | --- |
| Fluidic health | Inlet/outlet pressure, row flow, pump command vs response | Blocked channel, leak, bubble, pump failure, changing resistance | Inline pressure/flow service module on cassette manifold |
| Environment | Temperature, CO2, RH, incubator door/recovery state | Invalid culture environment, evaporation risk, controller failure | Independent logger plus cassette-level probe routing |
| Electrical cell state | TEER, impedance spectroscopy, ECIS-like readings | Barrier integrity, attachment, spreading, detachment, confluence-like trends | Sensor backplane with pogo/spring contacts to electrode pads |
| Chemical microenvironment | pH, dissolved oxygen, conductivity | Acidification, hypoxia, CO2/bicarbonate mismatch, media drift | Optical sensor spots or inline electrochemical service module |
| Metabolic state | Glucose, lactate, optional sampled analytes | Nutrient consumption, waste production, culture rate shifts | Automated sampling loop or small external analyzer integration |
| Visual confirmation | Brightfield/phase-like images, focus/exposure metrics | Morphology, bubbles, debris, gross contamination-like changes | Cassette imaging station or gantry tied to fiducials |

## Priority Build Order

1. Add pressure, flow, and leak sensing around the cassette and bench nest.
2. Add cassette-level environmental logging so the controller is not grading itself.
3. Add a dry `cassette_sensor_backplane` concept for reusable electrical contact.
4. Move toward a sensorized chip revision with electrode pads for TEER/impedance.
5. Add pH/O2 sensing through optical spots or an inline service module.
6. Add glucose/lactate by sampled media loop before attempting fully inline metabolite sensors.
7. Add imaging station/gantry as confirmation and training-data capture.

## Interpretation Model

The automation should keep a per-cassette and per-chip time series:

- Pump command.
- Flow and pressure.
- Temperature, CO2, RH.
- pH and dissolved oxygen where available.
- TEER/impedance values and frequency sweep metadata.
- Media exchange event logs.
- Waste/sample volumes.
- Image capture metadata and ML outputs.

The useful output is not one raw number. The useful output is a state classification:

- `normal`: all signals within validated range.
- `fluidic_fault`: pressure/flow/leak signal is abnormal.
- `environment_fault`: temp/CO2/RH/recovery is abnormal.
- `microenvironment_drift`: pH/O2/metabolic trend is abnormal.
- `cell_layer_change`: TEER/impedance trend changed beyond validated bounds.
- `visual_review_needed`: image metrics or model output disagree with sensor trend.
- `invalid_run`: missing calibration, missing sensor data, or out-of-bound control state.

## CAD and Electronics Implications

The mechanical stack should reserve space and interfaces for sensors now, even before all electronics are finalized:

- The cassette bench nest should keep leak collection and leak sensor wells accessible.
- The sterile tubing harness should keep pressure/flow sensor service points outside the sterile cassette where possible.
- The cassette should expose repeatable dry contact locations for future electrode interfaces.
- The chip format should leave room for TEER/impedance electrode pads and optional optical pH/O2 sensor windows.
- Cable routing must stay separated from wet tubing and must not interfere with robotic access.
- The imaging station should align to cassette fiducials and sensor data timestamps.

## Candidate Modules

| Module | Purpose | Notes |
| --- | --- | --- |
| `cassette_sensor_backplane` | Dry reusable pogo/spring contact interface for TEER/impedance pads | Should not touch sterile media. Needs shielding and calibration fixture. |
| `inline_sensor_service_module` | Pressure, flow, pH/O2, conductivity service block | Prefer replaceable wetted path or externally clamped sensors. |
| `cassette_imaging_station` | Fixed optical inspection tied to nest/cassette fiducials | Use for confirmation, model training, and anomaly review. |
| `sensorized_chip_revision` | Chip variant with electrode pads and sensor windows | Requires chip design update and validation gate. |
| `flow_validation_fixture` | No-cell calibration fixture for per-row/per-chip flow and pressure maps | Required before trusting live culture runs. |

## Validation Gates

- Flow/pressure calibration with known restrictions and open channels.
- Leak tray and leak sensor response test.
- TEER/impedance calibration against known resistors/capacitors and blank chips.
- pH/O2 calibration against reference standards.
- Environmental logger agreement against independent instruments.
- No-cell endurance run with media-equivalent fluid.
- Sensor/image disagreement review before automated state classifications are trusted.

## Design Boundary

These signals can support engineering control and early research monitoring. They do not, by themselves, validate biological quality, sterility, clinical safety, or regulatory readiness. Every biological interpretation must be tied to a documented validation study.
