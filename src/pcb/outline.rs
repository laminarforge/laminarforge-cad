use std::fmt::Write;
use crate::*;
use super::next_uuid;

pub fn write_board_outline(pcb: &mut String) {
    let w = PCB_LENGTH; // 100mm
    let h = PCB_WIDTH;  // 80mm

    // M3 mounting holes — 5mm inset so 6.35mm pad clears board edge
    let hole_inset = 5.0;
    let corners = [
        (hole_inset, hole_inset),
        (w - hole_inset, hole_inset),
        (hole_inset, h - hole_inset),
        (w - hole_inset, h - hole_inset),
    ];

    writeln!(pcb, "  (gr_line (start 0 0) (end {} 0) (layer \"Edge.Cuts\") (stroke (width 0.1) (type solid)))", w).unwrap();
    writeln!(pcb, "  (gr_line (start {} 0) (end {} {}) (layer \"Edge.Cuts\") (stroke (width 0.1) (type solid)))", w, w, h).unwrap();
    writeln!(pcb, "  (gr_line (start {} {}) (end 0 {}) (layer \"Edge.Cuts\") (stroke (width 0.1) (type solid)))", w, h, h).unwrap();
    writeln!(pcb, "  (gr_line (start 0 {}) (end 0 0) (layer \"Edge.Cuts\") (stroke (width 0.1) (type solid)))", h).unwrap();

    for (i, (x, y)) in corners.iter().enumerate() {
        writeln!(pcb, r#"  (footprint "MountingHole:MountingHole_3.2mm_M3"
    (layer "F.Cu")
    (at {} {})
    (tstamp "{}")
    (property "Reference" "H{}" (at 0 -4) (layer "F.Fab") (effects (font (size 0.8 0.8) (thickness 0.12))))
    (pad "1" thru_hole circle (at 0 0) (size 6.35 6.35) (drill 3.2) (layers "*.Cu" "*.Mask") (net 1 "GND"))
  )"#, x, y, next_uuid(), i + 1).unwrap();
    }
    pcb.push('\n');
}
