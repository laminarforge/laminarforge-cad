#![allow(dead_code, unused_variables)]
//! Controller PCB for the chip-stack thermal container (chip_incubator_v2 firmware).
//!
//! 2-layer 100x80mm PCB housed in an external control box next to the
//! thermal container. Drives all actuators and reads all sensors defined
//! in artifact A-E798C84D (Chip Incubator v2 firmware spec). GPIO assignments
//! below align with the firmware pin map from §1 of the spec.
//!
//! Deliverables:
//!   pcb/output/controller.kicad_pcb  — KiCad PCB (unrouted, ready for import)
//!   pcb/output/controller.dsn        — Specctra DSN for Freerouting
//!   pcb/output/controller_bom.csv    — bill of materials
//!   pcb/output/controller_cpl.csv    — component placement (pick-and-place)
//!
//! The module uses its own component/net list (decoupled from the lamp_v1
//! PCB), but reuses the existing `pcb::Component`/`pcb::Pad` types from
//! laminarforge_cad::pcb so pad geometry helpers remain compatible.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use laminarforge_cad::pcb::{next_uuid, Component, Pad};

// ───────────────────────── Board dimensions ─────────────────────────

const BOARD_W: f64 = 100.0; // X, mm
const BOARD_H: f64 = 80.0; // Y, mm

// Mounting hole inset
const MOUNT_INSET: f64 = 4.0;

const MOUNT_HOLES: &[(&str, f64, f64)] = &[
    ("H1", MOUNT_INSET, MOUNT_INSET),
    ("H2", BOARD_W - MOUNT_INSET, MOUNT_INSET),
    ("H3", MOUNT_INSET, BOARD_H - MOUNT_INSET),
    ("H4", BOARD_W - MOUNT_INSET, BOARD_H - MOUNT_INSET),
];

// ───────────────────────── Nets ─────────────────────────
//
// Net 0 is always the "unconnected" net.

const NET_UNCONNECTED: u32 = 0;
const NET_GND: u32 = 1;
const NET_24V: u32 = 2; // input rail
const NET_24V_SW: u32 = 3; // after reverse-polarity/TVS
const NET_12V: u32 = 4; // regulated/passthrough to fan + solenoid
const NET_5V: u32 = 5; // buck output
const NET_3V3: u32 = 6; // LDO output
const NET_VBUS: u32 = 7; // USB-C 5V
const NET_HEATER_SSR_MCU: u32 = 8; // from MCU GPIO4, before safety interlock
const NET_HEATER_SSR_OUT: u32 = 9; // to Crydom DC60S5 control input (positive)
const NET_FAN_PWM: u32 = 10;
const NET_FAN_GATE: u32 = 11;
const NET_FAN_OUT_N: u32 = 12; // low side of fan (MOSFET drain)
const NET_CO2_SOL_PWM: u32 = 13;
const NET_CO2_SOL_OUT_N: u32 = 14; // low side of solenoid
const NET_WINDOW_HEATER: u32 = 15; // MCU GPIO11, low-current on/off
const NET_SDA: u32 = 16;
const NET_SCL: u32 = 17;
const NET_DS18B20_DATA: u32 = 18; // 1-Wire
const NET_OVERTEMP_SW: u32 = 19; // KSD9700 switch signal to MCU GPIO10
const NET_MHZ19_TX: u32 = 20; // sensor TX → MCU RX
const NET_MHZ19_RX: u32 = 21; // sensor RX ← MCU TX
const NET_STEP: u32 = 22;
const NET_DIR: u32 = 23;
const NET_EN_N: u32 = 24; // TMC2209 EN, active low
const NET_TMC_UART_TX: u32 = 25;
const NET_TMC_UART_RX: u32 = 26;
const NET_USB_DP: u32 = 27;
const NET_USB_DN: u32 = 28;
const NET_USB_CC1: u32 = 29;
const NET_USB_CC2: u32 = 30;
const NET_ESP_EN: u32 = 31;
const NET_ESP_BOOT: u32 = 32; // GPIO0 strapping
const NET_LED_PWR: u32 = 33; // power-indicator cathode side
const NET_LED_WIFI: u32 = 34;
const NET_LED_HEAT: u32 = 35;
const NET_LED_FAULT: u32 = 36;
const NET_LED_WIFI_DRV: u32 = 37; // MCU GPIO driving WiFi LED
const NET_LED_HEAT_DRV: u32 = 38; // MCU GPIO driving heater LED
const NET_LED_FAULT_DRV: u32 = 39; // MCU GPIO driving fault LED
const NET_SWCLK: u32 = 40; // JTAG/SWD header
const NET_SWDIO: u32 = 41;
const NET_SWO: u32 = 42;
const NET_OT_A: u32 = 43; // KSD9700 side-A (series with SSR line before thermostat)
const NET_HEATER_SSR_PRE: u32 = 44; // post-MCU GPIO, pre-overtemp interlock

const TOTAL_NETS: u32 = 45;

const NET_NAMES: &[&str] = &[
    "",               // 0
    "GND",            // 1
    "+24V_IN",        // 2
    "+24V_SW",        // 3
    "+12V",           // 4
    "+5V",            // 5
    "+3V3",           // 6
    "VBUS",           // 7
    "HEATER_SSR_MCU", // 8
    "HEATER_SSR_OUT", // 9
    "FAN_PWM",        // 10
    "FAN_GATE",       // 11
    "FAN_OUT_N",      // 12
    "CO2_SOL_PWM",    // 13
    "CO2_SOL_OUT_N",  // 14
    "WINDOW_HEATER",  // 15
    "SDA",            // 16
    "SCL",            // 17
    "DS18B20_DATA",   // 18
    "OVERTEMP_SW",    // 19
    "MHZ19_TX",       // 20
    "MHZ19_RX",       // 21
    "STEP",           // 22
    "DIR",            // 23
    "EN_N",           // 24
    "TMC_UART_TX",    // 25
    "TMC_UART_RX",    // 26
    "USB_DP",         // 27
    "USB_DN",         // 28
    "USB_CC1",        // 29
    "USB_CC2",        // 30
    "ESP_EN",         // 31
    "ESP_BOOT",       // 32
    "LED_PWR",        // 33
    "LED_WIFI",       // 34
    "LED_HEAT",       // 35
    "LED_FAULT",      // 36
    "LED_WIFI_DRV",   // 37
    "LED_HEAT_DRV",   // 38
    "LED_FAULT_DRV",  // 39
    "SWCLK",          // 40
    "SWDIO",          // 41
    "SWO",            // 42
    "OT_A",           // 43
    "HEATER_SSR_PRE", // 44
];

fn net_name(id: u32) -> &'static str {
    NET_NAMES.get(id as usize).copied().unwrap_or("NET")
}

// ───────────────────────── Pad helpers ─────────────────────────

fn pad_smd(number: &'static str, x: f64, y: f64, w: f64, h: f64, net_id: u32) -> Pad {
    Pad {
        number,
        pad_type: "smd",
        shape: "rect",
        x,
        y,
        width: w,
        height: h,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
        net_id,
        net_name: net_name(net_id),
        drill: None,
    }
}

fn pad_th(number: &'static str, x: f64, y: f64, d_pad: f64, d_drill: f64, net_id: u32) -> Pad {
    Pad {
        number,
        pad_type: "thru_hole",
        shape: "circle",
        x,
        y,
        width: d_pad,
        height: d_pad,
        layers: "\"*.Cu\" \"*.Mask\"",
        net_id,
        net_name: net_name(net_id),
        drill: Some(d_drill),
    }
}

/// 0603 resistor/capacitor pads (2 pads 0.8x0.95, pitch 1.6mm).
fn r0603_pads(net_a: u32, net_b: u32) -> Vec<Pad> {
    vec![
        pad_smd("1", -0.8, 0.0, 0.95, 0.95, net_a),
        pad_smd("2", 0.8, 0.0, 0.95, 0.95, net_b),
    ]
}

/// 0805 pads (LED or larger passive).
fn r0805_pads(net_a: u32, net_b: u32) -> Vec<Pad> {
    vec![
        pad_smd("1", -1.0, 0.0, 1.2, 1.4, net_a),
        pad_smd("2", 1.0, 0.0, 1.2, 1.4, net_b),
    ]
}

/// SOT-23 MOSFET (IRLZ44N is actually TO-220; use as through-hole module).
/// Generic 3-lead SOT-23: 1=G, 2=S, 3=D.
fn sot23_pads(net_gate: u32, net_src: u32, net_drain: u32) -> Vec<Pad> {
    vec![
        pad_smd("1", -0.95, 1.0, 0.9, 0.8, net_gate),
        pad_smd("2", 0.95, 1.0, 0.9, 0.8, net_src),
        pad_smd("3", 0.0, -1.0, 0.9, 0.8, net_drain),
    ]
}

// ───────────────────────── Component builders ─────────────────────────

/// ESP32-S3-WROOM-1-N8R8 module. Pads laid out on castellated edges.
/// 18-pin left side, 18-pin right side, 3-pin bottom, plus center GND pad.
/// Pitch 1.27mm. Body 18x25.5mm, pads 1.5x0.9mm at x = ±8.5mm.
///
/// Pin map below matches the firmware spec §1.
fn esp32_pads() -> Vec<Pad> {
    let mut pads = Vec::new();

    // Center GND paddle
    pads.push(pad_smd("1", 0.0, 0.0, 6.0, 6.0, NET_GND));

    // Some key pins we actually use. GPIO numbers drive net selection.
    // Left-edge pin geometry (physical pins 2..19 at x=-8.5, y=-10.8..+10.8 by 1.27mm).
    // Right-edge pin geometry (physical pins 20..37 at x=+8.5, y=+10.8..-10.8 by -1.27mm).
    //
    // We place only pads for pins we actually connect. KiCad is happy without
    // the full set; DRC treats unused pads as unrouted if we omitted them.

    let left_y = |p: i32| -10.8 + (p - 2) as f64 * 1.27; // pin 2..19
    let right_y = |p: i32| 10.8 - (p - 20) as f64 * 1.27; // pin 20..37

    // Power pins (left side)
    pads.push(pad_smd("2", -8.5, left_y(2), 1.5, 0.9, NET_3V3)); // pin 2: 3V3
    pads.push(pad_smd("3", -8.5, left_y(3), 1.5, 0.9, NET_ESP_EN)); // pin 3: EN
    pads.push(pad_smd("15", -8.5, left_y(15), 1.5, 0.9, NET_GND)); // pin 15: GND

    // GPIO bank 1 (left side, pins 4..13 → GPIO4..12)
    pads.push(pad_smd("4", -8.5, left_y(4), 1.5, 0.9, NET_HEATER_SSR_MCU)); // GPIO4  heater SSR
    pads.push(pad_smd("5", -8.5, left_y(5), 1.5, 0.9, NET_FAN_PWM)); // GPIO5  fan
    pads.push(pad_smd("6", -8.5, left_y(6), 1.5, 0.9, NET_CO2_SOL_PWM)); // GPIO6  CO2 solenoid
    pads.push(pad_smd("7", -8.5, left_y(7), 1.5, 0.9, NET_SCL)); // GPIO7  SCL
    pads.push(pad_smd("8", -8.5, left_y(8), 1.5, 0.9, NET_SDA)); // GPIO8  SDA
    pads.push(pad_smd("9", -8.5, left_y(9), 1.5, 0.9, NET_DS18B20_DATA)); // GPIO9  1-Wire
    pads.push(pad_smd("10", -8.5, left_y(10), 1.5, 0.9, NET_OVERTEMP_SW)); // GPIO10 bimetallic trip
    pads.push(pad_smd("11", -8.5, left_y(11), 1.5, 0.9, NET_WINDOW_HEATER)); // GPIO11 window heater
    pads.push(pad_smd("12", -8.5, left_y(12), 1.5, 0.9, NET_LED_HEAT_DRV)); // GPIO12 heat LED
    pads.push(pad_smd("13", -8.5, left_y(13), 1.5, 0.9, NET_STEP)); // GPIO13 STEP
    pads.push(pad_smd("14", -8.5, left_y(14), 1.5, 0.9, NET_DIR)); // GPIO14 DIR

    // GPIO bank 2 (right side)
    pads.push(pad_smd("20", 8.5, right_y(20), 1.5, 0.9, NET_EN_N)); // GPIO15 EN_N
    pads.push(pad_smd("21", 8.5, right_y(21), 1.5, 0.9, NET_TMC_UART_TX)); // GPIO16
    pads.push(pad_smd("22", 8.5, right_y(22), 1.5, 0.9, NET_MHZ19_RX)); // GPIO17
    pads.push(pad_smd("23", 8.5, right_y(23), 1.5, 0.9, NET_MHZ19_TX)); // GPIO18
    pads.push(pad_smd("24", 8.5, right_y(24), 1.5, 0.9, NET_USB_DN)); // GPIO19 USB_D-
    pads.push(pad_smd("25", 8.5, right_y(25), 1.5, 0.9, NET_USB_DP)); // GPIO20 USB_D+
    pads.push(pad_smd("26", 8.5, right_y(26), 1.5, 0.9, NET_TMC_UART_RX)); // GPIO21
    pads.push(pad_smd("27", 8.5, right_y(27), 1.5, 0.9, NET_ESP_BOOT)); // GPIO0  strapping
    pads.push(pad_smd("28", 8.5, right_y(28), 1.5, 0.9, NET_LED_WIFI_DRV)); // GPIO45
    pads.push(pad_smd("29", 8.5, right_y(29), 1.5, 0.9, NET_LED_FAULT_DRV)); // GPIO46

    pads
}

/// SOIC-8 footprint (used for AMS1117-5V LDO placeholder, regulator, etc.)
fn soic8_pads_generic(nets: [u32; 8]) -> Vec<Pad> {
    let mut pads = Vec::with_capacity(8);
    for i in 0..8 {
        let pin = (i + 1) as i32;
        let (x, y) = if pin <= 4 {
            (-2.7, -1.905 + (pin as f64 - 1.0) * 1.27)
        } else {
            (2.7, -1.905 + (8.0 - pin as f64) * 1.27)
        };
        let num = Box::leak(format!("{}", pin).into_boxed_str());
        pads.push(pad_smd(num, x, y, 1.5, 0.6, nets[i]));
    }
    pads
}

/// LM2596 buck regulator (TO-263-5 / D2PAK).
/// Pin 1 = VIN, 2 = OUT, 3 = GND, 4 = FB, 5 = EN, tab = GND.
fn lm2596_pads() -> Vec<Pad> {
    vec![
        pad_smd("1", -3.5, -3.0, 1.4, 0.8, NET_24V_SW),
        pad_smd("2", -1.75, -3.0, 1.4, 0.8, NET_12V), // 12V switched node (post-inductor — simplified)
        pad_smd("3", 0.0, -3.0, 1.4, 0.8, NET_GND),
        pad_smd("4", 1.75, -3.0, 1.4, 0.8, NET_5V), // FB tied to 5V via divider
        pad_smd("5", 3.5, -3.0, 1.4, 0.8, NET_24V_SW),
        // Tab
        pad_smd("TAB", 0.0, 3.0, 9.0, 6.0, NET_GND),
    ]
}

/// AMS1117-3.3 (SOT-223): pin1=GND, pin2=VOUT, pin3=VIN, tab=VOUT.
fn ams1117_pads() -> Vec<Pad> {
    vec![
        pad_smd("1", -2.3, 3.1, 1.6, 0.9, NET_GND),
        pad_smd("2", 0.0, 3.1, 1.6, 0.9, NET_3V3),
        pad_smd("3", 2.3, 3.1, 1.6, 0.9, NET_5V),
        pad_smd("TAB", 0.0, -3.1, 3.5, 1.6, NET_3V3),
    ]
}

/// CH340N USB-UART bridge (SOP-8). Used as USB-to-UART for programming.
/// Pin map: 1=UD+, 2=UD-, 3=GND, 4=V3, 5=TXD, 6=RXD, 7=NC, 8=VCC.
fn ch340n_pads() -> Vec<Pad> {
    soic8_pads_generic([
        NET_USB_DP,      // 1 UD+
        NET_USB_DN,      // 2 UD-
        NET_GND,         // 3 GND
        NET_3V3,         // 4 V3 (internal LDO output, decouple to GND)
        NET_TMC_UART_TX, // 5 TXD (goes to ESP RX) — unused here, we use native USB, but wire to header
        NET_TMC_UART_RX, // 6 RXD — unused
        NET_UNCONNECTED, // 7 NC
        NET_5V,          // 8 VCC
    ])
}

/// USB-C receptacle (simplified 6-pin: VBUS, GND, D+, D-, CC1, CC2).
/// This is a through-hole style approximation; real USB-C would have 16 pins.
fn usbc_pads() -> Vec<Pad> {
    vec![
        pad_th("VBUS", -3.0, 0.0, 1.6, 0.9, NET_VBUS),
        pad_th("GND", -1.0, 0.0, 1.6, 0.9, NET_GND),
        pad_th("D+", 1.0, 0.0, 1.2, 0.7, NET_USB_DP),
        pad_th("D-", 3.0, 0.0, 1.2, 0.7, NET_USB_DN),
        pad_th("CC1", 5.0, 0.0, 1.2, 0.7, NET_USB_CC1),
        pad_th("CC2", -5.0, 0.0, 1.2, 0.7, NET_USB_CC2),
        // Mechanical mounting tabs
        pad_th("M1", -6.5, -2.5, 2.0, 1.2, NET_GND),
        pad_th("M2", 6.5, -2.5, 2.0, 1.2, NET_GND),
    ]
}

/// 2-pin screw terminal (5.08mm pitch).
fn screw_term_2(net_a: u32, net_b: u32) -> Vec<Pad> {
    vec![
        pad_th("1", -2.54, 0.0, 2.6, 1.3, net_a),
        pad_th("2", 2.54, 0.0, 2.6, 1.3, net_b),
    ]
}

/// 3-pin JST-PH remote connector (2.5mm pitch).
fn jst3_pads(net_a: u32, net_b: u32, net_c: u32) -> Vec<Pad> {
    vec![
        pad_th("1", -2.5, 0.0, 1.6, 0.8, net_a),
        pad_th("2", 0.0, 0.0, 1.6, 0.8, net_b),
        pad_th("3", 2.5, 0.0, 1.6, 0.8, net_c),
    ]
}

/// 4-pin JST-PH
fn jst4_pads(a: u32, b: u32, c: u32, d: u32) -> Vec<Pad> {
    vec![
        pad_th("1", -3.75, 0.0, 1.6, 0.8, a),
        pad_th("2", -1.25, 0.0, 1.6, 0.8, b),
        pad_th("3", 1.25, 0.0, 1.6, 0.8, c),
        pad_th("4", 3.75, 0.0, 1.6, 0.8, d),
    ]
}

/// 6-pin 0.1" header for TMC2209 breakout control.
/// Pinout: STEP, DIR, EN, VMOT(+), UART_TX, UART_RX.
fn tmc_header_pads() -> Vec<Pad> {
    let nets = [
        NET_STEP,
        NET_DIR,
        NET_EN_N,
        NET_24V_SW,
        NET_TMC_UART_TX,
        NET_TMC_UART_RX,
    ];
    let mut pads = Vec::with_capacity(6);
    for (i, &n) in nets.iter().enumerate() {
        let num = Box::leak(format!("{}", i + 1).into_boxed_str());
        pads.push(pad_th(num, i as f64 * 2.54 - 6.35, 0.0, 1.7, 1.0, n));
    }
    pads
}

/// 4-pin SWD/JTAG header: SWDIO, SWCLK, SWO, GND + ref 3V3.
fn swd_header_pads() -> Vec<Pad> {
    vec![
        pad_th("1", -3.81, 0.0, 1.7, 1.0, NET_3V3),
        pad_th("2", -1.27, 0.0, 1.7, 1.0, NET_SWDIO),
        pad_th("3", 1.27, 0.0, 1.7, 1.0, NET_SWCLK),
        pad_th("4", 3.81, 0.0, 1.7, 1.0, NET_GND),
    ]
}

/// KSD9700 bimetallic thermostat (normally-closed, opens at 100°C).
/// 2-pin axial device. In series with SSR control line.
fn ksd9700_pads() -> Vec<Pad> {
    vec![
        pad_th("1", -7.5, 0.0, 2.0, 1.0, NET_HEATER_SSR_PRE),
        pad_th("2", 7.5, 0.0, 2.0, 1.0, NET_HEATER_SSR_OUT),
    ]
}

/// DC barrel jack (2.1x5.5mm DC, center-positive).
fn barrel_jack_pads() -> Vec<Pad> {
    vec![
        // Pin 1: +V (center)
        pad_th("1", 0.0, 0.0, 2.5, 1.5, NET_24V),
        // Pin 2: GND (shell)
        pad_th("2", -4.5, 5.7, 2.5, 1.5, NET_GND),
        // Pin 3: switch / GND backup
        pad_th("3", 4.5, 5.7, 2.5, 1.5, NET_GND),
    ]
}

/// LED in 0805 package: cathode, anode.
fn led0805_pads(net_anode: u32, net_cathode: u32) -> Vec<Pad> {
    vec![
        pad_smd("1", -1.0, 0.0, 1.1, 1.2, net_anode), // anode (+)
        pad_smd("2", 1.0, 0.0, 1.1, 1.2, net_cathode), // cathode (−)
    ]
}

/// Generic 2-pin SMA TVS diode (DO-214AC): 1=cathode, 2=anode.
fn sma_tvs_pads(net_k: u32, net_a: u32) -> Vec<Pad> {
    vec![
        pad_smd("1", -2.0, 0.0, 1.8, 2.2, net_k),
        pad_smd("2", 2.0, 0.0, 1.8, 2.2, net_a),
    ]
}

/// IRLZ44N logic-level N-MOSFET in TO-220 (through-hole). Pin 1=G, 2=D, 3=S.
/// Tab is connected to drain.
fn irlz44n_pads(net_gate: u32, net_drain: u32, net_src: u32) -> Vec<Pad> {
    vec![
        pad_th("1", -2.54, 0.0, 2.2, 1.1, net_gate),
        pad_th("2", 0.0, 0.0, 2.2, 1.1, net_drain),
        pad_th("3", 2.54, 0.0, 2.2, 1.1, net_src),
    ]
}

/// Small EMR relay (e.g., G6K-2F, 8-pin surface-mount). Pin map:
/// 1,16 = coil; 4,13 = common; 6,11 = NO; 8,9 = NC. We simplify to 8 pads.
fn relay_pads(coil_plus: u32, coil_minus: u32, common_a: u32, no_a: u32) -> Vec<Pad> {
    vec![
        pad_th("1", -2.54, -3.81, 1.5, 0.9, coil_plus), // coil +
        pad_th("2", 2.54, -3.81, 1.5, 0.9, coil_minus), // coil −
        pad_th("3", -2.54, 0.0, 1.5, 0.9, common_a),    // common
        pad_th("4", 2.54, 0.0, 1.5, 0.9, no_a),         // normally open
        pad_th("5", -2.54, 3.81, 1.5, 0.9, NET_UNCONNECTED),
        pad_th("6", 2.54, 3.81, 1.5, 0.9, NET_UNCONNECTED),
    ]
}

// ───────────────────────── Component list ─────────────────────────

fn define_components() -> Vec<Component> {
    let mut c: Vec<Component> = Vec::new();

    // ── U1: ESP32-S3-WROOM-1-N8R8 (MCU, center-top) ──
    c.push(Component {
        reference: "U1",
        value: "ESP32-S3-WROOM-1-N8R8",
        footprint_lib: "laminarforge",
        footprint_name: "ESP32-S3-WROOM-1",
        lcsc: "C2913204",
        x: 50.0,
        y: 30.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "WiFi/BT/BLE MCU module",
        pads: esp32_pads(),
    });

    // ── U2: LM2596S-5.0 buck (24V → 5V), left side power region ──
    c.push(Component {
        reference: "U2",
        value: "LM2596S-5.0",
        footprint_lib: "laminarforge",
        footprint_name: "TO-263-5",
        lcsc: "C9864",
        x: 15.0,
        y: 18.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Buck regulator 24V->5V 2A",
        pads: lm2596_pads(),
    });

    // ── U3: AMS1117-3.3 LDO (5V → 3.3V) ──
    c.push(Component {
        reference: "U3",
        value: "AMS1117-3.3",
        footprint_lib: "laminarforge",
        footprint_name: "SOT-223",
        lcsc: "C6186",
        x: 30.0,
        y: 18.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "LDO 5V->3.3V 1A",
        pads: ams1117_pads(),
    });

    // ── U4: CH340N USB-UART bridge (programming fallback) ──
    c.push(Component {
        reference: "U4",
        value: "CH340N",
        footprint_lib: "laminarforge",
        footprint_name: "SOP-8",
        lcsc: "C506813",
        x: 85.0,
        y: 40.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "USB-UART bridge",
        pads: ch340n_pads(),
    });

    // ── J1: 24V DC barrel jack input (left edge, top) ──
    // v2 fix: moved from (8, 8) → (10, 5) so pad 3 at (14.5, 10.7) clears U2
    // TO-263-5 pad row (y=14.6) by ~3.15 mm (LM2596S datasheet keep-out).
    c.push(Component {
        reference: "J1",
        value: "DC_BARREL_2.1x5.5mm",
        footprint_lib: "laminarforge",
        footprint_name: "BarrelJack_2.1x5.5mm",
        lcsc: "C3296358",
        x: 10.0,
        y: 5.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "24V DC input",
        pads: barrel_jack_pads(),
    });

    // ── D1: reverse-polarity protection (SS54 schottky), left power region ──
    c.push(Component {
        reference: "D1",
        value: "SS54",
        footprint_lib: "laminarforge",
        footprint_name: "SMA",
        lcsc: "C22452",
        x: 8.0,
        y: 20.0,
        rotation: 90.0,
        layer: "F.Cu",
        description: "Schottky reverse-protection",
        pads: sma_tvs_pads(NET_24V_SW, NET_24V),
    });

    // ── D2: TVS diode (SMBJ30A, 30V) on 24V rail ──
    c.push(Component {
        reference: "D2",
        value: "SMBJ30A",
        footprint_lib: "laminarforge",
        footprint_name: "SMB",
        lcsc: "C8963",
        x: 8.0,
        y: 28.0,
        rotation: 90.0,
        layer: "F.Cu",
        description: "TVS 30V 600W",
        pads: sma_tvs_pads(NET_24V_SW, NET_GND),
    });

    // ── C1: 220uF electrolytic on 24V input ──
    // v2 fix: moved from (18, 10) → (36, 10). Pad 1 at (15.5, 10) collided
    // with J1 pad 3 after J1 was repositioned. New location clears L1 pad 2
    // (x=30) by 2.6 mm and U3 SOT-223 pad row (x 28.25-31.75, y 14.1-15.7).
    c.push(Component {
        reference: "C1",
        value: "220uF_35V",
        footprint_lib: "laminarforge",
        footprint_name: "CP_Radial_D8.0mm",
        lcsc: "C92068",
        x: 36.0,
        y: 10.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "24V input bulk cap",
        pads: vec![
            pad_th("1", -2.5, 0.0, 1.8, 0.9, NET_24V_SW),
            pad_th("2", 2.5, 0.0, 1.8, 0.9, NET_GND),
        ],
    });

    // ── C2: 220uF electrolytic on 5V rail ──
    // v2 fix: moved from (22, 22) → (25, 28). Previous placement put pad 1 at
    // (20, 22) directly on top of U2's TAB (x 10.5-19.5, y 18-24). New position
    // clears U2 TAB by 2.7 mm in x and is 4 mm below it in y.
    c.push(Component {
        reference: "C2",
        value: "220uF_10V",
        footprint_lib: "laminarforge",
        footprint_name: "CP_Radial_D6.3mm",
        lcsc: "C41238",
        x: 25.0,
        y: 28.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "5V rail bulk cap",
        pads: vec![
            pad_th("1", -2.0, 0.0, 1.6, 0.8, NET_5V),
            pad_th("2", 2.0, 0.0, 1.6, 0.8, NET_GND),
        ],
    });

    // ── L1: inductor 33uH for LM2596 output ──
    // v2 fix: moved from (20, 13) → (27, 11). Pad 1 at (24, 11) now clears U2
    // TO-263-5 pad row (x ≤ 18.5, y 14.6-15.4) by 2.6 mm in y and C1's pad 2
    // at (20.5, 10) by 3.5 mm in x.
    c.push(Component {
        reference: "L1",
        value: "33uH_CDRH127",
        footprint_lib: "laminarforge",
        footprint_name: "L_Wurth_WE-PD_7345",
        lcsc: "C1046",
        x: 27.0,
        y: 11.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "LM2596 output inductor",
        pads: vec![
            pad_smd("1", -3.0, 0.0, 2.5, 3.0, NET_12V),
            pad_smd("2", 3.0, 0.0, 2.5, 3.0, NET_5V),
        ],
    });

    // ── C3: 100nF decoupling near ESP32 ──
    for (i, x) in [45.0, 50.0, 55.0].iter().enumerate() {
        let rn = Box::leak(format!("C{}", i + 3).into_boxed_str());
        c.push(Component {
            reference: rn,
            value: "100nF",
            footprint_lib: "laminarforge",
            footprint_name: "C_0603",
            lcsc: "C14663",
            x: *x,
            y: 17.0,
            rotation: 0.0,
            layer: "F.Cu",
            description: "MCU decoupling",
            pads: r0603_pads(NET_3V3, NET_GND),
        });
    }

    // ── C6: 10uF on MCU 3V3 rail ──
    c.push(Component {
        reference: "C6",
        value: "10uF",
        footprint_lib: "laminarforge",
        footprint_name: "C_0805",
        lcsc: "C15850",
        x: 40.0,
        y: 17.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "MCU bulk decoupling",
        pads: r0805_pads(NET_3V3, NET_GND),
    });

    // ── C7..C8: decoupling near CH340 ──
    c.push(Component {
        reference: "C7",
        value: "100nF",
        footprint_lib: "laminarforge",
        footprint_name: "C_0603",
        lcsc: "C14663",
        x: 85.0,
        y: 46.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "CH340 VCC bypass",
        pads: r0603_pads(NET_5V, NET_GND),
    });

    // ── R1, R2: EN pull-up + BOOT pull-up on MCU ──
    // v2 fix: moved from (42, 21) and (58, 21) → (42, 43) and (58, 43).
    // Previous placement overlapped U1 ESP32-S3-WROOM-1 castellated pads
    // (left pads x 40.75-42.25, right pads x 57.75-59.25). New positions are
    // ~1.25 mm below U1's pad row bottom (y=41.25), meeting Espressif's 0.5 mm
    // module keep-out. X preserved so EN/BOOT traces can drop straight down.
    c.push(Component {
        reference: "R1",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C25804",
        x: 42.0,
        y: 43.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "EN pull-up",
        pads: r0603_pads(NET_3V3, NET_ESP_EN),
    });

    c.push(Component {
        reference: "R2",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C25804",
        x: 58.0,
        y: 43.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "BOOT pull-up",
        pads: r0603_pads(NET_3V3, NET_ESP_BOOT),
    });

    // ── R3, R4: I2C pull-ups (4.7k to 3V3) ──
    c.push(Component {
        reference: "R3",
        value: "4.7K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C23162",
        x: 44.0,
        y: 25.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "SDA pull-up",
        pads: r0603_pads(NET_3V3, NET_SDA),
    });

    c.push(Component {
        reference: "R4",
        value: "4.7K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C23162",
        x: 47.0,
        y: 25.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "SCL pull-up",
        pads: r0603_pads(NET_3V3, NET_SCL),
    });

    // ── R5: 4.7k pull-up for DS18B20 1-Wire ──
    c.push(Component {
        reference: "R5",
        value: "4.7K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C23162",
        x: 50.0,
        y: 25.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "1-Wire pull-up",
        pads: r0603_pads(NET_3V3, NET_DS18B20_DATA),
    });

    // ── R6: 1k series for SSR control signal ──
    c.push(Component {
        reference: "R6",
        value: "1K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C21190",
        x: 70.0,
        y: 65.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "SSR control series",
        pads: r0603_pads(NET_HEATER_SSR_MCU, NET_HEATER_SSR_PRE),
    });

    // ── R7: 100R gate resistor for IRLZ44N fan MOSFET ──
    c.push(Component {
        reference: "R7",
        value: "100R",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C22775",
        x: 42.0,
        y: 65.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Fan MOSFET gate resistor",
        pads: r0603_pads(NET_FAN_PWM, NET_FAN_GATE),
    });

    // ── R8: 100k gate pulldown for IRLZ44N ──
    c.push(Component {
        reference: "R8",
        value: "100K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C25741",
        x: 44.0,
        y: 67.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Fan MOSFET gate pulldown",
        pads: r0603_pads(NET_FAN_GATE, NET_GND),
    });

    // ── R9..R12: LED current-limit resistors (330R for 3V3 LEDs) ──
    let led_specs = [
        ("R9", NET_LED_PWR, NET_3V3), // power LED tied to 3V3
        ("R10", NET_LED_WIFI_DRV, NET_LED_WIFI),
        ("R11", NET_LED_HEAT_DRV, NET_LED_HEAT),
        ("R12", NET_LED_FAULT_DRV, NET_LED_FAULT),
    ];
    for (i, (ref_name, net_a, net_b)) in led_specs.iter().enumerate() {
        c.push(Component {
            reference: ref_name,
            value: "330R",
            footprint_lib: "laminarforge",
            footprint_name: "R_0603",
            lcsc: "C23138",
            x: 72.0 + i as f64 * 4.0,
            y: 15.0,
            rotation: 0.0,
            layer: "F.Cu",
            description: "LED current-limit",
            pads: r0603_pads(*net_a, *net_b),
        });
    }

    // ── LEDs ──
    let leds = [
        ("LD1", "GREEN_LED", NET_LED_PWR, NET_GND),   // power
        ("LD2", "BLUE_LED", NET_LED_WIFI, NET_GND),   // wifi
        ("LD3", "YELLOW_LED", NET_LED_HEAT, NET_GND), // heater
        ("LD4", "RED_LED", NET_LED_FAULT, NET_GND),   // fault
    ];
    for (i, (ref_name, val, na, nk)) in leds.iter().enumerate() {
        c.push(Component {
            reference: ref_name,
            value: val,
            footprint_lib: "laminarforge",
            footprint_name: "LED_0805",
            lcsc: "C72041",
            x: 72.0 + i as f64 * 4.0,
            y: 11.0,
            rotation: 0.0,
            layer: "F.Cu",
            description: "Status LED",
            pads: led0805_pads(*na, *nk),
        });
    }

    // ── Q1: IRLZ44N N-MOSFET for 12V blower fan (TO-220 through-hole) ──
    c.push(Component {
        reference: "Q1",
        value: "IRLZ44N",
        footprint_lib: "laminarforge",
        footprint_name: "TO-220-3_Vertical",
        lcsc: "C69224",
        x: 50.0,
        y: 65.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Fan MOSFET",
        pads: irlz44n_pads(NET_FAN_GATE, NET_FAN_OUT_N, NET_GND),
    });

    // ── D3: flyback diode (SS54) across fan ──
    c.push(Component {
        reference: "D3",
        value: "SS54",
        footprint_lib: "laminarforge",
        footprint_name: "SMA",
        lcsc: "C22452",
        x: 48.0,
        y: 70.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Fan flyback diode",
        pads: sma_tvs_pads(NET_12V, NET_FAN_OUT_N),
    });

    // ── Q2: IRLZ44N N-MOSFET for CO2 solenoid driver ──
    c.push(Component {
        reference: "Q2",
        value: "IRLZ44N",
        footprint_lib: "laminarforge",
        footprint_name: "TO-220-3_Vertical",
        lcsc: "C69224",
        x: 62.0,
        y: 65.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "CO2 solenoid MOSFET",
        pads: irlz44n_pads(NET_CO2_SOL_PWM, NET_CO2_SOL_OUT_N, NET_GND),
    });

    // ── D4: flyback diode across solenoid ──
    c.push(Component {
        reference: "D4",
        value: "SS54",
        footprint_lib: "laminarforge",
        footprint_name: "SMA",
        lcsc: "C22452",
        x: 60.0,
        y: 70.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Solenoid flyback diode",
        pads: sma_tvs_pads(NET_12V, NET_CO2_SOL_OUT_N),
    });

    // ── K1: EMR for CO2 solenoid (alternative / high-isolation path) ──
    c.push(Component {
        reference: "K1",
        value: "G6K-2F-Y",
        footprint_lib: "laminarforge",
        footprint_name: "Relay_DPDT_Omron_G6K",
        lcsc: "C37368",
        x: 78.0,
        y: 65.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "5V EMR CO2 solenoid (alt path)",
        pads: relay_pads(NET_5V, NET_CO2_SOL_PWM, NET_12V, NET_CO2_SOL_OUT_N),
    });

    // ── J2: Overtemp thermostat (KSD9700 100°C NC) — safety interlock ──
    c.push(Component {
        reference: "J2",
        value: "KSD9700_100C_NC",
        footprint_lib: "laminarforge",
        footprint_name: "ThermalCutoff_KSD9700",
        lcsc: "C2909862",
        x: 82.0,
        y: 55.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Overtemp cutoff (SSR interlock)",
        pads: ksd9700_pads(),
    });

    // ── J3: SSR output screw terminal (to Crydom DC60S5 +/−) ──
    c.push(Component {
        reference: "J3",
        value: "ScrewTerm_SSR_Ctrl",
        footprint_lib: "laminarforge",
        footprint_name: "ScrewTerm_2.54mm_2P",
        lcsc: "C8735",
        x: 93.0,
        y: 65.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "SSR control output",
        pads: screw_term_2(NET_HEATER_SSR_OUT, NET_GND),
    });

    // ── J4: Fan output screw terminal ──
    c.push(Component {
        reference: "J4",
        value: "ScrewTerm_Fan",
        footprint_lib: "laminarforge",
        footprint_name: "ScrewTerm_2.54mm_2P",
        lcsc: "C8735",
        x: 50.0,
        y: 75.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "12V fan output",
        pads: screw_term_2(NET_12V, NET_FAN_OUT_N),
    });

    // ── J5: Solenoid output screw terminal ──
    c.push(Component {
        reference: "J5",
        value: "ScrewTerm_Solenoid",
        footprint_lib: "laminarforge",
        footprint_name: "ScrewTerm_2.54mm_2P",
        lcsc: "C8735",
        x: 62.0,
        y: 75.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "5V CO2 solenoid output",
        pads: screw_term_2(NET_12V, NET_CO2_SOL_OUT_N),
    });

    // ── J6: SHT40 JST-PH 4-pin (SDA, SCL, 3V3, GND) — top edge ──
    c.push(Component {
        reference: "J6",
        value: "JST_SHT40",
        footprint_lib: "laminarforge",
        footprint_name: "JST_PH_4P",
        lcsc: "C131367",
        x: 25.0,
        y: 5.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "SHT40 remote connector",
        pads: jst4_pads(NET_3V3, NET_SDA, NET_SCL, NET_GND),
    });

    // ── J7: MH-Z19B CO2 sensor JST 4-pin (TX, RX, 5V, GND) ──
    c.push(Component {
        reference: "J7",
        value: "JST_MHZ19",
        footprint_lib: "laminarforge",
        footprint_name: "JST_PH_4P",
        lcsc: "C131367",
        x: 45.0,
        y: 5.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "MH-Z19B remote connector",
        pads: jst4_pads(NET_5V, NET_MHZ19_TX, NET_MHZ19_RX, NET_GND),
    });

    // ── J8: DS18B20 JST 3-pin (DATA, 3V3, GND) ──
    c.push(Component {
        reference: "J8",
        value: "JST_DS18B20",
        footprint_lib: "laminarforge",
        footprint_name: "JST_PH_3P",
        lcsc: "C131365",
        x: 65.0,
        y: 5.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "DS18B20 remote connector",
        pads: jst3_pads(NET_3V3, NET_DS18B20_DATA, NET_GND),
    });

    // ── J9: Window heater / CO2 sensor power JST 2-pin ──
    c.push(Component {
        reference: "J9",
        value: "JST_Window_Heater",
        footprint_lib: "laminarforge",
        footprint_name: "JST_PH_2P",
        lcsc: "C131366",
        x: 82.0,
        y: 5.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "Window anti-condensation heater",
        pads: vec![
            pad_th("1", -1.25, 0.0, 1.6, 0.8, NET_12V),
            pad_th("2", 1.25, 0.0, 1.6, 0.8, NET_WINDOW_HEATER),
        ],
    });

    // ── J10: USB-C receptacle (right edge, center) ──
    c.push(Component {
        reference: "J10",
        value: "USB-C_Receptacle",
        footprint_lib: "laminarforge",
        footprint_name: "USB_C_Receptacle_HRO-TYPE-C-31-M-12",
        lcsc: "C165948",
        x: 95.0,
        y: 40.0,
        rotation: 90.0,
        layer: "F.Cu",
        description: "USB-C programming port",
        pads: usbc_pads(),
    });

    // ── J11: TMC2209 stepper driver breakout header (6-pin, bottom edge) ──
    c.push(Component {
        reference: "J11",
        value: "TMC2209_Header",
        footprint_lib: "laminarforge",
        footprint_name: "PinHeader_1x6_2.54mm",
        lcsc: "C124378",
        x: 25.0,
        y: 72.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "TMC2209 stepper driver breakout",
        pads: tmc_header_pads(),
    });

    // ── J12: SWD/JTAG debug header (4-pin) ──
    c.push(Component {
        reference: "J12",
        value: "SWD_Header",
        footprint_lib: "laminarforge",
        footprint_name: "PinHeader_1x4_2.54mm",
        lcsc: "C124379",
        x: 88.0,
        y: 25.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "SWD/JTAG debug",
        pads: swd_header_pads(),
    });

    // ── R13: USB-C CC1 5.1k pulldown ──
    c.push(Component {
        reference: "R13",
        value: "5.1K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C23186",
        x: 90.0,
        y: 45.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "USB-C CC1 pulldown",
        pads: r0603_pads(NET_USB_CC1, NET_GND),
    });

    // ── R14: USB-C CC2 5.1k pulldown ──
    c.push(Component {
        reference: "R14",
        value: "5.1K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C23186",
        x: 90.0,
        y: 35.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "USB-C CC2 pulldown",
        pads: r0603_pads(NET_USB_CC2, NET_GND),
    });

    // ── R15: VBUS→5V OR-ing isolation (schottky via D5) ──
    c.push(Component {
        reference: "D5",
        value: "SS54",
        footprint_lib: "laminarforge",
        footprint_name: "SMA",
        lcsc: "C22452",
        x: 88.0,
        y: 30.0,
        rotation: 0.0,
        layer: "F.Cu",
        description: "VBUS->5V OR-ing schottky",
        pads: sma_tvs_pads(NET_5V, NET_VBUS),
    });

    c
}

// ───────────────────────── KiCad PCB writer ─────────────────────────

fn write_header(pcb: &mut String) {
    pcb.push_str(
        r#"(kicad_pcb
  (version 20240108)
  (generator "laminarforge_controller_pcb")
  (generator_version "1.0")
  (general
    (thickness 1.6)
    (legacy_teardrops no)
  )
  (paper "A4")
  (layers
    (0 "F.Cu" signal)
    (31 "B.Cu" signal)
    (32 "B.Adhes" user "B.Adhesive")
    (33 "F.Adhes" user "F.Adhesive")
    (34 "B.Paste" user)
    (35 "F.Paste" user)
    (36 "B.SilkS" user "B.Silkscreen")
    (37 "F.SilkS" user "F.Silkscreen")
    (38 "B.Mask" user "B.Mask")
    (39 "F.Mask" user "F.Mask")
    (40 "Dwgs.User" user "User.Drawings")
    (41 "Cmts.User" user "User.Comments")
    (42 "Eco1.User" user "User.Eco1")
    (43 "Eco2.User" user "User.Eco2")
    (44 "Edge.Cuts" user)
    (45 "Margin" user)
    (46 "B.CrtYd" user "B.Courtyard")
    (47 "F.CrtYd" user "F.Courtyard")
    (48 "B.Fab" user "B.Fabrication")
    (49 "F.Fab" user "F.Fabrication")
  )
"#,
    );
}

fn write_nets(pcb: &mut String) {
    for (i, name) in NET_NAMES.iter().enumerate() {
        if i == 0 {
            writeln!(pcb, "  (net 0 \"\")").unwrap();
        } else {
            writeln!(pcb, "  (net {} \"{}\")", i, name).unwrap();
        }
    }
    pcb.push('\n');
}

fn write_setup(pcb: &mut String) {
    pcb.push_str(
        r#"  (setup
    (pad_to_mask_clearance 0)
    (allow_soldermask_bridges_in_footprints yes)
  )
  (net_class Default "Default"
    (clearance 0.2)
    (trace_width 0.25)
    (via_dia 0.7)
    (via_drill 0.35)
    (uvia_dia 0.3)
    (uvia_drill 0.1)
  )
"#,
    );
}

fn write_board_outline(pcb: &mut String) {
    writeln!(pcb, "  (gr_line (start 0 0) (end {} 0) (layer \"Edge.Cuts\") (stroke (width 0.15) (type solid)))", BOARD_W).unwrap();
    writeln!(pcb, "  (gr_line (start {} 0) (end {} {}) (layer \"Edge.Cuts\") (stroke (width 0.15) (type solid)))", BOARD_W, BOARD_W, BOARD_H).unwrap();
    writeln!(pcb, "  (gr_line (start {} {}) (end 0 {}) (layer \"Edge.Cuts\") (stroke (width 0.15) (type solid)))", BOARD_W, BOARD_H, BOARD_H).unwrap();
    writeln!(pcb, "  (gr_line (start 0 {}) (end 0 0) (layer \"Edge.Cuts\") (stroke (width 0.15) (type solid)))", BOARD_H).unwrap();

    for &(ref_name, x, y) in MOUNT_HOLES.iter() {
        writeln!(pcb, r#"  (footprint "MountingHole:MountingHole_3.2mm_M3"
    (layer "F.Cu")
    (at {} {})
    (tstamp "{}")
    (property "Reference" "{}" (at 0 -4) (layer "F.Fab") (effects (font (size 0.8 0.8) (thickness 0.12))))
    (pad "1" thru_hole circle (at 0 0) (size 6.35 6.35) (drill 3.2) (layers "*.Cu" "*.Mask") (net 1 "GND"))
  )"#, x, y, next_uuid(), ref_name).unwrap();
    }
    pcb.push('\n');
}

fn write_footprint(pcb: &mut String, comp: &Component) {
    writeln!(
        pcb,
        "  (footprint \"{}:{}\"",
        comp.footprint_lib, comp.footprint_name
    )
    .unwrap();
    writeln!(pcb, "    (layer \"{}\")", comp.layer).unwrap();
    writeln!(pcb, "    (tstamp \"{}\")", next_uuid()).unwrap();
    writeln!(pcb, "    (at {} {} {})", comp.x, comp.y, comp.rotation).unwrap();
    writeln!(
        pcb,
        "    (property \"Reference\" \"{}\" (at 0 -3) (layer \"F.Fab\") (effects (font (size 0.8 0.8) (thickness 0.12))))",
        comp.reference
    ).unwrap();
    writeln!(
        pcb,
        "    (property \"Value\" \"{}\" (at 0 3) (layer \"F.Fab\") (effects (font (size 0.7 0.7) (thickness 0.1))))",
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
                pad.number,
                pad.pad_type,
                pad.shape,
                pad.x,
                pad.y,
                pad.width,
                pad.height,
                pad.layers,
                pad.net_id,
                pad.net_name
            )
            .unwrap();
        }
    }
    pcb.push_str("  )\n\n");
}

fn write_bom(path: &Path, comps: &[Component]) {
    let mut s = String::new();
    s.push_str("Reference,Value,LCSC,Footprint,Description\n");
    for c in comps {
        s.push_str(&format!(
            "{},{},{},{}:{},{}\n",
            c.reference, c.value, c.lcsc, c.footprint_lib, c.footprint_name, c.description
        ));
    }
    fs::write(path, s).expect("write BOM");
}

fn write_cpl(path: &Path, comps: &[Component]) {
    let mut s = String::new();
    s.push_str("Designator,Val,Package,Mid X,Mid Y,Rotation,Layer\n");
    for c in comps {
        let layer = if c.layer.contains("B.Cu") {
            "Bottom"
        } else {
            "Top"
        };
        s.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            c.reference, c.value, c.footprint_name, c.x, c.y, c.rotation, layer
        ));
    }
    fs::write(path, s).expect("write CPL");
}

// ───────────────────────── DSN writer (Specctra, for Freerouting) ─────────────────────────

fn mm_to_um(mm: f64) -> f64 {
    mm * 1000.0
}
fn dsn_y(mm: f64) -> f64 {
    -(mm * 1000.0)
}

fn padstack_name(pad: &Pad) -> String {
    let w_um = mm_to_um(pad.width);
    let h_um = mm_to_um(pad.height);
    if pad.pad_type == "thru_hole" || pad.drill.is_some() {
        let dia = w_um.max(h_um);
        format!("Round[A]Pad_{:.0}_um", dia)
    } else {
        format!("Rect[T]Pad_{:.0}x{:.0}_um", w_um, h_um)
    }
}

fn fp_key(c: &Component) -> String {
    format!("{}:{}", c.footprint_lib, c.footprint_name)
}

fn write_dsn(path: &Path, comps: &[Component]) {
    let mut s = String::with_capacity(256 * 1024);
    let pcb_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "controller.dsn".to_string());

    writeln!(s, "(pcb {}", pcb_name).unwrap();
    // parser
    writeln!(s, "  (parser").unwrap();
    writeln!(s, "    (string_quote \")").unwrap();
    writeln!(s, "    (space_in_quoted_tokens on)").unwrap();
    writeln!(s, "    (host_cad \"LaminarForge\")").unwrap();
    writeln!(s, "    (host_version \"1.0\")").unwrap();
    writeln!(s, "  )").unwrap();

    writeln!(s, "  (resolution um 10)").unwrap();
    writeln!(s, "  (unit um)").unwrap();

    // structure
    let board_x = mm_to_um(BOARD_W);
    let board_y_neg = dsn_y(BOARD_H);
    writeln!(s, "  (structure").unwrap();
    writeln!(s, "    (layer F.Cu (type signal) (property (index 0)))").unwrap();
    writeln!(s, "    (layer B.Cu (type signal) (property (index 1)))").unwrap();
    writeln!(
        s,
        "    (boundary (path pcb 0  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}))",
        0.0, 0.0, board_x, 0.0, board_x, board_y_neg, 0.0, board_y_neg, 0.0, 0.0
    ).unwrap();

    // GND planes on both layers
    for layer in &["F.Cu", "B.Cu"] {
        writeln!(
            s,
            "    (plane GND (polygon {} 0  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}))",
            layer, 0.0, 0.0, board_x, 0.0, board_x, board_y_neg, 0.0, board_y_neg, 0.0, 0.0
        ).unwrap();
    }

    // mounting hole keepouts
    for &(_, x, y) in MOUNT_HOLES {
        writeln!(
            s,
            "    (keepout \"\" (circle signal 6950 {:.0} {:.0}))",
            mm_to_um(x),
            dsn_y(y)
        )
        .unwrap();
    }

    writeln!(s, "    (via \"Via[0-1]_600:300_um\")").unwrap();
    let edge_w = mm_to_um(1.0);
    writeln!(
        s,
        "    (keepout \"\" (rect signal 0 0 {:.0} {:.0}))",
        board_x, -edge_w
    )
    .unwrap();
    writeln!(
        s,
        "    (keepout \"\" (rect signal 0 {:.0} {:.0} {:.0}))",
        board_y_neg + edge_w,
        board_x,
        board_y_neg
    )
    .unwrap();
    writeln!(
        s,
        "    (keepout \"\" (rect signal 0 0 {:.0} {:.0}))",
        edge_w, board_y_neg
    )
    .unwrap();
    writeln!(
        s,
        "    (keepout \"\" (rect signal {:.0} 0 {:.0} {:.0}))",
        board_x - edge_w,
        board_x,
        board_y_neg
    )
    .unwrap();

    writeln!(s, "    (rule (width 250) (clearance 230))").unwrap();
    writeln!(s, "  )").unwrap();

    // placement (grouped by footprint)
    let mut groups: BTreeMap<String, Vec<&Component>> = BTreeMap::new();
    for c in comps {
        groups.entry(fp_key(c)).or_default().push(c);
    }

    writeln!(s, "  (placement").unwrap();
    writeln!(s, "    (component MountingHole:MountingHole_3.2mm_M3").unwrap();
    for &(ref_name, x, y) in MOUNT_HOLES {
        writeln!(
            s,
            "      (place {} {:.0} {:.0} front 0.0)",
            ref_name,
            mm_to_um(x),
            dsn_y(y)
        )
        .unwrap();
    }
    writeln!(s, "    )").unwrap();

    for (k, cs) in &groups {
        writeln!(s, "    (component {}", k).unwrap();
        for c in cs {
            let side = if c.layer.contains("B.Cu") {
                "back"
            } else {
                "front"
            };
            let mut rot = c.rotation % 360.0;
            if rot >= 180.0 {
                rot -= 360.0;
            } else if rot < -180.0 {
                rot += 360.0;
            }
            writeln!(
                s,
                "      (place {} {:.0} {:.0} {} {:.1} (PN {}))",
                c.reference,
                mm_to_um(c.x),
                dsn_y(c.y),
                side,
                rot,
                c.value
            )
            .unwrap();
        }
        writeln!(s, "    )").unwrap();
    }
    writeln!(s, "  )").unwrap();

    // library
    writeln!(s, "  (library").unwrap();

    writeln!(s, "    (image MountingHole:MountingHole_3.2mm_M3").unwrap();
    writeln!(s, "      (pin Round[A]Pad_6350_um 1 0 0)").unwrap();
    writeln!(s, "    )").unwrap();

    struct PSDef {
        w: f64,
        h: f64,
        thru: bool,
    }
    let mut padstacks: BTreeMap<String, PSDef> = BTreeMap::new();
    padstacks.insert(
        "Round[A]Pad_6350_um".to_string(),
        PSDef {
            w: 6350.0,
            h: 6350.0,
            thru: true,
        },
    );

    let mut seen: BTreeSet<String> = BTreeSet::new();
    for c in comps {
        let k = fp_key(c);
        if seen.contains(&k) {
            continue;
        }
        seen.insert(k.clone());
        writeln!(s, "    (image {}", k).unwrap();
        for pad in &c.pads {
            let ps = padstack_name(pad);
            writeln!(
                s,
                "      (pin {} {} {:.0} {:.0})",
                ps,
                pad.number,
                mm_to_um(pad.x),
                dsn_y(pad.y)
            )
            .unwrap();
            padstacks.entry(ps).or_insert(PSDef {
                w: mm_to_um(pad.width),
                h: mm_to_um(pad.height),
                thru: pad.pad_type == "thru_hole" || pad.drill.is_some(),
            });
        }
        writeln!(s, "    )").unwrap();
    }

    for (name, d) in &padstacks {
        writeln!(s, "    (padstack {}", name).unwrap();
        if d.thru {
            let dia = d.w.max(d.h);
            writeln!(s, "      (shape (circle F.Cu {:.0}))", dia).unwrap();
            writeln!(s, "      (shape (circle B.Cu {:.0}))", dia).unwrap();
        } else {
            let hw = d.w / 2.0;
            let hh = d.h / 2.0;
            writeln!(
                s,
                "      (shape (rect F.Cu {:.0} {:.0} {:.0} {:.0}))",
                -hw, -hh, hw, hh
            )
            .unwrap();
        }
        writeln!(s, "      (attach off)").unwrap();
        writeln!(s, "    )").unwrap();
    }

    writeln!(s, "    (padstack \"Via[0-1]_600:300_um\"").unwrap();
    writeln!(s, "      (shape (circle F.Cu 600))").unwrap();
    writeln!(s, "      (shape (circle B.Cu 600))").unwrap();
    writeln!(s, "      (attach off)").unwrap();
    writeln!(s, "    )").unwrap();
    writeln!(s, "  )").unwrap();

    // network
    let mut net_pins: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for &(ref_name, _, _) in MOUNT_HOLES {
        net_pins
            .entry(NET_GND)
            .or_default()
            .push(format!("{}-1", ref_name));
    }
    for c in comps {
        for pad in &c.pads {
            if pad.net_id == NET_UNCONNECTED {
                continue;
            }
            net_pins
                .entry(pad.net_id)
                .or_default()
                .push(format!("{}-{}", c.reference, pad.number));
        }
    }

    writeln!(s, "  (network").unwrap();
    let mut all_names: Vec<String> = Vec::new();
    for (&nid, pins) in &net_pins {
        let n = net_name(nid);
        if n.is_empty() {
            continue;
        }
        all_names.push(n.to_string());
        write!(s, "    (net {}\n      (pins", n).unwrap();
        for p in pins {
            write!(s, " {}", p).unwrap();
        }
        writeln!(s, ")").unwrap();
        writeln!(s, "    )").unwrap();
    }
    write!(s, "    (class kicad_default").unwrap();
    for n in &all_names {
        write!(s, " {}", n).unwrap();
    }
    writeln!(s).unwrap();
    writeln!(s, "      (circuit (use_via \"Via[0-1]_600:300_um\"))").unwrap();
    writeln!(s, "      (rule (width 250) (clearance 230))").unwrap();
    writeln!(s, "    )").unwrap();
    writeln!(s, "  )").unwrap();

    writeln!(s, "  (wiring)").unwrap();
    writeln!(s, ")").unwrap();

    fs::write(path, s).expect("write DSN");
}

// ───────────────────────── Main ─────────────────────────

fn main() {
    let output_dir: PathBuf = Path::new("pcb/output").to_path_buf();
    fs::create_dir_all(&output_dir).expect("create output dir");

    let args: Vec<String> = std::env::args().collect();
    let from_ses = args.iter().position(|a| a == "--from-ses").map(|i| {
        args.get(i + 1)
            .unwrap_or_else(|| panic!("--from-ses requires a path argument"))
            .clone()
    });

    let comps = define_components();

    // Sanity check: report component count + net usage
    let mut net_counts: BTreeMap<u32, usize> = BTreeMap::new();
    for c in &comps {
        for p in &c.pads {
            *net_counts.entry(p.net_id).or_insert(0) += 1;
        }
    }
    println!(
        "Controller PCB: {} components, board {}x{}mm",
        comps.len(),
        BOARD_W,
        BOARD_H
    );
    println!("Net usage:");
    for (nid, cnt) in &net_counts {
        if *nid == NET_UNCONNECTED {
            continue;
        }
        println!("  {:>4}  {:<18}  {} pads", nid, net_name(*nid), cnt);
    }

    // Build KiCad .kicad_pcb
    let mut s = String::with_capacity(256 * 1024);
    write_header(&mut s);
    write_nets(&mut s);
    write_setup(&mut s);
    write_board_outline(&mut s);
    for c in &comps {
        write_footprint(&mut s, c);
    }

    // Import routed traces from Freerouting SES file (pure Rust)
    if let Some(ses_path) = &from_ses {
        let ses_data = laminarforge_cad::pcb::ses::parse_ses(Path::new(ses_path));
        let total_wires: usize = ses_data.routes.iter().map(|r| r.wires.len()).sum();
        let total_vias: usize = ses_data.routes.iter().map(|r| r.vias.len()).sum();
        println!(
            "\nImporting SES: {} nets, {} wires, {} vias",
            ses_data.routes.len(),
            total_wires,
            total_vias
        );
        laminarforge_cad::pcb::ses::write_ses_traces(&mut s, &ses_data);
    }

    // v2: GND pour zones on F.Cu and B.Cu. Required so DRC sees the GND
    // plane connection between every GND pad — without zones, KiCad flags
    // every GND pad as "unconnected" even though the DSN pour exists.
    // connect_pads yes (solid) avoids starved-thermal DRC errors on TAB pads
    // and castellated modules (U1 pin 15, U2 pin 3, J10 M2).
    for (layer, tstamp_seed) in [("F.Cu", 1u8), ("B.Cu", 2u8)] {
        writeln!(&mut s, "  (zone").unwrap();
        writeln!(&mut s, "    (net {}) (net_name \"GND\")", NET_GND).unwrap();
        writeln!(
            &mut s,
            "    (layer \"{}\") (tstamp \"{:08x}-0000-0000-0000-{:012x}\")",
            layer, tstamp_seed as u32, tstamp_seed as u64
        )
        .unwrap();
        writeln!(&mut s, "    (hatch edge 0.5)").unwrap();
        writeln!(&mut s, "    (connect_pads yes (clearance 0.2))").unwrap();
        writeln!(
            &mut s,
            "    (min_thickness 0.15) (filled_areas_thickness no)"
        )
        .unwrap();
        writeln!(&mut s, "    (fill yes (thermal_gap 0.25) (thermal_bridge_width 0.3) (smoothing none) (radius 0.5))").unwrap();
        writeln!(&mut s, "    (polygon (pts").unwrap();
        writeln!(
            &mut s,
            "      (xy 0 0) (xy {} 0) (xy {} {}) (xy 0 {})",
            BOARD_W, BOARD_W, BOARD_H, BOARD_H
        )
        .unwrap();
        writeln!(&mut s, "    ))").unwrap();
        writeln!(&mut s, "  )").unwrap();
    }

    s.push_str(")\n");

    let pcb_path = output_dir.join("controller.kicad_pcb");
    fs::write(&pcb_path, &s).expect("write pcb");
    println!("Written: {}", pcb_path.display());

    let bom_path = output_dir.join("controller_bom.csv");
    write_bom(&bom_path, &comps);
    println!("Written: {}", bom_path.display());

    let cpl_path = output_dir.join("controller_cpl.csv");
    write_cpl(&cpl_path, &comps);
    println!("Written: {}", cpl_path.display());

    // Always export DSN for Freerouting (unless we're just importing SES)
    if from_ses.is_none() {
        let dsn_path = output_dir.join("controller.dsn");
        write_dsn(&dsn_path, &comps);
        println!("Written: {}", dsn_path.display());

        println!("\nControl PCB generation complete.");
        println!("Run Freerouting:");
        println!("  /opt/homebrew/opt/openjdk/bin/java -jar tools/freerouting.jar");
        println!("  File -> Open Design -> {}", dsn_path.display());
    }

    // If --validate, run DRC
    if args.iter().any(|a| a == "--validate") {
        laminarforge_cad::pcb::export::validate_and_export(&pcb_path, &output_dir);
    }
}
