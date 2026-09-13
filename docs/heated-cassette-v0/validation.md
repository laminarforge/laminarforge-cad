# Commissioning and thermal acceptance

These are proposed V0 engineering acceptance criteria. They are not measured results or a biological protocol. Use water only until the hardware passes. Record failures and adjust the prototype; do not reinterpret a failed criterion as a pass.

## Electrical checks

With brick disconnected: verify each net against the terminal schedule, no short between heater conductors and metal, correct relay diode polarity, insulated unused NTC wires, correct Micro-Fit cavity continuity and retained cable jackets. Verify three-wire mains cord continuity without opening the power brick.

Power with heaters disconnected. Measure inlet polarity, adjust DDR output to 13.0 V, verify both PID supplies and confirm STOP/RESET operation. With RESET released, loss/restoration of power must leave each heater relay off. Opening either thermostat loop must independently drop its relay and keep it off after reclosure until RESET.

Configure both PIDs as specified in the electrical schedule, heating mode, Pt100 input and zero minimum output. Start at 37.0 C and 50% maximum duty. Simulate each RTD lead opening separately and verify output switches off. A valid-looking sensor short may escape this diagnostic; the separate thermostat remains necessary.

Connect each heater separately, then both. Accept measured combined steady input <=6.2 A, each heater resistance >=9.0 ohms and no terminal/connector heating outside component ratings. Verify control fuse survives repeated normal starts. The chosen current-limited supply does not guarantee fuse selectivity; inspect the protection behavior in a controlled fixture before unattended operation.

Measure mounted cutoff action in a supervised nonbiological fixture. The 67L050 has a 50 +/-5 C nominal trip band plus mounting lag; it does not cap samples at precisely 50 C. Demonstrate that the relay removes power when the thermostat opens even if the SSR command remains on. Holding RESET defeats the ordinary restart latch; label and release it after use.

## Temperature measurements

Use small calibrated immersion probes and an independent logger/reference system with combined uncertainty <=0.2 C near 37 C. Purchase or borrow an instrument meeting that requirement; the two control RTDs are not independent validation sensors. Fine-wire calibrated thermocouples or miniature RTDs can be used if their complete probe/logger calibration supports the uncertainty. Do not claim that a Class A label alone establishes system accuracy.

Place at least five probes in representative wells: four corners and center. Prefer nine positions including edge middles. Probe tips must be immersed, not touching aluminum or plastic bottoms; strain-relieve thin leads. Use a sacrificial lid with small probe passages and close gaps with removable tape. Route wires so they do not defeat the front gasket or get cut by the drawer. Account for altered evaporation/heat loss from instrumentation.

Route the fine probe wires through the dedicated M5 port at X=0, Z=25 in the moving front flange. Remove its M5x8 plug and insert the 10 mm PTFE sleeve to protect wires from threads. Lightly pack the remaining gap with removable silicone foam. Leave at least 200 mm external slack to the logger and secure the wires to the moving flange; the logger stays stationary. Refit the plug flush when unused. This is a draft-restricted instrument port, not a hermetic feedthrough.

Fill all wells consistently with 150 microliters of room-temperature water, or another recorded representative working volume within the vendor's well capacity. Record plate/lid model, fill volume, probe position/depth, ambient temperature and airflow. The cheap plate has different fluid paths/contact from a future tissue plate; repeat validation for that plate later.

## Test sequence

1. Cold start at a stable indoor ambient between 20 and 30 C. Record both metal readings and all fluid probes at least once per second. Monitor for overshoot. Do not infer readiness from the aluminum reaching its setpoint.
2. Hold until fluid drift is below 0.1 C over 10 minutes, then record a further 60 minutes. Proposed acceptance: every measured fluid location 36.5-37.5 C and simultaneous hottest-minus-coldest spread <=0.5 C.
3. Open the drawer fully for 30 seconds, close gently to the hard stops, and record recovery. Proposed recovery criterion: return to the same fluid acceptance band within 10 minutes without any location exceeding 38.0 C. Record actual results even if these targets need redesign.
4. Repeat five open/close cycles. Verify gasket contact, cable bend/clearance, no jacket slipping and no guide binding while warm. Complete 100 unpowered mechanical cycles as an initial assembly screen; this is not production life qualification.
5. Repeat at the intended minimum/maximum room conditions and intended plate loading. Weigh the lidded plate before/after a representative hold to quantify evaporation; do not treat this as humidity control.

Change one thermal setting at a time. The zone setpoints may need to differ from 37.0 C to produce 37.0 C water. Calibrate the control readings with an independent reference before changing offsets. If gradients remain, investigate probe placement, plate contact, airflow and removable exterior insulation before changing heater power. More power alone does not correct spatial nonuniformity.

## Calculation interpretation

The supplied finite-volume sheet model sweeps aluminum conductivity 150/160/175 W/(m K) and assumed effective heat-loss coefficients 5/12/20 W/(m2 K). It includes roof-side losses as edge sinks and controls the rear sensor site to 37 C at 22 C ambient. Inspect thermal-sizing.json and the temperature CSV maps for actual generated values.

At k=160, the initial model estimates about 0.20-0.80 C metal spread over the drawer's plate footprint and 0.40-1.56 C at the roof, across the assumed loss cases. This is a reason to map real water temperatures and room drafts; it is not evidence that the fluid uniformity target is already achieved. The heater capacity has substantial nominal steady-state margin in this simplified model, but unmodeled losses and startup loads remain.

Density 2700 kg/m3 and approximate specific heat 930 J/(kg K) are declared sizing inputs. Kaiser gives conductivity 167 W/(m K) at 20 C and specific heat 896 J/(kg K) at 100 C; NIST's published 6061 fit ends at 300 K. Neither is an exact 37 C property measurement. The model's conductivity range covers nearby published values. Sources: https://online.kaiseraluminum.com/depot/PublicProductInformation/Document/1015/Kaiser_Aluminum_6061_Sheet_Coil_and_Plate.pdf and https://www.nist.gov/mml/acmd/aluminum-6061-t6-uns-aa96061 .

## Commissioning record

Record assembly serial/revision and source manifest; plate/lid lots; actual gasket t_min/t_max, groove depth, adhesive thickness and shim stack; guide thickness/play; screw seating checks; heater cold resistance and on-current; DC supply voltages; RTD calibration and controller settings; cutoff trip observations; fault-test results; water volumes/probe coordinates; raw temperature CSVs; ambient conditions; recovery times; evaporation and mechanical-cycle findings.

Only after successful hardware testing should a separate plate-specific culture-environment validation be planned. Media, gas exchange, evaporation, sterility and tissue protocols remain separate from the enclosure's thermal test.
