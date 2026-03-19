use laminarforge_cad::pcb::components::define_components;
use laminarforge_cad::pcb::{pad_abs_pos, pad_idx};

fn main() {
    let components = define_components();

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           LaminarForge PCB — Absolute Pad Positions            ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    for comp in &components {
        println!("── {} ({}) @ ({:.4}, {:.4}) rot={:.0}° ──",
            comp.reference, comp.value, comp.x, comp.y, comp.rotation);
        for (idx, pad) in comp.pads.iter().enumerate() {
            let (ax, ay) = pad_abs_pos(comp, idx);
            let net_display = if pad.net_name.is_empty() { "NC" } else { pad.net_name };
            println!("  {} pad {}: ({:.4}, {:.4}) net={}",
                comp.reference, pad.number, ax, ay, net_display);
        }
        println!();
    }

    // ── Summary: Key routing targets grouped by signal type ──
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║                  Routing Target Summary by Signal              ║");
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    // Helper: find component by reference
    let c = |r: &str| -> &laminarforge_cad::pcb::Component {
        components.iter().find(|c| c.reference == r).unwrap()
    };
    let ap = |comp: &laminarforge_cad::pcb::Component, pn: &str| -> (f64, f64) {
        pad_abs_pos(comp, pad_idx(comp, pn))
    };

    // I2C bus
    println!("── I2C (SDA / SCL) ──");
    let u1 = c("U1");
    let u2 = c("U2");
    let r17 = c("R17");
    let r18 = c("R18");
    let (x, y) = ap(u1, "16"); println!("  SDA: U1 pin16  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(r17, "2"); println!("  SDA: R17 pad2  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u2, "9");  println!("  SDA: U2 pin9   ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u1, "17"); println!("  SCL: U1 pin17  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(r18, "2"); println!("  SCL: R18 pad2  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u2, "10"); println!("  SCL: U2 pin10  ({:.4}, {:.4})", x, y);
    println!();

    // MUX select
    println!("── MUX Select (S0 / S1 / S2) ──");
    let u3 = c("U3");
    let (x, y) = ap(u1, "18"); println!("  MUX_S0: U1 pin18 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u3, "11"); println!("  MUX_S0: U3 pin11 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u1, "19"); println!("  MUX_S1: U1 pin19 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u3, "10"); println!("  MUX_S1: U3 pin10 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u1, "20"); println!("  MUX_S2: U1 pin20 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u3, "9");  println!("  MUX_S2: U3 pin9  ({:.4}, {:.4})", x, y);
    println!();

    // MUX COM → ADC
    println!("── MUX COM → ADC ──");
    let (x, y) = ap(u3, "3");  println!("  MUX_COM: U3 pin3  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u2, "4");  println!("  ADC_AIN0: U2 pin4 ({:.4}, {:.4})", x, y);
    println!();

    // ADC AIN1 (thermistor)
    println!("── ADC AIN1 (Thermistor) ──");
    let r19 = c("R19");
    let j3 = c("J3");
    let (x, y) = ap(r19, "2"); println!("  ADC_AIN1: R19 pad2 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(j3, "1");  println!("  ADC_AIN1: J3 pin1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u2, "5");  println!("  ADC_AIN1: U2 pin5  ({:.4}, {:.4})", x, y);
    println!();

    // Heater PWM chain
    println!("── Heater PWM → Gate Driver → MOSFET ──");
    let u4 = c("U4");
    let q1 = c("Q1");
    let (x, y) = ap(u1, "15"); println!("  HEATER_PWM: U1 pin15 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u4, "1");  println!("  HEATER_PWM: U4 pin1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u4, "7");  println!("  MOSFET_GATE: U4 pin7 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(q1, "1");  println!("  MOSFET_GATE: Q1 pin1 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(q1, "3");  println!("  HEATER_P: Q1 pin3    ({:.4}, {:.4})", x, y);
    println!();

    // USB
    println!("── USB (DP / DN) ──");
    let j1 = c("J1");
    let (x, y) = ap(j1, "A6"); println!("  USB_DP: J1 padA6  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u1, "13"); println!("  USB_DP: U1 pin13  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(j1, "A7"); println!("  USB_DN: J1 padA7  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u1, "14"); println!("  USB_DN: U1 pin14  ({:.4}, {:.4})", x, y);
    println!();

    // Power rails: +12V
    println!("── +12V Power ──");
    let j2 = c("J2");
    let j4 = c("J4");
    let d9 = c("D9");
    let (x, y) = ap(j2, "1");  println!("  +12V: J2 pin1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(j4, "1");  println!("  +12V: J4 pin1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(d9, "2");  println!("  +12V: D9 pad2  ({:.4}, {:.4})", x, y);
    println!();

    // HEATER_P net
    println!("── HEATER_P ──");
    let (x, y) = ap(q1, "3");  println!("  HEATER_P: Q1 pin3 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(d9, "1");  println!("  HEATER_P: D9 pad1 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(j4, "2");  println!("  HEATER_P: J4 pin2 ({:.4}, {:.4})", x, y);
    println!();

    // +5V bus
    println!("── +5V Power ──");
    let (x, y) = ap(u4, "6"); println!("  +5V: U4 pin6 ({:.4}, {:.4})", x, y);
    for i in 0..8u32 {
        let rn: &str = Box::leak(format!("R{}", i + 9).into_boxed_str());
        let r = c(rn);
        let (x, y) = ap(r, "1"); println!("  +5V: {} pad1 ({:.4}, {:.4})", rn, x, y);
    }
    let c4 = c("C4");
    let c5 = c("C5");
    let (x, y) = ap(c4, "1"); println!("  +5V: C4 pad1 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(c5, "1"); println!("  +5V: C5 pad1 ({:.4}, {:.4})", x, y);
    println!();

    // +3V3 bus
    println!("── +3V3 Power ──");
    let (x, y) = ap(u1, "2");  println!("  +3V3: U1 pin2  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u1, "3");  println!("  +3V3: U1 pin3  ({:.4}, {:.4})", x, y);
    let c1 = c("C1");
    let c2 = c("C2");
    let c3 = c("C3");
    let (x, y) = ap(c1, "1");  println!("  +3V3: C1 pad1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(r17, "1"); println!("  +3V3: R17 pad1 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(r18, "1"); println!("  +3V3: R18 pad1 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(r19, "1"); println!("  +3V3: R19 pad1 ({:.4}, {:.4})", x, y);
    let (x, y) = ap(c2, "1");  println!("  +3V3: C2 pad1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u2, "8");  println!("  +3V3: U2 pin8  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(c3, "1");  println!("  +3V3: C3 pad1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u3, "16"); println!("  +3V3: U3 pin16 ({:.4}, {:.4})", x, y);
    println!();

    // LED_BASE_0..7
    println!("── LED Base Drives (LED_BASE_0..7) ──");
    for i in 0..8u32 {
        let pin: &str = Box::leak(format!("{}", 21 + i).into_boxed_str());
        let rn: &str = Box::leak(format!("R{}", i + 1).into_boxed_str());
        let qn: &str = Box::leak(format!("Q{}", i + 2).into_boxed_str());
        let r = c(rn);
        let q = c(qn);
        let (ex, ey) = ap(u1, pin);
        let (rx, ry) = ap(r, "1");
        let (qx, qy) = ap(q, "1");
        println!("  LED_BASE_{}: U1 pin{} ({:.4}, {:.4}) → {} pad1 ({:.4}, {:.4}) → {} pad1 ({:.4}, {:.4})",
            i, 21+i, ex, ey, rn, rx, ry, qn, qx, qy);
    }
    println!();

    // LED_COL_0..7
    println!("── LED Collector Chains (LED_COL_0..7) ──");
    for i in 0..8u32 {
        let dn: &str = Box::leak(format!("D{}", i + 1).into_boxed_str());
        let rn: &str = Box::leak(format!("R{}", i + 9).into_boxed_str());
        let qn: &str = Box::leak(format!("Q{}", i + 2).into_boxed_str());
        let d = c(dn);
        let r = c(rn);
        let q = c(qn);
        let (dx, dy) = ap(d, "1");
        let (rx, ry) = ap(r, "2");
        let (qx, qy) = ap(q, "3");
        println!("  LED_COL_{}: {} pad1 ({:.4}, {:.4}) → {} pad2 ({:.4}, {:.4}) → {} pad3 ({:.4}, {:.4})",
            i, dn, dx, dy, rn, rx, ry, qn, qx, qy);
    }
    println!();

    // MUX_Y0..7
    println!("── Photodiode → MUX Channels (MUX_Y0..7) ──");
    let mux_pins = ["13", "14", "15", "12", "1", "5", "2", "4"];
    for i in 0..8u32 {
        let pdn: &str = Box::leak(format!("PD{}", i + 1).into_boxed_str());
        let pd = c(pdn);
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, mux_pins[i as usize]);
        println!("  MUX_Y{}: {} pad1 ({:.4}, {:.4}) → U3 pin{} ({:.4}, {:.4})",
            i, pdn, px, py, mux_pins[i as usize], mx, my);
    }
    println!();

    // GND summary (not exhaustive, just key connection points)
    println!("── GND (key pads) ──");
    let (x, y) = ap(u1, "GND"); println!("  GND: U1 padGND ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u2, "1");   println!("  GND: U2 pin1   ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u2, "3");   println!("  GND: U2 pin3   ({:.4}, {:.4})", x, y);
    let (x, y) = ap(u4, "2");   println!("  GND: U4 pin2   ({:.4}, {:.4})", x, y);
    let (x, y) = ap(q1, "2");   println!("  GND: Q1 pin2   ({:.4}, {:.4})", x, y);
    let (x, y) = ap(j1, "A1");  println!("  GND: J1 padA1  ({:.4}, {:.4})", x, y);
    let (x, y) = ap(j2, "2");   println!("  GND: J2 pin2   ({:.4}, {:.4})", x, y);
    println!();

    // VBUS
    println!("── VBUS ──");
    let (x, y) = ap(j1, "A4"); println!("  VBUS: J1 padA4 ({:.4}, {:.4})", x, y);
    println!();
}
