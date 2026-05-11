use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

use super::nets::*;
use super::Component;
use crate::*;

/// Convert mm to DSN um (micrometers).
fn mm_to_um(mm: f64) -> f64 {
    mm * 1000.0
}

/// DSN Y coordinate: negate the KiCad mm value after converting to um.
fn dsn_y(mm: f64) -> f64 {
    -(mm * 1000.0)
}

/// Normalize rotation to [-180, 180) range.
fn dsn_rotation(deg: f64) -> f64 {
    let mut r = deg % 360.0;
    if r >= 180.0 {
        r -= 360.0;
    } else if r < -180.0 {
        r += 360.0;
    }
    r
}

/// Footprint key for grouping: "lib:name"
fn footprint_key(comp: &Component) -> String {
    format!("{}:{}", comp.footprint_lib, comp.footprint_name)
}

/// Generate a padstack name for a pad.
fn padstack_name(pad: &super::Pad) -> String {
    let w_um = mm_to_um(pad.width);
    let h_um = mm_to_um(pad.height);
    if pad.pad_type == "thru_hole" || pad.drill.is_some() {
        // Through-hole: use circle padstack (diameter = max of width, height)
        let dia = w_um.max(h_um);
        format!("Round[A]Pad_{:.0}_um", dia)
    } else {
        // SMD: rect padstack
        format!("Rect[T]Pad_{:.0}x{:.0}_um", w_um, h_um)
    }
}

/// Write the (parser ...) section.
fn write_parser(s: &mut String) {
    writeln!(s, "  (parser").unwrap();
    writeln!(s, "    (string_quote \")").unwrap();
    writeln!(s, "    (space_in_quoted_tokens on)").unwrap();
    writeln!(s, "    (host_cad \"LaminarForge\")").unwrap();
    writeln!(s, "    (host_version \"1.0\")").unwrap();
    writeln!(s, "  )").unwrap();
}

/// Write the (structure ...) section.
fn write_structure(s: &mut String) {
    let board_x = mm_to_um(PCB_LENGTH); // 100000
    let board_y_neg = dsn_y(PCB_WIDTH); // -80000

    writeln!(s, "  (structure").unwrap();

    // Layers
    writeln!(s, "    (layer F.Cu (type signal) (property (index 0)))").unwrap();
    writeln!(s, "    (layer B.Cu (type signal) (property (index 1)))").unwrap();

    // Board boundary: rectangle (0,0) to (100000, -80000)
    writeln!(
        s,
        "    (boundary (path pcb 0  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}))",
        0.0, 0.0,
        board_x, 0.0,
        board_x, board_y_neg,
        0.0, board_y_neg,
        0.0, 0.0,
    ).unwrap();

    // GND planes (polygons must be closed — first point = last point)
    // F.Cu: full board
    writeln!(
        s,
        "    (plane GND (polygon F.Cu 0  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}))",
        0.0, 0.0,
        board_x, 0.0,
        board_x, board_y_neg,
        0.0, board_y_neg,
        0.0, 0.0,
    ).unwrap();

    // B.Cu: full board
    writeln!(
        s,
        "    (plane GND (polygon B.Cu 0  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}  {:.0} {:.0}))",
        0.0, 0.0,
        board_x, 0.0,
        board_x, board_y_neg,
        0.0, board_y_neg,
        0.0, 0.0,
    ).unwrap();

    // Keepout zones around mounting holes — prevent Freerouting from
    // placing traces/vias near the M3 hole + pad (6.35mm pad + clearance).
    for &(_, x_mm, y_mm) in MOUNT_HOLES {
        let x_um = mm_to_um(x_mm);
        let y_um = dsn_y(y_mm);
        // Keepout radius: 6.35mm pad diameter / 2 + 0.3mm clearance = 3.475mm = 3475um
        writeln!(
            s,
            "    (keepout \"\" (circle signal 6950 {:.0} {:.0}))",
            x_um, y_um,
        )
        .unwrap();
    }

    // Via definition
    writeln!(s, "    (via \"Via[0-1]_600:300_um\")").unwrap();

    // Edge keepouts — 1mm strip along all 4 board edges to prevent
    // routing too close to the edge (JLCPCB requires 0.25mm min).
    let edge_w = mm_to_um(1.0); // 1mm keepout width
                                // Top edge
    writeln!(
        s,
        "    (keepout \"\" (rect signal 0 0 {:.0} {:.0}))",
        board_x, -edge_w
    )
    .unwrap();
    // Bottom edge
    writeln!(
        s,
        "    (keepout \"\" (rect signal 0 {:.0} {:.0} {:.0}))",
        board_y_neg + edge_w,
        board_x,
        board_y_neg
    )
    .unwrap();
    // Left edge
    writeln!(
        s,
        "    (keepout \"\" (rect signal 0 0 {:.0} {:.0}))",
        edge_w, board_y_neg
    )
    .unwrap();
    // Right edge
    writeln!(
        s,
        "    (keepout \"\" (rect signal {:.0} 0 {:.0} {:.0}))",
        board_x - edge_w,
        board_x,
        board_y_neg
    )
    .unwrap();

    // Default rules — clearance 230um gives margin above KiCad's 127um rule
    writeln!(s, "    (rule").unwrap();
    writeln!(s, "      (width 200)").unwrap();
    writeln!(s, "      (clearance 230)").unwrap();
    writeln!(s, "    )").unwrap();

    writeln!(s, "  )").unwrap();
}

/// Mounting hole positions (5mm inset from corners on 100x80mm board).
const MOUNT_HOLES: &[(&str, f64, f64)] = &[
    ("H1", 5.0, 5.0),
    ("H2", 95.0, 5.0),
    ("H3", 5.0, 75.0),
    ("H4", 95.0, 75.0),
];

/// Write the (placement ...) section.
/// Components with the same footprint are grouped under one (component ...) block.
fn write_placement(s: &mut String, components: &[Component]) {
    // Group by footprint key
    let mut groups: BTreeMap<String, Vec<&Component>> = BTreeMap::new();
    for comp in components {
        groups.entry(footprint_key(comp)).or_default().push(comp);
    }

    writeln!(s, "  (placement").unwrap();

    // Mounting holes
    writeln!(s, "    (component MountingHole:MountingHole_3.2mm_M3").unwrap();
    for &(ref_name, x_mm, y_mm) in MOUNT_HOLES {
        writeln!(
            s,
            "      (place {} {:.0} {:.0} front 0.0)",
            ref_name,
            mm_to_um(x_mm),
            dsn_y(y_mm),
        )
        .unwrap();
    }
    writeln!(s, "    )").unwrap();

    for (fp_key, comps) in &groups {
        writeln!(s, "    (component {}", fp_key).unwrap();
        for comp in comps {
            let x_um = mm_to_um(comp.x);
            let y_um = dsn_y(comp.y);
            let rot = dsn_rotation(comp.rotation);
            let side = if comp.layer.contains("B.Cu") {
                "back"
            } else {
                "front"
            };
            writeln!(
                s,
                "      (place {} {:.0} {:.0} {} {:.1} (PN {}))",
                comp.reference, x_um, y_um, side, rot, comp.value,
            )
            .unwrap();
        }
        writeln!(s, "    )").unwrap();
    }
    writeln!(s, "  )").unwrap();
}

/// Write the (library ...) section with image definitions and padstack definitions.
fn write_library(s: &mut String, components: &[Component]) {
    writeln!(s, "  (library").unwrap();

    // Mounting hole image (6.35mm TH circle pad, both layers)
    writeln!(s, "    (image MountingHole:MountingHole_3.2mm_M3").unwrap();
    writeln!(s, "      (pin Round[A]Pad_6350_um 1 0 0)").unwrap();
    writeln!(s, "    )").unwrap();

    // Collect unique footprints
    let mut seen_fp: BTreeSet<String> = BTreeSet::new();
    // Also collect unique padstacks
    let mut padstacks: BTreeMap<String, PadstackDef> = BTreeMap::new();

    // Pre-register the mounting hole padstack
    padstacks.insert(
        "Round[A]Pad_6350_um".to_string(),
        PadstackDef {
            width_um: 6350.0,
            height_um: 6350.0,
            is_thru: true,
        },
    );

    for comp in components {
        let key = footprint_key(comp);
        if seen_fp.contains(&key) {
            continue;
        }
        seen_fp.insert(key.clone());

        writeln!(s, "    (image {}", key).unwrap();
        for pad in &comp.pads {
            let ps_name = padstack_name(pad);
            let px_um = mm_to_um(pad.x);
            let py_um = dsn_y(pad.y);
            writeln!(
                s,
                "      (pin {} {} {:.0} {:.0})",
                ps_name, pad.number, px_um, py_um,
            )
            .unwrap();

            // Record padstack definition
            padstacks.entry(ps_name).or_insert_with(|| {
                let w_um = mm_to_um(pad.width);
                let h_um = mm_to_um(pad.height);
                let is_thru = pad.pad_type == "thru_hole" || pad.drill.is_some();
                PadstackDef {
                    width_um: w_um,
                    height_um: h_um,
                    is_thru,
                }
            });
        }
        writeln!(s, "    )").unwrap();
    }

    // Write padstack definitions
    for (name, def) in &padstacks {
        writeln!(s, "    (padstack {}", name).unwrap();
        if def.is_thru {
            // Through-hole: circle on both layers
            let dia = def.width_um.max(def.height_um);
            writeln!(s, "      (shape (circle F.Cu {:.0}))", dia).unwrap();
            writeln!(s, "      (shape (circle B.Cu {:.0}))", dia).unwrap();
        } else {
            // SMD: rect on F.Cu only
            let hw = def.width_um / 2.0;
            let hh = def.height_um / 2.0;
            writeln!(
                s,
                "      (shape (rect F.Cu {:.0} {:.0} {:.0} {:.0}))",
                -hw, -hh, hw, hh,
            )
            .unwrap();
        }
        writeln!(s, "      (attach off)").unwrap();
        writeln!(s, "    )").unwrap();
    }

    // Via padstack
    writeln!(s, "    (padstack \"Via[0-1]_600:300_um\"").unwrap();
    writeln!(s, "      (shape (circle F.Cu 600))").unwrap();
    writeln!(s, "      (shape (circle B.Cu 600))").unwrap();
    writeln!(s, "      (attach off)").unwrap();
    writeln!(s, "    )").unwrap();

    writeln!(s, "  )").unwrap();
}

struct PadstackDef {
    width_um: f64,
    height_um: f64,
    is_thru: bool,
}

/// Write the (network ...) section.
/// Groups pins by net_id. Skip net 0 (unconnected).
fn write_network(s: &mut String, components: &[Component]) {
    // Build net -> list of "REF-PIN" strings
    let mut net_pins: BTreeMap<u32, Vec<String>> = BTreeMap::new();

    // Add mounting hole pins to GND net
    for &(ref_name, _, _) in MOUNT_HOLES {
        net_pins
            .entry(NET_GND)
            .or_default()
            .push(format!("{}-1", ref_name));
    }

    for comp in components {
        for pad in &comp.pads {
            if pad.net_id == NET_UNCONNECTED {
                continue;
            }
            net_pins
                .entry(pad.net_id)
                .or_default()
                .push(format!("{}-{}", comp.reference, pad.number));
        }
    }

    writeln!(s, "  (network").unwrap();

    // Collect all net names for the class declaration
    let mut all_net_names: Vec<String> = Vec::new();

    for (&net_id, pins) in &net_pins {
        let name = net_name_for_id(net_id);
        if name.is_empty() {
            continue;
        }
        all_net_names.push(name.to_string());

        write!(s, "    (net {}\n      (pins", name).unwrap();
        for pin in pins {
            write!(s, " {}", pin).unwrap();
        }
        writeln!(s, ")").unwrap();
        writeln!(s, "    )").unwrap();
    }

    // Class definition
    write!(s, "    (class kicad_default").unwrap();
    for name in &all_net_names {
        write!(s, " {}", name).unwrap();
    }
    writeln!(s).unwrap();
    writeln!(s, "      (circuit").unwrap();
    writeln!(s, "        (use_via \"Via[0-1]_600:300_um\")").unwrap();
    writeln!(s, "      )").unwrap();
    writeln!(s, "      (rule").unwrap();
    writeln!(s, "        (width 200)").unwrap();
    writeln!(s, "        (clearance 230)").unwrap();
    writeln!(s, "      )").unwrap();
    writeln!(s, "    )").unwrap();

    writeln!(s, "  )").unwrap();
}

/// Write the (wiring ...) section. Empty — Freerouting handles all routing.
fn write_wiring(s: &mut String) {
    writeln!(s, "  (wiring").unwrap();
    writeln!(s, "  )").unwrap();
}

/// Export a Specctra DSN file for Freerouting autorouter.
pub fn write_dsn(path: &Path, components: &[Component]) {
    let mut s = String::with_capacity(256 * 1024);

    // Use the file name from the path as the pcb identifier
    let pcb_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "board.dsn".to_string());

    writeln!(s, "(pcb {}", pcb_name).unwrap();

    write_parser(&mut s);

    writeln!(s, "  (resolution um 10)").unwrap();
    writeln!(s, "  (unit um)").unwrap();

    write_structure(&mut s);
    write_placement(&mut s, components);
    write_library(&mut s, components);
    write_network(&mut s, components);
    write_wiring(&mut s);

    writeln!(s, ")").unwrap();

    std::fs::write(path, &s).expect("write .dsn");
}
