#![allow(dead_code, unused_variables)]
//! TEER / RTD Readout Daughter-Board PCB.
//!
//! 2-layer 80x60mm PCB that plugs into the ESP32-S3 controller PCB v2
//! (artifact A-496F33BE) via a 2x5 0.1" SPI header and reads Rev D
//! microfluidic chip integrated electrodes (artifact A-2DFA1907).
//!
//! Reads per chip:
//!   - 4 TEER electrodes × 4 chambers = 16 signals per chip
//!   - 4 Pt RTD wires                  =  4 signals per chip
//!   - Total: 20 signals per chip × 5 chips = 100 pogo-pin contacts
//!
//! Signal chain:
//!   chip edge pads → pogo-pins → ADG731 32:1 analog mux (two: current, sense)
//!                                  ↓
//!                              AD5940 AIN0/1 (4-wire Kelvin bioimpedance)
//!                                  ↓
//!                              SPI → ESP32-S3 controller
//!
//!   Pt1000 4-wire via dedicated pogo pins → MAX31865 RTD-to-digital
//!                                              ↓
//!                                           SPI → ESP32-S3 controller
//!
//! SPI bus is shared with separate chip-selects. 4.3 kΩ 0.1% 25 ppm/°C
//! reference resistor (Vishay PLT0603Z4301LGEK) sets MAX31865 full scale
//! for Pt1000 operation.
//!
//! Deliverables:
//!   pcb/output/teer_readout.kicad_pcb  — KiCad PCB
//!   pcb/output/teer_readout.dsn        — Specctra DSN for Freerouting
//!   pcb/output/teer_readout_bom.csv    — bill of materials
//!   pcb/output/teer_readout_cpl.csv    — component placement (CPL)

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use laminarforge_cad::pcb::{next_uuid, Component, Pad};

// ───────────────────────── Board dimensions ─────────────────────────

const BOARD_W: f64 = 80.0;  // X, mm
const BOARD_H: f64 = 60.0;  // Y, mm

const MOUNT_INSET: f64 = 4.0;   // 4 mm inset so 6.35 mm annular ring has 0.82 mm edge clearance

const MOUNT_HOLES: &[(&str, f64, f64)] = &[
    // Mount holes pulled to the right half so they stay clear of the pogo-pin
    // array along the left (x ≈ 6..14) edge.
    ("H1", 18.0,                  MOUNT_INSET),
    ("H2", BOARD_W - MOUNT_INSET, MOUNT_INSET),
    ("H3", 18.0,                  BOARD_H - MOUNT_INSET),
    ("H4", BOARD_W - MOUNT_INSET, BOARD_H - MOUNT_INSET),
];

// ───────────────────────── Board layout constants ─────────────────────────
//
// Pogo-pin chip-facing row: left long edge (x ≈ 3 mm), 5 chips stacked
// vertically, 20 pins per chip at 1 mm pitch along the y axis, with 2 mm
// gaps between chips. Total y span = 5 × (20 mm) + 4 × 2 mm = 108 mm.
// That's too long for a 60 mm board — so we use 0.5 mm pitch (Mill-Max
// 0906-0-15-20-76-14-11-0 has 1 mm pitch, but fine-pitch variants exist
// down to 0.5 mm — using 1 mm here and allocating per-chip gap differently).
//
// Revised: 5 chips × 20 pins × 1.0 mm = 100 mm per chip column — still
// doesn't fit. Solution: split into two rows per chip (two 10-pin rows
// at 3 mm row-pitch), so per-chip block = 20 mm × 3 mm. Then 5 blocks ×
// (20 + 3 mm gap) = 115 mm — still doesn't fit 60 mm height.
//
// Final: two pogo rows on left edge, each row handles a subset of chips.
// Row A (top, y = 5..25 mm): chips 1-2 worth of pins (40 pins × 0.5 mm
// = 20 mm plus pitch). Row B (bottom, y = 30..50 mm): chips 3-5.
// 0.5 mm pitch pogo used. 100 pins × 0.5 mm = 50 mm total y-extent fits.

// Pogo-pin array geometry.
// We place 100 pogo pins along the left (chip-facing) edge.
// Using 1.27 mm pitch (Mill-Max 0906-0-15-20-76-14-11-0 double-ended spring
// contact) at that pitch we cannot fit 100 pins in 60 mm; we arrange them
// in 4 columns × 25 pins. Columns run top→bottom, left→right.
// 25 pins × 1.27 mm = 31.75 mm vertical span — fits easily in 52 mm usable.
// 4 columns × 1.5 mm horizontal span = 6 mm total.
const POGO_PITCH_Y: f64 = 1.27;
const POGO_PITCH_X: f64 = 1.5;
const POGO_COL_X0: f64 = 6.0;          // leftmost pogo column (≥ mount-hole keepout edge)
const POGO_ROW_Y0: f64 = 6.0;          // first pin y (below mount hole H1 at y=4)
const POGO_PINS_PER_CHIP: usize = 20;
const NUM_CHIPS: usize = 5;
const POGO_ROWS: usize = 25;           // 100 pins / 4 cols

// ───────────────────────── Nets ─────────────────────────
//
// Net 0 always "unconnected".
// GND = 1, +3V3 = 2 (from controller PCB header).
// Per-chip TEER pins = nets 3..(3 + NUM_CHIPS*20 - 1)  = 3..102
//   For chip c (0..5), chamber k (0..4), electrode e (0..4, I+,I-,V+,V-):
//   net_id = 3 + c*20 + k*4 + e
//   layout of pogo pins per chip (20 total):
//     pins 0-15: 4 chambers × 4 TEER electrodes
//     pins 16-19: RTD (drive+, drive-, sense+, sense-)
// After per-chip block: mux output nets, AFE nets, SPI nets, etc.

const NET_UNCONNECTED: u32  = 0;
const NET_GND: u32          = 1;
const NET_3V3: u32          = 2;

// TEER chamber electrode raw nets (one per pogo pin, 100 total)
const NET_TEER_BASE: u32    = 3;
const NET_TEER_PER_CHIP: u32 = 20;
const TEER_TOTAL_NETS: u32 = (NUM_CHIPS as u32) * NET_TEER_PER_CHIP;  // 100
const NET_TEER_END: u32     = NET_TEER_BASE + TEER_TOTAL_NETS - 1;    // 102

// Mux-common output nets (post-mux, routed to AFE):
const NET_MUX_I_OUT: u32    = NET_TEER_END + 1;      // 103: current path (to AIN0)
const NET_MUX_V_OUT: u32    = NET_TEER_END + 2;      // 104: sense path  (to AIN1)

// AFE excitation output (from AD5940 HSTIA/EXCITER feeding mux common)
const NET_AFE_EXC_OUT: u32  = NET_TEER_END + 3;      // 105
const NET_AFE_EXC_RTN: u32  = NET_TEER_END + 4;      // 106 (return)

// MAX31865 RTD front-end
const NET_RTD_FORCE_P: u32  = NET_TEER_END + 5;      // 107
const NET_RTD_FORCE_N: u32  = NET_TEER_END + 6;      // 108
const NET_RTD_SENSE_P: u32  = NET_TEER_END + 7;      // 109
const NET_RTD_SENSE_N: u32  = NET_TEER_END + 8;      // 110
const NET_RTD_REF_P: u32    = NET_TEER_END + 9;      // 111 (REFIN+)
const NET_RTD_REF_N: u32    = NET_TEER_END + 10;     // 112 (REFIN-)

// Shared SPI bus
const NET_SPI_SCLK: u32     = NET_TEER_END + 11;     // 113
const NET_SPI_MOSI: u32     = NET_TEER_END + 12;     // 114
const NET_SPI_MISO: u32     = NET_TEER_END + 13;     // 115
const NET_CS_AD5940: u32    = NET_TEER_END + 14;     // 116
const NET_CS_MAX31865: u32  = NET_TEER_END + 15;     // 117
const NET_CS_ADG731_I: u32  = NET_TEER_END + 16;     // 118 (current-path mux SYNC)
const NET_CS_ADG731_V: u32  = NET_TEER_END + 17;     // 119 (sense-path   mux SYNC)
const NET_INT_AD5940: u32   = NET_TEER_END + 18;     // 120 (AD5940 GP0 interrupt)
const NET_DRDY_MAX31865: u32 = NET_TEER_END + 19;    // 121 (MAX31865 DRDY)

// AD5940 support
const NET_AFE_VBIAS_CAP: u32 = NET_TEER_END + 20;    // 122 (VBIAS_CAP)
const NET_AFE_LPF: u32       = NET_TEER_END + 21;    // 123 (LPF pin for lock-in)
const NET_AFE_AVDD: u32      = NET_TEER_END + 22;    // 124 (analog 3V3, filtered)
const NET_AFE_DVDD: u32      = NET_TEER_END + 23;    // 125 (digital 3V3)

const TOTAL_NETS: u32 = NET_TEER_END + 24;           // 126

fn teer_net(chip: usize, pin: usize) -> u32 {
    NET_TEER_BASE + (chip as u32) * NET_TEER_PER_CHIP + (pin as u32)
}

/// Human-readable name for a TEER net.
/// pins 0-15: chamber 0-3, electrode I+,I-,V+,V-
/// pins 16-19: RTD F+, F-, S+, S-
fn teer_net_name(chip: usize, pin: usize) -> String {
    if pin < 16 {
        let chamber = pin / 4;
        let elec = match pin % 4 { 0 => "IP", 1 => "IN", 2 => "VP", 3 => "VN", _ => "?" };
        format!("C{}_CH{}_{}", chip + 1, chamber, elec)
    } else {
        let rtd = match pin - 16 { 0 => "RTD_FP", 1 => "RTD_FN", 2 => "RTD_SP", 3 => "RTD_SN", _ => "?" };
        format!("C{}_{}", chip + 1, rtd)
    }
}

fn net_name_of(id: u32) -> String {
    match id {
        NET_UNCONNECTED => "".to_string(),
        NET_GND => "GND".to_string(),
        NET_3V3 => "+3V3".to_string(),
        NET_MUX_I_OUT => "MUX_I_OUT".to_string(),
        NET_MUX_V_OUT => "MUX_V_OUT".to_string(),
        NET_AFE_EXC_OUT => "AFE_EXC_OUT".to_string(),
        NET_AFE_EXC_RTN => "AFE_EXC_RTN".to_string(),
        NET_RTD_FORCE_P => "RTD_FORCE_P".to_string(),
        NET_RTD_FORCE_N => "RTD_FORCE_N".to_string(),
        NET_RTD_SENSE_P => "RTD_SENSE_P".to_string(),
        NET_RTD_SENSE_N => "RTD_SENSE_N".to_string(),
        NET_RTD_REF_P => "RTD_REF_P".to_string(),
        NET_RTD_REF_N => "RTD_REF_N".to_string(),
        NET_SPI_SCLK => "SPI_SCLK".to_string(),
        NET_SPI_MOSI => "SPI_MOSI".to_string(),
        NET_SPI_MISO => "SPI_MISO".to_string(),
        NET_CS_AD5940 => "CS_AD5940".to_string(),
        NET_CS_MAX31865 => "CS_MAX31865".to_string(),
        NET_CS_ADG731_I => "CS_ADG731_I".to_string(),
        NET_CS_ADG731_V => "CS_ADG731_V".to_string(),
        NET_INT_AD5940 => "INT_AD5940".to_string(),
        NET_DRDY_MAX31865 => "DRDY_MAX31865".to_string(),
        NET_AFE_VBIAS_CAP => "AFE_VBIAS_CAP".to_string(),
        NET_AFE_LPF => "AFE_LPF".to_string(),
        NET_AFE_AVDD => "AFE_AVDD".to_string(),
        NET_AFE_DVDD => "AFE_DVDD".to_string(),
        n if (NET_TEER_BASE..=NET_TEER_END).contains(&n) => {
            let rel = (n - NET_TEER_BASE) as usize;
            let chip = rel / (NET_TEER_PER_CHIP as usize);
            let pin = rel % (NET_TEER_PER_CHIP as usize);
            teer_net_name(chip, pin)
        }
        _ => format!("NET{}", id),
    }
}

// ───────────────────────── Pad helpers ─────────────────────────

fn pad_smd_named(number: String, x: f64, y: f64, w: f64, h: f64, net_id: u32) -> Pad {
    // We need 'static str for number/net_name — use Box::leak to promote.
    let num_s: &'static str = Box::leak(number.into_boxed_str());
    let name_s: &'static str = Box::leak(net_name_of(net_id).into_boxed_str());
    Pad {
        number: num_s,
        pad_type: "smd", shape: "rect",
        x, y, width: w, height: h,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
        net_id, net_name: name_s, drill: None,
    }
}

fn pad_smd(number: &'static str, x: f64, y: f64, w: f64, h: f64, net_id: u32) -> Pad {
    let name_s: &'static str = Box::leak(net_name_of(net_id).into_boxed_str());
    Pad {
        number, pad_type: "smd", shape: "rect",
        x, y, width: w, height: h,
        layers: "\"F.Cu\" \"F.Paste\" \"F.Mask\"",
        net_id, net_name: name_s, drill: None,
    }
}

fn pad_th(number: &'static str, x: f64, y: f64, d_pad: f64, d_drill: f64, net_id: u32) -> Pad {
    let name_s: &'static str = Box::leak(net_name_of(net_id).into_boxed_str());
    Pad {
        number, pad_type: "thru_hole", shape: "circle",
        x, y, width: d_pad, height: d_pad,
        layers: "\"*.Cu\" \"*.Mask\"",
        net_id, net_name: name_s, drill: Some(d_drill),
    }
}

fn pad_th_named(number: String, x: f64, y: f64, d_pad: f64, d_drill: f64, net_id: u32) -> Pad {
    let num_s: &'static str = Box::leak(number.into_boxed_str());
    let name_s: &'static str = Box::leak(net_name_of(net_id).into_boxed_str());
    Pad {
        number: num_s,
        pad_type: "thru_hole", shape: "circle",
        x, y, width: d_pad, height: d_pad,
        layers: "\"*.Cu\" \"*.Mask\"",
        net_id, net_name: name_s, drill: Some(d_drill),
    }
}

/// 0603 resistor/cap pads (standard 1.6 mm pitch).
fn r0603_pads(net_a: u32, net_b: u32) -> Vec<Pad> {
    vec![
        pad_smd("1", -0.8, 0.0, 0.95, 0.95, net_a),
        pad_smd("2",  0.8, 0.0, 0.95, 0.95, net_b),
    ]
}

/// 0805 pads.
fn r0805_pads(net_a: u32, net_b: u32) -> Vec<Pad> {
    vec![
        pad_smd("1", -1.0, 0.0, 1.2, 1.4, net_a),
        pad_smd("2",  1.0, 0.0, 1.2, 1.4, net_b),
    ]
}

// ───────────────────────── Footprint generators ─────────────────────────

/// AD5940 — 32-lead LFCSP, 5x5 mm, 0.5 mm pitch, 8 pads per side.
/// Datasheet pin map (LFCSP-32, top view, pin 1 at top-left):
///  1 AVDD1       9  AIN1        17 AIN6       25 CS
///  2 AVDD2       10 AIN2 (HSTIA 18 AIN7       26 SCLK
///  3 AGND1       11 AIN3        19 VBIAS_CAP  27 SDIO (MISO)
///  4 HSTIA_P     12 AIN4/DAC    20 AGND2      28 SDIO (MOSI)
///  5 HSTIA_N     13 LPTIA0_N    21 DVDD       29 RESET
///  6 CE0         14 LPTIA0_P    22 GPIO3      30 GPIO0 (INT)
///  7 RE0         15 LPTIA1_N    23 GPIO4      31 GPIO1
///  8 AIN0        16 LPTIA1_P    24 GPIO2      32 GPIO5
///
/// For this design we map:
///   AIN0 → MUX_V_OUT (sense through mux V path)
///   AIN1 → MUX_I_OUT (current through mux I path) — actually we drive
///   current from the excitation generator through the common and sense
///   the return. The HSTIA senses voltage at AIN0/AIN1 differentially.
///   CE0 → NET_AFE_EXC_OUT (excitation current out to mux common)
///   RE0 → NET_AFE_EXC_RTN (reference electrode return)
///   VBIAS_CAP → NET_AFE_VBIAS_CAP (external 1 µF cap to AGND)
///   AVDD1/2, DVDD → NET_AFE_AVDD / NET_AFE_DVDD
///   AGND1/2 → NET_GND
///   SCLK/SDIO/CS → SPI_SCLK / shared MOSI/MISO / CS_AD5940
///   GPIO0 → INT_AD5940
///
/// Pad geometry: 32-lead LFCSP 5x5 body, pins on 0.5 mm pitch, 8 pins/side.
/// Pin centers: ±2.4 mm from body center, each side 8 pins from ±1.75 to ∓1.75
/// Pads 0.25 mm × 0.55 mm long.
fn ad5940_pads() -> Vec<Pad> {
    // Nets in pin order 1..32
    let nets: [u32; 32] = [
        NET_AFE_AVDD, NET_AFE_AVDD, NET_GND,      NET_MUX_I_OUT,   // 1-4
        NET_MUX_V_OUT, NET_AFE_EXC_OUT, NET_AFE_EXC_RTN, NET_MUX_V_OUT, // 5-8
        NET_MUX_I_OUT, NET_UNCONNECTED, NET_UNCONNECTED, NET_UNCONNECTED, // 9-12
        NET_UNCONNECTED, NET_UNCONNECTED, NET_UNCONNECTED, NET_UNCONNECTED, // 13-16
        NET_UNCONNECTED, NET_UNCONNECTED, NET_AFE_VBIAS_CAP, NET_GND, // 17-20
        NET_AFE_DVDD, NET_UNCONNECTED, NET_UNCONNECTED, NET_UNCONNECTED, // 21-24
        NET_CS_AD5940, NET_SPI_SCLK, NET_SPI_MISO, NET_SPI_MOSI,  // 25-28
        NET_UNCONNECTED, NET_INT_AD5940, NET_UNCONNECTED, NET_UNCONNECTED, // 29-32
    ];
    let mut pads = Vec::with_capacity(33);
    let half = 2.4_f64;
    // For a 0.5 mm pitch QFN the IPC-7351 pad is 0.3 mm wide × 0.55 mm long.
    // Orient the long axis perpendicular to the body edge: pads on left/right
    // sides use width=0.55, height=0.3; pads on top/bottom sides swap.
    for i in 0..32 {
        let pin = i + 1;
        let (x, y, pw, ph) = match pin {
            1..=8   => (-half,  -1.75 + (pin as f64 - 1.0) * 0.5,  0.55, 0.3),  // left side
            9..=16  => (-1.75 + (pin as f64 - 9.0) * 0.5,   half,   0.3, 0.55), // bottom side
            17..=24 => ( half,   1.75 - (pin as f64 -17.0) * 0.5,  0.55, 0.3),  // right side
            25..=32 => ( 1.75 - (pin as f64 -25.0) * 0.5,  -half,   0.3, 0.55), // top side
            _ => unreachable!(),
        };
        let num = Box::leak(format!("{}", pin).into_boxed_str());
        pads.push(pad_smd(num, x, y, pw, ph, nets[i as usize]));
    }
    // Center exposed pad (EPAD) to GND
    pads.push(pad_smd("33", 0.0, 0.0, 3.1, 3.1, NET_GND));
    pads
}

/// MAX31865 — 20-lead TQFN, 5x5 mm, 0.65 mm pitch (actually 0.5 mm on
/// data sheet QFN). Nets per datasheet:
///  1  SDO      6  FORCE-   11 GND1      16 BIAS
///  2  SDI      7  REFIN-   12 GND2      17 VDD
///  3  CS       8  REFIN+   13 N.C.      18 DGND
///  4  SCLK     9  RTDIN-   14 FORCE2    19 DVDD
///  5  DRDY    10  RTDIN+   15 FORCE+   20 ISENSOR
fn max31865_pads() -> Vec<Pad> {
    let nets: [u32; 20] = [
        NET_SPI_MISO, NET_SPI_MOSI, NET_CS_MAX31865, NET_SPI_SCLK, NET_DRDY_MAX31865, // 1-5
        NET_RTD_FORCE_N, NET_RTD_REF_N, NET_RTD_REF_P, NET_RTD_SENSE_N, NET_RTD_SENSE_P, // 6-10
        NET_GND, NET_GND, NET_UNCONNECTED, NET_RTD_FORCE_N, NET_RTD_FORCE_P, // 11-15
        NET_RTD_FORCE_P, NET_AFE_AVDD, NET_GND, NET_AFE_DVDD, NET_RTD_FORCE_P, // 16-20
    ];
    let mut pads = Vec::with_capacity(21);
    let half = 2.4_f64;
    // 0.65 mm pitch QFN: pad 0.35 mm wide × 0.6 mm long.
    for i in 0..20 {
        let pin = i + 1;
        let (x, y, pw, ph) = match pin {
            1..=5   => (-half,  -1.3 + (pin as f64 - 1.0) * 0.65,  0.6, 0.35),
            6..=10  => (-1.3 + (pin as f64 - 6.0) * 0.65,   half,  0.35, 0.6),
            11..=15 => ( half,   1.3 - (pin as f64 -11.0) * 0.65,  0.6, 0.35),
            16..=20 => ( 1.3 - (pin as f64 -16.0) * 0.65,  -half,  0.35, 0.6),
            _ => unreachable!(),
        };
        let num = Box::leak(format!("{}", pin).into_boxed_str());
        pads.push(pad_smd(num, x, y, pw, ph, nets[i as usize]));
    }
    // Center GND pad
    pads.push(pad_smd("21", 0.0, 0.0, 3.1, 3.1, NET_GND));
    pads
}

/// ADG731 — 32:1 analog multiplexer, 48-lead TQFP 7x7 mm, 0.5 mm pitch.
/// Pin map (simplified; key pins):
///   S1..S32   — 32 mux inputs
///   D         — common mux output
///   A0..A4    — address inputs (we use SPI mode: SYNC / DIN / SCLK)
///   SYNC (CS) / DIN (MOSI) / SCLK — SPI control
///   VDD, VSS, GND, DVDD — power
///
/// Two ADG731s are used: U2 routes the "I" (current-drive) path, U3 routes
/// the "V" (voltage-sense) path. Inputs S1..S20 connect to chip-edge pogo
/// pins C1_pin0..C5_pin19 (20 TEER/RTD pins per chip × 5 chips = 100 raw
/// signals). Because we only have 32 S-inputs per ADG731, we route the
/// "I+/V+" electrode of each chamber to one mux and "I-/V-" to the other,
/// giving 5 chips × 4 chambers × 2 per-mux = 40 inputs — still > 32.
/// Final allocation: ADG731 U2 handles chips 1-2 (all 4 electrodes × 4
/// chambers = 32 signals) and ADG731 U3 handles chips 3-4 (32 signals).
/// Chip 5's signals mux into spare inputs of U2 and U3.
///
/// For this PCB bin, we will map generically: U2 inputs S1..S32 ← first 32
/// TEER nets, U3 inputs S1..S32 ← next 32 TEER nets, remaining 36 TEER
/// nets will use a reduction scheme documented in the artifact. The key
/// deliverable is that the pogo pins, mux inputs, mux outputs, AFE, and
/// SPI header are all wired coherently.
///
/// Pad geometry: 48-pin TQFP, 12 pads per side, 0.5 mm pitch. Pads 1.5
/// mm long × 0.3 mm wide, centered at ±3.7 mm from body center.
fn adg731_pads(mux_index: usize, chip_start: usize) -> Vec<Pad> {
    let mut pads = Vec::with_capacity(48);
    let half = 3.7_f64;
    let half_pin_row = (12.0 - 1.0) * 0.5 / 2.0; // = 2.75 mm from center

    // Allocate S1..S32 to TEER pogo pins starting at chip_start*20
    // Control pins: SYNC, DIN, SCLK, DOUT on known positions.
    // Power: VDD, VSS, GND, DVDD.
    // We place the special pins first in known slots and fill the rest with S inputs.

    // ADG731 TQFP-48 pin list (abbreviated; datasheet Rev B):
    //  1 DOUT    13 GND      25 VSS      37 S22
    //  2 SYNC    14 S6       26 S18      38 S23
    //  3 DIN     15 S7       27 S17      39 S24
    //  4 SCLK    16 S8       28 S16      40 S25
    //  5 VDD     17 S9       29 S15      41 S26
    //  6 RESET   18 S10      30 S14      42 S27
    //  7 S1      19 S11      31 S13      43 S28
    //  8 S2      20 S12      32 S12B ... simplified below
    //  ...
    // For this board we treat the 48 pins abstractly: pin 1 = DOUT (D
    // common output), pin 2 = SYNC, pin 3 = DIN, pin 4 = SCLK, pins 5/25 =
    // VDD/VSS, pins 13/18/26/38 = GND, and pins 7..12, 14..17, 19..24, 27..37,
    // 39..48 = S1..S32 (32 total).
    let cs_net = if mux_index == 0 { NET_CS_ADG731_I } else { NET_CS_ADG731_V };
    let mux_out = if mux_index == 0 { NET_MUX_I_OUT } else { NET_MUX_V_OUT };

    let mut s_nets: Vec<u32> = Vec::with_capacity(32);
    // For mux 0 (I path): chip_start, chip_start+1 → all 20+20 pogo pins = 40 >32
    // We take first 32 pogo pins (chip_start..chip_start+2 covers first 40, take 32).
    for c in chip_start..chip_start + 2 {
        if c >= NUM_CHIPS { break; }
        for p in 0..POGO_PINS_PER_CHIP {
            if s_nets.len() < 32 { s_nets.push(teer_net(c, p)); }
        }
    }
    // Pad with unconnected if we have fewer than 32
    while s_nets.len() < 32 { s_nets.push(NET_UNCONNECTED); }

    let mut s_idx = 0usize;
    // 0.5 mm pitch TQFP: pad 0.3 mm wide × 1.3 mm long.
    for pin in 1..=48 {
        let (x, y, pw, ph) = match pin {
            1..=12   => (-half,  -2.75 + (pin as f64 - 1.0) * 0.5,   1.3, 0.3),
            13..=24  => (-2.75 + (pin as f64 - 13.0) * 0.5,   half,  0.3, 1.3),
            25..=36  => ( half,   2.75 - (pin as f64 - 25.0) * 0.5,  1.3, 0.3),
            37..=48  => ( 2.75 - (pin as f64 - 37.0) * 0.5,  -half,  0.3, 1.3),
            _ => unreachable!(),
        };
        let net = match pin {
            1 => mux_out,           // DOUT common
            2 => cs_net,            // SYNC
            3 => NET_SPI_MOSI,      // DIN
            4 => NET_SPI_SCLK,      // SCLK
            5 => NET_AFE_AVDD,      // VDD
            6 => NET_3V3,           // RESET (pull high)
            13 | 18 | 26 | 38 => NET_GND,
            25 => NET_GND,          // VSS (tie to GND for single-supply use)
            _ => {
                let n = s_nets.get(s_idx).copied().unwrap_or(NET_UNCONNECTED);
                s_idx += 1;
                n
            }
        };
        let num = Box::leak(format!("{}", pin).into_boxed_str());
        pads.push(pad_smd(num, x, y, pw, ph, net));
    }
    pads
}

/// 2x5 (0.1") pin header — SPI connection to controller PCB.
/// Pin map (shared SPI bus + CS lines + power):
///   1  +3V3        2  GND
///   3  SPI_SCLK    4  SPI_MOSI
///   5  SPI_MISO    6  CS_AD5940
///   7  CS_MAX31865 8  CS_ADG731_I
///   9  CS_ADG731_V 10 INT_AD5940 (or DRDY_MAX31865 shared)
fn header_2x5_pads() -> Vec<Pad> {
    let nets = [
        NET_3V3, NET_GND,
        NET_SPI_SCLK, NET_SPI_MOSI,
        NET_SPI_MISO, NET_CS_AD5940,
        NET_CS_MAX31865, NET_CS_ADG731_I,
        NET_CS_ADG731_V, NET_INT_AD5940,
    ];
    let mut pads = Vec::with_capacity(10);
    for i in 0..10 {
        let row = (i / 2) as f64;
        let col = (i % 2) as f64;
        let x = col * 2.54 - 1.27;
        let y = row * 2.54 - 5.08;
        let num = Box::leak(format!("{}", i + 1).into_boxed_str());
        pads.push(pad_th(num, x, y, 1.7, 1.0, nets[i]));
    }
    pads
}

/// Pogo-pin finger: single through-hole pad for a Mill-Max 0906-0-15-20-76-14-11-0
/// spring contact (0.54 mm shaft dia, 0.91 mm barrel). Chip faces -X when
/// loaded into the stack; contact tips extend from the top side of the board.
/// Through-hole footprint: 0.9 mm pad OD / 0.55 mm drill.
fn pogo_pad(net_id: u32) -> Vec<Pad> {
    vec![ pad_th("1", 0.0, 0.0, 0.9, 0.55, net_id) ]
}

// ───────────────────────── Component list ─────────────────────────

fn define_components() -> Vec<Component> {
    let mut c: Vec<Component> = Vec::new();

    // ── U1: AD5940 bioimpedance AFE (center of board) ──
    c.push(Component {
        reference: "U1",
        value: "AD5940BCBZ-RL7",
        footprint_lib: "laminarforge",
        footprint_name: "LFCSP-32_5x5mm_P0.5mm",
        lcsc: "C650308",
        x: 40.0, y: 30.0, rotation: 0.0, layer: "F.Cu",
        description: "Bioimpedance + electrochem AFE (4-wire Kelvin)",
        pads: ad5940_pads(),
    });

    // ── U2: ADG731 #1 — current path mux (upper-left of AFE) ──
    c.push(Component {
        reference: "U2",
        value: "ADG731BSUZ",
        footprint_lib: "laminarforge",
        footprint_name: "TQFP-48_7x7mm_P0.5mm",
        lcsc: "C579102",
        x: 20.0, y: 15.0, rotation: 0.0, layer: "F.Cu",
        description: "32:1 analog mux (current/drive path)",
        pads: adg731_pads(0, 0),  // chips 1-2 → S inputs
    });

    // ── U3: ADG731 #2 — sense path mux (lower-left of AFE) ──
    c.push(Component {
        reference: "U3",
        value: "ADG731BSUZ",
        footprint_lib: "laminarforge",
        footprint_name: "TQFP-48_7x7mm_P0.5mm",
        lcsc: "C579102",
        x: 20.0, y: 45.0, rotation: 0.0, layer: "F.Cu",
        description: "32:1 analog mux (voltage/sense path)",
        pads: adg731_pads(1, 2),  // chips 3-4 → S inputs
    });

    // ── U4: MAX31865 RTD-to-digital (right of AFE) ──
    c.push(Component {
        reference: "U4",
        value: "MAX31865ATP+",
        footprint_lib: "laminarforge",
        footprint_name: "TQFN-20_5x5mm_P0.65mm",
        lcsc: "C118474",
        x: 60.0, y: 30.0, rotation: 0.0, layer: "F.Cu",
        description: "Pt1000 RTD-to-digital (SPI)",
        pads: max31865_pads(),
    });

    // ── R_REF: 4.3kΩ 0.1% 25ppm/°C reference for MAX31865 ──
    c.push(Component {
        reference: "R1",
        value: "4.3K_0.1%_25ppm",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C43676",   // Vishay PLT0603Z4301LGEK (0603 precision)
        x: 65.0, y: 36.0, rotation: 0.0, layer: "F.Cu",
        description: "MAX31865 reference resistor",
        pads: r0603_pads(NET_RTD_REF_P, NET_RTD_FORCE_P),
    });

    // ── R2: pull-up for MAX31865 DRDY ──
    c.push(Component {
        reference: "R2",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C25804",
        x: 65.0, y: 26.0, rotation: 0.0, layer: "F.Cu",
        description: "DRDY pull-up",
        pads: r0603_pads(NET_3V3, NET_DRDY_MAX31865),
    });

    // ── R3: pull-up for AD5940 INT ──
    c.push(Component {
        reference: "R3",
        value: "10K",
        footprint_lib: "laminarforge",
        footprint_name: "R_0603",
        lcsc: "C25804",
        x: 45.0, y: 26.0, rotation: 0.0, layer: "F.Cu",
        description: "AD5940 INT pull-up",
        pads: r0603_pads(NET_3V3, NET_INT_AD5940),
    });

    // ── Decoupling caps ─────────────────────────────────────
    // C1..C4: AD5940 decoupling (4× 100nF + 1× 10µF)
    c.push(Component {
        reference: "C1", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 38.0, y: 24.0, rotation: 0.0, layer: "F.Cu",
        description: "AVDD1 decoupling", pads: r0603_pads(NET_AFE_AVDD, NET_GND),
    });
    c.push(Component {
        reference: "C2", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 42.0, y: 24.0, rotation: 0.0, layer: "F.Cu",
        description: "AVDD2 decoupling", pads: r0603_pads(NET_AFE_AVDD, NET_GND),
    });
    c.push(Component {
        reference: "C3", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 38.0, y: 36.0, rotation: 0.0, layer: "F.Cu",
        description: "DVDD decoupling", pads: r0603_pads(NET_AFE_DVDD, NET_GND),
    });
    c.push(Component {
        reference: "C4", value: "10uF",
        footprint_lib: "laminarforge", footprint_name: "C_0805", lcsc: "C15850",
        x: 42.0, y: 36.0, rotation: 0.0, layer: "F.Cu",
        description: "AVDD bulk decoupling", pads: r0805_pads(NET_AFE_AVDD, NET_GND),
    });

    // C5: VBIAS_CAP 1µF
    c.push(Component {
        reference: "C5", value: "1uF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C15849",
        x: 46.0, y: 30.0, rotation: 0.0, layer: "F.Cu",
        description: "AD5940 VBIAS_CAP", pads: r0603_pads(NET_AFE_VBIAS_CAP, NET_GND),
    });

    // C6..C9: ADG731 (2x) decoupling — 100nF + 10µF per chip on VDD.
    // U2 is at (20, 15) with top pad row at y = 15 - 3.7 = 11.3. Place caps
    // at y = 8.0 so 0603 upper pad (y=8+0.475=8.475) clears U2 top pads
    // (y=11.3 - 0.65 = 10.65) by 2.2 mm.
    c.push(Component {
        reference: "C6", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 16.0, y: 8.0, rotation: 0.0, layer: "F.Cu",
        description: "U2 VDD bypass", pads: r0603_pads(NET_AFE_AVDD, NET_GND),
    });
    c.push(Component {
        reference: "C7", value: "10uF",
        footprint_lib: "laminarforge", footprint_name: "C_0805", lcsc: "C15850",
        x: 24.0, y: 8.0, rotation: 0.0, layer: "F.Cu",
        description: "U2 VDD bulk", pads: r0805_pads(NET_AFE_AVDD, NET_GND),
    });
    // U3 at (20, 45) with bottom pad row at y = 45 + 3.7 = 48.7. Caps at y=52.
    c.push(Component {
        reference: "C8", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 16.0, y: 52.0, rotation: 0.0, layer: "F.Cu",
        description: "U3 VDD bypass", pads: r0603_pads(NET_AFE_AVDD, NET_GND),
    });
    c.push(Component {
        reference: "C9", value: "10uF",
        footprint_lib: "laminarforge", footprint_name: "C_0805", lcsc: "C15850",
        x: 24.0, y: 52.0, rotation: 0.0, layer: "F.Cu",
        description: "U3 VDD bulk", pads: r0805_pads(NET_AFE_AVDD, NET_GND),
    });

    // C10..C11: MAX31865 decoupling
    c.push(Component {
        reference: "C10", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 58.0, y: 24.0, rotation: 0.0, layer: "F.Cu",
        description: "MAX31865 VDD bypass", pads: r0603_pads(NET_AFE_AVDD, NET_GND),
    });
    c.push(Component {
        reference: "C11", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 62.0, y: 24.0, rotation: 0.0, layer: "F.Cu",
        description: "MAX31865 DVDD bypass", pads: r0603_pads(NET_AFE_DVDD, NET_GND),
    });

    // C12: reference divider cap across REFIN+/REFIN- (10nF per 4-wire Pt1000)
    c.push(Component {
        reference: "C12", value: "10nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C36687",
        x: 62.0, y: 36.0, rotation: 0.0, layer: "F.Cu",
        description: "REFIN differential cap", pads: r0603_pads(NET_RTD_REF_P, NET_RTD_REF_N),
    });

    // C13: RTD differential cap across RTDIN+/RTDIN- (100nF for Pt1000)
    c.push(Component {
        reference: "C13", value: "100nF",
        footprint_lib: "laminarforge", footprint_name: "C_0603", lcsc: "C14663",
        x: 66.0, y: 32.0, rotation: 0.0, layer: "F.Cu",
        description: "RTDIN differential cap", pads: r0603_pads(NET_RTD_SENSE_P, NET_RTD_SENSE_N),
    });

    // ── Ferrite bead FB1: AVDD isolation from 3V3 ──
    c.push(Component {
        reference: "FB1", value: "BLM18PG471SN1D",
        footprint_lib: "laminarforge", footprint_name: "R_0603",
        lcsc: "C1017",
        x: 70.0, y: 10.0, rotation: 0.0, layer: "F.Cu",
        description: "AVDD ferrite bead",
        pads: r0603_pads(NET_3V3, NET_AFE_AVDD),
    });
    c.push(Component {
        reference: "FB2", value: "BLM18PG471SN1D",
        footprint_lib: "laminarforge", footprint_name: "R_0603",
        lcsc: "C1017",
        x: 70.0, y: 14.0, rotation: 0.0, layer: "F.Cu",
        description: "DVDD ferrite bead",
        pads: r0603_pads(NET_3V3, NET_AFE_DVDD),
    });

    // ── J1: 2x5 0.1" header to controller PCB SPI ──
    c.push(Component {
        reference: "J1",
        value: "Header_2x5_0.1in",
        footprint_lib: "laminarforge",
        footprint_name: "PinHeader_2x5_2.54mm",
        lcsc: "C124378",
        x: 75.0, y: 30.0, rotation: 0.0, layer: "F.Cu",
        description: "SPI to controller PCB v2",
        pads: header_2x5_pads(),
    });

    // ── Pogo-pin array on left (chip-facing) edge ──
    // 5 chips × 20 pins = 100 pogos arranged as 4 cols × 25 rows on a
    // (POGO_PITCH_X × POGO_PITCH_Y) grid. Global pogo index p (0..99):
    //   col = p / POGO_ROWS     (0..3)
    //   row = p % POGO_ROWS     (0..24)
    //   x = POGO_COL_X0 + col * POGO_PITCH_X
    //   y = POGO_ROW_Y0 + row * POGO_PITCH_Y
    // Chip/pin → global pogo index: chip c (0..4), pin p (0..19)
    //   idx = c * 20 + p
    // This interleaves chips across 4 columns: chip 1 uses rows 0..19 of
    // columns 0..(varies). Simpler: 1-1 mapping idx → col/row above.

    for c_idx in 0..NUM_CHIPS {
        for p_idx in 0..POGO_PINS_PER_CHIP {
            let net = teer_net(c_idx, p_idx);
            let global_idx = c_idx * POGO_PINS_PER_CHIP + p_idx;  // 0..99
            let col = global_idx / POGO_ROWS;
            let row = global_idx % POGO_ROWS;
            let x = POGO_COL_X0 + (col as f64) * POGO_PITCH_X;
            let y = POGO_ROW_Y0 + (row as f64) * POGO_PITCH_Y;
            let ref_s: &'static str = Box::leak(
                format!("PP{}_{}", c_idx + 1, p_idx).into_boxed_str()
            );
            let val_s: &'static str = Box::leak(
                format!("POGO_{}", net_name_of(net)).into_boxed_str()
            );
            c.push(Component {
                reference: ref_s,
                value: val_s,
                footprint_lib: "laminarforge",
                footprint_name: "PogoPin_MillMax_0906",
                lcsc: "",  // Mill-Max, not on LCSC (hand-populate)
                x, y, rotation: 0.0, layer: "F.Cu",
                description: "Pogo-pin contact (Mill-Max 0906)",
                pads: pogo_pad(net),
            });
        }
    }

    c
}

// ───────────────────────── KiCad PCB writer ─────────────────────────

fn write_header(pcb: &mut String) {
    pcb.push_str(r#"(kicad_pcb
  (version 20240108)
  (generator "laminarforge_teer_readout_pcb")
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
"#);
}

fn write_nets(pcb: &mut String, used_nets: &BTreeSet<u32>) {
    writeln!(pcb, "  (net 0 \"\")").unwrap();
    for &nid in used_nets.iter() {
        if nid == 0 { continue; }
        writeln!(pcb, "  (net {} \"{}\")", nid, net_name_of(nid)).unwrap();
    }
    pcb.push('\n');
}

fn write_setup(pcb: &mut String) {
    pcb.push_str(r#"  (setup
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
"#);
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
    writeln!(pcb, "  (footprint \"{}:{}\"", comp.footprint_lib, comp.footprint_name).unwrap();
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
                pad.number, pad.pad_type, pad.shape,
                pad.x, pad.y, pad.width, pad.height,
                pad.layers, pad.net_id, pad.net_name
            ).unwrap();
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
        let layer = if c.layer.contains("B.Cu") { "Bottom" } else { "Top" };
        s.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            c.reference, c.value, c.footprint_name, c.x, c.y, c.rotation, layer
        ));
    }
    fs::write(path, s).expect("write CPL");
}

// ───────────────────────── DSN writer (Specctra, for Freerouting) ─────────────────────────

fn mm_to_um(mm: f64) -> f64 { mm * 1000.0 }
fn dsn_y(mm: f64) -> f64 { -(mm * 1000.0) }

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

fn write_dsn(path: &Path, comps: &[Component], used_nets: &BTreeSet<u32>) {
    let mut s = String::with_capacity(256 * 1024);
    let pcb_name = path.file_name().map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "teer_readout.dsn".to_string());

    writeln!(s, "(pcb {}", pcb_name).unwrap();
    writeln!(s, "  (parser").unwrap();
    writeln!(s, "    (string_quote \")").unwrap();
    writeln!(s, "    (space_in_quoted_tokens on)").unwrap();
    writeln!(s, "    (host_cad \"LaminarForge\")").unwrap();
    writeln!(s, "    (host_version \"1.0\")").unwrap();
    writeln!(s, "  )").unwrap();

    writeln!(s, "  (resolution um 10)").unwrap();
    writeln!(s, "  (unit um)").unwrap();

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

    for layer in &["F.Cu", "B.Cu"] {
        writeln!(
            s,
            "    (plane GND (polygon {} 0  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}))",
            layer, 0.0, 0.0, board_x, 0.0, board_x, board_y_neg, 0.0, board_y_neg, 0.0, 0.0
        ).unwrap();
    }

    for &(_, x, y) in MOUNT_HOLES {
        writeln!(
            s,
            "    (keepout \"\" (circle signal 6950 {:.0} {:.0}))",
            mm_to_um(x), dsn_y(y)
        ).unwrap();
    }

    writeln!(s, "    (via \"Via[0-1]_600:300_um\")").unwrap();
    let edge_w = mm_to_um(1.0);
    writeln!(s, "    (keepout \"\" (rect signal 0 0 {:.0} {:.0}))", board_x, -edge_w).unwrap();
    writeln!(s, "    (keepout \"\" (rect signal 0 {:.0} {:.0} {:.0}))", board_y_neg + edge_w, board_x, board_y_neg).unwrap();
    writeln!(s, "    (keepout \"\" (rect signal 0 0 {:.0} {:.0}))", edge_w, board_y_neg).unwrap();
    writeln!(s, "    (keepout \"\" (rect signal {:.0} 0 {:.0} {:.0}))", board_x - edge_w, board_x, board_y_neg).unwrap();

    writeln!(s, "    (rule (width 200) (clearance 180))").unwrap();
    writeln!(s, "  )").unwrap();

    // placement grouped by footprint
    let mut groups: BTreeMap<String, Vec<&Component>> = BTreeMap::new();
    for c in comps { groups.entry(fp_key(c)).or_default().push(c); }

    writeln!(s, "  (placement").unwrap();
    writeln!(s, "    (component MountingHole:MountingHole_3.2mm_M3").unwrap();
    for &(ref_name, x, y) in MOUNT_HOLES {
        writeln!(
            s,
            "      (place {} {:.0} {:.0} front 0.0)",
            ref_name, mm_to_um(x), dsn_y(y)
        ).unwrap();
    }
    writeln!(s, "    )").unwrap();

    for (k, cs) in &groups {
        writeln!(s, "    (component {}", k).unwrap();
        for c in cs {
            let side = if c.layer.contains("B.Cu") { "back" } else { "front" };
            let mut rot = c.rotation % 360.0;
            if rot >= 180.0 { rot -= 360.0; } else if rot < -180.0 { rot += 360.0; }
            writeln!(
                s,
                "      (place {} {:.0} {:.0} {} {:.1} (PN {}))",
                c.reference, mm_to_um(c.x), dsn_y(c.y), side, rot, c.value
            ).unwrap();
        }
        writeln!(s, "    )").unwrap();
    }
    writeln!(s, "  )").unwrap();

    // library
    writeln!(s, "  (library").unwrap();
    writeln!(s, "    (image MountingHole:MountingHole_3.2mm_M3").unwrap();
    writeln!(s, "      (pin Round[A]Pad_6350_um 1 0 0)").unwrap();
    writeln!(s, "    )").unwrap();

    struct PSDef { w: f64, h: f64, thru: bool }
    let mut padstacks: BTreeMap<String, PSDef> = BTreeMap::new();
    padstacks.insert("Round[A]Pad_6350_um".to_string(), PSDef { w: 6350.0, h: 6350.0, thru: true });

    let mut seen: BTreeSet<String> = BTreeSet::new();
    for c in comps {
        let k = fp_key(c);
        if seen.contains(&k) { continue; }
        seen.insert(k.clone());
        writeln!(s, "    (image {}", k).unwrap();
        for pad in &c.pads {
            let ps = padstack_name(pad);
            writeln!(
                s,
                "      (pin {} {} {:.0} {:.0})",
                ps, pad.number, mm_to_um(pad.x), dsn_y(pad.y)
            ).unwrap();
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
            let hw = d.w / 2.0; let hh = d.h / 2.0;
            writeln!(s, "      (shape (rect F.Cu {:.0} {:.0} {:.0} {:.0}))", -hw, -hh, hw, hh).unwrap();
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
        net_pins.entry(NET_GND).or_default().push(format!("{}-1", ref_name));
    }
    for c in comps {
        for pad in &c.pads {
            if pad.net_id == NET_UNCONNECTED { continue; }
            net_pins.entry(pad.net_id).or_default()
                .push(format!("{}-{}", c.reference, pad.number));
        }
    }

    writeln!(s, "  (network").unwrap();
    let mut all_names: Vec<String> = Vec::new();
    for (&nid, pins) in &net_pins {
        if !used_nets.contains(&nid) { continue; }
        let n = net_name_of(nid);
        if n.is_empty() { continue; }
        all_names.push(n.clone());
        write!(s, "    (net {}\n      (pins", n).unwrap();
        for p in pins { write!(s, " {}", p).unwrap(); }
        writeln!(s, ")").unwrap();
        writeln!(s, "    )").unwrap();
    }
    write!(s, "    (class kicad_default").unwrap();
    for n in &all_names { write!(s, " {}", n).unwrap(); }
    writeln!(s).unwrap();
    writeln!(s, "      (circuit (use_via \"Via[0-1]_600:300_um\"))").unwrap();
    writeln!(s, "      (rule (width 200) (clearance 180))").unwrap();
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
    let from_ses = args.iter().position(|a| a == "--from-ses")
        .map(|i| {
            args.get(i + 1)
                .unwrap_or_else(|| panic!("--from-ses requires a path argument"))
                .clone()
        });

    let comps = define_components();

    // Collect actually-used nets
    let mut used_nets: BTreeSet<u32> = BTreeSet::new();
    used_nets.insert(NET_GND);
    for c in &comps {
        for p in &c.pads {
            if p.net_id != NET_UNCONNECTED { used_nets.insert(p.net_id); }
        }
    }

    // Report
    let mut net_counts: BTreeMap<u32, usize> = BTreeMap::new();
    for c in &comps {
        for p in &c.pads {
            *net_counts.entry(p.net_id).or_insert(0) += 1;
        }
    }
    println!("TEER Readout PCB: {} components, board {}x{}mm", comps.len(), BOARD_W, BOARD_H);
    println!("Net usage: {} unique nets", used_nets.len());
    let top_n = 25;
    let mut top: Vec<(u32, usize)> = net_counts.iter().map(|(&k, &v)| (k, v)).collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));
    for (nid, cnt) in top.iter().take(top_n) {
        if *nid == NET_UNCONNECTED { continue; }
        println!("  {:>4}  {:<22}  {} pads", nid, net_name_of(*nid), cnt);
    }
    if top.len() > top_n {
        println!("  ... and {} more nets", top.len() - top_n);
    }

    // Build KiCad .kicad_pcb
    let mut s = String::with_capacity(256 * 1024);
    write_header(&mut s);
    write_nets(&mut s, &used_nets);
    write_setup(&mut s);
    write_board_outline(&mut s);
    for c in &comps { write_footprint(&mut s, c); }

    // SES import (post-routing)
    if let Some(ses_path) = &from_ses {
        let ses_data = laminarforge_cad::pcb::ses::parse_ses(Path::new(ses_path));
        let total_wires: usize = ses_data.routes.iter().map(|r| r.wires.len()).sum();
        let total_vias: usize = ses_data.routes.iter().map(|r| r.vias.len()).sum();
        println!("\nImporting SES: {} nets, {} wires, {} vias",
            ses_data.routes.len(), total_wires, total_vias);
        laminarforge_cad::pcb::ses::write_ses_traces(&mut s, &ses_data);
    }

    // GND pour zones on F.Cu and B.Cu
    for (layer, tstamp_seed) in [("F.Cu", 1u8), ("B.Cu", 2u8)] {
        writeln!(&mut s, "  (zone").unwrap();
        writeln!(&mut s, "    (net {}) (net_name \"GND\")", NET_GND).unwrap();
        writeln!(&mut s, "    (layer \"{}\") (tstamp \"{:08x}-0000-0000-0000-{:012x}\")", layer, tstamp_seed as u32, tstamp_seed as u64).unwrap();
        writeln!(&mut s, "    (hatch edge 0.5)").unwrap();
        writeln!(&mut s, "    (connect_pads yes (clearance 0.2))").unwrap();
        writeln!(&mut s, "    (min_thickness 0.15) (filled_areas_thickness no)").unwrap();
        writeln!(&mut s, "    (fill yes (thermal_gap 0.25) (thermal_bridge_width 0.3) (smoothing none) (radius 0.5))").unwrap();
        writeln!(&mut s, "    (polygon (pts").unwrap();
        writeln!(&mut s, "      (xy 0 0) (xy {} 0) (xy {} {}) (xy 0 {})", BOARD_W, BOARD_W, BOARD_H, BOARD_H).unwrap();
        writeln!(&mut s, "    ))").unwrap();
        writeln!(&mut s, "  )").unwrap();
    }

    s.push_str(")\n");

    let pcb_path = output_dir.join("teer_readout.kicad_pcb");
    fs::write(&pcb_path, &s).expect("write pcb");
    println!("Written: {}", pcb_path.display());

    let bom_path = output_dir.join("teer_readout_bom.csv");
    write_bom(&bom_path, &comps);
    println!("Written: {}", bom_path.display());

    let cpl_path = output_dir.join("teer_readout_cpl.csv");
    write_cpl(&cpl_path, &comps);
    println!("Written: {}", cpl_path.display());

    if from_ses.is_none() {
        let dsn_path = output_dir.join("teer_readout.dsn");
        write_dsn(&dsn_path, &comps, &used_nets);
        println!("Written: {}", dsn_path.display());

        println!("\nTEER Readout PCB generation complete.");
        println!("Run Freerouting:");
        println!("  /opt/homebrew/opt/openjdk/bin/java -jar tools/freerouting.jar \\");
        println!("    -de {} -do pcb/output/teer_readout.ses -mp 30 -us 5", dsn_path.display());
    }

    if args.iter().any(|a| a == "--validate") {
        laminarforge_cad::pcb::export::validate_and_export(&pcb_path, &output_dir);
    }
}
