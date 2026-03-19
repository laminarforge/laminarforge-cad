use crate::*;
use super::{Component, write_trace, write_via, pad_abs_pos, pad_idx};
use super::nets::*;

fn comp<'a>(components: &'a [Component], reference: &str) -> &'a Component {
    components.iter().find(|c| c.reference == reference)
        .unwrap_or_else(|| panic!("component {} not found", reference))
}

fn ap(c: &Component, pad_num: &str) -> (f64, f64) {
    pad_abs_pos(c, pad_idx(c, pad_num))
}

pub fn write_signal_traces(pcb: &mut String, components: &[Component]) {
    let u1 = comp(components, "U1");
    let u2 = comp(components, "U2");
    let u3 = comp(components, "U3");
    let u4 = comp(components, "U4");
    let q1 = comp(components, "Q1");
    let d9 = comp(components, "D9");
    let d10 = comp(components, "D10");
    let j1 = comp(components, "J1");
    let j2 = comp(components, "J2");
    let j3 = comp(components, "J3");
    let j4 = comp(components, "J4");
    let u6 = comp(components, "U6");

    let sw = SIGNAL_TRACE_WIDTH; // 0.25
    let pw = POWER_TRACE_WIDTH;  // 0.5

    // ═══════════════════════════════════════════════════════════════════
    // USB_DN: J1 A7 (9.25,5) → U1 pin13 (12,23.83)
    // Pin 13 = GPIO19 = USB_D- per ESP32-S3-WROOM-1 datasheet.
    // Route DN first (shorter), then DP crosses under via B.Cu hop.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (jx, jy) = ap(j1, "A7");   // (9.25, 5)
        let (ex, ey) = ap(u1, "13");    // (12, 23.83)
        // Jog east to x=9.3 for clearance from SW1 pad2 (right edge x=9.0).
        write_trace(pcb, jx, jy, 9.3, jy, sw, "F.Cu", NET_USB_DN, "USB_DN");
        write_trace(pcb, 9.3, jy, 9.3, ey, sw, "F.Cu", NET_USB_DN, "USB_DN");
        write_trace(pcb, 9.3, ey, ex, ey, sw, "F.Cu", NET_USB_DN, "USB_DN");
    }

    // ═══════════════════════════════════════════════════════════════════
    // USB_DP: J1 A6 (9.75,5) → U1 pin14 (12,25.1)
    // Pin 14 = GPIO20 = USB_D+ per ESP32-S3-WROOM-1 datasheet.
    // F.Cu south at x=9.75, jog east to x=10.5 for B.Cu hop to
    // cross under DN horizontal at y=23.83 (x=9.3→12).
    // Via clearance: (10.5-0.35)=10.15 vs DN right=9.425: 0.725mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (jx, jy) = ap(j1, "A6");   // (9.75, 5)
        let (ex, ey) = ap(u1, "14");    // (12, 25.1)
        // F.Cu south at x=9.75 to y=22 (before DN horizontal at y=23.83)
        write_trace(pcb, jx, jy, jx, 22.0, sw, "F.Cu", NET_USB_DP, "USB_DP");
        // Jog east to x=10.5 for via clearance from DN at x=9.3
        write_trace(pcb, jx, 22.0, 10.5, 22.0, sw, "F.Cu", NET_USB_DP, "USB_DP");
        // B.Cu hop past DN horizontal
        write_via(pcb, 10.5, 22.0, VIA_PAD, VIA_DRILL, NET_USB_DP);
        write_trace(pcb, 10.5, 22.0, 10.5, 24.5, sw, "B.Cu", NET_USB_DP, "USB_DP");
        write_via(pcb, 10.5, 24.5, VIA_PAD, VIA_DRILL, NET_USB_DP);
        // F.Cu south to pin14 y, then east to pin14
        write_trace(pcb, 10.5, 24.5, 10.5, ey, sw, "F.Cu", NET_USB_DP, "USB_DP");
        write_trace(pcb, 10.5, ey, ex, ey, sw, "F.Cu", NET_USB_DP, "USB_DP");
    }

    // ═══════════════════════════════════════════════════════════════════
    // SDA: U1 pin16 (12,27.64) → R17 pad2 (45.95,10) → U2 pin9 (51.5,14.5)
    //
    // Route: F.Cu west to (9.0,27.64). Via. B.Cu north at x=9 to y=3. Via.
    //   F.Cu east at y=3 to x=43. South to y=5. Via. B.Cu east to R17 pad2.
    //   x=9.0 clears H1 (dist 4.47mm > pad radius 3.175mm). OK.
    //   B.Cu at x=9.0: HEATER_PWM B.Cu at x=10.5 gap=1.25mm. OK.
    //
    // SDA → U2: south to y=14, via, B.Cu east to x=53, via, F.Cu south, west to pin.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (ex, ey) = ap(u1, "16");    // (12, 27.64)
        let r17 = comp(components, "R17");
        let (r17p2x, _) = ap(r17, "2"); // (45.95, 10)
        let (u2p9x, u2p9y) = ap(u2, "9"); // (51.5, 14.5)

        // U1 pin → via → B.Cu north at x=9 (with F.Cu hop around LB0 at y=10)
        //   → via → F.Cu east at y=3 → B.Cu hop over SCL at x=42 → south to R17.
        //
        // F.Cu west to (9.0,27.64). Via. B.Cu north at x=9 to y=11.5.
        //   B.Cu west to (8.5,11.5). Via at (8.5,11.5). F.Cu north at x=8.5 to y=8.5.
        //   Via at (8.5,8.5). B.Cu east to (9,8.5). B.Cu north to (9,3). Via at (9,3).
        //
        //   This F.Cu hop at x=8.5 clears USB_DN at x=9.25: gap=0.5mm. OK.
        //   Via (8.5,11.5) right edge=8.85, USB_DN left=9.125: gap=0.275mm. OK.
        //   Via (8.5,8.5): dist to H1=4.95mm >> 3.3mm. OK.
        //   B.Cu at x=9 y=8.5-3: below LB0 at y=10. No crossing.
        write_trace(pcb, ex, ey, 9.0, ey, sw, "F.Cu", NET_SDA, "SDA");
        write_via(pcb, 9.0, ey, VIA_PAD, VIA_DRILL, NET_SDA);
        write_trace(pcb, 9.0, ey, 9.0, 11.5, sw, "B.Cu", NET_SDA, "SDA");
        write_trace(pcb, 9.0, 11.5, 8.5, 11.5, sw, "B.Cu", NET_SDA, "SDA");
        // Via moved from (8.5,11.5) to (8.5,12.0) to clear ESP_EN B.Cu at y=11.
        // Pad bottom=11.65, ESP_EN top=11.125, gap=0.525mm. OK.
        write_trace(pcb, 8.5, 11.5, 8.5, 12.0, sw, "B.Cu", NET_SDA, "SDA");
        write_via(pcb, 8.5, 12.0, VIA_PAD, VIA_DRILL, NET_SDA);
        write_trace(pcb, 8.5, 12.0, 8.5, 8.5, sw, "F.Cu", NET_SDA, "SDA");
        write_via(pcb, 8.5, 8.5, VIA_PAD, VIA_DRILL, NET_SDA);
        write_trace(pcb, 8.5, 8.5, 8.75, 8.5, sw, "B.Cu", NET_SDA, "SDA");
        write_trace(pcb, 8.75, 8.5, 8.75, 3.0, sw, "B.Cu", NET_SDA, "SDA");
        write_via(pcb, 8.75, 3.0, VIA_PAD, VIA_DRILL, NET_SDA);
        // SDA F.Cu at y=3 to x=31.5, then B.Cu at y=3.5 all the way to R17 pad2.
        // B.Cu at y=3.5 passes under SCL F.Cu at x=42 (different layer, no crossing).
        // C9 pad2 GND at (42,5) bottom edge y=4.1: gap to B.Cu y=3.625 = 0.475mm. OK.
        // U6 tab at (35,2.5) is F.Cu-only: B.Cu passes under it. OK.
        write_trace(pcb, 8.75, 3.0, 31.5, 3.0, sw, "F.Cu", NET_SDA, "SDA");
        write_via(pcb, 31.5, 3.0, VIA_PAD, VIA_DRILL, NET_SDA);
        write_trace(pcb, 31.5, 3.0, 31.5, 3.5, sw, "B.Cu", NET_SDA, "SDA");
        write_trace(pcb, 31.5, 3.5, r17p2x, 3.5, sw, "B.Cu", NET_SDA, "SDA");
        write_via(pcb, r17p2x, 3.5, VIA_PAD, VIA_DRILL, NET_SDA);
        write_trace(pcb, r17p2x, 3.5, r17p2x, 10.0, sw, "F.Cu", NET_SDA, "SDA");

        // SDA → U2 pin9: south to y=14, via, B.Cu east to x=53, via, F.Cu south, west to pin
        // Via at (53,14): +3V3 at x=54.05 edge=53.925, via edge=53.35, gap=0.575mm. OK.
        // C2 pad2 at (52.75,12): dist=2.02mm. OK.
        write_trace(pcb, r17p2x, 10.0, r17p2x, 14.0, sw, "F.Cu", NET_SDA, "SDA");
        write_via(pcb, r17p2x, 14.0, VIA_PAD, VIA_DRILL, NET_SDA);
        write_trace(pcb, r17p2x, 14.0, 53.0, 14.0, sw, "B.Cu", NET_SDA, "SDA");
        write_via(pcb, 53.0, 14.0, VIA_PAD, VIA_DRILL, NET_SDA);
        write_trace(pcb, 53.0, 14.0, 53.0, u2p9y, sw, "F.Cu", NET_SDA, "SDA");
        write_trace(pcb, 53.0, u2p9y, u2p9x, u2p9y, sw, "F.Cu", NET_SDA, "SDA");
    }

    // ═══════════════════════════════════════════════════════════════════
    // SCL: U1 pin17 (12,28.91) → R18 pad2 (51.95,10) → U2 pin10 (51.5,14)
    //
    // Route: F.Cu west to (7.0,28.91). Via. B.Cu west to (1.0,28.91). Via.
    //   F.Cu north at x=1.0 to y=1.5. East at y=1.5 to x=42. South to y=6.5.
    //   Via at (42,6.5). B.Cu east at y=6.5 to (52.0,6.5). Via at (52.0,6.5).
    //   F.Cu south at x=52.0 to R18 pad2 (51.95,10) [within pad]. Continue south.
    //
    // Clearance: SDA via (2.5,27.64) vs SCL via (1.0,28.91) dist=1.80mm. OK.
    //   SDA at x=2.5 vs SCL at x=1.0 gap=1.25mm. OK.
    //   B.Cu y=6.5 vs SDA B.Cu y=5.0 gap=1.25mm. OK.
    //   B.Cu y=6.5 vs +3V3 B.Cu y=7.5 gap=0.75mm. OK.
    //
    // SCL → U2: at x=52.0, south through C2 gap to y=14.
    //   C2 pad1(+3V3) at (51.25,12) right edge=51.625. Trace left edge=51.875. Gap=0.25mm. OK.
    //   C2 pad2(GND) at (52.75,12) left edge=52.375. Trace right edge=52.125. Gap=0.25mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (ex, ey) = ap(u1, "17");    // (12, 28.91)
        let (u2p10x, u2p10y) = ap(u2, "10"); // (51.5, 14)

        // SCL from U1 pin17: F.Cu west to x=8.5 (east of LB0 at x=7.5), via,
        // B.Cu west to x=5.0 (east of J5 PTH right 4.35, gap=0.3mm), via,
        // F.Cu north to y=27.5 (above J5 pin2 PTH top 27.88), west to x=1.0,
        // then via into B.Cu south run past switches.
        write_trace(pcb, ex, ey, 8.5, ey, sw, "F.Cu", NET_SCL, "SCL");
        write_via(pcb, 8.5, ey, VIA_PAD, VIA_DRILL, NET_SCL);
        // B.Cu south at x=8.5 to y=30 (below UART_TX horizontal at y=28.73
        // and J5 pin2 PTH bottom at y=29.58). Then west to x=2.75 (east of
        // +3V3 B.Cu at x=2.0, gap=0.275mm). Via, F.Cu west to x=1.0.
        write_trace(pcb, 8.5, ey, 8.5, 30.0, sw, "B.Cu", NET_SCL, "SCL");
        write_trace(pcb, 8.5, 30.0, 2.75, 30.0, sw, "B.Cu", NET_SCL, "SCL");
        write_via(pcb, 2.75, 30.0, VIA_PAD, VIA_DRILL, NET_SCL);
        write_trace(pcb, 2.75, 30.0, 1.0, 30.0, sw, "F.Cu", NET_SCL, "SCL");
        // Via at (1.0, 30), B.Cu south past switches to y=21.
        write_via(pcb, 1.0, 30.0, VIA_PAD, VIA_DRILL, NET_SCL);
        // B.Cu south past ESP_EN F.Cu hop at y=20.5 (different layer, no crossing).
        // Via at y=19.5 (north of ESP_EN hop). Pad bottom=19.15.
        // ESP_EN F.Cu at y=20.5 top edge=20.375. Gap=1.225mm. OK.
        // GND via at (0.75,18): dist=sqrt(0.0625+2.25)=1.52mm. OK.
        write_trace(pcb, 1.0, 30.0, 1.0, 19.5, sw, "B.Cu", NET_SCL, "SCL");
        write_via(pcb, 1.0, 19.5, VIA_PAD, VIA_DRILL, NET_SCL);
        // F.Cu north at x=1.0 to y=0.5. Board edge: 0.5-0.125=0.375mm > 0.25mm. OK.
        // East at y=0.5 to x=43.5. South at x=43.5 to y=6.5.
        write_trace(pcb, 1.0, 19.5, 1.0, 0.5, sw, "F.Cu", NET_SCL, "SCL");
        write_trace(pcb, 1.0, 0.5, 43.5, 0.5, sw, "F.Cu", NET_SCL, "SCL");
        write_trace(pcb, 43.5, 0.5, 43.5, 6.5, sw, "F.Cu", NET_SCL, "SCL");
        write_via(pcb, 43.5, 6.5, VIA_PAD, VIA_DRILL, NET_SCL);
        write_trace(pcb, 43.5, 6.5, 52.0, 6.5, sw, "B.Cu", NET_SCL, "SCL");
        write_via(pcb, 52.0, 6.5, VIA_PAD, VIA_DRILL, NET_SCL);
        write_trace(pcb, 52.0, 6.5, 52.0, 10.0, sw, "F.Cu", NET_SCL, "SCL");

        // SCL → U2 pin10: continue south at x=52.0 to y=14, west to pin
        write_trace(pcb, 52.0, 10.0, 52.0, u2p10y, sw, "F.Cu", NET_SCL, "SCL");
        write_trace(pcb, 52.0, u2p10y, u2p10x, u2p10y, sw, "F.Cu", NET_SCL, "SCL");
    }

    // ═══════════════════════════════════════════════════════════════════
    // HEATER_PWM: U1 pin15 (12,26.37) → U4 pin1 (69.3,13.095)
    //
    // Via at pin, B.Cu west to x=10.5, south to y=36, east to x=67. Via.
    // F.Cu north at x=67 to U4. East to pin.
    //
    // B.Cu at x=10.5 from y=26.37 to y=36: vertical B.Cu in electronics area.
    //   USB verticals are F.Cu at x=9.25/9.75. Different layer. OK.
    // B.Cu at y=36 from x=10.5 to x=67: clear of LED/optical area (y=50-72).
    //   LED_BASE B.Cu channels at y=10-18.89: far north. OK.
    // F.Cu at x=67 from y=36 north to y=13.095: near R7 pad1 (68.05,38) gap=0.925mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (ex, ey) = ap(u1, "15"); // (12, 26.37)
        let (tx, ty) = ap(u4, "1");  // (69.3, 13.095)

        write_via(pcb, ex, ey, VIA_PAD, VIA_DRILL, NET_HEATER_PWM);
        write_trace(pcb, ex, ey, 10.5, ey, sw, "B.Cu", NET_HEATER_PWM, "HEATER_PWM");
        write_trace(pcb, 10.5, ey, 10.5, 36.0, sw, "B.Cu", NET_HEATER_PWM, "HEATER_PWM");
        write_trace(pcb, 10.5, 36.0, 67.0, 36.0, sw, "B.Cu", NET_HEATER_PWM, "HEATER_PWM");
        write_via(pcb, 67.0, 36.0, VIA_PAD, VIA_DRILL, NET_HEATER_PWM);
        write_trace(pcb, 67.0, 36.0, 67.0, ty, sw, "F.Cu", NET_HEATER_PWM, "HEATER_PWM");
        write_trace(pcb, 67.0, ty, tx, ty, sw, "F.Cu", NET_HEATER_PWM, "HEATER_PWM");

        // Parallel INB (pin7) with INA (pin1): B.Cu stub from pin1 to pin7
        // Pin1 at (69.3, 13.095), Pin7 at (74.7, 14.365)
        // Via at pin1, B.Cu east to pin7 x, north to pin7 y, via
        let (inb_x, inb_y) = ap(u4, "7"); // (74.7, 14.365) INB
        write_via(pcb, tx, ty, VIA_PAD, VIA_DRILL, NET_HEATER_PWM);
        write_trace(pcb, tx, ty, inb_x, ty, sw, "B.Cu", NET_HEATER_PWM, "HEATER_PWM");
        write_trace(pcb, inb_x, ty, inb_x, inb_y, sw, "B.Cu", NET_HEATER_PWM, "HEATER_PWM");
        write_via(pcb, inb_x, inb_y, VIA_PAD, VIA_DRILL, NET_HEATER_PWM);
    }

    // ═══════════════════════════════════════════════════════════════════
    // MUX_S0, S1, S2: ESP32 pins → U3 right-side pins
    //
    // S0: U1 pin18 (12,30.18) → U3 pin11 (54.8,31.905)
    // S1: U1 pin19 (12,31.45) → U3 pin10 (54.8,33.175)
    // S2: U1 pin20 (12,32.72) → U3 pin9  (54.8,34.445)
    //
    // Via at U1 pin, B.Cu east to unique x, via, F.Cu south to pin y, west to pin.
    // S0 via at x=58, S1 via at x=59.5, S2 via at x=61.
    // ═══════════════════════════════════════════════════════════════════
    {
        let mux_s: [(&str, &str, u32, &str, f64); 3] = [
            ("18", "11", NET_MUX_S0, "MUX_S0", 58.0),
            ("19", "10", NET_MUX_S1, "MUX_S1", 59.5),
            ("20", "9",  NET_MUX_S2, "MUX_S2", 61.0),
        ];
        for (ep, mp, net, name, vx) in mux_s {
            let (ex, ey) = ap(u1, ep);
            let (mx, my) = ap(u3, mp);
            write_via(pcb, ex, ey, VIA_PAD, VIA_DRILL, net);
            write_trace(pcb, ex, ey, vx, ey, sw, "B.Cu", net, name);
            write_via(pcb, vx, ey, VIA_PAD, VIA_DRILL, net);
            write_trace(pcb, vx, ey, vx, my, sw, "F.Cu", net, name);
            write_trace(pcb, vx, my, mx, my, sw, "F.Cu", net, name);
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // MUX_COM: U3 pin3 (45.2,28.095) → U2 pin4 (48.5,15.5)
    //
    // Route: west from pin to x=41, north to y=20. Via. B.Cu east to (46.5,20). Via.
    // F.Cu south at x=46.5 to y=16.75. East at y=16.75 to x=49.5. North to y=15.5.
    // West to pin4 (48.5,15.5).
    //
    // This approaches pin4 from the EAST via x=49.5, avoiding the ADC_AIN1 F.Cu
    // vertical at x=47 (y=11.5→16). The detour goes SOUTH of ADC_AIN1's range.
    //   x=46.5 south to y=16.75: ADC_AIN1 at x=47 parallel gap=0.25mm. OK.
    //   y=16.75 east: ADC_AIN1 ends at y=16. Gap=0.5mm. OK.
    //   x=49.5 north: +3V3 at x=50.05, gap=0.3mm. pin5 at x=48.5 right=49.0, gap=0.375mm. OK.
    //   y=15.5 west to pin: pin3(GND) gap=0.225mm. pin5 gap=0.225mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (cx, cy) = ap(u3, "3");  // (45.2, 28.095)
        let (ax, ay) = ap(u2, "4");  // (48.5, 15.5)

        // West to x=41, north to y=20. Via. B.Cu east to x=46.5. Via.
        write_trace(pcb, cx, cy, 41.0, cy, sw, "F.Cu", NET_MUX_COM, "MUX_COM");
        write_trace(pcb, 41.0, cy, 41.0, 20.0, sw, "F.Cu", NET_MUX_COM, "MUX_COM");
        write_via(pcb, 41.0, 20.0, VIA_PAD, VIA_DRILL, NET_MUX_COM);
        write_trace(pcb, 41.0, 20.0, 46.5, 20.0, sw, "B.Cu", NET_MUX_COM, "MUX_COM");
        write_via(pcb, 46.5, 20.0, VIA_PAD, VIA_DRILL, NET_MUX_COM);
        // South past ADC_AIN1 region, east to x=49.5, north to y=15.5, west to pin
        write_trace(pcb, 46.5, 20.0, 46.5, 16.75, sw, "F.Cu", NET_MUX_COM, "MUX_COM");
        write_trace(pcb, 46.5, 16.75, 49.5, 16.75, sw, "F.Cu", NET_MUX_COM, "MUX_COM");
        write_trace(pcb, 49.5, 16.75, 49.5, ay, sw, "F.Cu", NET_MUX_COM, "MUX_COM");
        write_trace(pcb, 49.5, ay, ax, ay, sw, "F.Cu", NET_MUX_COM, "MUX_COM");
    }

    // ═══════════════════════════════════════════════════════════════════
    // ADC_AIN1: R19 pad2 (55.95,10) → J3 pin1 (60,8.73) → U2 pin5 (48.5,16)
    //
    // R19 → J3: F.Cu east at y=10 to x=60, north to J3 pin1.
    // R19 → U2: south at x=55.95 to y=11.5. Via. B.Cu west to (48.5,11.5). Via.
    //   F.Cu south at x=48.5 to y=16 (U2 pin5).
    //   x=48.5 is U2 pad column. pad1(GND) at (48.5,14), pad5 at (48.5,16).
    //   Our trace goes from y=11.5 south through pad1 to pad5. pad1 is GND, our net
    //   is ADC_AIN1. SHORT with GND!
    //
    //   Fix: use x=48 (0.5mm west of pad center). U2 pad edge at 48.5-0.5=48.0.
    //   Our trace at x=48 edge=48.125. Pad edge=48.0. Gap=48.125-48.0=0.125mm < 0.2mm. TOO CLOSE.
    //   Use x=47: pad edge=48.0. Trace edge=47.125. Gap=0.875mm. OK.
    //   F.Cu at x=47 from y=11.5 south to y=16, east to (48.5,16).
    //   MUX_COM F.Cu at x=47.5: gap=0.5-0.25=0.25mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        let r19 = comp(components, "R19");
        let (rx, ry) = ap(r19, "2");   // (55.95, 10)
        let (jx, jy) = ap(j3, "1");    // (60, 8.73)
        let (ax, ay) = ap(u2, "5");    // (48.5, 16)

        // R19 → J3
        write_trace(pcb, rx, ry, jx, ry, sw, "F.Cu", NET_ADC_AIN1, "ADC_AIN1");
        write_trace(pcb, jx, ry, jx, jy, sw, "F.Cu", NET_ADC_AIN1, "ADC_AIN1");

        // R19 → U2 pin5: south to y=11.5, via, B.Cu west to x=47, via, F.Cu south to y=16, east to pin.
        // MUX_COM now at x=46.5 (gap=0.375mm to ADC_AIN1 at x=47). No crossing.
        // +3V3 at x=50.05 edge=49.925. Via at (47,11.5) edge=47.35. Gap=2.575mm. OK.
        write_trace(pcb, rx, ry, rx, 11.5, sw, "F.Cu", NET_ADC_AIN1, "ADC_AIN1");
        write_via(pcb, rx, 11.5, VIA_PAD, VIA_DRILL, NET_ADC_AIN1);
        write_trace(pcb, rx, 11.5, 47.0, 11.5, sw, "B.Cu", NET_ADC_AIN1, "ADC_AIN1");
        write_via(pcb, 47.0, 11.5, VIA_PAD, VIA_DRILL, NET_ADC_AIN1);
        write_trace(pcb, 47.0, 11.5, 47.0, ay, sw, "F.Cu", NET_ADC_AIN1, "ADC_AIN1");
        write_trace(pcb, 47.0, ay, ax, ay, sw, "F.Cu", NET_ADC_AIN1, "ADC_AIN1");
    }

    // ═══════════════════════════════════════════════════════════════════
    // GATE_DRV: U4 pin5 OUTB (74.7,16.905) → R34 pad1 (79.05,18)
    // MOSFET_GATE: R34 pad2 (80.95,18) → Q1 pin1 (84.05,16)
    // GATE PULLDOWN: R35 pad1 (83.05,20) ← MOSFET_GATE, R35 pad2 (84.95,20) → GND
    //
    // TC4427A: INA+INB (pin1,7) = HEATER_PWM, OUTA+OUTB (pin4,5) = GATE_DRV
    // Route from pin5 (OUTB, right side) → series resistor → MOSFET gate + pulldown
    // ═══════════════════════════════════════════════════════════════════
    {
        let (tx, ty) = ap(u4, "5");  // (74.7, 16.905) — OUTB (GATE_DRV)

        let r34 = comp(components, "R34");
        let r35 = comp(components, "R35");
        let (r34p1x, r34p1y) = ap(r34, "1"); // (79.05, 18) GATE_DRV
        let (r34p2x, r34p2y) = ap(r34, "2"); // (80.95, 18) MOSFET_GATE
        let (r35p1x, r35p1y) = ap(r35, "1"); // (83.05, 20) MOSFET_GATE
        let (gx, gy) = ap(q1, "1");           // (84.05, 16) MOSFET_GATE

        // U4 pin5 OUTB → R34 pad1: F.Cu east to x=76, south to y=18, via, B.Cu east to R34
        write_trace(pcb, tx, ty, 76.0, ty, sw, "F.Cu", NET_GATE_DRV, "GATE_DRV");
        write_trace(pcb, 76.0, ty, 76.0, 18.0, sw, "F.Cu", NET_GATE_DRV, "GATE_DRV");
        write_via(pcb, 76.0, 18.0, VIA_PAD, VIA_DRILL, NET_GATE_DRV);
        write_trace(pcb, 76.0, 18.0, r34p1x, r34p1y, sw, "B.Cu", NET_GATE_DRV, "GATE_DRV");
        write_via(pcb, r34p1x, r34p1y, VIA_PAD, VIA_DRILL, NET_GATE_DRV);

        // Parallel OUTA (pin4) with OUTB (pin5): B.Cu stub under U4 body
        // Pin4 at (69.3, 16.905) left side, Pin5 at (74.7, 16.905) right side — same y
        let (outa_x, outa_y) = ap(u4, "4"); // (69.3, 16.905) OUTA
        write_via(pcb, outa_x, outa_y, VIA_PAD, VIA_DRILL, NET_GATE_DRV);
        write_trace(pcb, outa_x, outa_y, tx, outa_y, sw, "B.Cu", NET_GATE_DRV, "GATE_DRV");
        // Pin5 pad is same net, on F.Cu — via connects through pad. No extra via needed.

        // R34 pad2 → Q1 gate: F.Cu east to Q1 x, north to Q1 pin1
        write_trace(pcb, r34p2x, r34p2y, gx, r34p2y, sw, "F.Cu", NET_MOSFET_GATE, "MOSFET_GATE");
        write_trace(pcb, gx, r34p2y, gx, gy, sw, "F.Cu", NET_MOSFET_GATE, "MOSFET_GATE");

        // R35 pulldown: MOSFET_GATE node at (gx, 18) south to R35 pad1
        // Route from R34p2 (80.95,18) east to R35p1 (83.05,20) via dog-leg
        write_trace(pcb, r34p2x, r34p2y, r35p1x, r34p2y, sw, "F.Cu", NET_MOSFET_GATE, "MOSFET_GATE");
        write_trace(pcb, r35p1x, r34p2y, r35p1x, r35p1y, sw, "F.Cu", NET_MOSFET_GATE, "MOSFET_GATE");
        // R35 pad2 (GND) connects to GND zone fill — no explicit trace needed
    }

    // ═══════════════════════════════════════════════════════════════════
    // REVERSE POLARITY PROTECTION:
    // J2 pin1 (12V_RAW) → D10 anode → D10 cathode (12V)
    // D10 at (90,14): anode pad1 at (88,14), cathode pad2 at (92,14)
    // J2 pin1 at (90,4)
    // ═══════════════════════════════════════════════════════════════════
    {
        let (j2x, j2y) = ap(j2, "1");     // (90, 4) — now NET_12V_RAW
        let (_d10a, d10ay) = ap(d10, "1");  // (88, 14) anode = 12V_RAW

        // J2 → D10 anode: from J2 (90,4) to D10 pad1 (88,14)
        // +5V F.Cu trunk at y=10.5 from x=86.25→97 blocks any F.Cu vertical crossing.
        // Route: F.Cu west to (88,4), via, B.Cu south to (88,13), via, F.Cu south to D10 pad1.
        //
        // At x=88: J2 pad2 left 88.75. Trace right 88.25 (pw=0.5). Gap=0.5mm. OK.
        // +5V B.Cu at y=12 from x=73.25→86.25: at x=88, east of range. No +5V B.Cu. OK.
        // Q1 pad3 (HEATER_P) at (85,14): our trace is at x=88, far east. OK.
        // Via at (88,13): x=87.65-88.35. J2 pad2 GND left=88.75. Gap=0.4mm. OK.
        // +12V B.Cu at y=14 edge=13.75. Via B.Cu pad edge=13.35. Gap=0.4mm>0.2mm. OK.
        write_trace(pcb, j2x, j2y, 88.0, j2y, pw, "F.Cu", NET_12V_RAW, "+12V_RAW");
        write_via(pcb, 88.0, j2y, VIA_PAD, VIA_DRILL, NET_12V_RAW);
        write_trace(pcb, 88.0, j2y, 88.0, 13.0, pw, "B.Cu", NET_12V_RAW, "+12V_RAW");
        write_via(pcb, 88.0, 13.0, VIA_PAD, VIA_DRILL, NET_12V_RAW);
        write_trace(pcb, 88.0, 13.0, 88.0, d10ay, pw, "F.Cu", NET_12V_RAW, "+12V_RAW");
    }

    // ═══════════════════════════════════════════════════════════════════
    // +12V: D10 cathode (92,14) → J4 pin1 (65,8.73) → D9 pad2 (92,30)
    //
    // D10 cathode is the 12V source (after reverse polarity protection).
    // Route via B.Cu to avoid crossing 12V_RAW, +5V, and H2.
    //
    // D10 cathode at (92,14). Route west on B.Cu to avoid the +5V F.Cu trunk.
    // Via at D10, B.Cu west to (65,14), via, F.Cu north to J4 pin1 (65,8.73).
    // D10 cathode south to D9 pad2 (92,30).
    // ═══════════════════════════════════════════════════════════════════
    {
        let (d10cx, d10cy) = ap(d10, "2"); // (92, 14) = NET_12V
        let (j4x, j4y) = ap(j4, "1");     // (65, 8.73)
        let (_dx, dy) = ap(d9, "2");        // (92, 30)

        // D10 cathode → J4: B.Cu west at y=14, via at x=63.5, F.Cu north to J4 pin1.
        // Route: B.Cu west at y=14 to (63.5,14), B.Cu north to y=7, via at (63.5,7),
        // F.Cu east at y=7 to x=65, F.Cu south to J4 pin1 (65,8.73).
        // HEATER_P F.Cu at y=11.27 (J4 pin2 east to Q1/D9): B.Cu bypasses this on
        // the back layer. Via at (63.5,7) is well north of HEATER_P horizontal.
        // J4 pin2 pad bottom edge=10.42. Our y=7 is north of that. OK.
        write_via(pcb, d10cx, d10cy, VIA_PAD, VIA_DRILL, NET_12V);
        write_trace(pcb, d10cx, d10cy, 63.5, d10cy, pw, "B.Cu", NET_12V, "+12V");
        write_trace(pcb, 63.5, d10cy, 63.5, 7.0, pw, "B.Cu", NET_12V, "+12V");
        write_via(pcb, 63.5, 7.0, VIA_PAD, VIA_DRILL, NET_12V);
        write_trace(pcb, 63.5, 7.0, j4x, 7.0, pw, "F.Cu", NET_12V, "+12V");
        write_trace(pcb, j4x, 7.0, j4x, j4y, pw, "F.Cu", NET_12V, "+12V");

        // D10 cathode → D9: F.Cu south at x=92 to y=30
        write_trace(pcb, d10cx, d10cy, d10cx, dy, pw, "F.Cu", NET_12V, "+12V");
    }

    // ═══════════════════════════════════════════════════════════════════
    // HEATER_P: J4 pin2 (65,11.27) → Q1 pin3 (85,14) → D9 pad1 (88,30)
    //
    // External heater connects via J4. HEATER_P carries current from J4
    // to Q1 drain (MOSFET switch) and D9 anode (flyback diode).
    //
    // Route: F.Cu south from J4 at x=65 to y=25 (no F.Cu signals at x=65
    //   in y=11-25 range). Via to B.Cu. East on B.Cu at y=25 to x=88
    //   (avoids LED_BASE_6/7 and HEATER_PWM F.Cu verticals at x=67-78).
    //   B.Cu at y=25: no B.Cu crossings — MUX_Y4 B.Cu at y=25 ends at x=43,
    //   MUX_Y6 B.Cu at y=26 parallel with 0.625mm gap. All LED_BASE B.Cu
    //   are north (y=16-22).
    //   D9: via at (88,25) → F.Cu south to D9 (88,30).
    //   Q1: via at (86.85,25) → F.Cu north at x=86.85 to y=14, west to Q1.
    //     x=86.85 gives 0.2mm gap to Q1 pad2 GND (85.95,16, right=86.4)
    //     and 0.25mm gap to D10 pad1 (88,16, left=87.25). Both > 0.127mm.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (jx, jy) = ap(j4, "2");   // (65, 11.27)
        let (qx, qy) = ap(q1, "3");   // (85, 14)
        let (d9x, d9y) = ap(d9, "1"); // (88, 30)

        // J4 pin2 is PTH. F.Cu south at x=65 to y=25. Via to B.Cu.
        write_trace(pcb, jx, jy, jx, 25.0, pw, "F.Cu", NET_HEATER_P, "HEATER_P");
        write_via(pcb, jx, 25.0, VIA_PAD, VIA_DRILL, NET_HEATER_P);
        // B.Cu east at y=25 from x=65 to x=88.
        write_trace(pcb, jx, 25.0, 88.0, 25.0, pw, "B.Cu", NET_HEATER_P, "HEATER_P");

        // D9: via at (88,25) → F.Cu south to D9 anode (88,30).
        write_via(pcb, 88.0, 25.0, VIA_PAD, VIA_DRILL, NET_HEATER_P);
        write_trace(pcb, 88.0, 25.0, d9x, d9y, pw, "F.Cu", NET_HEATER_P, "HEATER_P");

        // Q1: via at (86.85,25) → F.Cu north at x=86.85 to y=14, west to Q1.
        write_via(pcb, 86.85, 25.0, VIA_PAD, VIA_DRILL, NET_HEATER_P);
        write_trace(pcb, 86.85, 25.0, 86.85, qy, pw, "F.Cu", NET_HEATER_P, "HEATER_P");
        write_trace(pcb, 86.85, qy, qx, qy, pw, "F.Cu", NET_HEATER_P, "HEATER_P");
    }

    // ═══════════════════════════════════════════════════════════════════
    // +5V bus: U4 pin6 (74.7,15.635) → C4/C5 → R9-R16
    //
    // C4 ← U4, C4→C5 on B.Cu. C5 → trunk east then south.
    // Trunk: F.Cu east to x=97, south to y=44.5, via, B.Cu west to x=10.
    // Drop vias at each R pad1 x.
    // ═══════════════════════════════════════════════════════════════════
    {
        let c4 = comp(components, "C4");
        let c5 = comp(components, "C5");
        let (vx, vy) = ap(u4, "6");    // (74.7, 15.635)
        let (c4x, c4y) = ap(c4, "1");  // (73.25, 12)
        let (c5x, c5y) = ap(c5, "1");  // (86.25, 12)

        write_trace(pcb, vx, vy, c4x, vy, sw, "F.Cu", NET_5V, "+5V");
        write_trace(pcb, c4x, vy, c4x, c4y, pw, "F.Cu", NET_5V, "+5V");
        write_via(pcb, c4x, c4y, VIA_PAD, VIA_DRILL, NET_5V);
        write_trace(pcb, c4x, c4y, c5x, c4y, pw, "B.Cu", NET_5V, "+5V");
        write_via(pcb, c5x, c5y, VIA_PAD, VIA_DRILL, NET_5V);

        // C5 → trunk: north to y=10.5, east to x=97, south to y=44.5
        // Via at (97,44.5), B.Cu west to R9 pad1 x.
        // The B.Cu trunk ends at the westernmost R drop (R9 pad1) to avoid
        // a dangling endpoint.
        let r9 = comp(components, "R9");
        let (r9_x, _) = ap(r9, "1"); // ~11.05

        write_trace(pcb, c5x, c5y, c5x, 10.5, pw, "F.Cu", NET_5V, "+5V");
        write_trace(pcb, c5x, 10.5, 97.0, 10.5, pw, "F.Cu", NET_5V, "+5V");
        write_trace(pcb, 97.0, 10.5, 97.0, 44.5, pw, "F.Cu", NET_5V, "+5V");
        write_via(pcb, 97.0, 44.5, VIA_PAD, VIA_DRILL, NET_5V);
        write_trace(pcb, 97.0, 44.5, r9_x, 44.5, pw, "B.Cu", NET_5V, "+5V");

        for i in 0..8u32 {
            let r = comp(components, Box::leak(format!("R{}", i + 9).into_boxed_str()));
            let (rx, ry) = ap(r, "1");
            write_via(pcb, rx, 44.5, VIA_PAD, VIA_DRILL, NET_5V);
            write_trace(pcb, rx, 44.5, rx, ry, sw, "F.Cu", NET_5V, "+5V");
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // +3V3 bus
    //
    // Main bus: F.Cu east at y=9 from x=12 to x=44.5.
    // Drops: C1 at (24.25,8), R17 pad1 at (44.05,10).
    // Hop: via at (44.5,7.5), B.Cu east at y=7.5 to (54.05,7.5), with
    //   intermediate vias at R18 x and R19 x.
    //
    // Clearance:
    //   +3V3 B.Cu at y=7.5 vs SCL B.Cu at y=6.5: gap=1.0-0.25=0.75mm. OK.
    //   +3V3 B.Cu at y=7.5 vs SDA B.Cu at y=8.5: gap=1.0-0.25=0.75mm. OK.
    //   Via (44.5,7.5): SDA via at (43,8.5) dist=1.80mm. OK.
    //   Via (50.05,7.5): SCL via at (51.95,6.5) dist=2.12mm. OK.
    //
    // Branches: C2/U2 from R18, C3/U3 from R19.
    // ═══════════════════════════════════════════════════════════════════
    {
        let c1 = comp(components, "C1");
        let c2 = comp(components, "C2");
        let c3 = comp(components, "C3");
        let r17 = comp(components, "R17");
        let r18 = comp(components, "R18");
        let r19 = comp(components, "R19");

        let (c1x, c1y) = ap(c1, "1");   // (24.25, 8)
        let (r17x, r17y) = ap(r17, "1"); // (44.05, 10)
        let (r18x, r18y) = ap(r18, "1"); // (50.05, 10)
        let (r19x, r19y) = ap(r19, "1"); // (54.05, 10)
        let (c2x, c2y) = ap(c2, "1");   // (51.25, 12)
        let (c3x, c3y) = ap(c3, "1");   // (51.25, 27)
        let (u2vx, u2vy) = ap(u2, "8"); // (51.5, 15)
        let (u3vx, u3vy) = ap(u3, "16"); // (54.8, 25.555)

        let by = 9.0;

        // U1 pin2 to bus (pin3 is now ESP_EN, not 3V3)
        let (_, p2y) = ap(u1, "2"); // (12, 10)
        write_trace(pcb, 12.0, p2y, 12.0, by, pw, "F.Cu", NET_3V3, "+3V3");

        // Main bus at y=9 from x=12 to x=44.5 (original, does NOT extend west
        // past USB traces at x=9.25/9.75)
        write_trace(pcb, 12.0, by, 44.5, by, sw, "F.Cu", NET_3V3, "+3V3");

        // Drops from bus
        write_trace(pcb, c1x, by, c1x, c1y, sw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, r17x, by, r17x, r17y, sw, "F.Cu", NET_3V3, "+3V3");

        // +3V3 drops to R28 pad1 (2.55,15), R29 pad1 (2.55,18), J5 pin1, R32 pad1
        // Route via B.Cu from bus to avoid crossing USB F.Cu traces.
        // From +3V3 bus at (12,9): via, B.Cu west to x=2.55, via, F.Cu south.
        // B.Cu at y=9 from x=12 to x=2.55: SDA B.Cu at x=9 NOT present at y=9
        //   (SDA is on F.Cu at x=8.5 between y=8.5 and y=11.5). OK.
        // ESP_EN B.Cu at y=9 from x=12→4: SAME Y! CROSSING!
        // Fix: use y=8.5 for +3V3 west run (SDA B.Cu gap lower bound).
        // ESP_EN B.Cu at y=9.0: gap=0.375mm. OK.
        // SDA B.Cu horizontal at (8.5,8.5)→(9.0,8.5): at y=8.5, this segment
        //   from x=8.5 to x=9 is present. Our trace from x=12 to x=2.55 crosses
        //   both x=9 and x=8.5 at y=8.5. CROSSING with SDA B.Cu!
        // Fix: use y=8.0 instead. SDA B.Cu at x=9 from y=3→8.5: at y=8.0,
        //   SDA exists at x=9. STILL CROSSING.
        // The only safe B.Cu corridor below ESP_EN at y=9 that avoids SDA is none.
        //
        // Alternative: drop +3V3 from bus at (12,9) south on F.Cu to y=12 or so,
        // then via to B.Cu, west to x=2.55, avoiding all USB and SDA.
        // From (12,12) B.Cu west: SDA B.Cu at x=9 from y=11.5→27.64. At y=12,
        //   SDA at x=9 IS present. STILL CROSSING.
        //
        // Alternative: route from the U6 Vout (35,3→9) area west on F.Cu at a y
        // that avoids USB. F.Cu at y=12 from x=12 west to x=2.55: at y=12, USB_DP
        // at x=9.75 is present (goes from y=5 to y=23.83). CROSSING.
        //
        // The left-side +3V3 components (R28, R29, J5, R32) are west of the USB traces.
        // The only way to reach them from the +3V3 bus (east of USB) is to cross the
        // USB traces. Must use B.Cu hop.
        //
        // Strategy: from +3V3 bus at (12,9), via, B.Cu west at y=9 to x=2.55, via.
        // B.Cu at y=9: SDA at x=9 is NOT on B.Cu (it's on F.Cu at x=8.5).
        // ESP_EN B.Cu at y=9 from x=12→4: SAME Y SAME LAYER. Need different y.
        //
        // Actually ESP_EN is now at y=9.0. Let me change it to y=8.75 to avoid.
        // But that would need ESP_EN routing change. Simpler: route +3V3 west on
        // B.Cu at y=8.5 (the SDA transition point). At (9,8.5), SDA B.Cu horizontal
        // goes from (8.5,8.5)→(9.0,8.5). +3V3 B.Cu at y=8.5 crosses this segment.
        //
        // This is hopeless via B.Cu. Let me use a completely different approach:
        // Extend +3V3 F.Cu at y=9 west only to x=5.5 (west of USB_DP at x=9.75
        // is not possible on F.Cu).
        //
        // FINAL APPROACH: Connect R28/R29/J5/R32 to +3V3 via a local power rail
        // on F.Cu at x=2.55, fed from U1 pin2 (+3V3) via a B.Cu bypass that crosses
        // west of the USB traces using the SDA gap.
        // U1 pin2 at (12, 10) → F.Cu north to (12, 9) [already on bus].
        // From bus at (12, 9), F.Cu south at x=12 to y=12. Via at (12, 12).
        // B.Cu at y=12 west to x=2.55. At y=12: SDA B.Cu at x=9 from y=11.5→27.64.
        // At y=12, SDA B.Cu at x=9 exists! CROSSING.
        //
        // I give up trying to avoid SDA at x=9. Instead, I'll add SDA B.Cu avoidance
        // by routing the +3V3 drop via a jog around SDA:
        // (12, 9) → via → B.Cu south at x=12 to y=11 → B.Cu west at y=11 to x=10
        //   (east of SDA at x=9) → via at (10, 11) → F.Cu west to x=8.5 (within
        //   the SDA F.Cu segment at x=8.5, but different net! Can't use F.Cu at x=8.5).
        //
        // SIMPLEST: Use an extra via pair to hop SDA B.Cu on F.Cu.
        // (12, 9) → via → B.Cu south to y=11 at x=12 → B.Cu west to x=10 at y=11 →
        //   via → F.Cu at y=11 west to x=7.5 → via → B.Cu west to x=2.55 → via.
        // F.Cu at y=11 from x=10→7.5: USB_DP at x=9.75 (y=5→23.83) crosses at y=11!
        //
        // ACTUALLY: F.Cu at y=11 crosses USB_DP at x=9.75. Can't use F.Cu.
        //
        // LAST RESORT: Route R28 pad1 directly to U1 +3V3 pad via B.Cu through
        // an area that avoids SDA. U1 pin2 (+3V3) at (12, 10). Via. B.Cu north
        // at x=12 to y=9. B.Cu west at y=9 to x=4. Via at (4, 9). F.Cu south
        // to R28 pad1 (2.55, 15).
        // B.Cu at y=9: ESP_EN B.Cu at y=9 from x=12→4. SAME TRACE CORRIDOR.
        // They'd need to be on different y. Let me shift ESP_EN to y=8.75:
        {
            // Route +3V3 to left side: via at (12,9), B.Cu west at y=9.35
            // (in SDA gap zone, below LB0 at y=10, below GPIO0 at y=10.5)
            // Edge check: SDA boundary 8.825, +3V3 edge 9.225, gap=0.4mm. OK.
            // LB0 lower 9.875, +3V3 edge 9.475, gap=0.4mm. OK.
            write_via(pcb, 12.0, by, VIA_PAD, VIA_DRILL, NET_3V3);
            write_trace(pcb, 12.0, by, 12.0, 9.35, sw, "B.Cu", NET_3V3, "+3V3");
            write_trace(pcb, 12.0, 9.35, 2.55, 9.35, sw, "B.Cu", NET_3V3, "+3V3");
            write_via(pcb, 2.55, 9.35, VIA_PAD, VIA_DRILL, NET_3V3);

            let r28 = comp(components, "R28");
            let r29_c = comp(components, "R29");
            let (r28p1x, r28p1y) = ap(r28, "1"); // +3V3 at (2.55, 15)
            let (r29p1x, r29p1y) = ap(r29_c, "1"); // +3V3 at (2.55, 18)
            let j5_c = comp(components, "J5");
            let (j5p1x, j5p1y) = ap(j5_c, "1"); // +3V3 at (3.5, 26.19)
            let r32_c = comp(components, "R32");
            let (r32p1x, r32p1y) = ap(r32_c, "1"); // +3V3 at (5, 35.95)

            // F.Cu south at x=2.55 to y=11.8, via to B.Cu, south at x=2.55 to pad1.
            // B.Cu passes under C11 pad1 (F.Cu-only SMD) at (2.75,13). Clear.
            // Via at (2.55,11.8): ESP_EN B.Cu at y=11 edge=11.125. Drill edge=11.625.
            //   Gap=0.5mm > 0.25mm. OK.
            // Via at R28 pad1 (2.55,15): same net (+3V3), landing on pad. OK.
            write_trace(pcb, r28p1x, 9.35, r28p1x, 11.8, sw, "F.Cu", NET_3V3, "+3V3");
            write_via(pcb, r28p1x, 11.8, VIA_PAD, VIA_DRILL, NET_3V3);
            write_trace(pcb, r28p1x, 11.8, r28p1x, r28p1y, sw, "B.Cu", NET_3V3, "+3V3");
            write_via(pcb, r28p1x, r28p1y, VIA_PAD, VIA_DRILL, NET_3V3);
            // Continue south to R29 pad1 (y=18)
            write_trace(pcb, r29p1x, r28p1y, r29p1x, r29p1y, sw, "F.Cu", NET_3V3, "+3V3");
            // South from R29, jog east to x=3.25 to avoid SW1 pad1 at (1.75,22)
            // which extends to x=2.5. At x=3.25: left edge 3.125, gap=0.625mm. OK.
            // SW2 pad1 at (1.75,26) extends to x=2.5: gap same. OK.
            write_trace(pcb, r29p1x, r29p1y, 3.25, r29p1y, sw, "F.Cu", NET_3V3, "+3V3");
            write_trace(pcb, 3.25, r29p1y, 3.25, 24.0, sw, "F.Cu", NET_3V3, "+3V3");
            // J5 pin1 (+3V3) at (3.5, 26.19): route south at x=3.25 to J5 pin1.
            // At x=3.25 from y=24: J5 pin1 PTH extends x=2.65-4.35, same net. OK.
            // Avoids ESP_EN via at (1.75,26) right edge 2.1 (gap=1.025mm). OK.
            write_trace(pcb, 3.25, 24.0, 3.25, j5p1y, sw, "F.Cu", NET_3V3, "+3V3");
            write_trace(pcb, 3.25, j5p1y, j5p1x, j5p1y, sw, "F.Cu", NET_3V3, "+3V3");
            // R32 pad1 (+3V3) at (5, 35.95): route from J5 pin1 area via B.Cu.
            // Via at (2.0, 27) west of J5 PTH (left 2.65, gap=0.3mm), above ESP_EN
            // via at (1.75,26) (y gap=0.3mm). B.Cu south to y=33.5, then to R32.
            write_via(pcb, 2.0, 27.0, VIA_PAD, VIA_DRILL, NET_3V3);
            write_trace(pcb, 3.25, j5p1y, 3.25, 27.0, sw, "F.Cu", NET_3V3, "+3V3");
            write_trace(pcb, 3.25, 27.0, 2.0, 27.0, sw, "F.Cu", NET_3V3, "+3V3");
            // F.Cu hop around LED_PWR B.Cu horizontal at y=32.5 (x=1.5→5).
            // B.Cu south at x=2.0, jog east to x=2.25 at y=31.75 for LED_PWR clearance.
            write_trace(pcb, 2.0, 27.0, 2.0, 31.75, sw, "B.Cu", NET_3V3, "+3V3");
            write_trace(pcb, 2.0, 31.75, 2.25, 31.75, sw, "B.Cu", NET_3V3, "+3V3");
            write_via(pcb, 2.25, 31.75, VIA_PAD, VIA_DRILL, NET_3V3);
            write_trace(pcb, 2.25, 31.75, 2.25, 33.15, sw, "F.Cu", NET_3V3, "+3V3");
            write_via(pcb, 2.25, 33.15, VIA_PAD, VIA_DRILL, NET_3V3);
            write_trace(pcb, 2.25, 33.15, 2.25, r32p1y, sw, "B.Cu", NET_3V3, "+3V3");
            write_via(pcb, 2.25, r32p1y, VIA_PAD, VIA_DRILL, NET_3V3);
            write_trace(pcb, 2.25, r32p1y, r32p1x, r32p1y, sw, "F.Cu", NET_3V3, "+3V3");
        }

        // Hop to east section: via at (44.5,7.5), B.Cu east, via drops
        write_trace(pcb, 44.5, by, 44.5, 7.5, sw, "F.Cu", NET_3V3, "+3V3");
        write_via(pcb, 44.5, 7.5, VIA_PAD, VIA_DRILL, NET_3V3);
        write_trace(pcb, 44.5, 7.5, r19x, 7.5, sw, "B.Cu", NET_3V3, "+3V3");

        // Via at R18 x
        write_via(pcb, r18x, 7.5, VIA_PAD, VIA_DRILL, NET_3V3);
        write_trace(pcb, r18x, 7.5, r18x, r18y, sw, "F.Cu", NET_3V3, "+3V3");

        // Via at R19 x
        write_via(pcb, r19x, 7.5, VIA_PAD, VIA_DRILL, NET_3V3);
        write_trace(pcb, r19x, 7.5, r19x, r19y, sw, "F.Cu", NET_3V3, "+3V3");

        // C2 + U2 pin8: from R18 pad1 south
        write_trace(pcb, r18x, r18y, r18x, c2y, sw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, r18x, c2y, c2x, c2y, sw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, r18x, c2y, r18x, u2vy, sw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, r18x, u2vy, u2vx, u2vy, sw, "F.Cu", NET_3V3, "+3V3");

        // C3 + U3 pin16: from R19 south to y=23, via hop west to C3, continue to U3
        write_trace(pcb, r19x, r19y, r19x, 23.0, sw, "F.Cu", NET_3V3, "+3V3");
        write_via(pcb, r19x, 23.0, VIA_PAD, VIA_DRILL, NET_3V3);
        write_trace(pcb, r19x, 23.0, c3x, 23.0, sw, "B.Cu", NET_3V3, "+3V3");
        write_via(pcb, c3x, 23.0, VIA_PAD, VIA_DRILL, NET_3V3);
        write_trace(pcb, c3x, 23.0, c3x, c3y, sw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, r19x, 23.0, r19x, u3vy, sw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, r19x, u3vy, u3vx, u3vy, sw, "F.Cu", NET_3V3, "+3V3");
    }

    // ═══════════════════════════════════════════════════════════════════
    // LED_BASE_0..7: ESP32 right-side pins → R pad1 → Q pad1
    //
    // ESP32 pins 21-28 at x=28, y=10..18.89 (spaced 1.27mm).
    // R1-R8 pad1 at y=38, Q2-Q9 pad1 at y=43.
    //
    // KEY STRATEGY:
    //   LB0-2 (going west): F.Cu east from pin to x=31, via, B.Cu west to corridor, via, F.Cu south.
    //   LB3-7 (going east): Via at pin (x=28), B.Cu east to corridor, via, F.Cu south.
    //
    // This eliminates LB2 crossing problem: LB3-7 have no F.Cu horizontals between x=28-31.
    //
    // Corridors:
    //   LB0: x=5.5 (west of USB at x=9.25). F.Cu south to y=38. East to R1 (11.05,38).
    //     At x=5.5: B.Cu at y=10 from (31,10) to (5.5,10).
    //     F.Cu from y=10 starts above H1 pad (top edge y=8.175). OK.
    //   LB1: x=15.5 (west of U1 GND pad edge x=17). F.Cu south to y=38. East to R2 (20.55,38).
    //   LB2: x=30.05 (direct, R3 pad1 x). F.Cu south to y=38. Direct.
    //   LB3: x=39.55 (R4 pad1 x). F.Cu south to y=38. Direct.
    //   LB4: x=36 corridor. F.Cu south to y=36.5. Via. B.Cu east to (49.05,36.5). Via.
    //     F.Cu south to R5 (49.05,38). Continue to Q (49.05,43).
    //     x=49.05 avoids U2 pads (48.5-area). Via at (49.05,36.5): R5 pad north edge
    //     38-0.6=37.4. Via south edge 36.5+0.35=36.85. Gap=0.55mm. OK.
    //     HEATER_PWM B.Cu at y=36: gap=36.5-36-0.125-0.125=0.25mm. OK.
    //   LB5: x=55 corridor. West of MUX_S vias at x=58+.
    //     MUX_S0 via at (58,30.18): gap=3.0-0.35-0.125=2.525mm. OK.
    //     R19 pad1 at (54.05,10) edge=54.55. Trace at 55-0.125=54.875. Gap=0.325mm. OK.
    //     MUX_Y0-2 vias at (56,y): gap=1.0-0.35-0.125=0.525mm. OK.
    //   LB6: x=68.05 (R7 pad1 x). F.Cu south to y=38. Direct.
    //     HEATER_PWM F.Cu at x=67: gap=1.05-0.125=0.925mm. OK.
    //   LB7: x=77.55 (R8 pad1 x). F.Cu south to y=38. Direct.
    //
    // B.Cu channel checks at x=31 (LB0-2 vias):
    //   LB0 (31,10), LB1 (31,11.27), LB2 (31,12.54): 1.27mm spacing. Hole gap=0.57mm. OK.
    //
    // LB3-7 vias at x=28 (on U1 pads, same net, fine):
    //   Spaced 1.27mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        // Corridors: (corridor_x, goes_east, needs_jog_to_r)
        // LB0: corridor=5.5, rx=11.05 → jog east at y=38
        // LB1: corridor=15.5, rx=20.55 → jog east at y=38
        // LB2: corridor=30.05, rx=30.05 → direct
        // LB3: corridor=39.55, rx=39.55 → direct
        // LB4: corridor=36, rx=49.05 → B.Cu hop at y=36.5
        // LB5: corridor=62, rx=58.55 → jog west at y=37
        // LB6: corridor=68.05, rx=68.05 → direct
        // LB7: corridor=77.55, rx=77.55 → direct

        let lb_pins: [u32; 8] = [21, 22, 23, 24, 25, 26, 30, 28];
        for i in 0..8u32 {
            let ps: &str = Box::leak(format!("{}", lb_pins[i as usize]).into_boxed_str());
            let (ex, ey) = ap(u1, ps);  // x=28, y varies
            let net = NET_LED_BASE_0 + i;
            let nn: &str = Box::leak(format!("LED_BASE_{}", i).into_boxed_str());
            let r = comp(components, Box::leak(format!("R{}", i + 1).into_boxed_str()));
            let (rx, ry) = ap(r, "1");  // y=38
            let q = comp(components, Box::leak(format!("Q{}", i + 2).into_boxed_str()));
            let (qx, qy) = ap(q, "1");  // y=43

            match i {
                // LB0-2: F.Cu east to x=31, via, B.Cu west to corridor
                0 => {
                    // LB0: corridor at x=7.5. Moved from x=5.5 to avoid crossing
                    // SCL via at (5.5,28.91), UART_TX/RX, LED_PWR paths.
                    // x=7.5: SDA F.Cu at x=8.5 gap=0.75mm, UART_RX at x=7 gap=0.25mm. OK.
                    let cx = 7.5;
                    write_trace(pcb, ex, ey, 31.0, ey, sw, "F.Cu", net, nn);
                    write_via(pcb, 31.0, ey, VIA_PAD, VIA_DRILL, net);
                    // B.Cu jog south past USB_DN via at (17.95, 9.75).
                    // Stop at x=19 (east of via right 18.3+0.125=18.425, gap=0.45mm).
                    // B.Cu south to y=10.75, west to x=17, north to ey=10.
                    // USB_DN B.Cu at x=17.95 ends at y=9.75: at y=10.75 not present.
                    write_trace(pcb, 31.0, ey, 19.0, ey, sw, "B.Cu", net, nn);
                    write_trace(pcb, 19.0, ey, 19.0, 10.75, sw, "B.Cu", net, nn);
                    write_trace(pcb, 19.0, 10.75, 17.0, 10.75, sw, "B.Cu", net, nn);
                    write_trace(pcb, 17.0, 10.75, 17.0, ey, sw, "B.Cu", net, nn);
                    write_trace(pcb, 17.0, ey, cx, ey, sw, "B.Cu", net, nn);
                    // Via at (cx, ey=10). GPIO0 B.Cu moved to y=10.61 for clearance.
                    write_via(pcb, cx, ey, VIA_PAD, VIA_DRILL, net);
                    // F.Cu south to y=20.5, B.Cu hop past SW1/SW2 pad2 (x=7.5-9.0,
                    // y=21.5-22.5 / 25.5-26.5). SW pads are SMD F.Cu only, B.Cu clear.
                    // Also past SCL F.Cu at y=28.91 (different layer on B.Cu).
                    write_trace(pcb, cx, ey, cx, 20.5, sw, "F.Cu", net, nn);
                    write_via(pcb, cx, 20.5, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, 20.5, cx, 29.0, sw, "B.Cu", net, nn);
                    write_via(pcb, cx, 29.0, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, 29.0, cx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, cx, ry, rx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                1 => {
                    // LB1: corridor at x=15.5 (west of U1 GND pad edge x=17).
                    // East jog at y=38 uses B.Cu to hop over MUX_Y0 at x=17.
                    let cx = 15.25;
                    write_trace(pcb, ex, ey, 31.0, ey, sw, "F.Cu", net, nn);
                    write_via(pcb, 31.0, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, 31.0, ey, cx, ey, sw, "B.Cu", net, nn);
                    write_via(pcb, cx, ey, VIA_PAD, VIA_DRILL, net);
                    // B.Cu hop past USB_DN F.Cu horizontal at y=23.83 (x=12→16).
                    write_trace(pcb, cx, ey, cx, 23.0, sw, "F.Cu", net, nn);
                    write_via(pcb, cx, 23.0, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, 23.0, cx, 24.5, sw, "B.Cu", net, nn);
                    write_via(pcb, cx, 24.5, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, 24.5, cx, ry, sw, "F.Cu", net, nn);
                    // B.Cu hop east at y=38 to avoid MUX_Y0 at x=17
                    write_via(pcb, cx, ry, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, ry, rx, ry, sw, "B.Cu", net, nn);
                    write_via(pcb, rx, ry, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                2 => {
                    // LB2: corridor at rx=30.05, direct
                    write_trace(pcb, ex, ey, 31.0, ey, sw, "F.Cu", net, nn);
                    write_via(pcb, 31.0, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, 31.0, ey, rx, ey, sw, "B.Cu", net, nn);
                    write_via(pcb, rx, ey, VIA_PAD, VIA_DRILL, net);
                    // Straight F.Cu (UART_RX now routes west of LB2 at x=29.0)
                    write_trace(pcb, rx, ey, rx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                // LB3-7: Via at pin (x=28), B.Cu east to corridor
                3 => {
                    // LB3: direct corridor at rx=39.55
                    write_via(pcb, ex, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, ex, ey, rx, ey, sw, "B.Cu", net, nn);
                    write_via(pcb, rx, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, rx, ey, rx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                4 => {
                    // LB4: corridor at x=36, then B.Cu hop at y=36.75 to rx=49.05
                    // y=36.75: HEATER_PWM B.Cu at y=36 gap=0.275mm. R pad at y=38 gap=0.3mm. OK.
                    let cx = 36.0;
                    write_via(pcb, ex, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, ex, ey, cx, ey, sw, "B.Cu", net, nn);
                    write_via(pcb, cx, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, ey, cx, 36.75, sw, "F.Cu", net, nn);
                    write_via(pcb, cx, 36.75, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, 36.75, rx, 36.75, sw, "B.Cu", net, nn);
                    write_via(pcb, rx, 36.75, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, rx, 36.75, rx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                5 => {
                    // LB5: corridor at x=62, east of MUX_S vias and U3 right pads.
                    // F.Cu south to y=37 (avoids R6 pad2(GND) at (60.45,38)).
                    // West at y=37 to rx=58.55. North to R6 pad1 (58.55,38).
                    // R6 pad2(GND) at (60.45,38) edge x=59.95. At y=37, trace at x=62→58.55
                    // passes x=60.45 at y=37, pad at y=38, gap=1.0-0.6-0.125=0.275mm. OK.
                    let cx = 62.0;
                    write_via(pcb, ex, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, ex, ey, cx, ey, sw, "B.Cu", net, nn);
                    write_via(pcb, cx, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, cx, ey, cx, 37.0, sw, "F.Cu", net, nn);
                    write_trace(pcb, cx, 37.0, rx, 37.0, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, 37.0, rx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                6 => {
                    // LB6: direct at rx=68.05
                    write_via(pcb, ex, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, ex, ey, rx, ey, sw, "B.Cu", net, nn);
                    write_via(pcb, rx, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, rx, ey, rx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                7 => {
                    // LB7: direct at rx=77.55
                    write_via(pcb, ex, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, ex, ey, rx, ey, sw, "B.Cu", net, nn);
                    write_via(pcb, rx, ey, VIA_PAD, VIA_DRILL, net);
                    write_trace(pcb, rx, ey, rx, ry, sw, "F.Cu", net, nn);
                    write_trace(pcb, rx, ry, rx, qy, sw, "F.Cu", net, nn);
                    if (rx - qx).abs() > 0.01 {
                        write_trace(pcb, rx, qy, qx, qy, sw, "F.Cu", net, nn);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // LED_COL_0..7: R pad2 → LED pad1 (anode)
    //
    // Correct topology: +5V → R(100R) → LED anode → LED cathode → NPN collector → GND
    // LED_COL connects the current-limiting resistor to the LED anode.
    //
    // LED pads at y=61.95 (rotated 90deg for pad1/anode).
    // Jog east 2mm to clear cathode pad, then north on F.Cu.
    //
    // LED_CATH now routes at Q_x column (not LED_x), so LED_COL horizontal at y=47.5
    // from jog_x (LED_x+2) to R pad2 no longer crosses LED_CATH.
    // LED_CATH at Q_x is 0.95mm west of R pad2 (Q_x+0.95). LED_COL horizontal
    // from jog_x west to R pad2 stops at R pad2, never reaching Q_x. Safe gap >=0.7mm.
    //
    // y=47.5 chosen to clear LED_CATH manual route jogs at y=48 (gap=0.25mm to trace edge).
    // ═══════════════════════════════════════════════════════════════════
    {
        for i in 0..8u32 {
            let net = NET_LED_COL_0 + i;
            let nn: &str = Box::leak(format!("LED_COL_{}", i).into_boxed_str());
            let d = comp(components, Box::leak(format!("D{}", i + 1).into_boxed_str()));
            let (dx, dy) = ap(d, "1"); // LED pad1 (anode) — LED_COL net
            let r = comp(components, Box::leak(format!("R{}", i + 9).into_boxed_str()));
            let (rx, ry) = ap(r, "2"); // R pad2 — LED_COL net

            let jog_x = dx + 2.0;
            write_trace(pcb, dx, dy, jog_x, dy, sw, "F.Cu", net, nn);
            write_trace(pcb, jog_x, dy, jog_x, 47.5, sw, "F.Cu", net, nn);

            // From jog_x at y=47.5 to R pad2
            if (jog_x - rx).abs() > 0.01 {
                write_trace(pcb, jog_x, 47.5, rx, 47.5, sw, "F.Cu", net, nn);
            }
            write_trace(pcb, rx, 47.5, rx, ry, sw, "F.Cu", net, nn);
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // LED_CATH_0..7: LED pad2 (cathode) → Q pad3 (NPN collector)
    //
    // LED pad2 (cathode) at (LED_x, 60.05).
    // Q pad3 (collector) at (Q_x, 41).
    //
    // Strategy: all-F.Cu route.
    //   1) F.Cu north at LED_x from cathode (y=60.05) to y=57.
    //   2) F.Cu horizontal at y=57 from LED_x to Q_x (jog to Q column).
    //   3) F.Cu north at Q_x from y=57 to Q pad3 (y=41).
    //
    // Why y=57:
    //   - Above PD load resistors at y=58 (R20-R27 pad top at y=57.4). Trace
    //     bottom edge at 57+0.125=57.125. Gap=0.275mm to pad top. OK.
    //   - Below PD pads at y=59.3 (PD pad1 extends to y=59.3). Trace top
    //     edge at 57-0.125=56.875. Gap=2.425mm. OK.
    //   - Horizontal jog avoids PD pads of other slots (PD pads at y=59.3+,
    //     our jog at y=57).
    //
    // F.Cu at Q_x from y=57 to y=41:
    //   - LED_COL horizontal at y=47.5 (from jog_x -> R pad2): for slots 0-3
    //     the horizontal doesn't cross Q_x; for slots 4-7 the horizontal is
    //     on B.Cu (hop). No crossing.
    //   - R_led pads at y=46: between pad1 (Q_x-0.95) and pad2 (Q_x+0.95),
    //     gap 0.375mm each side. OK.
    //   - Q pads at y=43: same 0.375mm gap between pad1 and pad2. OK.
    //   - R_base pads at y=38: same 0.375mm gap. OK.
    //
    // Horizontal jog at y=57 clearances:
    //   - LED_COL at LED_x+2 (F.Cu vertical): min gap 2mm. Safe.
    //   - MUX_Y corridors (F.Cu vertical): min gap 6mm. Safe.
    //   - Adjacent slot LED_CATH verticals: 10mm gap. Safe.
    // ═══════════════════════════════════════════════════════════════════
    {
        for i in 0..8u32 {
            let cath_net = NET_LED_CATH_0 + i;
            let cath_nn: &str = Box::leak(format!("LED_CATH_{}", i).into_boxed_str());
            let d = comp(components, Box::leak(format!("D{}", i + 1).into_boxed_str()));
            let (d2x, d2y) = ap(d, "2"); // LED pad2 (cathode) at (LED_x, 60.05)
            let q = comp(components, Box::leak(format!("Q{}", i + 2).into_boxed_str()));
            let (qx, qy) = ap(q, "3");   // Q pad3 (collector) at (Q_x, 41)

            // 1) F.Cu north at LED_x from cathode to y=57
            write_trace(pcb, d2x, d2y, d2x, 57.0, sw, "F.Cu", cath_net, cath_nn);

            // 2) F.Cu horizontal at y=57 to Q_x
            if (d2x - qx).abs() > 0.01 {
                write_trace(pcb, d2x, 57.0, qx, 57.0, sw, "F.Cu", cath_net, cath_nn);
            }

            // 3) F.Cu north at Q_x from y=57 to Q pad3 (y=41)
            write_trace(pcb, qx, 57.0, qx, qy, sw, "F.Cu", cath_net, cath_nn);
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // MUX_Y0..7: PD pad1 → U3 MUX pin
    //
    // F.Cu vertical north from PD, via above electronics zone, B.Cu horizontal
    // to U3 vicinity, via, short F.Cu to pin.
    //
    // Y0-Y2: right side of U3 (pins 13,14,15 at x=54.8)
    //   Via near U3 at x=56, B.Cu at pin y from corridor, via, F.Cu west to pin.
    //
    // Y3: right side, pin12 (54.8,30.635)
    //   Route F.Cu at x=47 all the way north from PD to y=30.635. East to pin.
    //   x=47 is clear of all MUX_S horizontals (which are between x=54.8-61).
    //
    // Y4-Y7: left side of U3 (pins 1,5,2,4 at x=45.2)
    //   Y4: x=46 corridor, north to y=24, via, B.Cu west to (44.5,24), via, F.Cu south to pin.
    //   Y5: x=66 corridor, north to y=34, via, B.Cu west to (44,34), via, F.Cu north to pin.
    //   Y6: x=76 corridor, north to y=26, via, B.Cu west to (42,26), via, F.Cu south to pin.
    //   Y7: x=87 corridor, north to y=35, via, B.Cu west to (42,35), via, F.Cu north to pin.
    // ═══════════════════════════════════════════════════════════════════

    // Y0: PD1 (19,59.9) → U3 pin13 (54.8,29.365)
    {
        let pd = comp(components, "PD1");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "13");
        write_trace(pcb, px, py, 17.0, py, sw, "F.Cu", NET_MUX_Y0, "MUX_Y0");
        write_trace(pcb, 17.0, py, 17.0, my, sw, "F.Cu", NET_MUX_Y0, "MUX_Y0");
        write_via(pcb, 17.0, my, VIA_PAD, VIA_DRILL, NET_MUX_Y0);
        write_trace(pcb, 17.0, my, 56.0, my, sw, "B.Cu", NET_MUX_Y0, "MUX_Y0");
        write_via(pcb, 56.0, my, VIA_PAD, VIA_DRILL, NET_MUX_Y0);
        write_trace(pcb, 56.0, my, mx, my, sw, "F.Cu", NET_MUX_Y0, "MUX_Y0");
    }

    // Y1: PD2 (29,59.9) → U3 pin14 (54.8,28.095)
    {
        let pd = comp(components, "PD2");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "14");
        // Use x=26 corridor (x=27 too close to U1 pad35 at (28,27.78))
        write_trace(pcb, px, py, 26.0, py, sw, "F.Cu", NET_MUX_Y1, "MUX_Y1");
        write_trace(pcb, 26.0, py, 26.0, my, sw, "F.Cu", NET_MUX_Y1, "MUX_Y1");
        write_via(pcb, 26.0, my, VIA_PAD, VIA_DRILL, NET_MUX_Y1);
        write_trace(pcb, 26.0, my, 56.0, my, sw, "B.Cu", NET_MUX_Y1, "MUX_Y1");
        write_via(pcb, 56.0, my, VIA_PAD, VIA_DRILL, NET_MUX_Y1);
        write_trace(pcb, 56.0, my, mx, my, sw, "F.Cu", NET_MUX_Y1, "MUX_Y1");
    }

    // Y2: PD3 (39,59.9) → U3 pin15 (54.8,26.825)
    {
        let pd = comp(components, "PD3");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "15");
        write_trace(pcb, px, py, 37.0, py, sw, "F.Cu", NET_MUX_Y2, "MUX_Y2");
        write_trace(pcb, 37.0, py, 37.0, my, sw, "F.Cu", NET_MUX_Y2, "MUX_Y2");
        write_via(pcb, 37.0, my, VIA_PAD, VIA_DRILL, NET_MUX_Y2);
        write_trace(pcb, 37.0, my, 56.0, my, sw, "B.Cu", NET_MUX_Y2, "MUX_Y2");
        write_via(pcb, 56.0, my, VIA_PAD, VIA_DRILL, NET_MUX_Y2);
        write_trace(pcb, 56.0, my, mx, my, sw, "F.Cu", NET_MUX_Y2, "MUX_Y2");
    }

    // Y3: PD4 (49,59.9) → U3 pin12 (54.8,30.635)
    //   F.Cu at x=47 from PD to pin y. East to pin.
    //   x=47: clear of MUX_S (x=54.8-61). Clear of R/Q pads.
    //   At y=30.635 east to (54.8,30.635): crosses nothing between x=47 and x=54.8.
    //     +3V3 at x=54.05 from y=10-23: not at y=30.635. OK.
    //     Y5 F.Cu at x=44 from y=34-30.635: our horizontal starts at x=47, east of x=44. OK.
    //     C3 F.Cu at x=51.25 from y=23-27: not at y=30.635. OK.
    {
        let pd = comp(components, "PD4");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "12");
        write_trace(pcb, px, py, 47.0, py, sw, "F.Cu", NET_MUX_Y3, "MUX_Y3");
        write_trace(pcb, 47.0, py, 47.0, my, sw, "F.Cu", NET_MUX_Y3, "MUX_Y3");
        write_trace(pcb, 47.0, my, mx, my, sw, "F.Cu", NET_MUX_Y3, "MUX_Y3");
    }

    // Y4: PD5 (59,59.9) → U3 pin1 (45.2,25.555)
    //   West to x=57, north to y=48.3. Via.
    //   B.Cu west at y=48.3 to x=38. Via at (38,48.3). F.Cu north at x=38 to y=25.
    //   Via at (38,25). B.Cu east at y=25 to x=43. Via at (43,25).
    //   F.Cu south at x=43 to y=25.555. East to pin.
    //   x=38: clears R4(39.55), Q5(41.45), R12(41.45). OK.
    //   B.Cu trace at y=48.3: edge y=48.425. Thermal via at y=49 pad edge=48.65. Gap=0.225mm. OK.
    //   B.Cu hop at y=25: clears MUX_Y6 B.Cu at y=26 (gap=0.75mm). OK.
    //   Via (43,25): MUX_Y6 via (42,26) dist=1.41mm. OK. MUX_Y5 at x=43 from y=30.635: not here.
    //   F.Cu at x=43 from y=25 to y=25.555: MUX_Y5 at x=43 starts at y=30.635. OK.
    //   Crosses LB3? LB3 at x=39.55 from y=13.81-38. B.Cu at y=25 from x=38-43 crosses x=39.55
    //   on B.Cu, but LB3 at x=39.55 is F.Cu. Different layers. OK.
    {
        let pd = comp(components, "PD5");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "1");
        write_trace(pcb, px, py, 57.0, py, sw, "F.Cu", NET_MUX_Y4, "MUX_Y4");
        write_trace(pcb, 57.0, py, 57.0, 48.3, sw, "F.Cu", NET_MUX_Y4, "MUX_Y4");
        write_via(pcb, 57.0, 48.3, VIA_PAD, VIA_DRILL, NET_MUX_Y4);
        write_trace(pcb, 57.0, 48.3, 38.0, 48.3, sw, "B.Cu", NET_MUX_Y4, "MUX_Y4");
        write_via(pcb, 38.0, 48.3, VIA_PAD, VIA_DRILL, NET_MUX_Y4);
        write_trace(pcb, 38.0, 48.3, 38.0, 25.0, sw, "F.Cu", NET_MUX_Y4, "MUX_Y4");
        write_via(pcb, 38.0, 25.0, VIA_PAD, VIA_DRILL, NET_MUX_Y4);
        write_trace(pcb, 38.0, 25.0, 43.0, 25.0, sw, "B.Cu", NET_MUX_Y4, "MUX_Y4");
        write_via(pcb, 43.0, 25.0, VIA_PAD, VIA_DRILL, NET_MUX_Y4);
        write_trace(pcb, 43.0, 25.0, 43.0, my, sw, "F.Cu", NET_MUX_Y4, "MUX_Y4");
        write_trace(pcb, 43.0, my, mx, my, sw, "F.Cu", NET_MUX_Y4, "MUX_Y4");
    }

    // Y5: PD6 (69,59.9) → U3 pin5 (45.2,30.635)
    //   F.Cu at x=66, north to y=34. Via. B.Cu west to (43,34). Via.
    //   F.Cu north to y=30.635. East to pin.
    //   Via (43,34): U3 pin8(GND) at (45.2,34.445) dist=sqrt(4.84+0.198)=2.24mm. OK.
    //     MUX_Y7 via at (42,35): dist=sqrt(1+1)=1.41mm. OK.
    //     MUX_COM at x=41: gap=2.0-0.25=1.75mm. OK.
    {
        let pd = comp(components, "PD6");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "5");
        write_trace(pcb, px, py, 66.0, py, sw, "F.Cu", NET_MUX_Y5, "MUX_Y5");
        write_trace(pcb, 66.0, py, 66.0, 34.0, sw, "F.Cu", NET_MUX_Y5, "MUX_Y5");
        write_via(pcb, 66.0, 34.0, VIA_PAD, VIA_DRILL, NET_MUX_Y5);
        write_trace(pcb, 66.0, 34.0, 43.0, 34.0, sw, "B.Cu", NET_MUX_Y5, "MUX_Y5");
        write_via(pcb, 43.0, 34.0, VIA_PAD, VIA_DRILL, NET_MUX_Y5);
        write_trace(pcb, 43.0, 34.0, 43.0, my, sw, "F.Cu", NET_MUX_Y5, "MUX_Y5");
        write_trace(pcb, 43.0, my, mx, my, sw, "F.Cu", NET_MUX_Y5, "MUX_Y5");
    }

    // Y6: PD7 (79,59.9) → U3 pin2 (45.2,26.825)
    //   F.Cu at x=76, north to y=26. Via. B.Cu west to (42,26). Via.
    //   F.Cu south to y=26.825. East to pin.
    //   Via (42,26): Y4 via at (44.5,24) dist=2.83mm. OK.
    //   MUX_COM at x=43.5 from y=28.095-20: at y=26 gap=1.5-0.35-0.125=1.025mm. OK.
    {
        let pd = comp(components, "PD7");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "2");
        write_trace(pcb, px, py, 76.0, py, sw, "F.Cu", NET_MUX_Y6, "MUX_Y6");
        write_trace(pcb, 76.0, py, 76.0, 26.0, sw, "F.Cu", NET_MUX_Y6, "MUX_Y6");
        write_via(pcb, 76.0, 26.0, VIA_PAD, VIA_DRILL, NET_MUX_Y6);
        write_trace(pcb, 76.0, 26.0, 42.0, 26.0, sw, "B.Cu", NET_MUX_Y6, "MUX_Y6");
        write_via(pcb, 42.0, 26.0, VIA_PAD, VIA_DRILL, NET_MUX_Y6);
        write_trace(pcb, 42.0, 26.0, 42.0, my, sw, "F.Cu", NET_MUX_Y6, "MUX_Y6");
        write_trace(pcb, 42.0, my, mx, my, sw, "F.Cu", NET_MUX_Y6, "MUX_Y6");
    }

    // Y7: PD8 (89,59.9) → U3 pin4 (45.2,29.365)
    //   F.Cu at x=87, north to y=35. Via. B.Cu west to (42,35). Via.
    //   F.Cu north to y=29.365. East to pin.
    //   Via (42,35): Y5 via at (44,34) dist=2.24mm. OK.
    //   MUX_COM at x=43.5: gap=1.5-0.35-0.125=1.025mm. OK.
    //   HEATER_PWM B.Cu at y=36: gap=1.0-0.25=0.75mm. OK.
    {
        let pd = comp(components, "PD8");
        let (px, py) = ap(pd, "1");
        let (mx, my) = ap(u3, "4");
        write_trace(pcb, px, py, 87.0, py, sw, "F.Cu", NET_MUX_Y7, "MUX_Y7");
        write_trace(pcb, 87.0, py, 87.0, 35.0, sw, "F.Cu", NET_MUX_Y7, "MUX_Y7");
        write_via(pcb, 87.0, 35.0, VIA_PAD, VIA_DRILL, NET_MUX_Y7);
        write_trace(pcb, 87.0, 35.0, 42.0, 35.0, sw, "B.Cu", NET_MUX_Y7, "MUX_Y7");
        write_via(pcb, 42.0, 35.0, VIA_PAD, VIA_DRILL, NET_MUX_Y7);
        write_trace(pcb, 42.0, 35.0, 42.0, my, sw, "F.Cu", NET_MUX_Y7, "MUX_Y7");
        write_trace(pcb, 42.0, my, mx, my, sw, "F.Cu", NET_MUX_Y7, "MUX_Y7");
    }

    // ═══════════════════════════════════════════════════════════════════
    // VOLTAGE REGULATOR ROUTING
    //
    // U5 removed — 5V now comes from VBUS via D14 (SS34 Schottky).
    // U6 at (35,2): Vin(pin3)=(37.3,3), Vout(pin2)=(35,3), Tab=(35,0.5)
    // C6 at (72,5): now +5V cap (fed by D14 cathode, no longer 12V).
    // C7 at (80.5,5): 5V output cap (now fed by D14 cathode). Pad1=(79,5), Pad2=(82,5).
    // C8 at (32,5): 5V input cap.   C9 at (38,5): 3V3 output cap.
    // C10 at (93,14): 12V bulk cap.
    //
    // KEY ROUTING CONSTRAINTS:
    //   SDA F.Cu at y=3 from x=9 to x=41 — do NOT cross at y=3
    //   SCL F.Cu at y=1.5 from x=1 to x=42 — do NOT cross at y=1.5
    //   HEATER_PWM F.Cu at y=13.095 from x=67 to x=69.3
    //   +5V F.Cu trunk at x=86.25..97 from y=10.5 to y=44.5
    //
    // Strategy: keep all cap routing SOUTH of y=3 (y>=4) or use B.Cu hops.
    // C6: now +5V cap, fed by D14 cathode route (no 12V stub needed).
    // 5V to U6 Vin via D14 cathode area at y=0.75 B.Cu west hop.
    // C8/C9: use B.Cu at y=4.5 to hop from U6 pins to caps.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (u6_vin_x, u6_vin_y) = ap(u6, "3"); // (37.3, 3)
        let (u6_vout_x, u6_vout_y) = ap(u6, "2"); // (35, 3)
        let c6 = comp(components, "C6");
        let c7 = comp(components, "C7");
        let c8 = comp(components, "C8");
        let c9 = comp(components, "C9");
        let c10 = comp(components, "C10");
        let (_c6p1x, _c6p1y) = ap(c6, "1"); // +5V pad at (70.5, 5) — routed via D14
        let (c7p1x, c7p1y) = ap(c7, "1"); // 5V pad at (79, 5)
        let (c8p1x, c8p1y) = ap(c8, "1"); // 5V pad at (30.5, 5)
        let (c9p1x, c9p1y) = ap(c9, "1"); // 3V3 pad at (36.5, 5)
        let (c10p1x, c10p1y) = ap(c10, "1");

        // ── C6 (now +5V cap via D14 cathode) ──
        // C6 pad1 is +5V, fed by D14 cathode route below. No +12V stub needed.

        // ── 5V to U6 Vin ──
        // D14 cathode feeds +5V bus. Route from C7 area north to y=2.25
        // (south of VBUS B.Cu at y=1.5, gap=0.25mm), B.Cu west to U6 Vin x,
        // via, F.Cu south to pin. Eliminates B.Cu vertical at x=37.3 that
        // crossed VBUS B.Cu at y=1.5.
        write_trace(pcb, c7p1x, c7p1y, c7p1x, 2.25, pw, "F.Cu", NET_5V, "+5V");
        write_via(pcb, c7p1x, 2.25, VIA_PAD, VIA_DRILL, NET_5V);
        write_trace(pcb, c7p1x, 2.25, u6_vin_x, 2.25, pw, "B.Cu", NET_5V, "+5V");
        write_via(pcb, u6_vin_x, 2.25, VIA_PAD, VIA_DRILL, NET_5V);
        write_trace(pcb, u6_vin_x, 2.25, u6_vin_x, u6_vin_y, pw, "F.Cu", NET_5V, "+5V");

        // ── C8 (5V input cap) ──
        // From U6 Vin east to x=38, south to y=6.5, B.Cu west to C8
        write_trace(pcb, u6_vin_x, u6_vin_y, 38.0, u6_vin_y, pw, "F.Cu", NET_5V, "+5V");
        write_trace(pcb, 38.0, u6_vin_y, 38.0, 6.5, pw, "F.Cu", NET_5V, "+5V");
        write_via(pcb, 38.0, 6.5, VIA_PAD, VIA_DRILL, NET_5V);
        write_trace(pcb, 38.0, 6.5, c8p1x, 6.5, pw, "B.Cu", NET_5V, "+5V");
        write_via(pcb, c8p1x, 6.5, VIA_PAD, VIA_DRILL, NET_5V);
        write_trace(pcb, c8p1x, 6.5, c8p1x, c8p1y, pw, "F.Cu", NET_5V, "+5V");

        // ── U6 Vout (3.3V) → C9 ──
        // Route south from Vout (35,5) to y=7.5, east to C9 pad1, north.
        // Avoids +5V pin at (37.3,5) which blocks direct east route.
        write_trace(pcb, u6_vout_x, u6_vout_y, u6_vout_x, 7.5, pw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, u6_vout_x, 7.5, c9p1x, 7.5, pw, "F.Cu", NET_3V3, "+3V3");
        write_trace(pcb, c9p1x, 7.5, c9p1x, c9p1y, pw, "F.Cu", NET_3V3, "+3V3");

        // ── Connect U6 Vout/Tab to +3V3 bus ──
        // Direct F.Cu from tab (35,2.5) to Vout pin (35,5). Both F.Cu pads, same net.
        let (u6_tab_x, u6_tab_y) = ap(u6, "4");
        write_trace(pcb, u6_tab_x, u6_tab_y, u6_vout_x, u6_vout_y, pw, "F.Cu", NET_3V3, "+3V3");

        // U6 Vout south to y=9 for +3V3 bus connection
        write_trace(pcb, u6_vout_x, u6_vout_y, u6_vout_x, 9.0, pw, "F.Cu", NET_3V3, "+3V3");

        // ── Connect +5V to bus trunk ──
        // C7 pad1 south to y=12, B.Cu east to C5 area at (86.25,12)
        write_trace(pcb, c7p1x, c7p1y, c7p1x, 12.0, pw, "F.Cu", NET_5V, "+5V");
        write_via(pcb, c7p1x, 12.0, VIA_PAD, VIA_DRILL, NET_5V);
        write_trace(pcb, c7p1x, 12.0, 86.25, 12.0, pw, "B.Cu", NET_5V, "+5V");

        // ── C10 (12V bulk cap near D10) ──
        // Connect from D10 cathode east to C10 (both at y=16)
        let (d10cx, d10cy) = ap(d10, "2"); // (92, 16) = NET_12V
        write_trace(pcb, d10cx, d10cy, c10p1x, c10p1y, pw, "F.Cu", NET_12V, "+12V");
    }

    // ═══════════════════════════════════════════════════════════════════
    // VBUS → D14 (SS34 Schottky) → +5V power path
    //
    // USB-C VBUS from J1 area → D14 at (75,2) → 5V bus
    // Route: existing VBUS via at (15,6) → B.Cu south to y=1.5 → east to (73,1.5)
    //   → via → F.Cu south to D14 anode (73,2).
    // D14 cathode (77,2) → F.Cu to +5V bus connection.
    // ═══════════════════════════════════════════════════════════════════
    {
        let d14 = comp(components, "D14");
        let (d14_ax, d14_ay) = ap(d14, "1"); // (73, 2) VBUS anode
        let (d14_cx, d14_cy) = ap(d14, "2"); // (77, 2) +5V cathode

        // VBUS from USB-C area: T-junction from U7→J1 B.Cu horizontal at y=5.
        // U7 via at (17,6), B.Cu south to (17,5), B.Cu west to (11.5,5).
        // Tap at (15,5) going south to y=1.5.
        write_trace(pcb, 15.0, 5.0, 15.0, 1.5, pw, "B.Cu", NET_VBUS, "VBUS");
        // B.Cu east at y=1.5 to x=73 (main VBUS trunk to D14)
        write_trace(pcb, 15.0, 1.5, 73.0, 1.5, pw, "B.Cu", NET_VBUS, "VBUS");

        // C12 VBUS decoupling cap at (14, 1.5): pad1 VBUS at (13.25, 1.5)
        // Extend VBUS B.Cu trunk west from x=15 to C12 pad1, via to F.Cu pad
        let c12 = comp(components, "C12");
        let (c12p1x, c12p1y) = ap(c12, "1"); // VBUS pad at (13.25, 1.5)
        write_trace(pcb, 15.0, 1.5, c12p1x, c12p1y, pw, "B.Cu", NET_VBUS, "VBUS");
        write_via(pcb, c12p1x, c12p1y, VIA_PAD, VIA_DRILL, NET_VBUS);
        // Via to F.Cu
        write_via(pcb, 73.0, 1.5, VIA_PAD, VIA_DRILL, NET_VBUS);
        // F.Cu south to D14 anode
        write_trace(pcb, 73.0, 1.5, d14_ax, d14_ay, pw, "F.Cu", NET_VBUS, "VBUS");

        // D14 cathode to +5V bus: east to C7 area and west to C6.
        // Route at y=3.7 to pass between D14 pads (bottom edge y=3.3)
        // and C6 pad2 GND (top edge y~4.1).
        write_trace(pcb, d14_cx, d14_cy, d14_cx, 3.7, sw, "F.Cu", NET_5V, "+5V");
        // East to C7 pad1 at (79, 5): east from D14 cathode, south to pad
        write_trace(pcb, d14_cx, 3.7, 79.0, 3.7, sw, "F.Cu", NET_5V, "+5V");
        write_trace(pcb, 79.0, 3.7, 79.0, 5.0, sw, "F.Cu", NET_5V, "+5V");
        // West to C6 pad1 (+5V) at (70.5, 5)
        write_trace(pcb, d14_cx, 3.7, 70.5, 3.7, sw, "F.Cu", NET_5V, "+5V");
        write_trace(pcb, 70.5, 3.7, 70.5, 5.0, sw, "F.Cu", NET_5V, "+5V");
    }

    // ═══════════════════════════════════════════════════════════════════
    // PD LOAD RESISTORS (R20-R27): MUX_Y → GND
    // Short stubs — GND pad connects via zone fill.
    // MUX_Y pad connects to existing PD pad1 net via short trace.
    // R20-R27 at y=58, same x as each PD. PD pad1 connects via MUX_Y vertical.
    // These are close to PDs, so MUX_Y trace passes nearby. Connect R pad1 to
    // the PD pad1 with a short hop.
    // ═══════════════════════════════════════════════════════════════════
    {
        for i in 0..8u32 {
            let net = NET_MUX_Y0 + i;
            let nn: &str = Box::leak(format!("MUX_Y{}", i).into_boxed_str());
            let r = comp(components, Box::leak(format!("R{}", i + 20).into_boxed_str()));
            let pd = comp(components, Box::leak(format!("PD{}", i + 1).into_boxed_str()));
            let (rp1x, rp1y) = ap(r, "1"); // MUX_Y pad
            let (pdp1x, pdp1y) = ap(pd, "1"); // PD anode (MUX_Y)

            // Short trace from PD pad1 to R pad1 (same net, both at similar x)
            write_trace(pcb, pdp1x, pdp1y, rp1x, pdp1y, sw, "F.Cu", net, nn);
            write_trace(pcb, rp1x, pdp1y, rp1x, rp1y, sw, "F.Cu", net, nn);
        }
    }

    // ═══════════════════════════════════════════════════════════════════
    // USB CC RESISTORS (R30, R31): USB_CC1/CC2 → GND
    // R30 at (11.0, 9.5) rot=270: pad1 CC1 at (11.0, 8.75), pad2 GND at (11.0, 10.25)
    // R31 at (14.0, 11.0) rot=270: pad1 CC2 at (14.0, 10.25), pad2 GND at (14.0, 11.75)
    //
    // CC1: F.Cu south from J1 A5 (10.25,5), east to x=11, south to R30 pad1 (11.0, 8.75).
    // CC2: F.Cu from J1 B5 (9.5,3.8) east to x=14, south to R31 pad1 (14.0, 10.25).
    // ═══════════════════════════════════════════════════════════════════
    {
        let r30 = comp(components, "R30");
        let r31 = comp(components, "R31");
        let (cc1_jx, cc1_jy) = ap(j1, "A5"); // USB CC1 pad (10.25, 5)
        let (r30p1x, r30p1y) = ap(r30, "1");  // CC1 at (11.0, 8.75)

        // CC1: F.Cu from A5 south to y=6, east to R30 pad1 x (now at x=11), south to pad.
        // DP at x=9.75 right edge 9.875. CC1 at x=10.25 left edge 10.125. Gap=0.25mm. OK.
        // R30 now at x=11: pad1 CC1 at (11, 6.25).
        write_trace(pcb, cc1_jx, cc1_jy, cc1_jx, 6.0, sw, "F.Cu", NET_USB_CC1, "USB_CC1");
        write_trace(pcb, cc1_jx, 6.0, r30p1x, 6.0, sw, "F.Cu", NET_USB_CC1, "USB_CC1");
        write_trace(pcb, r30p1x, 6.0, r30p1x, r30p1y, sw, "F.Cu", NET_USB_CC1, "USB_CC1");

        let (cc2_jx, cc2_jy) = ap(j1, "B5"); // USB CC2 pad (9.5, 3.8)
        let (r31p1x, r31p1y) = ap(r31, "1"); // CC2 at (14.0, 10.25)

        // CC2: F.Cu from B5 east to R31 pad1 directly. R31 at (19,5), pad1 at (18.25,5).
        // Route: east at y=3.8 to x=18.25, south to R31 pad1 (18.25, 5).
        // VBUS at y=3.5 from x=17→13: gap=3.8-3.5-0.25=0.05mm at x range 13-17.
        // Fix: route CC2 north first to y=4.3 (gap to VBUS=4.3-3.5-0.25=0.55mm),
        // then east, then south to pad.
        // Route CC2 east at pad y (3.8) to avoid DP pad A6 at y=4.5-5.5.
        write_trace(pcb, cc2_jx, cc2_jy, r31p1x, cc2_jy, sw, "F.Cu", NET_USB_CC2, "USB_CC2");
        write_trace(pcb, r31p1x, cc2_jy, r31p1x, r31p1y, sw, "F.Cu", NET_USB_CC2, "USB_CC2");
    }

    // ═══════════════════════════════════════════════════════════════════
    // ESP32 EN: U1 pin3 → R28 pad2 → C11 pad1 → SW2 pad1
    //
    // R28 at (3.5,15): pad1 +3V3 at (2.55,15), pad2 ESP_EN at (4.45,15)
    // C11 at (3.5,13): pad1 ESP_EN at (2.75,13), pad2 GND at (4.25,13)
    // SW2 at (3.5,26): pad1 ESP_EN at (0.25,26)
    // U1 pin3 at (12,11.27): ESP_EN
    //
    // Route: F.Cu west to x=10.5, via, B.Cu at y=11 west to x=3.5
    //   (SDA gap zone — no SDA B.Cu at y=11). Via. F.Cu south at x=3.5
    //   to C11, R28. Then F.Cu south at x=3 to SW2.
    //
    // B.Cu at y=11: no crossings with CC (all F.Cu), LB0 (at y=10),
    //   GPIO0 (at y=10.5), +3V3 (at y=9.35), SDA (gap zone).
    // ═══════════════════════════════════════════════════════════════════
    {
        let (u1_en_x, u1_en_y) = ap(u1, "3"); // (12, 11.27)
        let r28 = comp(components, "R28");
        let c11 = comp(components, "C11");
        let sw2 = comp(components, "SW2");
        let (r28p2x, r28p2y) = ap(r28, "2"); // ESP_EN at (4.45, 15)
        let (c11p1x, c11p1y) = ap(c11, "1"); // ESP_EN at (2.75, 13)
        let (_sw2p1x, sw2p1y) = ap(sw2, "1"); // ESP_EN pad1 at (1.75, 26)

        // F.Cu west from pin3 to x=10.5 (east of GPIO0 at x=11.5, safe gap)
        write_trace(pcb, u1_en_x, u1_en_y, 10.5, u1_en_y, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        write_via(pcb, 10.5, u1_en_y, VIA_PAD, VIA_DRILL, NET_ESP_EN);
        // B.Cu south from y=11.27 to y=11, then west at y=11 to x=3.5
        write_trace(pcb, 10.5, u1_en_y, 10.5, 11.0, sw, "B.Cu", NET_ESP_EN, "ESP_EN");
        write_trace(pcb, 10.5, 11.0, 3.5, 11.0, sw, "B.Cu", NET_ESP_EN, "ESP_EN");
        write_via(pcb, 3.5, 11.0, VIA_PAD, VIA_DRILL, NET_ESP_EN);

        // F.Cu south at x=3.5 to C11 area (passes between C11 pads at y=13)
        write_trace(pcb, 3.5, 11.0, 3.5, c11p1y, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        // Stub west to C11 pad1 (2.75, 13)
        write_trace(pcb, 3.5, c11p1y, c11p1x, c11p1y, sw, "F.Cu", NET_ESP_EN, "ESP_EN");

        // Continue south to R28 pad2 (4.45, 15)
        write_trace(pcb, 3.5, c11p1y, 3.5, r28p2y, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        write_trace(pcb, 3.5, r28p2y, r28p2x, r28p2y, sw, "F.Cu", NET_ESP_EN, "ESP_EN");

        // R28 pad2 → SW2: F.Cu south at x=3.5 past R28 pad1, then B.Cu hop west.
        // R28 pad1 extends x=2.05-3.05, y=14.4-15.6. At x=3.5 gap=3.375-3.05=0.325mm. OK.
        // Continue south to y=16.5 (below R28), via, B.Cu west to x=1.75, via, F.Cu south to SW2.
        // +3V3 B.Cu at x=2.55 from y=11.8-15: at y=16.5 it's below. No conflict.
        write_trace(pcb, 3.5, r28p2y, 3.5, 16.5, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        write_via(pcb, 3.5, 16.5, VIA_PAD, VIA_DRILL, NET_ESP_EN);
        write_trace(pcb, 3.5, 16.5, 1.75, 16.5, sw, "B.Cu", NET_ESP_EN, "ESP_EN");
        write_via(pcb, 1.75, 16.5, VIA_PAD, VIA_DRILL, NET_ESP_EN);
        // F.Cu south at x=1.75 from y=16.5, with F.Cu hop around SW1 pad1 at
        // (1.75, 22) which extends x=1.0-2.5, y=21.5-22.5.
        // Via at (1.75, 20.5), F.Cu west to x=0.5 (west of pad left edge 1.0),
        // south past pad, east back to x=1.75, via, continue south.
        write_trace(pcb, 1.75, 16.5, 1.75, 20.5, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        write_trace(pcb, 1.75, 20.5, 0.5, 20.5, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        write_trace(pcb, 0.5, 20.5, 0.5, 23.5, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        write_trace(pcb, 0.5, 23.5, 1.75, 23.5, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
        write_trace(pcb, 1.75, 23.5, 1.75, sw2p1y, sw, "F.Cu", NET_ESP_EN, "ESP_EN");
    }

    // ═══════════════════════════════════════════════════════════════════
    // ESP32 GPIO0: U1 pin27 → R29 pad2 → SW1 pad1
    //
    // R29 at (3.5,18): pad1 +3V3 at (2.55,18), pad2 ESP_GPIO0 at (4.45,18)
    // SW1 at (3.5,22): pad1 ESP_GPIO0 at (0.25,22)
    // U1 pin27 at (28, 17.62) — right side of ESP32
    //
    // Route: via, B.Cu west to x=11.5, south to y=10.5 (SDA gap, above LB0
    //   at y=10). Via. F.Cu hop west over LB0 B.Cu at y=10 to x=9.5. Via.
    //   B.Cu west at y=10.5 from x=9.5 to x=4.5. Via. F.Cu south to R29.
    //
    // GPIO0 B.Cu at y=10.5: no crossings (CC all F.Cu, LB0 at y=10, ESP_EN
    //   at y=11, +3V3 at y=9.35, SDA gap zone). LB0 upper edge 10.125,
    //   GPIO0 lower edge 10.375, gap=0.25mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (u1_gpio0_x, u1_gpio0_y) = ap(u1, "27"); // (28, 17.62)
        let r29 = comp(components, "R29");
        let sw1 = comp(components, "SW1");
        let (r29p2x, r29p2y) = ap(r29, "2"); // ESP_GPIO0 at (4.45, 18)
        let (sw1p1x, sw1p1y) = ap(sw1, "1"); // ESP_GPIO0 at (0.25, 22)

        // Pin27 (28,17.62) → via → B.Cu west to x=11.5 at y=17.62
        write_via(pcb, u1_gpio0_x, u1_gpio0_y, VIA_PAD, VIA_DRILL, NET_ESP_GPIO0);
        write_trace(pcb, u1_gpio0_x, u1_gpio0_y, 11.5, u1_gpio0_y, sw, "B.Cu", NET_ESP_GPIO0, "ESP_GPIO0");

        // B.Cu south at x=11.5 from y=17.62 to y=12 (above U1 pad2/pad3 zone),
        // then west to x=10.5, then south to y=10.5 to avoid U1 pads at x=11.25+.
        // U1 pad3 (ESP_EN) at (12,11.27): extends x=11.25-12.75, y=10.82-11.72.
        // U1 pad2 (+3V3) at (12,10): extends x=11.25-12.75, y=9.55-10.45.
        // At y=12: above both pads. At x=10.5: west of pads.
        write_trace(pcb, 11.5, u1_gpio0_y, 11.5, 12.0, sw, "B.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        // B.Cu south at x=11.5 to y=10.5, then west to x=5. Avoids x=10.5 which
        // overlaps ESP_EN via at (10.5,11.27). At y=10.5 passing x=10.5: trace top
        // edge 10.625, ESP_EN via bottom 10.92, gap=0.295mm. OK.
        // B.Cu horizontal moved to y=10.61 to clear LB0 via at (7.5, 10):
        // LB0 via pad top=10.35, GPIO0 bottom=10.485, gap=0.135mm>0.127mm. OK.
        // ESP_EN B.Cu at y=11 bottom=10.875, GPIO0 top=10.735, gap=0.14mm>0.127mm. OK.
        write_trace(pcb, 11.5, 12.0, 11.5, 10.61, sw, "B.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_trace(pcb, 11.5, 10.61, 5.0, 10.61, sw, "B.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_trace(pcb, 5.0, 10.61, 5.0, 10.0, sw, "B.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_via(pcb, 5.0, 10.0, VIA_PAD, VIA_DRILL, NET_ESP_GPIO0);

        // F.Cu south at x=5.0 from y=10.0, stay west of LB0 F.Cu at x=7.5.
        // Via to B.Cu at y=12 to hop past R28 pad (F.Cu, x=3.95-4.95, y=14.4-15.6).
        // ESP_EN B.Cu at y=11 from x=3.5-10.5: F.Cu crosses it safely (different layer).
        write_trace(pcb, 5.0, 10.0, 4.5, 10.0, sw, "F.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_trace(pcb, 4.5, 10.0, 4.5, 12.0, sw, "F.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_via(pcb, 4.5, 12.0, VIA_PAD, VIA_DRILL, NET_ESP_GPIO0);
        // B.Cu south past R28 pad (F.Cu only, B.Cu clear)
        write_trace(pcb, 4.5, 12.0, 4.5, 16.5, sw, "B.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_via(pcb, 4.5, 16.5, VIA_PAD, VIA_DRILL, NET_ESP_GPIO0);
        // F.Cu south to R29 pad2 (4.45, 18)
        write_trace(pcb, 4.5, 16.5, 4.5, r29p2y, sw, "F.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_trace(pcb, 4.5, r29p2y, r29p2x, r29p2y, sw, "F.Cu", NET_ESP_GPIO0, "ESP_GPIO0");

        // R29 pad2 → SW1: F.Cu south to y=22, via, B.Cu west to SW1
        write_trace(pcb, r29p2x, r29p2y, r29p2x, sw1p1y, sw, "F.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_via(pcb, r29p2x, sw1p1y, VIA_PAD, VIA_DRILL, NET_ESP_GPIO0);
        write_trace(pcb, r29p2x, sw1p1y, sw1p1x, sw1p1y, sw, "B.Cu", NET_ESP_GPIO0, "ESP_GPIO0");
        write_via(pcb, sw1p1x, sw1p1y, VIA_PAD, VIA_DRILL, NET_ESP_GPIO0);
    }

    // ═══════════════════════════════════════════════════════════════════
    // UART: U1 pin36 (RX) → J5 pin3, U1 pin37 (TX) → J5 pin2
    //
    // U1 pin36 at (28, 29.05), pin37 at (28, 30.32) — right side of ESP32
    // J5 at (3.5,30): pin2=UART_TX at (3.5,27.46), pin3=UART_RX at (3.5,30.0)
    //
    // Cannot use B.Cu west from x=28 because MUX_S0/S1/S2 B.Cu traces run
    // at y=30.18/31.45/32.72 from x=12 to x=58+.
    //
    // Strategy: F.Cu south from U1 pins to y=34 (below MUX_S B.Cu), then
    // via to B.Cu, west to J5, via back to F.Cu for J5 approach.
    // ═══════════════════════════════════════════════════════════════════
    {
        let j5 = comp(components, "J5");
        let (u1_rx_x, u1_rx_y) = ap(u1, "36"); // UART_RX at (28, 29.05)
        let (u1_tx_x, u1_tx_y) = ap(u1, "37"); // UART_TX at (28, 30.32)
        let (j5p2x, j5p2y) = ap(j5, "2");  // UART_TX
        let (j5p3x, j5p3y) = ap(j5, "3");  // UART_RX

        // TX: pin37 (28, 30.32) → J5 pin2 (3.5, 28.73)
        // Fan-out: F.Cu east to x=28.5, south to y=34.
        // Via. B.Cu west at y=34. Then hop HEATER_PWM and reach J5.
        write_trace(pcb, u1_tx_x, u1_tx_y, 28.5, u1_tx_y, sw, "F.Cu", NET_UART_TX, "UART_TX");
        write_trace(pcb, 28.5, u1_tx_y, 28.5, 34.0, sw, "F.Cu", NET_UART_TX, "UART_TX");
        write_via(pcb, 28.5, 34.0, VIA_PAD, VIA_DRILL, NET_UART_TX);
        write_trace(pcb, 28.5, 34.0, 12.0, 34.0, sw, "B.Cu", NET_UART_TX, "UART_TX");
        write_via(pcb, 12.0, 34.0, VIA_PAD, VIA_DRILL, NET_UART_TX);
        write_trace(pcb, 12.0, 34.0, 9.0, 34.0, sw, "F.Cu", NET_UART_TX, "UART_TX");
        write_via(pcb, 9.0, 34.0, VIA_PAD, VIA_DRILL, NET_UART_TX);
        write_trace(pcb, 9.0, 34.0, 6.0, 34.0, sw, "B.Cu", NET_UART_TX, "UART_TX");
        write_via(pcb, 6.0, 34.0, VIA_PAD, VIA_DRILL, NET_UART_TX);
        // B.Cu hop around UART_RX F.Cu horizontal at y=31.27 (x=3.5→7).
        write_trace(pcb, 6.0, 34.0, 6.0, 32.0, sw, "F.Cu", NET_UART_TX, "UART_TX");
        write_via(pcb, 6.0, 32.0, VIA_PAD, VIA_DRILL, NET_UART_TX);
        write_trace(pcb, 6.0, 32.0, 6.0, 30.65, sw, "B.Cu", NET_UART_TX, "UART_TX");
        write_via(pcb, 6.0, 30.65, VIA_PAD, VIA_DRILL, NET_UART_TX);
        write_trace(pcb, 6.0, 30.65, 6.0, j5p2y, sw, "F.Cu", NET_UART_TX, "UART_TX");
        write_trace(pcb, 6.0, j5p2y, j5p2x, j5p2y, sw, "F.Cu", NET_UART_TX, "UART_TX");

        // RX: pin36 (28, 29.05) → J5 pin3 (3.5, 31.27)
        // Fan-out: F.Cu east to x=29.25 (west of LB2 at x=30.05), south to y=35.
        write_trace(pcb, u1_rx_x, u1_rx_y, 29.25, u1_rx_y, sw, "F.Cu", NET_UART_RX, "UART_RX");
        write_trace(pcb, 29.25, u1_rx_y, 29.25, 35.0, sw, "F.Cu", NET_UART_RX, "UART_RX");
        write_via(pcb, 29.25, 35.0, VIA_PAD, VIA_DRILL, NET_UART_RX);
        write_trace(pcb, 29.25, 35.0, 12.0, 35.0, sw, "B.Cu", NET_UART_RX, "UART_RX");
        write_via(pcb, 12.0, 35.0, VIA_PAD, VIA_DRILL, NET_UART_RX);
        write_trace(pcb, 12.0, 35.0, 9.0, 35.0, sw, "F.Cu", NET_UART_RX, "UART_RX");
        write_via(pcb, 9.0, 35.0, VIA_PAD, VIA_DRILL, NET_UART_RX);
        write_trace(pcb, 9.0, 35.0, 6.75, 35.0, sw, "B.Cu", NET_UART_RX, "UART_RX");
        write_via(pcb, 6.75, 35.0, VIA_PAD, VIA_DRILL, NET_UART_RX);
        write_trace(pcb, 6.75, 35.0, 6.75, j5p3y, sw, "F.Cu", NET_UART_RX, "UART_RX");
        write_trace(pcb, 6.75, j5p3y, j5p3x, j5p3y, sw, "F.Cu", NET_UART_RX, "UART_RX");
    }

    // ═══════════════════════════════════════════════════════════════════
    // USB ESD PROTECTION (U7 at (17,7)):
    // USB_DP: F.Cu at y=8 from U7 pin1 west to x=9.75 (taps DP vertical)
    // USB_DN: F.Cu at y=9 from U7 pin3 west to x=9.25 (taps DN vertical)
    // VBUS: U7 pin5 → F.Cu north → B.Cu west → J1 A4
    // GND: U7 pin2 → zone fill
    // ═══════════════════════════════════════════════════════════════════
    {
        let u7 = comp(components, "U7");
        let (u7_dp1_x, u7_dp1_y) = ap(u7, "1"); // USB_DP at (14.05, 8)
        let (u7_dn3_x, u7_dn3_y) = ap(u7, "3"); // USB_DN at (15.95, 8)
        let (u7_dn4_x, u7_dn4_y) = ap(u7, "4"); // USB_DN at (15.95, 6)
        let (u7_vbus_x, u7_vbus_y) = ap(u7, "5"); // VBUS at (15, 6)
        let (u7_dp6_x, u7_dp6_y) = ap(u7, "6"); // USB_DP at (14.05, 6)

        // Connect U7 pin1 (DP) to USB_DP vertical at x=9.75 via B.Cu hop.
        // F.Cu stops at x=13 to avoid R30 pads, then B.Cu west to via on DP vertical.
        // SDA B.Cu at x=9 gap to via at (9.75,8): 9.4-9.125=0.275mm>0.127mm. OK.
        write_trace(pcb, u7_dp1_x, u7_dp1_y, 13.0, u7_dp1_y, sw, "F.Cu", NET_USB_DP, "USB_DP");
        write_via(pcb, 13.0, u7_dp1_y, VIA_PAD, VIA_DRILL, NET_USB_DP);
        write_trace(pcb, 13.0, u7_dp1_y, 10.0, u7_dp1_y, sw, "B.Cu", NET_USB_DP, "USB_DP");
        write_via(pcb, 10.0, u7_dp1_y, VIA_PAD, VIA_DRILL, NET_USB_DP);
        write_trace(pcb, 10.0, u7_dp1_y, 9.75, u7_dp1_y, sw, "F.Cu", NET_USB_DP, "USB_DP");

        // Connect U7 pin6 (DP) to pin1 — simple F.Cu vertical
        write_trace(pcb, u7_dp1_x, u7_dp1_y, u7_dp6_x, u7_dp6_y, sw, "F.Cu", NET_USB_DP, "USB_DP");

        // Connect U7 pin3 (DN) to USB_DN vertical at x=9.25.
        // Use B.Cu at y=6.5 to avoid SDA B.Cu at x=9 (y=3-8.5) overlap.
        // Via at (u7_dn3_x, 6.5): F.Cu south from pin3 (y=8) to y=6.5. Via.
        // B.Cu west at y=6.5 to (9.25, 6.5). Via. Connect to DN vertical on F.Cu.
        // SDA B.Cu at x=9 from y=3 to y=8.5: via (9.25, 6.5) extends x=8.9-9.6, y=6.15-6.85.
        //   SDA at x=9, x=8.875-9.125. Via x=8.9-9.6. Overlap in x: 8.9-9.125.
        //   SDA at y=6.5: SDA B.Cu goes from (9,3) to (9,8.5). At y=6.5 SDA B.Cu exists!
        //   Still crosses. Move to y=6.0 instead.
        // At y=6.0: via (9.25, 6.0) extends y=5.65-6.35. SDA at x=9 from y=3-8.5 at y=6: exists!
        // This is the fundamental issue: SDA B.Cu at x=9 runs from y=3 to y=8.5.
        // Any via at x=9.25 in that y range will overlap.
        // Fix: connect DN to its vertical ABOVE the SDA B.Cu area.
        // DN vertical at x=9.25 from J1 (y=5) south. At y=5, SDA B.Cu at x=9 exists (y=3-8.5).
        // The DN vertical is at x=9.25, SDA at x=9. Trace widths 0.25mm each.
        // DN right edge: 9.375. SDA right edge: 9.125. DN left: 9.125. SDA right: 9.125. Gap=0.
        // They're actually touching at edges already. Let me check the original DN routing.
        // DN goes from J1 A7 (9.25, 5) straight south to U1 pin14 (12, 25.1).
        // The DN vertical at x=9.25 and SDA B.Cu at x=9 are on different layers
        // (DN on F.Cu, SDA on B.Cu in that segment). No conflict there.
        // The issue is only with the U7 connection via.
        //
        // Solution: route U7 DN on F.Cu only. From U7 pin3 (15.95, 8) south on F.Cu
        // to y=25.1 (U1 pin14 y), then the DN vertical at x=9.25 already reaches there.
        // Actually U7 pin3 just needs to reach the DN vertical at x=9.25.
        // Route: F.Cu west from (15.95, 8) to x=9.25 at y=8. But DP trace at y=8
        // from x=9.75 to 14.05 is also on F.Cu at y=8! Same y, crossing.
        // Fix: route DN from U7 pin3 south at x=15.95 to y=5 (same as J1 row).
        // Then west on F.Cu at y=5 to x=9.25. But J1 pads are at y=5 (x=7.25-12.75).
        // This would cross J1 pads. Not viable.
        //
        // Alternative: route on B.Cu at y=5.5, below J1 pads (y=4.5-5.5) and above SDA.
        // Actually J1 pads extend to y=5.5 (height 1.0mm, center at y=5). At y=5.5: edge.
        // Use y=4.5 (north of J1 pads). J1 pads top edge = 5-0.5 = 4.5. Touching. Use y=4.0.
        // B.Cu at y=4.0: SDA B.Cu at x=9 from y=3-8.5. At y=4, x=9 exists. Via would overlap.
        //
        // Simplest: tap the DN vertical at x=9.25 where it crosses y=25.1 (U1 pin14).
        // No separate U7 connection needed — U7 pin3/4 already connect internally.
        // But U7 has separate pins for each data line that need external routing.
        //
        // Best approach: Use a via at x that avoids SDA B.Cu at x=9.
        // Route U7 DN (15.95, 8) west on F.Cu to x=10 (east of SDA x=9.125). Via at (10, 8).
        // B.Cu south at x=10 to y=5.5. Then B.Cu west to x=9.25 at y=5.5. Via.
        // B.Cu at y=5.5: SDA B.Cu at x=9 from y=3-8.5 at x=8.875-9.125. At (9.25,5.5):
        //   via extends x=8.9-9.6, y=5.15-5.85. SDA x=8.875-9.125. Overlap: 8.9-9.125 at y=5.5.
        //   SDA B.Cu at y=5.5 exists (y=3-8.5 range). OVERLAP.
        //
        // The only clean approach: tap DN F.Cu vertical at x=9.25 directly with F.Cu trace.
        // U7 DN at (15.95, 8). F.Cu south to y=24 (just above U1 pin14 at y=25.1) then
        // west to x=9.25 on F.Cu at y=24. But that's a very long trace.
        //
        // Actually, re-check: is the SDA B.Cu at x=9 actually a problem for a via at x=9.25?
        // Via drill=0.35mm, pad=0.7mm. Via at (9.25, 7): x=8.9-9.6, y=6.65-7.35.
        // SDA B.Cu at x=9: trace x=8.875-9.125. At y=7 SDA B.Cu exists.
        // Via pad on B.Cu overlaps SDA trace. Different nets -> SHORT.
        //
        // Fix: avoid x=9 entirely. Route DN from U7 via a jog. F.Cu from U7 pin3 (15.95,8)
        // west to x=10.5 at y=8 (stops east of SDA). Via at (10.5, 8). B.Cu south to y=2.
        // B.Cu west at y=2 to x=9.25 (SDA B.Cu at x=9 exists at y=2? SDA B.Cu at x=9 from
        // y=3 to y=8.5. At y=2: NO SDA. OK!). Via at (9.25, 2). F.Cu south to y=5 to connect
        // to DN vertical. Wait, DN vertical starts at J1 A7 (9.25, 5).
        // DN F.Cu from (9.25, 5) south to U1. We need to connect (9.25, 2) to (9.25, 5).
        // That means DN F.Cu at x=9.25 from y=2 to y=5. SDA F.Cu at y=3 from x=9 to x=31.5.
        // At x=9.25, SDA F.Cu at y=3: trace x=9-31.5 at y=2.875-3.125. Our trace at x=9.25:
        // x=9.125-9.375. Overlap at y=3! CROSSING!
        //
        // Cleanest: connect U7 DN to the existing DN vertical via B.Cu without touching x=9.
        // F.Cu from pin3 (15.95, 8) south to y=25.1 at x=15.95 (west of U1 right pads at x=28).
        // Then F.Cu west at y=25.1 to x=12 (U1 pin14 x). This is pin14 location.
        // Actually pin14 at (12, 25.1) is already connected by the DN vertical from J1.
        // U7 pin3 needs to connect to the same net. Route from U7 pin3 south along x=15.95
        // to y=25.1, west to x=12. This taps directly into U1 pin14. But U1 pin14 is at (12,25.1).
        // We'd be routing from (15.95,8) south to (15.95,25.1) west to (12,25.1). That's fine.
        // At x=15.95: no other signal traces in that corridor. U1 is at x=12-28. Left pads at x=12.
        // x=15.95 is between U1 edge (x=12) and center area. Check: MUX_COM F.Cu at x=41:
        // far east. HEATER_PWM via at (12,26.37): at y=26.37, x=12. Our trace at x=15.95: gap=3.95mm. OK.
        // +3V3 bus at y=9 from x=12-44.5. At (15.95, 9): trace passes through bus. CROSSING!
        // Fix: via before bus at (15.95, 9.5), B.Cu south past bus, via after.
        // Or start on B.Cu from U7 to avoid the bus entirely.
        // U7 DN → U1 pin14: U7 pin3 at (17.95, 8). Route south at x=17.95 to y=8.5,
        // via, B.Cu south past +3V3 bus at y=9 to y=9.75, via, F.Cu south/west to U1.
        // CC2 at x=18.25 (R31 pad): gap=18.25-17.95-0.25=0.05mm at y=5.
        // Fix: jog west to x=16 before going south past U1 GND pad zone.
        // Via moved from y=8.5 to y=8.15 to clear +3V3 F.Cu at y=9
        // (pad top=8.5, +3V3 edge=8.875, gap=0.375mm). OK.
        write_trace(pcb, u7_dn3_x, u7_dn3_y, u7_dn3_x, 8.15, sw, "F.Cu", NET_USB_DN, "USB_DN");
        write_via(pcb, u7_dn3_x, 8.15, VIA_PAD, VIA_DRILL, NET_USB_DN);
        write_trace(pcb, u7_dn3_x, 8.15, u7_dn3_x, 9.75, sw, "B.Cu", NET_USB_DN, "USB_DN");
        write_via(pcb, u7_dn3_x, 9.75, VIA_PAD, VIA_DRILL, NET_USB_DN);
        // F.Cu south to y=16, west to x=16 (west of U1 GND pad), south to U1 pin13
        // Pin 13 = GPIO19 = USB_D- at (12, 23.83)
        write_trace(pcb, u7_dn3_x, 9.75, u7_dn3_x, 16.0, sw, "F.Cu", NET_USB_DN, "USB_DN");
        write_trace(pcb, u7_dn3_x, 16.0, 16.0, 16.0, sw, "F.Cu", NET_USB_DN, "USB_DN");
        write_trace(pcb, 16.0, 16.0, 16.0, 23.83, sw, "F.Cu", NET_USB_DN, "USB_DN");
        write_trace(pcb, 16.0, 23.83, 12.0, 23.83, sw, "F.Cu", NET_USB_DN, "USB_DN");

        // Connect U7 pin4 (DN) to pin3 — simple F.Cu vertical
        write_trace(pcb, u7_dn3_x, u7_dn3_y, u7_dn4_x, u7_dn4_y, sw, "F.Cu", NET_USB_DN, "USB_DN");

        // Connect U7 pin5 (VBUS) to J1 A4 (10.75, 5) via B.Cu to avoid CC2 crossing.
        // Via at pin5, B.Cu south to y=5, west to x=11.5. Via. F.Cu west to A4 pad.
        // B.Cu at y=5: SDA B.Cu at x=9 gap=11.5-9.125=2.375mm. OK.
        let (j1_vbus_x, j1_vbus_y) = ap(j1, "A4"); // VBUS at (10.75, 5)
        write_via(pcb, u7_vbus_x, u7_vbus_y, VIA_PAD, VIA_DRILL, NET_VBUS);
        write_trace(pcb, u7_vbus_x, u7_vbus_y, u7_vbus_x, j1_vbus_y, sw, "B.Cu", NET_VBUS, "VBUS");
        write_trace(pcb, u7_vbus_x, j1_vbus_y, 11.5, j1_vbus_y, sw, "B.Cu", NET_VBUS, "VBUS");
        write_via(pcb, 11.5, j1_vbus_y, VIA_PAD, VIA_DRILL, NET_VBUS);
        write_trace(pcb, 11.5, j1_vbus_y, j1_vbus_x, j1_vbus_y, sw, "F.Cu", NET_VBUS, "VBUS");
    }

    // ═══════════════════════════════════════════════════════════════════
    // STATUS LEDs:
    // R32 (power LED): +3V3 → R32 pad1, R32 pad2 → D11 pad1 (LED_PWR_ANODE)
    // R33 (activity LED): GPIO_ACT → R33 pad1, R33 pad2 → D12 pad1 (LED_ACT_ANODE)
    // D11/D12 pad2 → GND (zone fill)
    // R32 at (5,35) rot=90: pad1 +3V3 at (5,35.95), pad2 LED_PWR at (5,34.05)
    // R33 at (5,40) rot=90: pad1 GPIO_ACT at (5,40.95), pad2 LED_ACT at (5,39.05)
    // D11 at (5,42) rot=90: pad1 LED_PWR at (5,42.95), pad2 GND at (5,41.05)
    // D12 at (5,46) rot=90: pad1 LED_ACT at (5,46.95), pad2 GND at (5,45.05)
    // ═══════════════════════════════════════════════════════════════════
    {
        let r32 = comp(components, "R32");
        let d11 = comp(components, "D11");
        let (r32p2x, r32p2y) = ap(r32, "2"); // LED_PWR_ANODE at (5, 34.05)
        let (d11p1x, d11p1y) = ap(d11, "1"); // LED_PWR_ANODE at (5, 42.95)

        // LED_PWR_ANODE: R32 pad2 (5,34.05) → D11 pad1 (5,42.95).
        // Route on B.Cu at x=1.5 west bypass (west of all UART, ESP_GPIO_ACT traces).
        // Via at R32 pad2, B.Cu north to y=32.5 (above J5 pin4 GND at y=32.96-34.66).
        // B.Cu west at y=32.5 to x=1.5. South to D11 pad1 y. East to pad. Via.
        // +3V3 B.Cu at x=2.2 from y=33.5 to y=35.95: at y=32.5 doesn't exist. OK.
        // J5 pin4 at (3.5, 33.81): top edge 32.96. Our y=32.5: gap=0.375mm. OK.
        // SCL F.Cu at x=1.0: our B.Cu on different layer. OK.
        write_via(pcb, r32p2x, r32p2y, VIA_PAD, VIA_DRILL, NET_LED_PWR_ANODE);
        write_trace(pcb, r32p2x, r32p2y, r32p2x, 32.5, sw, "B.Cu", NET_LED_PWR_ANODE, "LED_PWR_ANODE");
        write_trace(pcb, r32p2x, 32.5, 1.5, 32.5, sw, "B.Cu", NET_LED_PWR_ANODE, "LED_PWR_ANODE");
        // F.Cu hop around LED_ACT B.Cu horizontal at y=39.05 (x=0.5→5).
        write_trace(pcb, 1.5, 32.5, 1.5, 38.25, sw, "B.Cu", NET_LED_PWR_ANODE, "LED_PWR_ANODE");
        write_via(pcb, 1.5, 38.25, VIA_PAD, VIA_DRILL, NET_LED_PWR_ANODE);
        write_trace(pcb, 1.5, 38.25, 1.5, 39.85, sw, "F.Cu", NET_LED_PWR_ANODE, "LED_PWR_ANODE");
        write_via(pcb, 1.5, 39.85, VIA_PAD, VIA_DRILL, NET_LED_PWR_ANODE);
        write_trace(pcb, 1.5, 39.85, 1.5, d11p1y, sw, "B.Cu", NET_LED_PWR_ANODE, "LED_PWR_ANODE");
        write_trace(pcb, 1.5, d11p1y, d11p1x, d11p1y, sw, "B.Cu", NET_LED_PWR_ANODE, "LED_PWR_ANODE");
        write_via(pcb, d11p1x, d11p1y, VIA_PAD, VIA_DRILL, NET_LED_PWR_ANODE);

        let r33 = comp(components, "R33");
        let d12 = comp(components, "D12");
        let (r33p2x, r33p2y) = ap(r33, "2"); // LED_ACT_ANODE at (5, 39.05)
        let (d12p1x, d12p1y) = ap(d12, "1"); // LED_ACT_ANODE at (5, 46.95)

        // LED_ACT_ANODE: R33 pad2 (5,39.05) → D12 pad1 (5,46.95).
        // Route on B.Cu at x=0.5 (west of LED_PWR at x=1.5) to avoid crossing
        // LED_PWR horizontal B.Cu at y=42.95 (x=1.5→5.0).
        // Board edge clearance: 0.5-0.125=0.375mm > 0.25mm. OK.
        // LED_PWR at x=1.5: gap=1.5-0.5-0.125-0.125=0.75mm. OK.
        write_via(pcb, r33p2x, r33p2y, VIA_PAD, VIA_DRILL, NET_LED_ACT_ANODE);
        write_trace(pcb, r33p2x, r33p2y, 0.5, r33p2y, sw, "B.Cu", NET_LED_ACT_ANODE, "LED_ACT_ANODE");
        write_trace(pcb, 0.5, r33p2y, 0.5, d12p1y, sw, "B.Cu", NET_LED_ACT_ANODE, "LED_ACT_ANODE");
        write_trace(pcb, 0.5, d12p1y, d12p1x, d12p1y, sw, "B.Cu", NET_LED_ACT_ANODE, "LED_ACT_ANODE");
        write_via(pcb, d12p1x, d12p1y, VIA_PAD, VIA_DRILL, NET_LED_ACT_ANODE);

        // Activity LED GPIO: U1 pin35 → R33 pad1
        // pin35 at (28,27.78). R33 at (5,40) rot=90. pad1 at (5, 40.95).
        let (u1_act_x, u1_act_y) = ap(u1, "35"); // ESP_GPIO_ACT
        let (r33p1x, r33p1y) = ap(r33, "1"); // (5, 40.95)

        // Fan-out: F.Cu west to x=27.0 (west of U1 pad edge at x=27.55),
        // F.Cu south to y=37.0 (clear of all east-routed signals from U1),
        // via, B.Cu west to x=12.
        write_trace(pcb, u1_act_x, u1_act_y, 26.75, u1_act_y, sw, "F.Cu", NET_ESP_GPIO_ACT, "ESP_GPIO_ACT");
        write_trace(pcb, 26.75, u1_act_y, 26.75, 37.0, sw, "F.Cu", NET_ESP_GPIO_ACT, "ESP_GPIO_ACT");
        write_via(pcb, 26.75, 37.0, VIA_PAD, VIA_DRILL, NET_ESP_GPIO_ACT);
        write_trace(pcb, 26.75, 37.0, 12.0, 37.0, sw, "B.Cu", NET_ESP_GPIO_ACT, "ESP_GPIO_ACT");
        write_via(pcb, 12.0, 37.0, VIA_PAD, VIA_DRILL, NET_ESP_GPIO_ACT);
        write_trace(pcb, 12.0, 37.0, 9.0, 37.0, sw, "F.Cu", NET_ESP_GPIO_ACT, "ESP_GPIO_ACT");
        write_via(pcb, 9.0, 37.0, VIA_PAD, VIA_DRILL, NET_ESP_GPIO_ACT);
        write_trace(pcb, 9.0, 37.0, 3.0, 37.0, sw, "B.Cu", NET_ESP_GPIO_ACT, "ESP_GPIO_ACT");
        write_via(pcb, 3.0, 37.0, VIA_PAD, VIA_DRILL, NET_ESP_GPIO_ACT);
        // F.Cu south at x=3.0 to R33 pad1 (5, 40.95)
        write_trace(pcb, 3.0, 37.0, 3.0, r33p1y, sw, "F.Cu", NET_ESP_GPIO_ACT, "ESP_GPIO_ACT");
        write_trace(pcb, 3.0, r33p1y, r33p1x, r33p1y, sw, "F.Cu", NET_ESP_GPIO_ACT, "ESP_GPIO_ACT");
    }

    // ═══════════════════════════════════════════════════════════════════
    // MANUALLY ROUTED AUTOROUTER FAILURES
    //
    // 9 nets the grid autorouter could not complete. Routed by hand with
    // clearance analysis against thermal vias at y=49 and all existing
    // signal/power traces.
    // ═══════════════════════════════════════════════════════════════════

    // +3V3 U2→C3 already connected via bus routing (R18/R19). No manual route needed.

    // ── 2. MUX_Y3: U3 pin12 (54.8,30.635) → R23 pad1 (48.05,58) ──
    // F.Cu west from U3 pin12 to x=48, south through y=49 thermal via
    // corridor (between vias at x=47 and x=50), continue to y=58, west
    // to R23 pad1. x=48 clears U3 left pads (x≈45.2) and LED component
    // pads at x≈49.05 (Q6, R5, R13, +5V/LB4 vias).
    // At y=49, x=48: via x=47 pad right=47.35, via x=50 pad left=49.65.
    //   Trace edges 47.875-48.125. Gap to 47.35=0.525mm, to 49.65=1.525mm. OK.
    // Pads at x≈49.05: left edge ~48.75. Gap = 48.75-48.125 = 0.625mm. OK.
    {
        let (u3p12x, u3p12y) = ap(u3, "12"); // (54.8, 30.635)
        let r23 = comp(components, "R23");
        let (r23p1x, r23p1y) = ap(r23, "1"); // (48.05, 58)

        write_trace(pcb, u3p12x, u3p12y, 48.0, u3p12y, sw, "F.Cu", NET_MUX_Y3, "MUX_Y3");
        write_trace(pcb, 48.0, u3p12y, 48.0, r23p1y, sw, "F.Cu", NET_MUX_Y3, "MUX_Y3");
        write_trace(pcb, 48.0, r23p1y, r23p1x, r23p1y, sw, "F.Cu", NET_MUX_Y3, "MUX_Y3");
    }

    // ESP_GPIO0 already routed at primary route section (U1→R29 pad2). No manual route needed.

    // ── 4. USB_DN: U7 pin4 (17.95,6) → U7 pin3 (17.95,8) ──
    // Simple 2mm F.Cu vertical. U7 pin5(VBUS) at (15,6) right edge=15.3.
    //   Trace left edge=17.825. Gap=2.525mm. OK.
    // U7 pin2(GND) at (17,8) right edge=17.3. Trace left=17.825. Gap=0.525mm. OK.
    {
        let u7 = comp(components, "U7");
        let (u7p4x, u7p4y) = ap(u7, "4"); // (17.95, 6)
        let (u7p3x, u7p3y) = ap(u7, "3"); // (17.95, 8)

        write_trace(pcb, u7p4x, u7p4y, u7p3x, u7p3y, sw, "F.Cu", NET_USB_DN, "USB_DN");
    }

    // USB_DP U7→J1: REMOVED — U7 already connected to DP vertical via pin1 B.Cu hop
    // (lines above: pin1 → x=13 via → B.Cu → x=10 via → F.Cu → x=9.75).
    // Pin6 → pin1 via F.Cu vertical at x=14.05. No separate J1 connection needed.

    // ── 6. LED_CATH_2: Q4 pin3 (31.0,41) → D3 pad2 (31.0,60.05) ──
    // F.Cu vertical at x=31 from y=41 to y=60.05. Passes straight through
    // thermal via corridor at y=49: via x=32 pad left=31.65, trace right=31.125.
    //   Gap=0.525mm. OK. Via x=29 pad right=29.35, trace left=30.875. Gap=1.525mm. OK.
    {
        let q4 = comp(components, "Q4");
        let d3 = comp(components, "D3");
        let (q4p3x, q4p3y) = ap(q4, "3"); // (31.0, 41)
        let (d3p2x, d3p2y) = ap(d3, "2"); // (31.0, 60.05)

        write_trace(pcb, q4p3x, q4p3y, d3p2x, d3p2y, sw, "F.Cu", NET_LED_CATH_2, "LED_CATH_2");
    }

    // ── 7. LED_CATH_3: Q5 pin3 (40.5,41) → D4 pad2 (41.0,60.05) ──
    // Via at x=41 pad left=40.65. Direct route at x=40.5 right edge=40.625.
    //   Gap=0.025mm. TOO CLOSE!
    // Fix: south at x=40.5 to y=48, jog west to x=39.5 (safe corridor
    // between vias at x=38 and x=41). Through y=49 at x=39.5. After y=49.5,
    // jog east to x=41. South to y=60.05.
    // Via x=38 pad right=38.35, trace left=39.375. Gap=1.025mm. OK.
    // Via x=41 pad left=40.65, trace right=39.625. Gap=1.025mm. OK.
    {
        let q5 = comp(components, "Q5");
        let d4 = comp(components, "D4");
        let (q5p3x, q5p3y) = ap(q5, "3"); // (40.5, 41)
        let (d4p2x, d4p2y) = ap(d4, "2"); // (41.0, 60.05)

        write_trace(pcb, q5p3x, q5p3y, q5p3x, 48.0, sw, "F.Cu", NET_LED_CATH_3, "LED_CATH_3");
        write_trace(pcb, q5p3x, 48.0, 39.5, 48.0, sw, "F.Cu", NET_LED_CATH_3, "LED_CATH_3");
        write_trace(pcb, 39.5, 48.0, 39.5, 49.5, sw, "F.Cu", NET_LED_CATH_3, "LED_CATH_3");
        write_trace(pcb, 39.5, 49.5, d4p2x, 49.5, sw, "F.Cu", NET_LED_CATH_3, "LED_CATH_3");
        write_trace(pcb, d4p2x, 49.5, d4p2x, d4p2y, sw, "F.Cu", NET_LED_CATH_3, "LED_CATH_3");
    }

    // ── 8. LED_CATH_4: Q6 pin3 (50.0,41) → D5 pad2 (51.0,60.05) ──
    // Via at x=50 pad center=50. Direct route at x=50 collides!
    // Fix: south at x=50 to y=48, jog east to x=51.5 (safe corridor
    // between vias at x=50 and x=53). Through y=49 at x=51.5. After y=49.5,
    // jog west to x=51. South to y=60.05.
    // Via x=50 pad right=50.35, trace left=51.375. Gap=1.025mm. OK.
    // Via x=53 pad left=52.65, trace right=51.625. Gap=1.025mm. OK.
    {
        let q6 = comp(components, "Q6");
        let d5 = comp(components, "D5");
        let (q6p3x, q6p3y) = ap(q6, "3"); // (50.0, 41)
        let (d5p2x, d5p2y) = ap(d5, "2"); // (51.0, 60.05)

        write_trace(pcb, q6p3x, q6p3y, q6p3x, 48.0, sw, "F.Cu", NET_LED_CATH_4, "LED_CATH_4");
        write_trace(pcb, q6p3x, 48.0, 51.5, 48.0, sw, "F.Cu", NET_LED_CATH_4, "LED_CATH_4");
        write_trace(pcb, 51.5, 48.0, 51.5, 49.5, sw, "F.Cu", NET_LED_CATH_4, "LED_CATH_4");
        write_trace(pcb, 51.5, 49.5, d5p2x, 49.5, sw, "F.Cu", NET_LED_CATH_4, "LED_CATH_4");
        write_trace(pcb, d5p2x, 49.5, d5p2x, d5p2y, sw, "F.Cu", NET_LED_CATH_4, "LED_CATH_4");
    }

    // LED_COL_7 handled by LED_COL loop (jog_x=83) — no manual route needed.

    // ═══════════════════════════════════════════════════════════════════
    // GND: U2 pad1 (48.5,14) to pad3 (48.5,15) — explicit trace to ensure
    // zone fill can reach both GND pads in the congested ADC area.
    // Route west of pad2 (<no net> at 48.5,14.5) using x=47.5 jog:
    //   pad1 (48.5,14) → west to (47.5,14) → south to (47.5,15) → east to pad3 (48.5,15).
    // Clearance: pad2 left edge=48.0, trace right edge=47.625. Gap=0.375mm. OK.
    //   ADC_AIN1 at x=47 right edge=47.125, trace left edge=47.375. Gap=0.25mm. OK.
    // ═══════════════════════════════════════════════════════════════════
    {
        let (p1x, p1y) = ap(u2, "1"); // (48.5, 14.0)
        let (p3x, p3y) = ap(u2, "3"); // (48.5, 15.0)
        write_trace(pcb, p1x, p1y, 47.5, p1y, sw, "F.Cu", NET_GND, "GND");
        write_trace(pcb, 47.5, p1y, 47.5, p3y, sw, "F.Cu", NET_GND, "GND");
        write_trace(pcb, 47.5, p3y, p3x, p3y, sw, "F.Cu", NET_GND, "GND");
    }

    // ═══════════════════════════════════════════════════════════════════
    // GND stitching vias: connect F.Cu and B.Cu ground zones.
    //
    // Zone coverage (with external heater, B.Cu is now full board):
    //   F.Cu: full board GND zone
    //   B.Cu: full board GND zone
    //
    // Avoid:
    //   - Mounting holes: H1(5,5), H2(95,5), H3(5,75), H4(95,75)
    //   - Existing signal vias on the y=3 row at x=9,41,43
    // ═══════════════════════════════════════════════════════════════════
    {
        let gnd_stitch_locations: &[(f64, f64)] = &[
            // Corner vias to prevent isolated GND islands near board edges
            (2.0, 2.0), (98.0, 2.0), (2.0, 78.0), (98.0, 78.0),
            // Top edge row: (83,2) east of +5V B.Cu and VBUS, (97,2) at far east
            (83.0, 2.0), (97.0, 2.0),
            // Island R2: LDO area (x=18-44, y=1-9) — zone fill creates
            // an isolated F.Cu GND region here with no vias to B.Cu.
            // U6 GND pad at (32.7, 5), C8/C9 caps. Need stitching via.
            // (25, 4.0): south of VBUS B.Cu at y=1.5, clear of SCL F.Cu at y=0.5,
            // and >0.25mm from board edge.
            (25.0, 4.0),
            // Island R3: USB/ESD area (x=10-18, y=4-11) — zone fill
            // creates isolated F.Cu GND near J1 and U7. Need stitching via.
            // (14,9) hits +3V3 F.Cu trace. (13,7) is clear of all traces.
            (13.0, 7.0),
            // U7 inter-pad sliver: zone fill creates tiny isolated F.Cu GND
            // polygon between U7 pin rows (x≈16.5-17.5, y≈6.7-8.6).
            // This via connects it through B.Cu GND zone.
            (17.0, 7.3),
            // Mid-electronics row (y=20)
            (93.0, 20.0),
            // y=45 row — below D12 at y=46, above R/Q pads at y=38-43.
            // Moved to (1.5,48) to avoid LED_ACT B.Cu at x=0.5 from y=39-47.
            (1.5, 48.0), (93.0, 46.0),
            // y=47 row
            (93.0, 47.0), (97.0, 47.0),
            // Former heater zone — B.Cu now available for GND stitching
            // Avoid LED_CATH verticals at Q_x, LED_COL at jog_x, MUX_Y B.Cu
            (7.0, 55.0), (46.0, 55.0), (95.0, 55.0),
            (7.0, 65.0), (46.0, 65.0), (95.0, 65.0),
            // y=74 row
            (3.0, 74.0), (50.0, 74.0), (97.0, 74.0),
            // Bottom edge (y=78)
            (50.0, 78.0),
        ];
        for &(x, y) in gnd_stitch_locations {
            write_via(pcb, x, y, VIA_PAD, VIA_DRILL, NET_GND);
        }
    }

    pcb.push('\n');
}
