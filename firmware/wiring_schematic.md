# LaminarForge Liquid Handler -- Wiring Schematic

**Ticket:** T-F5D355C5
**Date:** 2026-03-28
**References:** OTTO liquid handler (optocoupler limit switches), MULA 4-axis architecture

---

## System Overview

The LaminarForge liquid handler uses a dual-controller architecture:

| Controller | MCU | Role | Power Rail |
|---|---|---|---|
| Motion Controller | ESP32-S3 #1 | FluidNC firmware, 4-axis stepper control | 24V / 5V / 3.3V |
| Environmental Controller | ESP32-S3 #2 | PID temp, CO2, humidity, rocker servo | 12V / 5V / 3.3V |

Communication between controllers is via a single GPIO signal line (motion-interlock) with optional UART for richer messaging.

---

## 1. Power Distribution

```
                    +-----------+
   AC Mains ------->| 24V PSU   |----+---> 24V Rail (steppers, heater pads)
                    | (350W)    |    |
                    +-----------+    |
                                     |
                              +------+------+
                              |             |
                        +-----v-----+ +-----v-----+
                        | LM2596    | | LM2596    |
                        | 24V->12V  | | 24V->5V   |
                        | Buck      | | Buck      |
                        +-----+-----+ +-----+-----+
                              |             |
                              v             v
                         12V Rail       5V Rail
                    (solenoid, fan,   (servo, ESP32
                     Peltier, CO2)    via USB or
                                       LDO -> 3.3V)
                                            |
                                      +-----v-----+
                                      | AMS1117   |
                                      | 5V->3.3V  |
                                      | LDO       |
                                      +-----+-----+
                                            |
                                            v
                                        3.3V Rail
                                   (sensors, TMC UART,
                                    ESP32 GPIO logic)
```

### Power Budget Estimates

| Load | Voltage | Max Current | Notes |
|---|---|---|---|
| 4x NEMA 17 steppers | 24V | 4.0A total | 1.0A RMS each via TMC2209 |
| 2x Silicone heater pads | 24V | 4.0A total | ~50W each |
| 2x TEC1-12706 Peltier | 12V | 12.0A total | 6A each at full drive |
| 1x 12V solenoid | 12V | 0.5A | CO2 valve |
| 1x 12V fan | 12V | 0.3A | Circulation fan |
| 1x SprintIR-6S-5 | 3.3V | 0.035A | CO2 sensor |
| 1x BME280 | 3.3V | 0.001A | Humidity/temp/pressure |
| 2x DS18B20 | 3.3V | 0.002A | Temperature probes |
| 1x MG996R servo | 5V | 2.5A peak | Rocker platform |
| 2x ESP32-S3 | 5V (USB) | 0.5A each | Via USB or 5V rail |

**Total estimated 24V draw:** ~8A continuous (16A peak with heaters + steppers)
**Recommended PSU:** 24V / 15A (360W) with headroom

### Fusing

| Circuit | Fuse Rating | Type |
|---|---|---|
| 24V main rail | 15A | Blade fuse, inline |
| 24V stepper branch | 6A | Blade fuse |
| 24V heater branch | 8A | Blade fuse |
| 12V rail (after buck) | 15A | Blade fuse (Peltier draw) |
| 5V rail (after buck) | 5A | Blade fuse |

---

## 2. Motion Controller (ESP32-S3 #1 -- FluidNC)

### 2.1 TMC2209 Stepper Drivers (UART Mode)

All four TMC2209 drivers share a single UART bus. Each driver is addressed via the MS1/MS2 pin configuration. The ESP32 communicates with each driver individually using its 2-bit address.

#### TMC2209 UART Address Configuration

| Driver | Axis | MS1 | MS2 | UART Address |
|---|---|---|---|---|
| TMC2209 #1 | X (gantry left-right) | GND | GND | 0b00 (0) |
| TMC2209 #2 | Y (gantry forward-back) | VCC_IO | GND | 0b01 (1) |
| TMC2209 #3 | Z (pipette up-down) | GND | VCC_IO | 0b10 (2) |
| TMC2209 #4 | I (syringe pump) | VCC_IO | VCC_IO | 0b11 (3) |

**MS1/MS2 Notes:**
- Directly tie MS1/MS2 to GND or VCC_IO (3.3V) -- do NOT leave floating.
- VCC_IO on TMC2209 is typically 3.3V (from the on-board regulator or external 3.3V).
- If using Bigtreetech TMC2209 modules, MS1/MS2 pads are on the bottom of the PCB.

#### UART Wiring

The TMC2209 UART is a single-wire bidirectional protocol, but for reliability we use two ESP32 GPIOs with a 1kOhm resistor on the TX line:

```
ESP32 GPIO17 (TMC_UART_TX) ---[1kOhm]---+--- TMC2209 #1 PDN_UART
                                         +--- TMC2209 #2 PDN_UART
ESP32 GPIO18 (TMC_UART_RX) -------------+--- TMC2209 #3 PDN_UART
                                         +--- TMC2209 #4 PDN_UART
```

**Alternative (single-wire):** Tie TX and RX together through a 1kOhm resistor on TX, connect all PDN_UART pins to the junction. FluidNC supports this via `uart_num: 1` in the YAML config.

#### Pin Assignment -- Motion Controller

| ESP32-S3 GPIO | Function | Connected To | Notes |
|---|---|---|---|
| GPIO1 | X_STEP | TMC2209 #1 STEP | |
| GPIO2 | X_DIR | TMC2209 #1 DIR | |
| GPIO3 | Y_STEP | TMC2209 #2 STEP | |
| GPIO4 | Y_DIR | TMC2209 #2 DIR | |
| GPIO5 | Z_STEP | TMC2209 #3 STEP | |
| GPIO6 | Z_DIR | TMC2209 #3 DIR | |
| GPIO7 | I_STEP | TMC2209 #4 STEP | Syringe pump axis |
| GPIO8 | I_DIR | TMC2209 #4 DIR | |
| GPIO9 | STEPPER_EN | All TMC2209 EN (shared) | Active LOW, shared enable |
| GPIO17 | TMC_UART_TX | All TMC2209 PDN_UART | Via 1kOhm series resistor |
| GPIO18 | TMC_UART_RX | All TMC2209 PDN_UART | Direct connection |
| GPIO10 | X_MIN_LIMIT | Optocoupler output | Active LOW with pullup |
| GPIO11 | Y_MIN_LIMIT | Optocoupler output | Active LOW with pullup |
| GPIO12 | Z_MIN_LIMIT | Optocoupler output | Active LOW with pullup |
| GPIO13 | I_MIN_LIMIT | Optocoupler output | Active LOW with pullup |
| GPIO14 | MOTION_INTERLOCK | To Env Controller GPIO14 | OUTPUT: HIGH=moving |
| GPIO15 | PROBE_PIN | Optional Z-probe | For deck calibration |
| GPIO16 | COOLANT_FLOOD | Optional aux relay | Future expansion |
| GPIO43 | USB_TX (UART0) | USB-C debug console | FluidNC serial output |
| GPIO44 | USB_RX (UART0) | USB-C debug console | FluidNC serial input |

#### TMC2209 Wiring Per Driver

Each TMC2209 module connections:

```
                    +------------------+
      24V --------->| VM               |
      GND --------->| GND              |
                    |                  |
  ESP32 STEP ------>| STEP             |-----------> Stepper Coil A+
  ESP32 DIR ------->| DIR              |-----------> Stepper Coil A-
  ESP32 EN -------->| EN    TMC2209    |-----------> Stepper Coil B+
                    |                  |-----------> Stepper Coil B-
  MS1 (per table)-->| MS1              |
  MS2 (per table)-->| MS2              |
                    |                  |
  UART bus -------->| PDN_UART         |
                    |                  |
      3.3V -------->| VCC_IO           |
                    +------------------+
```

#### TMC2209 Current Settings

For NEMA 17 motors rated at 1.5A peak (1.06A RMS):

- **Target RMS current:** 1.0A (leaving ~6% margin)
- **UART configuration (preferred):** Set via FluidNC YAML or StallGuard registers
  - `run_current: 1000` (mA RMS) in FluidNC config
  - `hold_current: 500` (mA RMS) -- 50% for idle hold
- **VREF (if not using UART current control):**
  - V_ref = I_RMS x 1.41 x R_sense x 2
  - V_ref = 1.0 x 1.41 x 0.11 x 2 = 0.31V (measure at VREF test point)
- **Rsense on typical TMC2209 boards:** 0.11 Ohm

**StealthChop / SpreadCycle:**
- Enable StealthChop for quiet operation below ~100mm/s
- SpreadCycle auto-engages above velocity threshold
- Configure via UART: `tpwmthrs: 100` in FluidNC

### 2.2 Limit Switches (Optocoupler-Isolated)

Per the OTTO liquid handler design, limit switches use optocoupler isolation for noise immunity in the 24V stepper environment.

#### Optocoupler Circuit (per switch)

```
   3.3V (ESP32 side)
     |
    [4.7kOhm] pullup
     |
     +-----------> ESP32 GPIO (X/Y/Z/I_MIN)
     |
   +-+--+
   |    | PC817
   | OC |  Optocoupler
   |    |
   +-+--+
     |
    GND (ESP32 side)

--- isolation barrier ---

   24V (motor side)         Limit Switch
     |                      (NC, normally closed)
    [1kOhm]                     |
     |                          |
     +------[LED anode]---------+
                |
            [LED cathode]
                |
               GND (motor side)
```

**Component:** PC817 or similar (CTR > 50%)

**Switch wiring:**
- Use NC (normally closed) limit switches for fail-safe operation
- When switch is triggered (opens), optocoupler LED turns off, output goes HIGH
- ESP32 reads HIGH = switch triggered, LOW = not triggered
- Configure in FluidNC as active HIGH or invert in software

**Wire routing:**
- Use shielded cable for limit switch wires running near stepper motors
- Keep limit switch wires physically separated from stepper motor cables
- Tie shield to motor-side GND at one end only

---

## 3. Environmental Controller (ESP32-S3 #2)

### 3.1 Pin Assignment -- Environmental Controller

| ESP32-S3 GPIO | Function | Connected To | Notes |
|---|---|---|---|
| GPIO1 | DS18B20_DATA | 2x DS18B20 (1-Wire bus) | 4.7kOhm pullup to 3.3V |
| GPIO2 | HEATER_PAD_1 | IRLZ44N gate (heater 1) | Via 100Ohm gate resistor |
| GPIO3 | HEATER_PAD_2 | IRLZ44N gate (heater 2) | Via 100Ohm gate resistor |
| GPIO4 | CO2_SOLENOID | IRLZ44N gate (12V solenoid) | Via 100Ohm gate resistor |
| GPIO5 | FAN_CTRL | IRLZ44N gate (12V fan) | Via 100Ohm gate resistor |
| GPIO6 | PELTIER_H_BRIDGE_IN1 | L298N or BTS7960 IN1 | Peltier module 1 |
| GPIO7 | PELTIER_H_BRIDGE_IN2 | L298N or BTS7960 IN2 | Peltier module 1 |
| GPIO8 | PELTIER_H_BRIDGE_IN3 | L298N or BTS7960 IN3 | Peltier module 2 |
| GPIO9 | PELTIER_H_BRIDGE_IN4 | L298N or BTS7960 IN4 | Peltier module 2 |
| GPIO10 | PELTIER_EN_A | H-bridge enable A | PWM for power control |
| GPIO11 | PELTIER_EN_B | H-bridge enable B | PWM for power control |
| GPIO12 | SERVO_PWM | MG996R signal wire | 50Hz PWM, 5V tolerant signal |
| GPIO14 | MOTION_INTERLOCK | From Motion Ctrl GPIO14 | INPUT: HIGH=pause rocker |
| GPIO17 | CO2_UART_TX | SprintIR-6S-5 RX | 3.3V UART, 9600 baud |
| GPIO18 | CO2_UART_RX | SprintIR-6S-5 TX | 3.3V UART, 9600 baud |
| GPIO21 | I2C_SDA | BME280 SDA | 4.7kOhm pullup to 3.3V |
| GPIO22 | I2C_SCL | BME280 SCL | 4.7kOhm pullup to 3.3V |
| GPIO43 | USB_TX (UART0) | USB-C debug console | Serial monitor |
| GPIO44 | USB_RX (UART0) | USB-C debug console | Serial monitor |

### 3.2 DS18B20 Temperature Sensors (1-Wire Bus)

Two DS18B20 probes share a single 1-Wire bus on GPIO1.

```
    3.3V
     |
   [4.7kOhm]
     |
     +-----------> ESP32 GPIO1 (DS18B20_DATA)
     |
     +-----------> DS18B20 #1 DATA (probe: build plate / reservoir 1)
     |
     +-----------> DS18B20 #2 DATA (probe: reservoir 2)

   DS18B20 VDD ---> 3.3V
   DS18B20 GND ---> GND
```

**Notes:**
- Each DS18B20 has a unique 64-bit ROM address; enumerate at startup to identify probes.
- Use parasitic power mode only if cable runs exceed 3m; otherwise use normal power (VDD to 3.3V).
- Conversion time: ~750ms at 12-bit resolution. Read both sensors sequentially or issue a bus-wide convert command.

### 3.3 Silicone Heater Pads (MOSFET-Switched, 24V)

Two silicone heater pads (e.g., 50W each, 24V) are driven via N-channel MOSFETs with PWM for PID temperature control.

```
                     24V
                      |
                 [Heater Pad]
                      |
                  Drain (IRLZ44N)
                      |
  ESP32 GPIO -[100R]- Gate
                      |
                  Source --- GND
                      |
                   [10kOhm] (gate-to-source pulldown)
```

**MOSFET:** IRLZ44N (logic-level, Vgs(th) ~1.0-2.0V, Rds(on) ~0.022Ohm at Vgs=5V)
- At 3.3V Vgs, the IRLZ44N is fully on for loads up to ~5A. Confirmed suitable.
- The 100Ohm gate resistor limits inrush current and damps oscillation.
- The 10kOhm gate-to-source pulldown ensures MOSFET stays OFF when ESP32 pin is floating (during boot).

**PWM settings:**
- Frequency: 1kHz (sufficient for thermal mass of silicone pad)
- Resolution: 8-bit (0-255 duty cycle)
- PID loop runs at 1Hz sample rate (DS18B20 conversion takes ~750ms)

### 3.4 CO2 Solenoid Valve (MOSFET-Switched, 12V)

```
                     12V
                      |
                 +----+----+
                 |         |
            [Solenoid]  [1N4007]  <-- Flyback diode (cathode to 12V)
                 |         |
                 +----+----+
                      |
                  Drain (IRLZ44N)
                      |
  ESP32 GPIO4 -[100R]- Gate
                      |
                  Source --- GND
                      |
                   [10kOhm] pulldown
```

**CRITICAL: Flyback diode required.** The solenoid is an inductive load. Without the 1N4007 (or equivalent) across the solenoid, voltage spikes during turn-off will destroy the MOSFET or cause EMI resets on the ESP32.

### 3.5 SprintIR-6S-5 CO2 Sensor (UART)

```
  SprintIR-6S-5         ESP32-S3 #2
  +-----------+          +-----------+
  | TX -------|--------->| GPIO18 RX |
  | RX -------|<---------| GPIO17 TX |
  | VCC ------|--- 3.3V              |
  | GND ------|--- GND               |
  +-----------+          +-----------+
```

**Notes:**
- Default baud: 9600, 8N1
- Operating voltage: 3.0-3.6V (3.3V from ESP32 LDO is fine)
- Measurement range: 0-5% CO2 (50,000 ppm)
- Response time: < 3.5s (T90)
- Command protocol: ASCII, send `Z\r\n` for CO2 reading, `z\r\n` for filtered reading
- Warm-up time: ~30 seconds for stable readings

### 3.6 BME280 Humidity Sensor (I2C)

```
  BME280 Breakout       ESP32-S3 #2
  +-------------+        +-----------+
  | SDA --------|------->| GPIO21    |
  | SCL --------|------->| GPIO22    |
  | VCC --------|--- 3.3V            |
  | GND --------|--- GND             |
  | SDO --------|--- GND (addr 0x76) |
  +-------------+        +-----------+
        |    |
      [4.7k][4.7k]  pullups to 3.3V
        |    |
       SDA  SCL
```

**I2C Address:**
- SDO to GND: 0x76
- SDO to VCC: 0x77
- Default: 0x76

**Measurement capabilities:**
- Temperature: -40 to +85C (accuracy +/-1C)
- Humidity: 0-100% RH (accuracy +/-3%)
- Pressure: 300-1100 hPa

### 3.7 MG996R Servo (Rocker Platform)

```
  MG996R Servo           ESP32-S3 #2
  +------------+          +-----------+
  | Signal ----|<---------| GPIO12    |  (PWM, 50Hz)
  | VCC -------|--- 5V Rail (NOT 3.3V)
  | GND -------|--- GND (shared with ESP32)
  +------------+          +-----------+
```

**IMPORTANT:** The MG996R draws up to 2.5A stall current at 5V. Power it from the 5V buck converter, NOT from the ESP32 USB 5V pin (which is limited to ~500mA).

**PWM Configuration:**
- Frequency: 50Hz (20ms period)
- Pulse width: 500us (0 deg) to 2500us (180 deg)
- Use ESP32 LEDC peripheral, channel 0, 16-bit resolution
- Rocker motion: oscillate between two angles (e.g., 60 deg and 120 deg) at configurable frequency (0.5-2 Hz)

**Motion Interlock:**
- Before moving servo, check GPIO14 (MOTION_INTERLOCK)
- If HIGH: motion controller is moving the robot -- pause rocking
- If LOW: robot is idle -- safe to rock
- Debounce the interlock signal (50ms) to avoid glitches during rapid stop/start

### 3.8 Peltier Modules via H-Bridge

Two TEC1-12706 Peltier modules are driven via an H-bridge (BTS7960 recommended for current capacity, or L298N for lower-power testing).

```
  ESP32-S3 #2            BTS7960 H-Bridge #1
  +-----------+           +------------------+
  | GPIO6 IN1-|---------->| RPWM             |---> TEC1-12706 #1
  | GPIO7 IN2-|---------->| LPWM             |---> (hot/cold side
  | GPIO10 EN-|---------->| R_EN + L_EN      |     depends on polarity)
  +-----------+           | VCC --- 12V      |
                          | GND --- GND      |
                          +------------------+

  (Repeat for GPIO8/GPIO9/GPIO11 -> BTS7960 #2 -> TEC1-12706 #2)
```

**BTS7960 vs L298N:**
- BTS7960: handles 43A continuous, suitable for TEC1-12706 (6A max)
- L298N: handles 2A continuous (4A peak) -- NOT sufficient for full-power Peltier drive
- **Recommendation:** Use BTS7960 for production, L298N acceptable for low-power bench testing only

**Peltier control logic:**
- IN1 HIGH + IN2 LOW = forward polarity (one side hot, one side cold)
- IN1 LOW + IN2 HIGH = reverse polarity (swap hot/cold)
- EN pin: PWM for power modulation (PID output drives duty cycle)
- PWM frequency: 10kHz (reduces audible noise from Peltier)
- NEVER drive both IN1 and IN2 HIGH simultaneously (shoot-through)

**TEC1-12706 Specs:**
- Vmax: 12V
- Imax: 6A
- Qmax: 50W (heat pumping capacity)
- dTmax: 66C
- Requires heatsink + fan on hot side

### 3.9 Circulation Fan (MOSFET-Switched, 12V)

Same circuit as solenoid (Section 3.4) but on GPIO5:

```
                     12V
                      |
                 +----+----+
                 |         |
              [Fan]     [1N4007]  <-- Flyback diode
                 |         |
                 +----+----+
                      |
                  Drain (IRLZ44N)
                      |
  ESP32 GPIO5 -[100R]- Gate
                      |
                  Source --- GND
                      |
                   [10kOhm] pulldown
```

**Notes:**
- Fan motor is inductive -- flyback diode required
- PWM at 25kHz for silent speed control (or simple ON/OFF)

---

## 4. Inter-Controller Communication

### 4.1 Simple GPIO Interlock (Minimum Viable)

```
  Motion Controller              Environmental Controller
  ESP32-S3 #1                    ESP32-S3 #2
  +-----------+                  +-----------+
  | GPIO14 ---|------[1kOhm]--->| GPIO14    |
  | (OUTPUT)  |                  | (INPUT)   |
  | GND ------|--- shared GND --| GND       |
  +-----------+                  +-----------+
```

**Protocol:**
- Motion controller sets GPIO14 HIGH before any axis move
- Motion controller sets GPIO14 LOW after move completes and motors idle
- Environmental controller polls GPIO14 at 50ms interval
- Servo rocking pauses within 100ms of interlock going HIGH

**IMPORTANT:** Both ESP32 boards must share a common GND. The 1kOhm series resistor protects against accidental mis-configuration (both pins set as OUTPUT).

### 4.2 Optional UART Communication (Richer Protocol)

For future expansion (e.g., requesting temperature data from the motion controller, coordinating sequences):

```
  Motion Controller              Environmental Controller
  ESP32-S3 #1                    ESP32-S3 #2
  +-----------+                  +-----------+
  | GPIO15 TX-|---------------->| GPIO15 RX |
  | GPIO16 RX-|<----------------| GPIO16 TX |
  | GND ------|--- shared GND --| GND       |
  +-----------+                  +-----------+
```

- Baud: 115200, 8N1
- Simple JSON or fixed-width message protocol
- Messages: `{"cmd":"pause_rocker"}`, `{"cmd":"resume_rocker"}`, `{"cmd":"get_temp"}`, etc.

---

## 5. Complete Wiring Summary Tables

### 5.1 Motion Controller (ESP32-S3 #1) -- All Connections

| GPIO | Signal | Direction | Destination | Wire Color (suggested) |
|---|---|---|---|---|
| 1 | X_STEP | OUT | TMC2209 #1 STEP | Yellow |
| 2 | X_DIR | OUT | TMC2209 #1 DIR | Orange |
| 3 | Y_STEP | OUT | TMC2209 #2 STEP | Yellow |
| 4 | Y_DIR | OUT | TMC2209 #2 DIR | Orange |
| 5 | Z_STEP | OUT | TMC2209 #3 STEP | Yellow |
| 6 | Z_DIR | OUT | TMC2209 #3 DIR | Orange |
| 7 | I_STEP | OUT | TMC2209 #4 STEP | Yellow |
| 8 | I_DIR | OUT | TMC2209 #4 DIR | Orange |
| 9 | STEPPER_EN | OUT | All TMC2209 EN | Green |
| 10 | X_MIN | IN | Optocoupler (X limit) | White |
| 11 | Y_MIN | IN | Optocoupler (Y limit) | White |
| 12 | Z_MIN | IN | Optocoupler (Z limit) | White |
| 13 | I_MIN | IN | Optocoupler (I limit) | White |
| 14 | INTERLOCK | OUT | Env Ctrl GPIO14 | Blue |
| 15 | PROBE | IN | Z-probe (optional) | Purple |
| 17 | TMC_UART_TX | OUT | TMC2209 PDN_UART bus | Grey |
| 18 | TMC_UART_RX | IN | TMC2209 PDN_UART bus | Grey |
| 43 | USB_TX | OUT | USB debug | -- |
| 44 | USB_RX | IN | USB debug | -- |

### 5.2 Environmental Controller (ESP32-S3 #2) -- All Connections

| GPIO | Signal | Direction | Destination | Wire Color (suggested) |
|---|---|---|---|---|
| 1 | DS18B20_DATA | I/O | 2x DS18B20 (1-Wire) | Red |
| 2 | HEATER_1 | OUT | IRLZ44N gate (heater pad 1) | Brown |
| 3 | HEATER_2 | OUT | IRLZ44N gate (heater pad 2) | Brown |
| 4 | CO2_SOLENOID | OUT | IRLZ44N gate (12V solenoid) | Brown |
| 5 | FAN_CTRL | OUT | IRLZ44N gate (12V fan) | Brown |
| 6 | PELTIER_IN1 | OUT | BTS7960 #1 RPWM | Orange |
| 7 | PELTIER_IN2 | OUT | BTS7960 #1 LPWM | Orange |
| 8 | PELTIER_IN3 | OUT | BTS7960 #2 RPWM | Orange |
| 9 | PELTIER_IN4 | OUT | BTS7960 #2 LPWM | Orange |
| 10 | PELTIER_EN_A | OUT | BTS7960 #1 R_EN+L_EN | Yellow |
| 11 | PELTIER_EN_B | OUT | BTS7960 #2 R_EN+L_EN | Yellow |
| 12 | SERVO_PWM | OUT | MG996R signal | White |
| 14 | INTERLOCK | IN | Motion Ctrl GPIO14 | Blue |
| 17 | CO2_TX | OUT | SprintIR-6S-5 RX | Green |
| 18 | CO2_RX | IN | SprintIR-6S-5 TX | Green |
| 21 | I2C_SDA | I/O | BME280 SDA | Violet |
| 22 | I2C_SCL | OUT | BME280 SCL | Violet |
| 43 | USB_TX | OUT | USB debug | -- |
| 44 | USB_RX | IN | USB debug | -- |

---

## 6. Safety Notes

### 6.1 Flyback / Snubber Protection

Every inductive load (solenoid, fan motor, relay coil) MUST have a flyback diode:

| Component | Diode | Placement |
|---|---|---|
| CO2 solenoid valve | 1N4007 | Across solenoid, cathode to +12V |
| 12V circulation fan | 1N4007 | Across motor, cathode to +12V |
| Any relay coils | 1N4007 | Across coil, cathode to +V |

Failure to install flyback diodes will result in voltage spikes that can destroy MOSFETs and cause ESP32 resets.

### 6.2 MOSFET Gate Protection

All MOSFET gates must have:
- **100 Ohm series resistor** on the gate line (limits current, damps oscillation)
- **10 kOhm pulldown resistor** from gate to source (ensures OFF state during ESP32 boot when GPIOs are floating/high-Z)

Optional but recommended:
- **15V Zener diode** from gate to source (clamps gate voltage, protects against ESD)

### 6.3 ESP32-S3 Boot Pin Considerations

Some ESP32-S3 GPIOs have special behavior during boot. Avoid using these for MOSFET gates (where an unintended HIGH during boot could activate a heater or solenoid):

| GPIO | Boot Behavior | Safe for MOSFET? |
|---|---|---|
| GPIO0 | Strapping pin (boot mode) | NO -- avoid |
| GPIO3 | Strapping pin (JTAG) | Use with caution |
| GPIO45 | Strapping pin (VDD_SPI) | NO -- avoid |
| GPIO46 | Strapping pin (boot mode) | NO -- avoid |
| GPIO19/20 | USB D-/D+ | NO -- reserved for USB |

The pin assignments in this document avoid all boot-sensitive GPIOs for safety-critical outputs.

### 6.4 Thermal Protection

- **Heater pads:** Implement software thermal runaway protection. If temperature exceeds setpoint + 10C, cut PWM to zero and flag an error.
- **Peltier modules:** Hot side temperature must not exceed 80C. Monitor via DS18B20 or thermistor on heatsink.
- **TMC2209:** Internal thermal shutdown at ~150C. Enable UART thermal monitoring in FluidNC.
- **All MOSFETs switching >2A:** Use heatsink or copper pour on PCB under drain pad.

### 6.5 Grounding

- **Star grounding** from the 24V PSU ground terminal. Do not daisy-chain grounds between high-current loads.
- Stepper motor GND and heater GND should be separate runs back to the PSU.
- ESP32 signal GND and power GND must be tied together at a single point (star).
- Keep 3.3V logic wires away from 24V power wires. Use separate cable harnesses.

### 6.6 EMI / Noise

- Use **shielded cable** for limit switch wires (per OTTO design)
- Add **100nF ceramic capacitor** close to each TMC2209 VM pin (bulk decoupling)
- Add **100uF electrolytic** on the 24V input to the stepper driver cluster
- Add **10uF + 100nF** on each ESP32 3.3V rail (close to VCC pins)
- Keep UART wires (TMC, CO2 sensor) short (< 30cm) or use twisted pair

---

## 7. Bill of Materials (Key Components)

| Qty | Component | Specifications | Notes |
|---|---|---|---|
| 2 | ESP32-S3 DevKitC-1 | N16R8 (16MB flash, 8MB PSRAM) | USB-C, dual-core 240MHz |
| 4 | TMC2209 Stepper Driver | UART mode, Rsense 0.11 Ohm | BTT or Fysetc modules |
| 4 | NEMA 17 Stepper Motor | 1.5A, 1.8 deg, 40mm body | 42BYGHW811 or equivalent |
| 4 | PC817 Optocoupler | CTR > 50%, 4-pin DIP | For limit switch isolation |
| 4 | Mechanical Limit Switch | NC contact, lever actuator | Omron D2F-L or equivalent |
| 4 | IRLZ44N N-MOSFET | Logic-level, Vgs(th) 1-2V, 47A | TO-220 package |
| 2 | DS18B20 | Waterproof probe, 1m cable | Stainless steel tip |
| 2 | Silicone Heater Pad | 24V, 50W, sized to reservoir | Adhesive backing |
| 1 | SprintIR-6S-5 | CO2 sensor, 0-5%, UART | 3.3V, ~35mA |
| 1 | BME280 Breakout | I2C, 3.3V | GY-BME280 or Adafruit |
| 1 | MG996R Servo | Metal gear, 10kg-cm torque | 5V power rail |
| 2 | TEC1-12706 | Peltier module, 12V, 6A, 50W | 40x40mm |
| 2 | BTS7960 H-Bridge Module | 43A, dual half-bridge | For Peltier direction control |
| 1 | 12V Solenoid Valve | Normally closed, 1/4" barb | CO2 gas line |
| 1 | 12V DC Fan | 80mm or 120mm, brushless | Enclosure circulation |
| 1 | 24V PSU | 15A, 360W, enclosed | Mean Well LRS-350-24 |
| 2 | LM2596 Buck Converter | Adjustable, 3A | One set to 12V, one to 5V |
| 1 | AMS1117-3.3 LDO | 3.3V, 1A, SOT-223 | If not using ESP32 onboard reg |
| 8 | 1N4007 Diode | 1A, 1000V | Flyback protection |
| 4 | 4.7kOhm Resistor | 1/4W | I2C and 1-Wire pullups |
| 4 | 100 Ohm Resistor | 1/4W | MOSFET gate series |
| 4 | 10kOhm Resistor | 1/4W | MOSFET gate pulldown |
| 1 | 1kOhm Resistor | 1/4W | TMC UART TX series |
| -- | Blade Fuses + Holders | 5A, 6A, 8A, 15A | Inline, per power branch |
| -- | 100nF Ceramic Caps | MLCC, 50V | Bulk decoupling, 10+ pcs |
| -- | 100uF Electrolytic | 50V | 24V input bulk cap |
| -- | Wago 221 Connectors | 2/3/5 port | Inline wire-to-wire |
| -- | JST-XH Connectors | 2/3/4 pin | Sensor and signal cables |
| -- | XT60 Connectors | 60A rated | 24V power distribution |
| -- | 18 AWG Silicone Wire | Red/Black | Power distribution (24V) |
| -- | 22 AWG Hookup Wire | Assorted colors | Signal wires |
| -- | Shielded Cable | 2-conductor | Limit switch runs |
| -- | Cable Glands | PG7 / PG9 | If using enclosure |

---

## 8. FluidNC Configuration Reference

Relevant FluidNC YAML snippet for the motion controller pin mapping:

```yaml
board: LaminarForge Motion Controller
name: LaminarForge

stepping:
  engine: RMT
  idle_ms: 250
  dir_delay_us: 1
  pulse_us: 4
  disable_delay_us: 0

uart1:
  txd_pin: gpio.17
  rxd_pin: gpio.18
  baud: 115200
  mode: 8N1

axes:
  shared_stepper_disable_pin: gpio.9

  x:
    steps_per_mm: 80.0
    max_rate_mm_per_min: 5000
    acceleration_mm_per_sec2: 200
    max_travel_mm: 300
    homing:
      cycle: 1
      mpos_mm: 0
      positive_direction: false
    motor0:
      tmc_2209:
        uart_num: 1
        addr: 0
        r_sense_ohms: 0.110
        run_amps: 1.000
        hold_amps: 0.500
        microsteps: 16
        stallguard: 10
      step_pin: gpio.1
      direction_pin: gpio.2

  y:
    steps_per_mm: 80.0
    max_rate_mm_per_min: 5000
    acceleration_mm_per_sec2: 200
    max_travel_mm: 200
    homing:
      cycle: 2
      mpos_mm: 0
      positive_direction: false
    motor0:
      tmc_2209:
        uart_num: 1
        addr: 1
        r_sense_ohms: 0.110
        run_amps: 1.000
        hold_amps: 0.500
        microsteps: 16
        stallguard: 10
      step_pin: gpio.3
      direction_pin: gpio.4

  z:
    steps_per_mm: 400.0
    max_rate_mm_per_min: 2000
    acceleration_mm_per_sec2: 100
    max_travel_mm: 100
    homing:
      cycle: 3
      mpos_mm: 0
      positive_direction: true
    motor0:
      tmc_2209:
        uart_num: 1
        addr: 2
        r_sense_ohms: 0.110
        run_amps: 1.000
        hold_amps: 0.500
        microsteps: 16
        stallguard: 10
      step_pin: gpio.5
      direction_pin: gpio.6

  i:
    steps_per_mm: 200.0
    max_rate_mm_per_min: 1000
    acceleration_mm_per_sec2: 50
    max_travel_mm: 50
    homing:
      cycle: 4
      mpos_mm: 0
      positive_direction: false
    motor0:
      tmc_2209:
        uart_num: 1
        addr: 3
        r_sense_ohms: 0.110
        run_amps: 1.000
        hold_amps: 0.500
        microsteps: 16
        stallguard: 10
      step_pin: gpio.7
      direction_pin: gpio.8

control:
  safety_door_pin: NO_PIN

coolant:
  flood_pin: gpio.16

probe:
  pin: gpio.15:low:pu
```

---

## 9. Assembly Checklist

Before powering on:

- [ ] Verify 24V PSU output with multimeter (no load)
- [ ] Verify 12V buck output (adjust trimpot)
- [ ] Verify 5V buck output (adjust trimpot)
- [ ] Check all flyback diodes are installed with correct polarity (cathode band toward +V)
- [ ] Check MOSFET gate pulldown resistors are installed
- [ ] Verify TMC2209 MS1/MS2 address pins match the table in Section 2.1
- [ ] Verify TMC2209 VREF or UART current settings before connecting motors
- [ ] Connect stepper motors one at a time; verify rotation direction
- [ ] Test each limit switch triggers correctly (check with `$Limits` in FluidNC console)
- [ ] Test DS18B20 sensors enumerate with correct addresses
- [ ] Test heater pad MOSFET switching (low duty cycle first, monitor temperature)
- [ ] Test CO2 sensor UART communication (send `Z\r\n`, expect numeric response)
- [ ] Test BME280 I2C communication (scan address 0x76)
- [ ] Test servo range of motion at low speed
- [ ] Test Peltier polarity (feel hot/cold sides, verify direction control)
- [ ] Test inter-controller interlock (assert HIGH, verify env controller pauses)
- [ ] Run full homing cycle at reduced speed
- [ ] Monitor all temperatures during 30-minute burn-in test
