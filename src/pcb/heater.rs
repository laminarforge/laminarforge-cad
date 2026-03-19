use std::fmt::Write;
use crate::*;
use super::{HeaterStats, next_uuid};
use super::nets::NET_HEATER_P;

pub fn write_heater_serpentine(pcb: &mut String) -> HeaterStats {
    // Heater zone on B.Cu: serpentine trace
    // KiCad coords: Y increases downward. Board bottom = Y=80.
    // Heater zone: Y from 50mm to 72mm (22mm tall)
    let x_start = HEATER_ZONE_X_START; // 8mm
    let x_end = x_start + HEATER_ZONE_LENGTH; // 92mm
    let y_top = PCB_WIDTH - HEATER_ZONE_Y_START - HEATER_ZONE_WIDTH; // 50mm
    let y_bottom = PCB_WIDTH - HEATER_ZONE_Y_START; // 72mm

    let trace_w = HEATER_TRACE_WIDTH;
    let pitch = HEATER_TRACE_PITCH; // 0.4mm

    let zone_height = y_bottom - y_top; // 22mm
    let num_passes = ((zone_height - trace_w) / pitch).floor() as usize + 1;

    let x_inset = 0.5;
    let trace_x_start = x_start + x_inset;
    let trace_x_end = x_end - x_inset;
    let trace_length = trace_x_end - trace_x_start;

    let mut total_length = 0.0;

    // ALL passes use HEATER_P — single net for entire serpentine
    for i in 0..num_passes {
        let y = y_top + trace_w / 2.0 + i as f64 * pitch;

        let (sx, ex) = if i % 2 == 0 {
            (trace_x_start, trace_x_end)
        } else {
            (trace_x_end, trace_x_start)
        };

        writeln!(
            pcb,
            "  (segment (start {} {}) (end {} {}) (width {}) (layer \"B.Cu\") (net {}) (tstamp \"{}\"))",
            sx, y, ex, y, trace_w, NET_HEATER_P, next_uuid()
        ).unwrap();
        total_length += trace_length;

        if i < num_passes - 1 {
            let y_next = y + pitch;
            let conn_x = if i % 2 == 0 { trace_x_end } else { trace_x_start };
            writeln!(
                pcb,
                "  (segment (start {} {}) (end {} {}) (width {}) (layer \"B.Cu\") (net {}) (tstamp \"{}\"))",
                conn_x, y, conn_x, y_next, trace_w, NET_HEATER_P, next_uuid()
            ).unwrap();
            total_length += pitch;
        }
    }

    let length_m = total_length / 1000.0;
    let resistance = COPPER_RESISTIVITY * length_m / (COPPER_THICKNESS_M * (trace_w / 1000.0));

    pcb.push('\n');

    HeaterStats {
        passes: num_passes,
        length_mm: total_length,
        resistance,
    }
}
