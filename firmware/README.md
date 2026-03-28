# LaminarForge Liquid Handler - FluidNC Firmware

ESP32-S3 firmware configuration for the 4-axis liquid handling robot.

## Hardware Overview

| Component | Specification |
|-----------|--------------|
| MCU | ESP32-S3 DevKitC |
| Drivers | 4x TMC2209 (UART mode, shared bus) |
| PSU | 24V |
| Axes | X, Y (belt), Z (geared belt 3:1), I (lead screw syringe) |

## Pin Assignments

### TMC2209 UART Bus (shared single-wire)

| Signal | GPIO |
|--------|------|
| UART TX | GPIO 17 |
| UART RX | GPIO 18 |

All four TMC2209 drivers share this UART bus. Each driver is addressed individually (addr 0-3).

### X-Axis (Belt-driven gantry)

| Signal | GPIO | TMC2209 Addr |
|--------|------|-------------|
| Step | GPIO 6 | 0 |
| Direction | GPIO 5 | |
| Enable | GPIO 7 | |
| Limit (min) | GPIO 4 | |

### Y-Axis (Belt-driven gantry)

| Signal | GPIO | TMC2209 Addr |
|--------|------|-------------|
| Step | GPIO 8 | 1 |
| Direction | GPIO 16 | |
| Enable | GPIO 9 | |
| Limit (min) | GPIO 15 | |

### Z-Axis (Geared belt, 3:1 reduction)

| Signal | GPIO | TMC2209 Addr |
|--------|------|-------------|
| Step | GPIO 12 | 2 |
| Direction | GPIO 11 | |
| Enable | GPIO 13 | |
| Limit (min) | GPIO 10 | |

### I-Axis (Syringe plunger, T8x2 lead screw)

| Signal | GPIO | TMC2209 Addr |
|--------|------|-------------|
| Step | GPIO 47 | 3 |
| Direction | GPIO 21 | |
| Enable | GPIO 48 | |
| Limit (min) | GPIO 14 | |

### Auxiliary Pins

| Function | GPIO | Notes |
|----------|------|-------|
| Probe | GPIO 38 | ADC1-capable, for liquid level detection |
| Valve 1 (flood) | GPIO 39 | Solenoid valve control |
| Valve 2 (mist) | GPIO 40 | Solenoid valve control |
| Status LED | GPIO 41 | Digital output 0 |
| Error LED | GPIO 42 | Digital output 1 |
| I2C SDA | GPIO 1 | Reserved for sensors |
| I2C SCL | GPIO 2 | Reserved for sensors |

### Avoided Pins

The following ESP32-S3 strapping pins are intentionally NOT used:

- **GPIO 0** - Boot mode strapping pin
- **GPIO 3** - JTAG strapping pin
- **GPIO 45** - VDD_SPI voltage strapping pin
- **GPIO 46** - Boot mode / log output strapping pin

## Steps/mm Calculations

All axes use 200 steps/rev motors with 16 microsteps (3200 steps/rev).

| Axis | Mechanism | Calculation | Steps/mm |
|------|-----------|-------------|----------|
| X | GT2 20T belt | 3200 / (pi x 12.73mm) = 3200 / 39.98 | **80.04** |
| Y | GT2 20T belt | Same as X | **80.04** |
| Z | GT2 20T + 3:1 gear | 80.04 x 3 | **240.12** |
| I | T8x2 lead screw | 3200 / 2mm lead | **1600.00** |

## Syringe Calibration

Hamilton 1710RN 100uL syringe:

- Barrel inner diameter: 1.46mm
- Barrel cross-section area: pi x (0.73)^2 = 1.674 mm^2
- Travel per microliter: 1 mm^3 / 1.674 mm^2 = **0.597 mm/uL**
- 30mm mechanical travel = ~50.2 uL usable volume
- 1 step = 1/1600 mm = 0.000625 mm = **0.00105 uL** (sub-nanoliter resolution)

## Motion Parameters

| Axis | Max Speed | Acceleration | Max Travel |
|------|-----------|-------------|------------|
| X | 100 mm/s (6000 mm/min) | 500 mm/s^2 | 300 mm |
| Y | 100 mm/s (6000 mm/min) | 500 mm/s^2 | 300 mm |
| Z | 50 mm/s (3000 mm/min) | 200 mm/s^2 | 100 mm |
| I | 5 mm/s (300 mm/min) | 100 mm/s^2 | 30 mm |

The I-axis uses deliberately low acceleration and speed to prevent bubble formation and ensure precise dispensing.

## Homing Sequence

All axes home to their minimum (negative) position in this order:

1. **Cycle 1 - X-axis**: Homes to min via microswitch on GPIO 4
2. **Cycle 2 - Y-axis**: Homes to min via microswitch on GPIO 15
3. **Cycle 3 - Z-axis**: Homes to min (retracted/up) via microswitch on GPIO 10
4. **Cycle 4 - I-axis**: Homes to min (plunger fully retracted) via microswitch on GPIO 14

Z homes before I so the pipette tip is safely raised before the syringe retracts. The I-axis uses a very slow homing speed (1 mm/s feed) to avoid damaging the syringe or creating excessive vacuum.

## Flashing FluidNC to ESP32-S3

### Prerequisites

- Python 3.8+
- `esptool.py` (`pip install esptool`)
- FluidNC firmware binary (download from [FluidNC releases](https://github.com/bdring/FluidNC/releases))
- USB-C cable connected to ESP32-S3 DevKitC

### Step 1: Erase flash

```bash
esptool.py --chip esp32s3 --port /dev/tty.usbmodem* erase_flash
```

### Step 2: Flash FluidNC firmware

```bash
esptool.py --chip esp32s3 --port /dev/tty.usbmodem* \
  --baud 460800 \
  write_flash \
  0x0000 bootloader.bin \
  0x8000 partitions.bin \
  0x10000 firmware.bin \
  0x3d0000 spiffs.bin
```

Alternatively, use the FluidNC WebUI installer at [http://install.fluidnc.com](http://install.fluidnc.com) for a browser-based flash.

### Step 3: Upload configuration

1. Connect to the `LaminarForge` WiFi AP (password: `laminarforge`)
2. Open `http://192.168.0.1` in a browser
3. Navigate to the **Files** tab in the FluidNC WebUI
4. Upload `fluidnc_config.yaml` as the machine config
5. Restart the ESP32 (or send `$Bye` then power cycle)

Alternatively, upload via serial:

```bash
# Using FluidNC's built-in file upload over serial
# Connect at 115200 baud, then use $LocalFS/Upload command
picocom -b 115200 /dev/tty.usbmodem*
```

### Step 4: Verify

After reboot, connect via serial or WebUI and check:

```
$SS   # Show settings summary
$$    # Show all settings
$H    # Home all axes (ensure limit switches are wired)
?     # Status report
```

## TMC2209 UART Wiring

The four TMC2209 drivers share a single UART bus. Each driver must have its
address pins (MS1, MS2) set to give it a unique address:

| Driver | Axis | MS1 | MS2 | Address |
|--------|------|-----|-----|---------|
| U1 | X | LOW | LOW | 0 |
| U2 | Y | HIGH | LOW | 1 |
| U3 | Z | LOW | HIGH | 2 |
| U4 | I | HIGH | HIGH | 3 |

The UART TX pin connects to all four PDN_UART pins through 1k ohm resistors.
The UART RX pin connects to all four PDN_UART pins directly (active-low
open-drain outputs on the TMC2209 side).

## Troubleshooting

- **Motors don't move**: Check 24V supply, verify enable pins are being driven LOW (active low on TMC2209)
- **Erratic motion**: Verify UART addresses match MS1/MS2 pin settings on each driver
- **Homing fails**: Check limit switch wiring (normally open, pulled up internally). Verify with `?` status that switch state changes when actuated
- **Syringe stalls**: Reduce I-axis run current or increase stallguard threshold. The lead screw has inherent friction that can trigger false stall detection
- **WiFi won't connect**: The AP defaults to `LaminarForge` / `laminarforge`. If reconfigured for STA mode, check SSID and password in the YAML
