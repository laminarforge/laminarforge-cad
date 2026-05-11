use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Workstation Enclosure (3-Zone Desk Enclosure) ───
//
// 3D-printed (PETG) frame segments for a 3-zone partitioned desk enclosure.
// Same construction method as the still air box: printed frame rails with
// panel channel slots for laser-cut acrylic panels.
//
// The full enclosure is 1800×700×600mm — far too large to print as one piece.
// This model generates:
// 1. A single representative rail segment (≤240mm, the max printable length)
// 2. A 3-way corner connector
//
// The frame is divided into three zones by two internal partition rails:
// - Zone 1 (500mm): Sterile work area with arm holes in front panel
// - Zone 2 (700mm): Equipment zone (heating blocks, optics, dispensing)
// - Zone 3 (560mm): 3D printer zone with exhaust port in back panel
//
// Rail cross-section: 20×20mm with 3.5×5mm panel channel slot
// Acrylic panels: 3mm thick, slide into channel slots
//
// Exports:
// - output/workstation_rail_segment.stl (single 240mm segment)
// - output/workstation_corner.stl (3-way corner connector)

/// Alignment pin dimensions (same as SAB rails)
const PIN_DIAMETER: f64 = 4.0;
const PIN_SOCKET_DIAMETER: f64 = 4.3;
const PIN_DEPTH: f64 = 10.0;
const PINS_PER_JOINT: usize = 2;
const PIN_OFFSET: f64 = 5.0;

fn main() {
    // ── Rail segment (240mm max printable length) ──
    //
    // A generic rail segment with:
    // - Panel channel slot on one face (+Y)
    // - Alignment pin on +X end
    // - Alignment socket on -X end

    let seg_length = WS_MAX_PRINT_LENGTH; // 240mm
    let rail_w = WS_RAIL_SIZE; // 20mm
    let rail_h = WS_RAIL_SIZE; // 20mm

    let body = centered_cube("seg_body", seg_length, rail_w, rail_h);

    // Panel channel slot on +Y face (acrylic slides in from the side)
    let channel = centered_cube(
        "channel",
        seg_length + 2.0,
        WS_CHANNEL_DEPTH,
        WS_CHANNEL_WIDTH,
    )
    .translate(0.0, rail_w / 2.0 - WS_CHANNEL_DEPTH / 2.0 + 0.1, 0.0);

    let mut segment = body - channel;

    // Alignment sockets on -X end
    for pin_idx in 0..PINS_PER_JOINT {
        let z_offset = if pin_idx == 0 {
            -PIN_OFFSET
        } else {
            PIN_OFFSET
        };
        let socket = centered_cylinder(
            &format!("socket_l_{pin_idx}"),
            PIN_SOCKET_DIAMETER / 2.0,
            PIN_DEPTH + 0.5,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-seg_length / 2.0 + PIN_DEPTH / 2.0 - 0.25, 0.0, z_offset);
        segment = segment - socket;
    }

    // Alignment pins on +X end
    for pin_idx in 0..PINS_PER_JOINT {
        let z_offset = if pin_idx == 0 {
            -PIN_OFFSET
        } else {
            PIN_OFFSET
        };
        let pin = centered_cylinder(
            &format!("pin_r_{pin_idx}"),
            PIN_DIAMETER / 2.0,
            PIN_DEPTH,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(seg_length / 2.0 + PIN_DEPTH / 2.0, 0.0, z_offset);
        segment = segment + pin;
    }

    segment
        .write_stl("output/workstation_rail_segment.stl")
        .unwrap();

    // ── Corner connector ──
    //
    // 3-way joint where three perpendicular rails meet.
    // Body is a cube (WS_RAIL_SIZE^3 = 20×20×20mm).
    // Three square sockets accept rail ends (slide-in fit).
    // Panel channel slots continue through the corner on two faces
    // so acrylic panels have a continuous slot.
    // M3 set screw holes lock rails in place.

    let body_size = WS_RAIL_SIZE;
    let socket_size = WS_RAIL_SIZE - 2.0; // 18mm, snug fit
    let socket_depth = 15.0;
    let set_screw_d = 3.2; // M3 clearance
    let set_screw_len = body_size + 2.0;

    let corner_body = centered_cube("corner_body", body_size, body_size, body_size);

    // +X socket
    let socket_x = centered_cube("socket_x", socket_depth + 0.1, socket_size, socket_size)
        .translate(body_size / 2.0 - socket_depth / 2.0 + 0.05, 0.0, 0.0);

    // +Y socket
    let socket_y = centered_cube("socket_y", socket_size, socket_depth + 0.1, socket_size)
        .translate(0.0, body_size / 2.0 - socket_depth / 2.0 + 0.05, 0.0);

    // +Z socket
    let socket_z = centered_cube("socket_z", socket_size, socket_size, socket_depth + 0.1)
        .translate(0.0, 0.0, body_size / 2.0 - socket_depth / 2.0 + 0.05);

    // Panel channel continuations through corner (two perpendicular faces)
    // +Y face channel (continues the horizontal panel slot)
    let corner_channel_y = centered_cube(
        "corner_ch_y",
        body_size + 2.0,
        WS_CHANNEL_DEPTH,
        WS_CHANNEL_WIDTH,
    )
    .translate(0.0, body_size / 2.0 - WS_CHANNEL_DEPTH / 2.0 + 0.1, 0.0);

    // +Z face channel (continues the vertical panel slot)
    let corner_channel_z = centered_cube(
        "corner_ch_z",
        body_size + 2.0,
        WS_CHANNEL_WIDTH,
        WS_CHANNEL_DEPTH,
    )
    .translate(0.0, 0.0, body_size / 2.0 - WS_CHANNEL_DEPTH / 2.0 + 0.1);

    // Set screw holes (one per socket, perpendicular to rail axis)
    let screw_center = body_size / 2.0 - socket_depth / 2.0;

    // Set screw for X socket: enters from +Y face
    let screw_x = centered_cylinder("screw_x", set_screw_d / 2.0, set_screw_len, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(screw_center, 0.0, 0.0);

    // Set screw for Y socket: enters from +X face
    let screw_y = centered_cylinder("screw_y", set_screw_d / 2.0, set_screw_len, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, screw_center, 0.0);

    // Set screw for Z socket: enters from +X face
    let screw_z = centered_cylinder("screw_z", set_screw_d / 2.0, set_screw_len, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, screw_center);

    let corner = corner_body
        - socket_x
        - socket_y
        - socket_z
        - corner_channel_y
        - corner_channel_z
        - screw_x
        - screw_y
        - screw_z;

    corner.write_stl("output/workstation_corner.stl").unwrap();

    // ── Segment count calculations ──

    let width_segs = (WS_TOTAL_WIDTH / WS_MAX_PRINT_LENGTH).ceil() as usize;
    let depth_segs = (WS_DEPTH / WS_MAX_PRINT_LENGTH).ceil() as usize;
    let height_segs = (WS_HEIGHT / WS_MAX_PRINT_LENGTH).ceil() as usize;

    // Partition rails (2× vertical partitions, each is depth × height)
    // Each partition has 4 vertical rails + 4 depth rails = 8 rails
    // Plus the partition acrylic panel

    // Frame edges: 4 width + 4 depth + 4 height = 12 edges on outer frame
    // Plus 2 partition frames (4 vertical + 2 top/bottom depth = 6 rails each)
    let outer_width_rails = 4_usize; // top/bottom front/back
    let outer_depth_rails = 4_usize;
    let outer_height_rails = 4_usize;
    let partition_depth_rails = 2 * 2_usize; // 2 partitions × 2 depth rails (top + bottom)
    let partition_height_rails = 2 * 2_usize; // 2 partitions × 2 vertical rails (front + back)

    let total_width_segments = outer_width_rails * width_segs;
    let total_depth_segments = (outer_depth_rails + partition_depth_rails) * depth_segs;
    let total_height_segments = (outer_height_rails + partition_height_rails) * height_segs;
    let total_segments = total_width_segments + total_depth_segments + total_height_segments;

    // Corners: 8 outer + 4 per partition × 2 = 16 total
    let outer_corners = 8_usize;
    let partition_corners = 2 * 4_usize;
    let total_corners = outer_corners + partition_corners;

    println!("Exported: output/workstation_rail_segment.stl");
    println!("Exported: output/workstation_corner.stl");
    println!();
    println!("── Workstation Enclosure Specs ──");
    println!("  Overall:          {WS_TOTAL_WIDTH:.0}mm x {WS_DEPTH:.0}mm x {WS_HEIGHT:.0}mm");
    println!("  Zone 1 (sterile): {WS_ZONE1_WIDTH:.0}mm wide (arm holes in front panel)");
    println!("  Zone 2 (equip):   {WS_ZONE2_WIDTH:.0}mm wide");
    println!(
        "  Zone 3 (printer): {WS_ZONE3_WIDTH:.0}mm wide (exhaust: {WS_EXHAUST_DIAMETER:.0}mm port)"
    );
    println!("  Partitions:       2x {WS_PARTITION_THICKNESS:.0}mm acrylic dividers");
    println!();
    println!("── Rail Specs ──");
    println!("  Cross-section:    {rail_w:.0}mm x {rail_h:.0}mm");
    println!("  Panel channel:    {WS_CHANNEL_WIDTH:.1}mm x {WS_CHANNEL_DEPTH:.0}mm slot");
    println!("  Panel thickness:  {WS_PANEL_THICKNESS:.0}mm acrylic");
    println!("  Max segment:      {seg_length:.0}mm");
    println!("  Alignment pins:   {PIN_DIAMETER:.0}mm dia, {PIN_DEPTH:.0}mm deep ({PINS_PER_JOINT} per joint)");
    println!();
    println!("── Frame BOM ──");
    println!("  Corner connectors: {total_corners}x ({outer_corners} outer + {partition_corners} partition)");
    println!("  Width segments:    {total_width_segments}x ({outer_width_rails} rails x {width_segs} segs)");
    println!(
        "  Depth segments:    {total_depth_segments}x ({} rails x {depth_segs} segs)",
        outer_depth_rails + partition_depth_rails
    );
    println!(
        "  Height segments:   {total_height_segments}x ({} rails x {height_segs} segs)",
        outer_height_rails + partition_height_rails
    );
    println!("  Total prints:      {total_segments} segments + {total_corners} corners");
    println!("  Door height:       {WS_DOOR_HEIGHT:.0}mm (clearance: {WS_DOOR_CLEARANCE:.0}mm)");
    println!("  Material:          PETG");
}
