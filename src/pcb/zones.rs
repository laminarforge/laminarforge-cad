use super::nets::NET_GND;
use crate::*;
use std::fmt::Write;

pub fn write_copper_zones(pcb: &mut String) {
    // Zone 1: F.Cu ground pour — full board
    writeln!(
        pcb,
        r#"  (zone
    (net {})
    (net_name "GND")
    (layer "F.Cu")
    (uuid "a1b2c3d4-0003-0003-0003-000000000003")
    (hatch edge 0.5)
    (connect_pads yes (clearance 0.15))
    (min_thickness 0.15)
    (fill yes (thermal_gap 0.3) (thermal_bridge_width 0.3))
    (polygon
      (pts
        (xy 0 0)
        (xy {} 0)
        (xy {} {})
        (xy 0 {})
      )
    )
  )"#,
        NET_GND, PCB_LENGTH, PCB_LENGTH, PCB_WIDTH, PCB_WIDTH,
    )
    .unwrap();

    // Zone 2: B.Cu ground pour — full board
    writeln!(
        pcb,
        r#"  (zone
    (net {})
    (net_name "GND")
    (layer "B.Cu")
    (uuid "a1b2c3d4-0002-0002-0002-000000000002")
    (hatch edge 0.5)
    (connect_pads yes (clearance 0.15))
    (min_thickness 0.15)
    (fill yes (thermal_gap 0.3) (thermal_bridge_width 0.3))
    (polygon
      (pts
        (xy 0 0)
        (xy {} 0)
        (xy {} {})
        (xy 0 {})
      )
    )
  )"#,
        NET_GND, PCB_LENGTH, PCB_LENGTH, PCB_WIDTH, PCB_WIDTH,
    )
    .unwrap();

    pcb.push('\n');
}
