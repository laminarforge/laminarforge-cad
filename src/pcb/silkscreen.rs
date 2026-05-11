use crate::*;
use std::fmt::Write;

pub fn write_silkscreen(pcb: &mut String) {
    writeln!(
        pcb,
        r#"  (gr_text "LaminarForge LAMP v1" (at 50 78) (layer "F.SilkS")
    (effects (font (size 1.5 1.5) (thickness 0.2)) (justify left))
  )"#
    )
    .unwrap();

    writeln!(
        pcb,
        r#"  (gr_text "CERN-OHL-S v2" (at 50 76) (layer "F.SilkS")
    (effects (font (size 0.8 0.8) (thickness 0.12)) (justify left))
  )"#
    )
    .unwrap();

    let hx_start = HEATER_ZONE_X_START;
    let hy_top = PCB_WIDTH - HEATER_ZONE_Y_START - HEATER_ZONE_WIDTH;
    let hy_bottom = PCB_WIDTH - HEATER_ZONE_Y_START;
    let slot_y = hy_top - 2.0;
    let first_slot_x =
        hx_start + (HEATER_ZONE_LENGTH - (NUM_SLOTS as f64 - 1.0) * SLOT_SPACING) / 2.0;

    for i in 0..NUM_SLOTS {
        let x = first_slot_x + i as f64 * SLOT_SPACING;
        writeln!(
            pcb,
            r#"  (gr_text "{}" (at {} {}) (layer "F.SilkS")
    (effects (font (size 0.8 0.8) (thickness 0.12)))
  )"#,
            i + 1,
            x,
            slot_y
        )
        .unwrap();
    }

    let hx_end = hx_start + HEATER_ZONE_LENGTH;
    writeln!(pcb, "  (gr_rect (start {} {}) (end {} {}) (layer \"F.SilkS\") (stroke (width 0.15) (type solid)))",
        hx_start, hy_top, hx_end, hy_bottom).unwrap();

    pcb.push('\n');
}
