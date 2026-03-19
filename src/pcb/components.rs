use crate::*;
use super::{Component, Pad};
use super::nets::*;
use super::footprints::*;

pub fn define_components() -> Vec<Component> {
    let mut comps = Vec::new();

    let hx_start = HEATER_ZONE_X_START;
    let hy_top = PCB_WIDTH - HEATER_ZONE_Y_START - HEATER_ZONE_WIDTH;
    let hy_bottom = PCB_WIDTH - HEATER_ZONE_Y_START;
    let slot_y_center = (hy_top + hy_bottom) / 2.0;
    let first_slot_x = hx_start + (HEATER_ZONE_LENGTH - (NUM_SLOTS as f64 - 1.0) * SLOT_SPACING) / 2.0;

    // ── ESP32-S3-WROOM-1-N8 (U1) ──
    comps.push(Component {
        reference: "U1",
        value: "ESP32-S3-WROOM-1-N8",
        footprint_lib: "laminarforge",
        footprint_name: "ESP32-S3-WROOM-1",
        lcsc: "C2913198",
        x: 20.0, y: 20.0, rotation: 0.0,
        layer: "F.Cu",
        description: "WiFi/BT MCU module",
        pads: esp32_pads(),
    });

    // ── ADS1115IDGSR (U2) ── MSOP-10
    comps.push(Component {
        reference: "U2",
        value: "ADS1115IDGSR",
        footprint_lib: "laminarforge",
        footprint_name: "MSOP-10",
        lcsc: "C37593",
        x: 50.0, y: 15.0, rotation: 0.0,
        layer: "F.Cu",
        description: "16-bit I2C ADC",
        pads: msop10_pads(),
    });

    // ── CD74HC4051M (U3) ── SOIC-16
    comps.push(Component {
        reference: "U3",
        value: "CD74HC4051M",
        footprint_lib: "laminarforge",
        footprint_name: "SOIC-16W",
        lcsc: "C139301",
        x: 50.0, y: 30.0, rotation: 0.0,
        layer: "F.Cu",
        description: "8:1 analog multiplexer",
        pads: soic16w_pads(),
    });

    // ── TC4427ACOA713 (U4) ── SOIC-8
    comps.push(Component {
        reference: "U4",
        value: "TC4427ACOA713",
        footprint_lib: "laminarforge",
        footprint_name: "SOIC-8",
        lcsc: "C636886",
        x: 72.0, y: 15.0, rotation: 0.0,
        layer: "F.Cu",
        description: "Dual MOSFET gate driver",
        pads: soic8_pads(),
    });

    // ── AO3400A (Q1) ── SOT-23
    comps.push(Component {
        reference: "Q1",
        value: "AO3400A",
        footprint_lib: "laminarforge",
        footprint_name: "SOT-23",
        lcsc: "C20917",
        x: 85.0, y: 15.0, rotation: 0.0,
        layer: "F.Cu",
        description: "N-ch MOSFET heater switch",
        pads: sot23_pads_mosfet(),
    });

    // ── 8x MMBT3904 (Q2-Q9) ──
    for i in 0..NUM_SLOTS {
        let ref_name = Box::leak(format!("Q{}", i + 2).into_boxed_str());
        comps.push(Component {
            reference: ref_name,
            value: "MMBT3904",
            footprint_lib: "laminarforge",
            footprint_name: "SOT-23",
            lcsc: "C20526",
            x: 12.0 + i as f64 * 9.5, y: 42.0, rotation: 0.0,
            layer: "F.Cu",
            description: "NPN LED driver",
            pads: sot23_pads_npn(i),
        });
    }

    // ── 8x Blue LEDs (D1-D8) ──
    for i in 0..NUM_SLOTS {
        let ref_name = Box::leak(format!("D{}", i + 1).into_boxed_str());
        let slot_x = first_slot_x + i as f64 * SLOT_SPACING;
        let led_x = slot_x - 4.0;
        comps.push(Component {
            reference: ref_name,
            value: "Blue_LED_460nm",
            footprint_lib: "laminarforge",
            footprint_name: "LED_0805",
            lcsc: "C2286",
            x: led_x, y: slot_y_center, rotation: 90.0,
            layer: "F.Cu",
            description: "460nm blue LED",
            pads: led_0805_pads(i),
        });
    }

    // ── 8x Photodiodes (PD1-PD8) ──
    for i in 0..NUM_SLOTS {
        let ref_name = Box::leak(format!("PD{}", i + 1).into_boxed_str());
        let slot_x = first_slot_x + i as f64 * SLOT_SPACING;
        let pd_x = slot_x + 4.0;
        comps.push(Component {
            reference: ref_name,
            value: "VEMD5510C",
            footprint_lib: "laminarforge",
            footprint_name: "VEMD5510C",
            lcsc: "C2526474",
            x: pd_x, y: slot_y_center, rotation: 270.0,
            layer: "F.Cu",
            description: "Green-sensitive photodiode",
            pads: photodiode_pads(i),
        });
    }

    // ── 8x 1K base resistors (R1-R8) ──
    for i in 0..NUM_SLOTS {
        let ref_name = Box::leak(format!("R{}", i + 1).into_boxed_str());
        let net_name = Box::leak(format!("LED_BASE_{}", i).into_boxed_str()) as &'static str;
        comps.push(Component {
            reference: ref_name,
            value: "1K",
            footprint_lib: "laminarforge",
            footprint_name: "R_0805",
            lcsc: "C17513",
            x: 12.0 + i as f64 * 9.5, y: 38.0, rotation: 0.0,
            layer: "F.Cu",
            description: "1K NPN base resistor",
            pads: r0805_pads(NET_LED_BASE_0 + i as u32, net_name, NET_GND, "GND"),
        });
    }

    // ── 8x 100R LED resistors (R9-R16) ──
    for i in 0..NUM_SLOTS {
        let ref_name = Box::leak(format!("R{}", i + 9).into_boxed_str());
        let net_name = Box::leak(format!("LED_COL_{}", i).into_boxed_str()) as &'static str;
        comps.push(Component {
            reference: ref_name,
            value: "100R",
            footprint_lib: "laminarforge",
            footprint_name: "R_0805",
            lcsc: "C17408",
            x: 12.0 + i as f64 * 9.5, y: 46.0, rotation: 0.0,
            layer: "F.Cu",
            description: "100R LED current limiter",
            pads: r0805_pads(NET_5V, "+5V", NET_LED_COL_0 + i as u32, net_name),
        });
    }

    // ── I2C pull-ups (R17, R18) ──
    comps.push(Component {
        reference: "R17",
        value: "4.7K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17673",
        x: 45.0, y: 10.0, rotation: 0.0,
        layer: "F.Cu",
        description: "I2C SDA pull-up",
        pads: r0805_pads(NET_3V3, "+3V3", NET_SDA, "SDA"),
    });
    comps.push(Component {
        reference: "R18",
        value: "4.7K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17673",
        x: 51.0, y: 10.0, rotation: 0.0, // moved from x=48 for clearance
        layer: "F.Cu",
        description: "I2C SCL pull-up",
        pads: r0805_pads(NET_3V3, "+3V3", NET_SCL, "SCL"),
    });

    // ── Thermistor divider (R19) ──
    comps.push(Component {
        reference: "R19",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17414",
        x: 55.0, y: 10.0, rotation: 0.0,
        layer: "F.Cu",
        description: "Thermistor divider 10K",
        pads: r0805_pads(NET_3V3, "+3V3", NET_ADC_AIN1, "ADC_AIN1"),
    });

    // ── 8x 100K photodiode load resistors (R20-R27) ──
    for i in 0..NUM_SLOTS {
        let ref_name = Box::leak(format!("R{}", i + 20).into_boxed_str());
        let mux_name = Box::leak(format!("MUX_Y{}", i).into_boxed_str()) as &'static str;
        let slot_x = first_slot_x + i as f64 * SLOT_SPACING;
        let pd_x = slot_x + 4.0; // same x as photodiode
        comps.push(Component {
            reference: ref_name,
            value: "100K",
            footprint_lib: "laminarforge",
            footprint_name: "R_0805",
            lcsc: "C17407",
            x: pd_x, y: 58.0, rotation: 0.0,
            layer: "F.Cu",
            description: "PD load resistor (I-to-V)",
            pads: r0805_pads(NET_MUX_Y0 + i as u32, mux_name, NET_GND, "GND"),
        });
    }

    // ── EN pull-up (R28) ──
    // Moved west to x=3.5 to clear USB_DN F.Cu at x=9.25 and LB0 at x=5.5
    comps.push(Component {
        reference: "R28",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17414",
        x: 3.5, y: 15.0, rotation: 0.0,
        layer: "F.Cu",
        description: "ESP32 EN pull-up",
        pads: r0805_pads(NET_3V3, "+3V3", NET_ESP_EN, "ESP_EN"),
    });

    // ── GPIO0 pull-up (R29) ──
    // Moved west to x=3.5 to clear USB_DN F.Cu at x=9.25 and LB0 at x=5.5
    comps.push(Component {
        reference: "R29",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17414",
        x: 3.5, y: 18.0, rotation: 0.0,
        layer: "F.Cu",
        description: "ESP32 GPIO0 pull-up (normal boot)",
        pads: r0805_pads(NET_3V3, "+3V3", NET_ESP_GPIO0, "ESP_GPIO0"),
    });

    // ── USB CC pull-down resistors (R30, R31) ──
    // R30 at (11, 7) rot=270: pad1 CC1 at (11, 6.25), pad2 GND at (11, 7.75).
    //   Moved east from x=10.25 to x=11 to avoid USB_DP vertical at x=9.75.
    // R31 at (19.0, 5.0): CC2 pull-down further from J1.
    comps.push(Component {
        reference: "R30",
        value: "5.1K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C25905",
        x: 11.0, y: 7.0, rotation: 270.0,
        layer: "F.Cu",
        description: "USB CC1 5.1K pull-down",
        pads: r0603_pads(NET_USB_CC1, "USB_CC1", NET_GND, "GND"),
    });
    // R31: Moved near USB connector area, away from ESP32 pads
    comps.push(Component {
        reference: "R31",
        value: "5.1K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C25905",
        x: 19.0, y: 5.0, rotation: 0.0,
        layer: "F.Cu",
        description: "USB CC2 5.1K pull-down",
        pads: r0603_pads(NET_USB_CC2, "USB_CC2", NET_GND, "GND"),
    });

    // ── Power LED resistor (R32) and activity LED resistor (R33) ──
    comps.push(Component {
        reference: "R32",
        value: "1K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17513",
        x: 5.0, y: 35.0, rotation: 90.0,
        layer: "F.Cu",
        description: "Power LED current limiter",
        pads: r0805_pads(NET_3V3, "+3V3", NET_LED_PWR_ANODE, "LED_PWR_ANODE"),
    });
    comps.push(Component {
        reference: "R33",
        value: "1K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17513",
        x: 5.0, y: 40.0, rotation: 90.0,
        layer: "F.Cu",
        description: "Activity LED current limiter",
        pads: r0805_pads(NET_ESP_GPIO_ACT, "ESP_GPIO_ACT", NET_LED_ACT_ANODE, "LED_ACT_ANODE"),
    });

    // ── Decoupling caps (C1-C5) ── now parametric nets
    comps.push(Component {
        reference: "C1", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603",
        lcsc: "C14663", x: 25.0, y: 8.0, rotation: 0.0,
        layer: "F.Cu", description: "ESP32 decoupling",
        pads: c0603_pads(NET_3V3, "+3V3", NET_GND, "GND"),
    });
    comps.push(Component {
        reference: "C2", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603",
        lcsc: "C14663", x: 52.0, y: 12.0, rotation: 0.0,
        layer: "F.Cu", description: "ADS1115 decoupling",
        pads: c0603_pads(NET_3V3, "+3V3", NET_GND, "GND"),
    });
    comps.push(Component {
        reference: "C3", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603",
        lcsc: "C14663", x: 52.0, y: 27.0, rotation: 0.0,
        layer: "F.Cu", description: "CD74HC4051 decoupling",
        pads: c0603_pads(NET_3V3, "+3V3", NET_GND, "GND"),
    });
    comps.push(Component {
        reference: "C4", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603",
        lcsc: "C14663", x: 74.0, y: 12.0, rotation: 0.0,
        layer: "F.Cu", description: "TC4427 decoupling",
        pads: c0603_pads(NET_5V, "+5V", NET_GND, "GND"),
    });
    comps.push(Component {
        reference: "C5", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603",
        lcsc: "C14663", x: 87.0, y: 12.0, rotation: 0.0,
        layer: "F.Cu", description: "MOSFET gate decoupling",
        pads: c0603_pads(NET_5V, "+5V", NET_GND, "GND"),
    });

    // ── SS34 Schottky (D9) — flyback diode ──
    comps.push(Component {
        reference: "D9",
        value: "SS34",
        footprint_lib: "laminarforge",
        footprint_name: "D_SMA",
        lcsc: "C8678",
        x: 90.0, y: 30.0, rotation: 0.0,
        layer: "F.Cu",
        description: "Schottky flyback diode",
        pads: sma_diode_pads(),
    });

    // ── SS34 Schottky (D10) — reverse polarity protection ──
    comps.push(Component {
        reference: "D10",
        value: "SS34",
        footprint_lib: "laminarforge",
        footprint_name: "D_SMA",
        lcsc: "C8678",
        x: 90.0, y: 16.0, rotation: 0.0,
        layer: "F.Cu",
        description: "Reverse polarity protection",
        pads: sma_diode_pads_polarity(),
    });

    // ── Power LED (D11, green 0805) ──
    // Moved to y=41 for clearance from R33 pad2 at (5,39.05).
    // R33 pad2 (LED_ACT_ANODE) at (5,39.05): y=38.55-39.55 (after rot).
    // D11 pad1 (LED_PWR_ANODE) at (5,41.95): y=41.45-42.45.
    // Gap = 41.45-39.55 = 1.9mm. OK.
    // D11 pad2 (GND) at (5,40.05): y=39.55-40.55.
    // Gap to R33 pad2: 39.55-39.55 = 0mm. Still touching!
    // Move to y=42 instead: D11 pad2 at (5,41.05): y=40.55-41.55.
    // R33 pad2 at y=38.55-39.55. Gap=40.55-39.55=1.0mm. OK.
    // D11 pad1 at (5,42.95): y=42.45-43.45.
    comps.push(Component {
        reference: "D11",
        value: "Green_LED",
        footprint_lib: "laminarforge",
        footprint_name: "LED_0805_STATUS",
        lcsc: "C2297",
        x: 5.0, y: 44.0, rotation: 90.0,
        layer: "F.Cu",
        description: "3.3V power indicator",
        pads: vec![
            Pad {
                number: "1", pad_type: "smd", shape: "rect",
                x: -0.95, y: 0.0, width: 1.0, height: 1.2,
                layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
                net_id: NET_LED_PWR_ANODE, net_name: "LED_PWR_ANODE", drill: None,
            },
            Pad {
                number: "2", pad_type: "smd", shape: "rect",
                x: 0.95, y: 0.0, width: 1.0, height: 1.2,
                layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
                net_id: NET_GND, net_name: "GND", drill: None,
            },
        ],
    });

    // ── Activity LED (D12, blue 0805) ──
    comps.push(Component {
        reference: "D12",
        value: "Blue_LED",
        footprint_lib: "laminarforge",
        footprint_name: "LED_0805_STATUS",
        lcsc: "C2286",
        x: 5.0, y: 48.0, rotation: 90.0,
        layer: "F.Cu",
        description: "Activity indicator",
        pads: vec![
            Pad {
                number: "1", pad_type: "smd", shape: "rect",
                x: -0.95, y: 0.0, width: 1.0, height: 1.2,
                layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
                net_id: NET_LED_ACT_ANODE, net_name: "LED_ACT_ANODE", drill: None,
            },
            Pad {
                number: "2", pad_type: "smd", shape: "rect",
                x: 0.95, y: 0.0, width: 1.0, height: 1.2,
                layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
                net_id: NET_GND, net_name: "GND", drill: None,
            },
        ],
    });

    // ── USB-C (J1) ──
    comps.push(Component {
        reference: "J1",
        value: "USB-C",
        footprint_lib: "laminarforge",
        footprint_name: "USB_C_Receptacle",
        lcsc: "C168688",
        x: 10.0, y: 5.0, rotation: 180.0,
        layer: "F.Cu",
        description: "USB-C power/data connector",
        pads: usbc_pads(),
    });

    // ── Barrel jack (J2) ── moved to y=8 for edge clearance
    comps.push(Component {
        reference: "J2",
        value: "DC_5525",
        footprint_lib: "laminarforge",
        footprint_name: "BarrelJack_SMD",
        lcsc: "C381116",
        x: 90.0, y: 8.0, rotation: 0.0,
        layer: "F.Cu",
        description: "12V DC barrel jack",
        pads: barrel_jack_pads(),
    });

    // ── Thermistor pads (J3) ──
    comps.push(Component {
        reference: "J3",
        value: "NTC_CONN",
        footprint_lib: "laminarforge",
        footprint_name: "PinHeader_1x02",
        lcsc: "",
        x: 60.0, y: 10.0, rotation: 0.0,
        layer: "F.Cu",
        description: "NTC thermistor wire pads",
        pads: pin_header_2_pads(NET_ADC_AIN1, "ADC_AIN1", NET_GND, "GND"),
    });

    // ── External heater connector (J4) ──
    // Pin1: +12V, Pin2: HEATER_P (MOSFET drain side)
    // External heater element connects between these two pins.
    comps.push(Component {
        reference: "J4",
        value: "HEATER_CONN",
        footprint_lib: "laminarforge",
        footprint_name: "PinHeader_1x02",
        lcsc: "",
        x: 65.0, y: 10.0, rotation: 0.0,
        layer: "F.Cu",
        description: "External heater connector",
        pads: pin_header_2_pads(NET_12V, "+12V", NET_HEATER_P, "HEATER_P"),
    });

    // ── UART debug header (J5) ──
    comps.push(Component {
        reference: "J5",
        value: "UART_HDR",
        footprint_lib: "laminarforge",
        footprint_name: "PinHeader_1x04",
        lcsc: "",
        x: 3.5, y: 30.0, rotation: 0.0,
        layer: "F.Cu",
        description: "UART debug header",
        pads: pin_header_4_pads([
            (NET_3V3, "+3V3"),
            (NET_UART_TX, "UART_TX"),
            (NET_UART_RX, "UART_RX"),
            (NET_GND, "GND"),
        ]),
    });

    // ═══════════════════════════════════════════════════════════════════
    // VOLTAGE REGULATORS
    // ═══════════════════════════════════════════════════════════════════

    // ── U5 REMOVED: AMS1117-5.0 replaced by D14 (VBUS→5V Schottky) ──
    // 12V only powers heater. All electronics powered via USB-C 5V.
    // This eliminates the thermal issue (12V→5V LDO dissipating 1.3-4.8W).

    // ── 3.3V LDO — AMS1117-3.3 (U6) ──
    // Placed at y=2 so tab (y=-1.5 local = y=0.5 abs) is above the SDA F.Cu trace at y=3.
    comps.push(Component {
        reference: "U6",
        value: "AMS1117-3.3",
        footprint_lib: "laminarforge",
        footprint_name: "SOT-223",
        lcsc: "C6186",
        x: 35.0, y: 4.0, rotation: 0.0,
        layer: "F.Cu",
        description: "3.3V LDO regulator",
        pads: sot223_pads(NET_5V, "+5V", NET_3V3, "+3V3"),
    });

    // ── USBLC6-2SC6 ESD protection (U7) ──
    // Moved east from x=15 to x=17 to clear J1 S1 GND pad (x=13.82-14.82)
    comps.push(Component {
        reference: "U7",
        value: "USBLC6-2SC6",
        footprint_lib: "laminarforge",
        footprint_name: "SOT-23-6",
        lcsc: "C7519",
        x: 17.0, y: 7.0, rotation: 0.0,
        layer: "F.Cu",
        description: "USB ESD protection",
        pads: sot236_pads_esd(),
    });

    // ═══════════════════════════════════════════════════════════════════
    // BULK CAPACITORS
    // ═══════════════════════════════════════════════════════════════════

    // ── C6: 22µF 5V bulk cap (near D14 VBUS→5V diode) ──
    comps.push(Component {
        reference: "C6", value: "22uF",
        footprint_lib: "laminarforge", footprint_name: "C_1206",
        lcsc: "C5672", x: 72.0, y: 5.0, rotation: 0.0,
        layer: "F.Cu", description: "5V bulk cap (USB power)",
        pads: c1206_pads(NET_5V, "+5V", NET_GND, "GND"),
    });

    // ── C7: 22µF 5V output cap (near U5) ──
    comps.push(Component {
        reference: "C7", value: "22uF",
        footprint_lib: "laminarforge", footprint_name: "C_1206",
        lcsc: "C5672", x: 80.5, y: 5.0, rotation: 0.0,
        layer: "F.Cu", description: "5V LDO output cap",
        pads: c1206_pads(NET_5V, "+5V", NET_GND, "GND"),
    });

    // ── C8: 22µF 5V→3.3V input cap (near U6 at (35,2)) ──
    comps.push(Component {
        reference: "C8", value: "22uF",
        footprint_lib: "laminarforge", footprint_name: "C_1206",
        lcsc: "C5672", x: 32.0, y: 5.0, rotation: 0.0,
        layer: "F.Cu", description: "3.3V LDO input cap",
        pads: c1206_pads(NET_5V, "+5V", NET_GND, "GND"),
    });

    // ── C9: 22µF 3.3V output cap (near U6) ──
    comps.push(Component {
        reference: "C9", value: "22uF",
        footprint_lib: "laminarforge", footprint_name: "C_1206",
        lcsc: "C5672", x: 40.5, y: 5.0, rotation: 0.0,
        layer: "F.Cu", description: "3.3V LDO output cap",
        pads: c1206_pads(NET_3V3, "+3V3", NET_GND, "GND"),
    });

    // ── C10: 22µF 12V input bulk cap (near J2) ──
    comps.push(Component {
        reference: "C10", value: "22uF",
        footprint_lib: "laminarforge", footprint_name: "C_1206",
        lcsc: "C5672", x: 93.0, y: 16.0, rotation: 0.0,
        layer: "F.Cu", description: "12V input bulk cap",
        pads: c1206_pads(NET_12V, "+12V", NET_GND, "GND"),
    });

    // ── C11: 1µF EN RC filter cap ──
    // Moved west to x=3.5 y=13 to clear USB_DN and sit near R28
    comps.push(Component {
        reference: "C11", value: "1uF",
        footprint_lib: "laminarforge", footprint_name: "C_0603",
        lcsc: "C15849", x: 3.5, y: 13.0, rotation: 0.0,
        layer: "F.Cu", description: "ESP32 EN RC delay cap",
        pads: c0603_pads(NET_ESP_EN, "ESP_EN", NET_GND, "GND"),
    });

    // ── C12: 1µF VBUS decoupling cap (USB spec requires 1-10µF on VBUS) ──
    // Placed at (14, 1.5) near J1, west of VBUS B.Cu trunk at y=1.5.
    // SCL F.Cu at y=0.5: gap to cap pad top = 0.4mm. OK.
    // SDA F.Cu at y=3: gap to cap pad bottom = 1.0mm. OK.
    comps.push(Component {
        reference: "C12", value: "1uF",
        footprint_lib: "laminarforge", footprint_name: "C_0603",
        lcsc: "C15849", x: 14.0, y: 1.5, rotation: 0.0,
        layer: "F.Cu", description: "VBUS decoupling cap (USB spec)",
        pads: c0603_pads(NET_VBUS, "VBUS", NET_GND, "GND"),
    });

    // ═══════════════════════════════════════════════════════════════════
    // BOOT AND RESET BUTTONS
    // ═══════════════════════════════════════════════════════════════════

    // ── BOOT button (SW1): GPIO0 to GND ──
    comps.push(Component {
        reference: "SW1",
        value: "BOOT",
        footprint_lib: "laminarforge",
        footprint_name: "SW_Tactile_6mm",
        lcsc: "C318884",
        x: 5.0, y: 22.0, rotation: 0.0,
        layer: "F.Cu",
        description: "BOOT button (GPIO0 to GND)",
        pads: tactile_switch_pads(NET_ESP_GPIO0, "ESP_GPIO0", NET_GND, "GND"),
    });

    // ── RESET button (SW2): EN to GND ──
    comps.push(Component {
        reference: "SW2",
        value: "RESET",
        footprint_lib: "laminarforge",
        footprint_name: "SW_Tactile_6mm",
        lcsc: "C318884",
        x: 5.0, y: 26.0, rotation: 0.0,
        layer: "F.Cu",
        description: "RESET button (EN to GND)",
        pads: tactile_switch_pads(NET_ESP_EN, "ESP_EN", NET_GND, "GND"),
    });

    // ── Gate series resistor (R34) — limits gate ringing/EMI ──
    comps.push(Component {
        reference: "R34",
        value: "100R",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17408",
        x: 80.0, y: 18.0, rotation: 0.0,
        layer: "F.Cu",
        description: "Gate series resistor (ringing control)",
        pads: r0805_pads(NET_GATE_DRV, "GATE_DRV", NET_MOSFET_GATE, "MOSFET_GATE"),
    });

    // ── Gate pulldown (R35) — prevents floating gate during boot ──
    comps.push(Component {
        reference: "R35",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0805",
        lcsc: "C17414",
        x: 84.0, y: 20.0, rotation: 0.0,
        layer: "F.Cu",
        description: "MOSFET gate pulldown (boot safety)",
        pads: r0805_pads(NET_MOSFET_GATE, "MOSFET_GATE", NET_GND, "GND"),
    });

    // ── D14: VBUS → 5V Schottky (replaces AMS1117-5.0) ──
    comps.push(Component {
        reference: "D14",
        value: "SS34",
        footprint_lib: "laminarforge",
        footprint_name: "D_SMA",
        lcsc: "C8678",
        x: 75.0, y: 2.0, rotation: 0.0,
        layer: "F.Cu",
        description: "VBUS to 5V Schottky diode",
        pads: sma_diode_pads_vbus(),
    });

    comps
}
