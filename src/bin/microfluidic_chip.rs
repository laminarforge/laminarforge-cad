use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Microfluidic Chip Mold ───
//
// Resin-printed (Elegoo Saturn 4 Ultra 16K) NEGATIVE mold for PDMS
// casting. Raised features on this mold become channels and chambers
// in the PDMS chip after casting and peeling.
//
// Workflow:
//   1. Print this mold on resin printer
//   2. Pour PDMS over mold, degas, cure at 65C for 2h
//   3. Peel PDMS slab off mold
//   4. Bond PDMS to glass slide (oxygen plasma or corona treatment)
//   5. Connect PTFE tubing to inlet/outlet ports
//
// Chip layout (top view):
//   [Inlet Port] ── inlet channel ── [Culture Chamber] ── outlet channel ── [Outlet Port]
//
// The mold is the INVERSE: raised ridges become channels, raised
// plateau becomes the chamber cavity in the final PDMS chip.
//
// Dimensions: 50mm x 25mm (standard glass slide size)
// Channel width: 400um, depth: 150um
// Chamber: 10mm x 5mm x 150um
//
// Material: high-resolution resin (12um XY on Saturn 4 Ultra 16K)

fn main() {
    // ── Dimensions (from lib.rs constants) ──

    let length = CHIP_LENGTH; // 50mm (X)
    let width = CHIP_WIDTH; // 25mm (Y)
    let thickness = CHIP_MOLD_THICKNESS; // 5mm (Z)

    let ch_width = CHIP_CHANNEL_WIDTH; // 0.4mm
    let ch_depth = CHIP_CHANNEL_DEPTH; // 0.15mm (raised height on mold)
    let ch_length = CHIP_CHANNEL_LENGTH; // 8mm

    let chamber_length = CHIP_CHAMBER_LENGTH; // 10mm
    let chamber_width = CHIP_CHAMBER_WIDTH; // 5mm
    let chamber_depth = CHIP_CHAMBER_DEPTH; // 0.15mm

    let port_diameter = CHIP_PORT_DIAMETER; // 1.5mm
    let port_depth = CHIP_PORT_DEPTH; // 3mm

    let align_size = CHIP_ALIGNMENT_MARK_SIZE; // 1mm

    // ── Base slab ──

    let base = centered_cube("base", length, width, thickness);

    // ── Raised channel features (on top surface, Z+) ──
    // These are ADDITIVE — they stick up from the mold surface.
    // In the PDMS, they become recessed channels.

    let mold_top_z = thickness / 2.0 + ch_depth / 2.0;

    // Culture chamber (center of chip)
    let chamber = centered_cube("chamber", chamber_length, chamber_width, chamber_depth)
        .translate(0.0, 0.0, mold_top_z);

    // Inlet channel: from inlet port to chamber left edge
    // Inlet port is at -X end, channel runs along X axis (Y=0)
    let inlet_x_end = -(chamber_length / 2.0); // where channel meets chamber
    let inlet_x_start = inlet_x_end - ch_length; // channel start (port location)
    let inlet_channel = centered_cube("inlet_channel", ch_length, ch_width, ch_depth).translate(
        inlet_x_start + ch_length / 2.0,
        0.0,
        mold_top_z,
    );

    // Outlet channel: from chamber right edge to outlet port
    let outlet_x_start = chamber_length / 2.0; // where channel leaves chamber
    let outlet_x_end = outlet_x_start + ch_length;
    let outlet_channel = centered_cube("outlet_channel", ch_length, ch_width, ch_depth).translate(
        outlet_x_start + ch_length / 2.0,
        0.0,
        mold_top_z,
    );

    // ── Port holes ──
    // Vertical holes at inlet and outlet positions for tubing insertion.
    // These go INTO the mold (subtracted) so the PDMS has through-holes.

    let inlet_port = centered_cylinder(
        "inlet_port",
        port_diameter / 2.0,
        port_depth + ch_depth + 0.1,
        24,
    )
    .translate(
        inlet_x_start,
        0.0,
        thickness / 2.0 - port_depth / 2.0 + ch_depth / 2.0,
    );

    let outlet_port = centered_cylinder(
        "outlet_port",
        port_diameter / 2.0,
        port_depth + ch_depth + 0.1,
        24,
    )
    .translate(
        outlet_x_end,
        0.0,
        thickness / 2.0 - port_depth / 2.0 + ch_depth / 2.0,
    );

    // ── Alignment marks ──
    // 4 cross marks at corners for layer alignment during bonding.
    // Each cross: two perpendicular thin ridges on the top surface.

    let cross_thickness = 0.2; // mm (ridge width)
    let cross_height = 0.1; // mm (ridge height, above mold surface)
    let corner_inset = 3.0; // mm from edge

    let corner_positions: [(f64, f64); 4] = [
        (
            -(length / 2.0 - corner_inset),
            -(width / 2.0 - corner_inset),
        ),
        ((length / 2.0 - corner_inset), -(width / 2.0 - corner_inset)),
        (-(length / 2.0 - corner_inset), (width / 2.0 - corner_inset)),
        ((length / 2.0 - corner_inset), (width / 2.0 - corner_inset)),
    ];

    let cross_z = thickness / 2.0 + cross_height / 2.0;
    let mut alignment_marks = Part::empty("alignment");

    for (i, &(cx, cy)) in corner_positions.iter().enumerate() {
        // Horizontal bar of cross
        let h_bar = centered_cube(
            format!("cross_h_{i}"),
            align_size,
            cross_thickness,
            cross_height,
        )
        .translate(cx, cy, cross_z);

        // Vertical bar of cross
        let v_bar = centered_cube(
            format!("cross_v_{i}"),
            cross_thickness,
            align_size,
            cross_height,
        )
        .translate(cx, cy, cross_z);

        alignment_marks = alignment_marks + h_bar + v_bar;
    }

    // ── Assemble ──
    // Base + raised features (additive) - port holes (subtractive)

    let mold = (base + chamber + inlet_channel + outlet_channel + alignment_marks)
        - inlet_port
        - outlet_port;

    // ── Export ──

    mold.write_stl("output/microfluidic_chip.stl").unwrap();

    let total_channel_length = ch_length * 2.0 + chamber_length;
    println!("Exported: output/microfluidic_chip.stl");
    println!();
    println!("── Microfluidic Chip Mold Specs ──");
    println!("  Base:           {length:.0}mm x {width:.0}mm x {thickness:.0}mm");
    println!("  Channel width:  {ch_width:.1}mm (400um)");
    println!("  Channel depth:  {ch_depth:.2}mm (150um, raised on mold)");
    println!("  Inlet channel:  {ch_length:.0}mm long");
    println!(
        "  Culture chamber: {chamber_length:.0}mm x {chamber_width:.0}mm x {chamber_depth:.2}mm"
    );
    println!("  Outlet channel: {ch_length:.0}mm long");
    println!("  Total path:     {total_channel_length:.0}mm (inlet + chamber + outlet)");
    println!("  Port holes:     2x {port_diameter:.1}mm dia x {port_depth:.0}mm deep");
    println!("  Alignment:      4x {align_size:.0}mm cross marks at corners");
    println!("  Mold type:      NEGATIVE (raised features = channels in PDMS)");
    println!("  Printer:        Elegoo Saturn 4 Ultra 16K (12um XY)");
    println!("  Material:       high-resolution resin");
}
