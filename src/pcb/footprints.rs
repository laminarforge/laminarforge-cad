use std::fmt::Write;
use super::{Component, Pad, next_uuid};
use super::nets::*;

pub fn write_footprint(pcb: &mut String, comp: &Component) {
    writeln!(pcb, "  (footprint \"{}:{}\"", comp.footprint_lib, comp.footprint_name).unwrap();
    writeln!(pcb, "    (layer \"{}\")", comp.layer).unwrap();
    writeln!(pcb, "    (tstamp \"{}\")", next_uuid()).unwrap();
    writeln!(pcb, "    (at {} {} {})", comp.x, comp.y, comp.rotation).unwrap();

    // Reference on F.Fab (not F.SilkS — avoids silk_over_copper violations)
    writeln!(
        pcb,
        "    (property \"Reference\" \"{}\" (at 0 -3) (layer \"F.Fab\") (effects (font (size 0.8 0.8) (thickness 0.12))))",
        comp.reference
    ).unwrap();

    writeln!(
        pcb,
        "    (property \"Value\" \"{}\" (at 0 3) (layer \"F.Fab\") (effects (font (size 0.8 0.8) (thickness 0.12))))",
        comp.value
    ).unwrap();

    if !comp.lcsc.is_empty() {
        writeln!(
            pcb,
            "    (property \"LCSC\" \"{}\" (at 0 4.5) (layer \"F.Fab\") (effects (font (size 0.5 0.5) (thickness 0.08))))",
            comp.lcsc
        ).unwrap();
    }

    for pad in &comp.pads {
        if let Some(drill) = pad.drill {
            writeln!(
                pcb,
                "    (pad \"{}\" {} {} (at {} {}) (size {} {}) (drill {}) (layers {}) (net {} \"{}\"))",
                pad.number, pad.pad_type, pad.shape,
                pad.x, pad.y, pad.width, pad.height,
                drill, pad.layers, pad.net_id, pad.net_name
            ).unwrap();
        } else {
            writeln!(
                pcb,
                "    (pad \"{}\" {} {} (at {} {}) (size {} {}) (layers {}) (net {} \"{}\"))",
                pad.number, pad.pad_type, pad.shape,
                pad.x, pad.y, pad.width, pad.height,
                pad.layers, pad.net_id, pad.net_name
            ).unwrap();
        }
    }

    pcb.push_str("  )\n\n");
}

// ─── Pad Helpers ───

pub fn esp32_pads() -> Vec<Pad> {
    let mut pads = Vec::new();
    pads.push(Pad {
        number: "GND", pad_type: "smd", shape: "rect",
        x: 0.0, y: 0.0, width: 6.0, height: 6.0,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_GND, net_name: "GND", drill: None,
    });
    pads.push(Pad {
        number: "2", pad_type: "smd", shape: "rect",
        x: -8.0, y: -10.0, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_3V3, net_name: "+3V3", drill: None,
    });
    // Pin 3 = EN: external RC pull-up, not tied directly to 3V3
    pads.push(Pad {
        number: "3", pad_type: "smd", shape: "rect",
        x: -8.0, y: -8.73, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_ESP_EN, net_name: "ESP_EN", drill: None,
    });
    pads.push(Pad {
        number: "15", pad_type: "smd", shape: "rect",
        x: -8.0, y: 6.37, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_HEATER_PWM, net_name: "HEATER_PWM", drill: None,
    });
    pads.push(Pad {
        number: "16", pad_type: "smd", shape: "rect",
        x: -8.0, y: 7.64, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_SDA, net_name: "SDA", drill: None,
    });
    pads.push(Pad {
        number: "17", pad_type: "smd", shape: "rect",
        x: -8.0, y: 8.91, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_SCL, net_name: "SCL", drill: None,
    });
    pads.push(Pad {
        number: "18", pad_type: "smd", shape: "rect",
        x: -8.0, y: 10.18, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_MUX_S0, net_name: "MUX_S0", drill: None,
    });
    pads.push(Pad {
        number: "19", pad_type: "smd", shape: "rect",
        x: -8.0, y: 11.45, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_MUX_S1, net_name: "MUX_S1", drill: None,
    });
    pads.push(Pad {
        number: "20", pad_type: "smd", shape: "rect",
        x: -8.0, y: 12.72, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_MUX_S2, net_name: "MUX_S2", drill: None,
    });
    // LED base drives on right side of module.
    // Pins 21-26 → LB0-5, pin 30 → LB6, pin 28 → LB7.
    // Pin 27 is reserved for ESP_GPIO0 (defined separately below).
    // Pin 30 chosen for LB6 to avoid MUX_COM B.Cu at y=20 (pin 29 y=20.16 too close).
    let lb_pins: [u32; 8] = [21, 22, 23, 24, 25, 26, 30, 28];
    for i in 0..8u32 {
        let pin = lb_pins[i as usize];
        let pin_str = Box::leak(format!("{}", pin).into_boxed_str());
        let net_name_str = Box::leak(format!("LED_BASE_{}", i).into_boxed_str());
        // Right-side y: pin N → y = -10.0 + (N - 21) * 1.27
        let pin_y = -10.0 + (pin as f64 - 21.0) * 1.27;
        pads.push(Pad {
            number: pin_str, pad_type: "smd", shape: "rect",
            x: 8.0, y: pin_y, width: 1.5, height: 0.9,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_LED_BASE_0 + i,
            net_name: net_name_str,
            drill: None,
        });
    }
    pads.push(Pad {
        number: "13", pad_type: "smd", shape: "rect",
        x: -8.0, y: 3.83, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_USB_DN, net_name: "USB_DN", drill: None,
    });
    pads.push(Pad {
        number: "14", pad_type: "smd", shape: "rect",
        x: -8.0, y: 5.10, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"", net_id: NET_USB_DP, net_name: "USB_DP", drill: None,
    });
    // Right side continuation: pins 29+ at x=8.0
    // Pin 27 = GPIO0 (right side, y=-10.0 + 6*1.27 = -2.38)
    pads.push(Pad {
        number: "27", pad_type: "smd", shape: "rect",
        x: 8.0, y: -10.0 + 6.0 * 1.27, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
        net_id: NET_ESP_GPIO0, net_name: "ESP_GPIO0", drill: None,
    });
    // Pin 36 = U0RXD (right side, y=-10.0 + 15*1.27 = 9.05)
    pads.push(Pad {
        number: "36", pad_type: "smd", shape: "rect",
        x: 8.0, y: -10.0 + 15.0 * 1.27, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
        net_id: NET_UART_RX, net_name: "UART_RX", drill: None,
    });
    // Pin 37 = U0TXD (right side, y=-10.0 + 16*1.27 = 10.32)
    pads.push(Pad {
        number: "37", pad_type: "smd", shape: "rect",
        x: 8.0, y: -10.0 + 16.0 * 1.27, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
        net_id: NET_UART_TX, net_name: "UART_TX", drill: None,
    });
    // Pin 35 = spare GPIO for activity LED (right side, y=-10.0 + 14*1.27 = 7.78)
    pads.push(Pad {
        number: "35", pad_type: "smd", shape: "rect",
        x: 8.0, y: -10.0 + 14.0 * 1.27, width: 1.5, height: 0.9,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
        net_id: NET_ESP_GPIO_ACT, net_name: "ESP_GPIO_ACT", drill: None,
    });
    pads
}

pub fn msop10_pads() -> Vec<Pad> {
    let pin_nets: [(u32, &str); 10] = [
        (NET_GND, "GND"),
        (NET_UNCONNECTED, ""),
        (NET_GND, "GND"),
        (NET_MUX_COM, "MUX_COM"),
        (NET_ADC_AIN1, "ADC_AIN1"),
        (NET_UNCONNECTED, ""),
        (NET_UNCONNECTED, ""),
        (NET_3V3, "+3V3"),
        (NET_SDA, "SDA"),
        (NET_SCL, "SCL"),
    ];
    let mut pads = Vec::new();
    for i in 0..10 {
        let pin = i + 1;
        let (x, y) = if pin <= 5 {
            (-1.5, -1.0 + (pin as f64 - 1.0) * 0.5)
        } else {
            (1.5, -1.0 + (10.0 - pin as f64) * 0.5)
        };
        let pin_str = Box::leak(format!("{}", pin).into_boxed_str());
        let (net_id, net_name) = pin_nets[i];
        pads.push(Pad {
            number: pin_str, pad_type: "smd", shape: "rect",
            x, y, width: 1.0, height: 0.3,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id, net_name, drill: None,
        });
    }
    pads
}

pub fn soic16w_pads() -> Vec<Pad> {
    let pin_nets: [(u32, &str); 16] = [
        (NET_MUX_Y4, "MUX_Y4"),
        (NET_MUX_Y6, "MUX_Y6"),
        (NET_MUX_COM, "MUX_COM"),
        (NET_MUX_Y7, "MUX_Y7"),
        (NET_MUX_Y5, "MUX_Y5"),
        (NET_GND, "GND"),
        (NET_GND, "GND"),
        (NET_GND, "GND"),
        (NET_MUX_S2, "MUX_S2"),
        (NET_MUX_S1, "MUX_S1"),
        (NET_MUX_S0, "MUX_S0"),
        (NET_MUX_Y3, "MUX_Y3"),
        (NET_MUX_Y0, "MUX_Y0"),
        (NET_MUX_Y1, "MUX_Y1"),
        (NET_MUX_Y2, "MUX_Y2"),
        (NET_3V3, "+3V3"),
    ];
    let mut pads = Vec::new();
    for i in 0..16 {
        let pin = i + 1;
        let (x, y) = if pin <= 8 {
            (-4.8, -4.445 + (pin as f64 - 1.0) * 1.27)
        } else {
            (4.8, -4.445 + (16.0 - pin as f64) * 1.27)
        };
        let pin_str = Box::leak(format!("{}", pin).into_boxed_str());
        let (net_id, net_name) = pin_nets[i];
        pads.push(Pad {
            number: pin_str, pad_type: "smd", shape: "rect",
            x, y, width: 1.5, height: 0.6,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id, net_name, drill: None,
        });
    }
    pads
}

pub fn soic8_pads() -> Vec<Pad> {
    // TC4427A SOIC-8 pinout:
    //   Pin 1: INA      Pin 8: NC
    //   Pin 2: NC       Pin 7: INB
    //   Pin 3: GND      Pin 6: VDD
    //   Pin 4: OUTA     Pin 5: OUTB
    // Both channels driven in parallel for doubled gate drive current.
    let pin_nets: [(u32, &str); 8] = [
        (NET_HEATER_PWM, "HEATER_PWM"),  // Pin 1: INA
        (NET_UNCONNECTED, ""),            // Pin 2: NC
        (NET_GND, "GND"),                 // Pin 3: GND
        (NET_GATE_DRV, "GATE_DRV"),       // Pin 4: OUTA → series resistor → MOSFET gate
        (NET_GATE_DRV, "GATE_DRV"),       // Pin 5: OUTB → paralleled with OUTA
        (NET_5V, "+5V"),                  // Pin 6: VDD
        (NET_HEATER_PWM, "HEATER_PWM"),  // Pin 7: INB → paralleled with INA
        (NET_UNCONNECTED, ""),            // Pin 8: NC
    ];
    let mut pads = Vec::new();
    for i in 0..8 {
        let pin = i + 1;
        let (x, y) = if pin <= 4 {
            (-2.7, -1.905 + (pin as f64 - 1.0) * 1.27)
        } else {
            (2.7, -1.905 + (8.0 - pin as f64) * 1.27)
        };
        let pin_str = Box::leak(format!("{}", pin).into_boxed_str());
        let (net_id, net_name) = pin_nets[i];
        pads.push(Pad {
            number: pin_str, pad_type: "smd", shape: "rect",
            x, y, width: 1.5, height: 0.6,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id, net_name, drill: None,
        });
    }
    pads
}

pub fn sot23_pads_mosfet() -> Vec<Pad> {
    // AO3400A: 1=Gate, 2=Source(GND), 3=Drain(HEATER_P)
    // Drain connects to serpentine END — current flows through serpentine from HEATER_P to HEATER_P.
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -0.95, y: 1.0, width: 0.9, height: 0.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_MOSFET_GATE, net_name: "MOSFET_GATE", drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 0.95, y: 1.0, width: 0.9, height: 0.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "3", pad_type: "smd", shape: "rect",
            x: 0.0, y: -1.0, width: 0.9, height: 0.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_HEATER_P, net_name: "HEATER_P", drill: None,
        },
    ]
}

pub fn sot23_pads_npn(slot: usize) -> Vec<Pad> {
    let base_net_name = Box::leak(format!("LED_BASE_{}", slot).into_boxed_str());
    // Fix: collector connects to LED cathode, not LED_COL (corrected topology)
    let cath_net_name = Box::leak(format!("LED_CATH_{}", slot).into_boxed_str());
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -0.95, y: 1.0, width: 0.9, height: 0.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_LED_BASE_0 + slot as u32, net_name: base_net_name, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 0.95, y: 1.0, width: 0.9, height: 0.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "3", pad_type: "smd", shape: "rect",
            x: 0.0, y: -1.0, width: 0.9, height: 0.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_LED_CATH_0 + slot as u32, net_name: cath_net_name, drill: None,
        },
    ]
}

pub fn led_0805_pads(slot: usize) -> Vec<Pad> {
    let col_net_name = Box::leak(format!("LED_COL_{}", slot).into_boxed_str());
    // Fix: pad2 (cathode) connects to LED_CATH, not GND
    // Correct topology: +5V → R(100R) → LED anode → LED cathode → NPN collector → GND
    let cath_net_name = Box::leak(format!("LED_CATH_{}", slot).into_boxed_str());
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -0.95, y: 0.0, width: 1.0, height: 1.2,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_LED_COL_0 + slot as u32, net_name: col_net_name, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 0.95, y: 0.0, width: 1.0, height: 1.2,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_LED_CATH_0 + slot as u32, net_name: cath_net_name, drill: None,
        },
    ]
}

pub fn photodiode_pads(slot: usize) -> Vec<Pad> {
    let mux_net_name = Box::leak(format!("MUX_Y{}", slot).into_boxed_str());
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -1.1, y: 0.0, width: 1.2, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_MUX_Y0 + slot as u32, net_name: mux_net_name, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 1.1, y: 0.0, width: 1.2, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
    ]
}

pub fn r0805_pads(net1: u32, name1: &'static str, net2: u32, name2: &'static str) -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "roundrect",
            x: -0.95, y: 0.0, width: 1.0, height: 1.2,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net1, net_name: name1, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "roundrect",
            x: 0.95, y: 0.0, width: 1.0, height: 1.2,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net2, net_name: name2, drill: None,
        },
    ]
}

pub fn c0603_pads(net1: u32, name1: &'static str, net2: u32, name2: &'static str) -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "roundrect",
            x: -0.75, y: 0.0, width: 0.8, height: 0.95,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net1, net_name: name1, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "roundrect",
            x: 0.75, y: 0.0, width: 0.8, height: 0.95,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net2, net_name: name2, drill: None,
        },
    ]
}

pub fn sma_diode_pads() -> Vec<Pad> {
    // SS34 flyback: anode to HEATER_P (Q1 drain side), cathode to 12V
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -2.0, y: 0.0, width: 1.5, height: 2.3,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_HEATER_P, net_name: "HEATER_P", drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 2.0, y: 0.0, width: 1.5, height: 2.3,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_12V, net_name: "+12V", drill: None,
        },
    ]
}

pub fn usbc_pads() -> Vec<Pad> {
    vec![
        Pad {
            number: "A1", pad_type: "smd", shape: "rect",
            x: -2.75, y: 0.0, width: 0.6, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "A4", pad_type: "smd", shape: "rect",
            x: -0.75, y: 0.0, width: 0.3, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_VBUS, net_name: "VBUS", drill: None,
        },
        Pad {
            number: "A6", pad_type: "smd", shape: "rect",
            x: 0.25, y: 0.0, width: 0.3, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_DP, net_name: "USB_DP", drill: None,
        },
        Pad {
            number: "A7", pad_type: "smd", shape: "rect",
            x: 0.75, y: 0.0, width: 0.3, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_DN, net_name: "USB_DN", drill: None,
        },
        Pad {
            number: "A12", pad_type: "smd", shape: "rect",
            x: 2.75, y: 0.0, width: 0.6, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "S1", pad_type: "smd", shape: "rect",
            x: -4.32, y: -1.5, width: 1.0, height: 2.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "S2", pad_type: "smd", shape: "rect",
            x: 4.32, y: -1.5, width: 1.0, height: 2.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        // CC1 (A5) and CC2 (B5) for USB-C device detection
        Pad {
            number: "A5", pad_type: "smd", shape: "rect",
            x: -0.25, y: 0.0, width: 0.3, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_CC1, net_name: "USB_CC1", drill: None,
        },
        // B5 (CC2) on B-row: offset in y to separate from A-row pads.
        // B-row at y=1.2 local → abs y = 5 - 1.2 = 3.8 (north of A-row at y=5).
        // A-row pad north edge at abs y=4.5. B5 south edge at abs y=3.8+0.5=4.3.
        // Gap = 4.5 - 4.3 = 0.2mm. OK for clearance.
        Pad {
            number: "B5", pad_type: "smd", shape: "rect",
            x: 0.5, y: 1.2, width: 0.3, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_CC2, net_name: "USB_CC2", drill: None,
        },
    ]
}

pub fn barrel_jack_pads() -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: 0.0, y: -4.0, width: 2.5, height: 2.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_12V_RAW, net_name: "+12V_RAW", drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 0.0, y: 4.0, width: 2.5, height: 2.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "3", pad_type: "smd", shape: "rect",
            x: 5.0, y: 0.0, width: 2.0, height: 2.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
    ]
}

pub fn pin_header_2_pads(net1: u32, name1: &'static str, net2: u32, name2: &'static str) -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "thru_hole", shape: "circle",
            x: 0.0, y: -1.27, width: 1.7, height: 1.7,
            layers: "\"*.Cu\" \"*.Mask\"",
            net_id: net1, net_name: name1, drill: Some(1.0),
        },
        Pad {
            number: "2", pad_type: "thru_hole", shape: "circle",
            x: 0.0, y: 1.27, width: 1.7, height: 1.7,
            layers: "\"*.Cu\" \"*.Mask\"",
            net_id: net2, net_name: name2, drill: Some(1.0),
        },
    ]
}

// ── SOT-223 pads (AMS1117 voltage regulators) ──
// Pin 1=GND/Adjust, Pin 2=Vout, Pin 3=Vin, Tab=Vout
pub fn sot223_pads(vin_net: u32, vin_name: &'static str,
                   vout_net: u32, vout_name: &'static str) -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -2.3, y: 1.0, width: 1.0, height: 1.5,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 0.0, y: 1.0, width: 1.0, height: 1.5,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: vout_net, net_name: vout_name, drill: None,
        },
        Pad {
            number: "3", pad_type: "smd", shape: "rect",
            x: 2.3, y: 1.0, width: 1.0, height: 1.5,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: vin_net, net_name: vin_name, drill: None,
        },
        Pad {
            number: "4", pad_type: "smd", shape: "rect",
            x: 0.0, y: -1.5, width: 3.0, height: 2.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: vout_net, net_name: vout_name, drill: None,
        },
    ]
}

// ── 1206 capacitor pads ──
pub fn c1206_pads(net1: u32, name1: &'static str, net2: u32, name2: &'static str) -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "roundrect",
            x: -1.5, y: 0.0, width: 1.2, height: 1.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net1, net_name: name1, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "roundrect",
            x: 1.5, y: 0.0, width: 1.2, height: 1.8,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net2, net_name: name2, drill: None,
        },
    ]
}

// ── Tactile switch pads (6mm SMD, LCSC C318884) ──
pub fn tactile_switch_pads(net1: u32, name1: &'static str,
                           net2: u32, name2: &'static str) -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -3.25, y: 0.0, width: 1.5, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net1, net_name: name1, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 3.25, y: 0.0, width: 1.5, height: 1.0,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net2, net_name: name2, drill: None,
        },
    ]
}

// ── Pin header 1x4 pads (UART debug header) ──
pub fn pin_header_4_pads(nets: [(u32, &'static str); 4]) -> Vec<Pad> {
    let mut pads = Vec::new();
    for (i, (net, name)) in nets.iter().enumerate() {
        let pin_str = Box::leak(format!("{}", i + 1).into_boxed_str());
        pads.push(Pad {
            number: pin_str, pad_type: "thru_hole", shape: "circle",
            x: 0.0, y: -3.81 + i as f64 * 2.54, width: 1.7, height: 1.7,
            layers: "\"*.Cu\" \"*.Mask\"",
            net_id: *net, net_name: name, drill: Some(1.0),
        });
    }
    pads
}

// ── SOT-23-6 pads (USBLC6-2SC6 ESD protection) ──
pub fn sot236_pads_esd() -> Vec<Pad> {
    // USBLC6-2SC6: 1=I/O1(DP), 2=GND, 3=I/O2(DN), 4=I/O2(DN), 5=VBUS, 6=I/O1(DP)
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -0.95, y: 1.0, width: 0.6, height: 0.7,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_DP, net_name: "USB_DP", drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 0.0, y: 1.0, width: 0.6, height: 0.7,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_GND, net_name: "GND", drill: None,
        },
        Pad {
            number: "3", pad_type: "smd", shape: "rect",
            x: 0.95, y: 1.0, width: 0.6, height: 0.7,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_DN, net_name: "USB_DN", drill: None,
        },
        Pad {
            number: "4", pad_type: "smd", shape: "rect",
            x: 0.95, y: -1.0, width: 0.6, height: 0.7,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_DN, net_name: "USB_DN", drill: None,
        },
        Pad {
            number: "5", pad_type: "smd", shape: "rect",
            x: 0.0, y: -1.0, width: 0.6, height: 0.7,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_VBUS, net_name: "VBUS", drill: None,
        },
        Pad {
            number: "6", pad_type: "smd", shape: "rect",
            x: -0.95, y: -1.0, width: 0.6, height: 0.7,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_USB_DP, net_name: "USB_DP", drill: None,
        },
    ]
}

// ── SMA diode pads for reverse polarity protection ──
// D10: anode=12V_RAW, cathode=12V
pub fn sma_diode_pads_polarity() -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -2.0, y: 0.0, width: 1.5, height: 2.3,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_12V_RAW, net_name: "+12V_RAW", drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 2.0, y: 0.0, width: 1.5, height: 2.3,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_12V, net_name: "+12V", drill: None,
        },
    ]
}

// ── SMA diode pads for VBUS → 5V power path ──
// D14: anode=VBUS, cathode=+5V
pub fn sma_diode_pads_vbus() -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "rect",
            x: -2.0, y: 0.0, width: 1.5, height: 2.3,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_VBUS, net_name: "VBUS", drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "rect",
            x: 2.0, y: 0.0, width: 1.5, height: 2.3,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: NET_5V, net_name: "+5V", drill: None,
        },
    ]
}

// ── 0603 resistor pads (same as c0603 but for resistors) ──
pub fn r0603_pads(net1: u32, name1: &'static str, net2: u32, name2: &'static str) -> Vec<Pad> {
    vec![
        Pad {
            number: "1", pad_type: "smd", shape: "roundrect",
            x: -0.75, y: 0.0, width: 0.8, height: 0.95,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net1, net_name: name1, drill: None,
        },
        Pad {
            number: "2", pad_type: "smd", shape: "roundrect",
            x: 0.75, y: 0.0, width: 0.8, height: 0.95,
            layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
            net_id: net2, net_name: name2, drill: None,
        },
    ]
}
