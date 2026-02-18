use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Still Air Box Rail Segments ───
//
// 3D-printed (PETG) rail segments that connect corner connectors to form
// the still air box frame. Each rail has:
//
// - Cross-section: SAB_RAIL_WIDTH × SAB_RAIL_HEIGHT (20×20mm)
// - Panel channel slot on one face (3.5×5mm) for acrylic panel insertion
// - Split into printable segments ≤240mm (Bambu A1 bed: 256mm)
// - Alignment pins/sockets at split points for precise assembly
//
// Three rail lengths are exported:
// - 400mm (SAB_INNER_WIDTH): 2 segments (200mm each)
// - 300mm (SAB_INNER_DEPTH): 2 segments (150mm each)
// - 300mm vertical: same as depth but for vertical rails
//
// Rails slide into corner connector sockets and are locked with M3 set screws.

/// Maximum printable segment length
const MAX_SEGMENT_LENGTH: f64 = 240.0;

/// Alignment pin dimensions
const PIN_DIAMETER: f64 = 4.0;
const PIN_SOCKET_DIAMETER: f64 = 4.3; // 0.3mm clearance for fit
const PIN_DEPTH: f64 = 10.0;

/// Number of alignment pins per joint
const PINS_PER_JOINT: usize = 2;

/// Pin offset from rail center (positioned symmetrically)
const PIN_OFFSET: f64 = 5.0;

fn build_rail(name: &str, total_length: f64) -> Vec<(String, Part)> {
    let rail_w = SAB_RAIL_WIDTH;   // 20mm
    let rail_h = SAB_RAIL_HEIGHT;  // 20mm

    // Determine number of segments
    let num_segments = (total_length / MAX_SEGMENT_LENGTH).ceil() as usize;
    let segment_length = total_length / num_segments as f64;

    let mut segments: Vec<(String, Part)> = Vec::new();

    for seg_idx in 0..num_segments {
        let seg_name = if num_segments == 1 {
            format!("{name}")
        } else {
            format!("{name}_seg{}", seg_idx + 1)
        };

        // Base rail segment
        let body = centered_cube("body", segment_length, rail_w, rail_h);

        // Panel channel slot along one face (+Y face, centered in Z)
        // Channel runs the full length of the segment, cut from face inward
        let channel_cut = centered_cube(
            "channel_cut",
            segment_length + 2.0,
            SAB_CHANNEL_DEPTH,
            SAB_CHANNEL_WIDTH,
        )
        .translate(
            0.0,
            rail_w / 2.0 - SAB_CHANNEL_DEPTH / 2.0 + 0.1,
            0.0,
        );

        let mut segment = body - channel_cut;

        // ── Alignment features at split joints ──

        // Left end (-X): socket if not the first segment
        if seg_idx > 0 {
            for pin_idx in 0..PINS_PER_JOINT {
                let z_offset = if pin_idx == 0 { -PIN_OFFSET } else { PIN_OFFSET };
                let socket = centered_cylinder(
                    &format!("socket_l_{pin_idx}"),
                    PIN_SOCKET_DIAMETER / 2.0,
                    PIN_DEPTH + 0.5,
                    24,
                )
                .rotate(0.0, 90.0, 0.0) // align along X axis
                .translate(-segment_length / 2.0 + PIN_DEPTH / 2.0 - 0.25, 0.0, z_offset);

                segment = segment - socket;
            }
        }

        // Right end (+X): pin if not the last segment
        if seg_idx < num_segments - 1 {
            for pin_idx in 0..PINS_PER_JOINT {
                let z_offset = if pin_idx == 0 { -PIN_OFFSET } else { PIN_OFFSET };
                let pin = centered_cylinder(
                    &format!("pin_r_{pin_idx}"),
                    PIN_DIAMETER / 2.0,
                    PIN_DEPTH,
                    24,
                )
                .rotate(0.0, 90.0, 0.0) // align along X axis
                .translate(segment_length / 2.0 + PIN_DEPTH / 2.0, 0.0, z_offset);

                segment = segment + pin;
            }
        }

        segments.push((seg_name, segment));
    }

    segments
}

fn main() {
    println!("── Still Air Box Rail Segments ──");
    println!();

    // ── 400mm rails (SAB_INNER_WIDTH) ──
    let rail_400_segments = build_rail("rail_400", SAB_INNER_WIDTH);
    let num_400 = rail_400_segments.len();
    let seg_len_400 = SAB_INNER_WIDTH / num_400 as f64;

    // Union all segments at their final positions for the full rail export
    let mut rail_400 = Part::empty("rail_400_full");
    for (i, (_, seg)) in rail_400_segments.iter().enumerate() {
        let offset_x = -(SAB_INNER_WIDTH / 2.0) + seg_len_400 / 2.0 + (i as f64) * seg_len_400;
        rail_400 = rail_400 + seg.translate(offset_x, 0.0, 0.0);
    }
    rail_400
        .write_stl("output/still_air_box_rail_400.stl")
        .unwrap();
    println!("Exported: output/still_air_box_rail_400.stl");
    println!("  Total length:     {:.0}mm", SAB_INNER_WIDTH);
    println!("  Segments:         {num_400}× {seg_len_400:.0}mm each");
    println!("  Cross-section:    {:.0}mm × {:.0}mm", SAB_RAIL_WIDTH, SAB_RAIL_HEIGHT);
    println!("  Panel channel:    {:.1}mm × {:.0}mm slot", SAB_CHANNEL_WIDTH, SAB_CHANNEL_DEPTH);
    println!("  Alignment pins:   {PIN_DIAMETER:.0}mm dia, {PIN_DEPTH:.0}mm deep");
    println!("  Alignment sockets: {PIN_SOCKET_DIAMETER:.1}mm dia (0.3mm clearance)");
    println!("  Pins per joint:   {PINS_PER_JOINT}");
    println!();

    // ── 300mm depth rails (SAB_INNER_DEPTH) ──
    let rail_300_segments = build_rail("rail_300", SAB_INNER_DEPTH);
    let num_300 = rail_300_segments.len();
    let seg_len_300 = SAB_INNER_DEPTH / num_300 as f64;

    let mut rail_300 = Part::empty("rail_300_full");
    for (i, (_, seg)) in rail_300_segments.iter().enumerate() {
        let offset_x = -(SAB_INNER_DEPTH / 2.0) + seg_len_300 / 2.0 + (i as f64) * seg_len_300;
        rail_300 = rail_300 + seg.translate(offset_x, 0.0, 0.0);
    }
    rail_300
        .write_stl("output/still_air_box_rail_300.stl")
        .unwrap();
    println!("Exported: output/still_air_box_rail_300.stl");
    println!("  Total length:     {:.0}mm", SAB_INNER_DEPTH);
    println!("  Segments:         {num_300}× {seg_len_300:.0}mm each");
    println!();

    // ── 300mm vertical rails (same length, exported separately) ──
    // Vertical rails are identical geometry to depth rails but are
    // installed vertically. Exported separately for BOM clarity.
    let rail_vert_segments = build_rail("rail_300v", SAB_INNER_HEIGHT);
    let num_vert = rail_vert_segments.len();
    let seg_len_vert = SAB_INNER_HEIGHT / num_vert as f64;

    let mut rail_vert = Part::empty("rail_300v_full");
    for (i, (_, seg)) in rail_vert_segments.iter().enumerate() {
        let offset_x = -(SAB_INNER_HEIGHT / 2.0) + seg_len_vert / 2.0 + (i as f64) * seg_len_vert;
        rail_vert = rail_vert + seg.translate(offset_x, 0.0, 0.0);
    }
    rail_vert
        .write_stl("output/still_air_box_rail_300_vert.stl")
        .unwrap();
    println!("Exported: output/still_air_box_rail_300_vert.stl");
    println!("  Total length:     {:.0}mm (vertical)", SAB_INNER_HEIGHT);
    println!("  Segments:         {num_vert}× {seg_len_vert:.0}mm each");
    println!();

    // ── Summary ──
    println!("── SAB Frame BOM ──");
    println!("  Corner connectors: 8×");
    println!("  400mm rails:       4× ({num_400} segments each = {} prints)", num_400 * 4);
    println!("  300mm depth rails: 4× ({num_300} segments each = {} prints)", num_300 * 4);
    println!("  300mm vert rails:  4× ({num_vert} segments each = {} prints)", num_vert * 4);
    println!("  Material:          PETG");
}
