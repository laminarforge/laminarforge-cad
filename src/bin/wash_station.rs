use vcad::{centered_cube, centered_cylinder, Part};

// ─── Wash Station Body ───
//
// 3D-printed (PETG) wash station for the liquid handler's fixed-tip
// wash protocol. Three positions: bleach well, DI rinse well, blot pad.
//
// Ticket: T-69DCDE7F
// Spec: 80mm × 120mm footprint, 2× narrow wells (3mm ID, 20mm deep),
//       1× blot pad holder, gravity drain to waste bottle.
//
// Wash protocol (from gap analysis A-4AC88482):
//   1. Bleach well: aspirate/expel ×3, hold 5s each (kills carryover)
//   2. DI rinse well: aspirate/expel ×3 (removes bleach residue)
//   3. Blot pad: touch tip 1s (removes hanging droplet)
//
// Each well has:
//   - Fluid inlet at bottom (1.5mm dia for peristaltic pump tubing)
//   - Overflow port near top rim → routes to common drain channel
//   - Continuous flow during wash cycle keeps fluid fresh
//
// Material: PETG (or HDPE for better chemical resistance)

fn main() {
    // ── Dimensions ──

    let body_length = 120.0; // X
    let body_width = 80.0; // Y
    let body_height = 30.0; // Z (tall enough to contain splashing)
    let wall = 3.0; // mm wall thickness
    let floor = 3.0; // mm floor thickness

    // ── Well dimensions ──
    // Narrow cylindrical wells for needle immersion washing
    let well_id = 3.0; // mm inner diameter (just fits 0.72mm needle with margin)
    let well_depth = 20.0; // mm deep
    let well_wall = 2.0; // mm wall around each well
    let well_od = well_id + well_wall * 2.0; // 7mm outer diameter

    // Well positions along X axis (centered in Y)
    let well_spacing = 35.0; // mm between wells and blot pad
    let well1_x = -well_spacing; // bleach well
    let well2_x = 0.0; // DI rinse well
    let blot_x = well_spacing; // blot pad

    // ── Fluid inlet/outlet ports ──
    let inlet_d = 1.5; // mm — for PTFE tubing from peristaltic pump
    let overflow_d = 2.0; // mm — overflow port near well rim
    let _overflow_z = body_height / 2.0 - floor - 3.0; // 3mm below top of well

    // ── Blot pad holder ──
    let blot_length = 25.0; // X
    let blot_width = 25.0; // Y
    let blot_depth = 5.0; // mm recess depth (holds folded Kimwipe)

    // ── Drain ──
    let drain_d = 5.0; // mm — gravity drain to waste bottle
                       // Drain at -X end, through the bottom
    let drain_x = -(body_length / 2.0) + wall + drain_d;

    // ── Internal drain channel ──
    // Connects well overflows to the drain port
    // Runs along -Y edge, below the well rim level
    let channel_width = 4.0; // mm
    let channel_depth = 4.0; // mm
    let channel_y = -(body_width / 2.0) + wall + channel_width / 2.0 + 2.0;
    let channel_z_center = body_height / 2.0 - floor - well_depth + channel_depth / 2.0;

    // ── Mounting holes ──
    let mount_hole_d = 3.2; // M3 clearance
    let mount_spacing_x = 90.0; // center-to-center
    let mount_spacing_y = 60.0; // center-to-center

    // ── Splash guard rim ──
    // Slightly raised rim around the top perimeter
    let rim_height = 3.0;
    let _rim_width = 2.0;

    // ── Build body ──

    let outer = centered_cube("outer", body_length, body_width, body_height + rim_height);

    // Hollow out interior (open top)
    let inner_length = body_length - wall * 2.0;
    let inner_width = body_width - wall * 2.0;
    let inner_height = body_height - floor + rim_height;
    let cavity = centered_cube("cavity", inner_length, inner_width, inner_height + 0.1).translate(
        0.0,
        0.0,
        floor / 2.0 + 0.05,
    );

    // Add floor back (the cavity cut creates open top, floor remains)
    // Actually, centered body minus cavity from top gives us walls + floor.

    // ── Well cylinders (positive — raised well tubes inside the body) ──
    // Wells are raised cylindrical columns on the floor, hollowed out
    let mut well_solids = Part::empty("well_solids");
    let mut well_bores = Part::empty("well_bores");

    for (i, wx) in [well1_x, well2_x].iter().enumerate() {
        // Outer well cylinder
        let well_outer = centered_cylinder(
            &format!("well_outer_{i}"),
            well_od / 2.0,
            well_depth + floor,
            32,
        )
        .translate(
            *wx,
            0.0,
            -(body_height + rim_height) / 2.0 + (well_depth + floor) / 2.0,
        );
        well_solids = well_solids + well_outer;

        // Inner bore (the actual well cavity)
        let well_bore = centered_cylinder(
            &format!("well_bore_{i}"),
            well_id / 2.0,
            well_depth + 1.0,
            32,
        )
        .translate(
            *wx,
            0.0,
            -(body_height + rim_height) / 2.0 + floor + well_depth / 2.0 + 0.5,
        );
        well_bores = well_bores + well_bore;

        // Fluid inlet through bottom of well (for peristaltic pump feed)
        let inlet = centered_cylinder(&format!("well_inlet_{i}"), inlet_d / 2.0, floor + 2.0, 24)
            .translate(*wx, 0.0, -(body_height + rim_height) / 2.0 + floor / 2.0);
        well_bores = well_bores + inlet;

        // Overflow port (side hole near top of well, connects to drain channel)
        let overflow = centered_cylinder(
            &format!("well_overflow_{i}"),
            overflow_d / 2.0,
            well_od + 4.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            *wx,
            -(well_od / 2.0),
            -(body_height + rim_height) / 2.0 + floor + well_depth - 3.0,
        );
        well_bores = well_bores + overflow;
    }

    // ── Blot pad pocket ──
    // Rectangular recess that holds a folded Kimwipe or filter paper
    let blot_pocket = centered_cube("blot_pocket", blot_length, blot_width, blot_depth + 0.1)
        .translate(
            blot_x,
            0.0,
            -(body_height + rim_height) / 2.0 + floor + blot_depth / 2.0,
        );

    // Small raised ridge in blot pocket center (ensures tip contact with pad)
    let blot_ridge = centered_cube("blot_ridge", blot_length - 4.0, 2.0, 1.5).translate(
        blot_x,
        0.0,
        -(body_height + rim_height) / 2.0 + floor + blot_depth + 1.5 / 2.0 - 0.5,
    );

    // ── Internal drain channel ──
    // Trough that collects overflow from both wells and routes to drain port
    let drain_channel = centered_cube(
        "drain_channel",
        body_length - wall * 2.0 - 4.0,
        channel_width,
        channel_depth,
    )
    .translate(0.0, channel_y, channel_z_center);

    // ── Drain port ──
    // Through the bottom at -X end
    let drain = centered_cylinder("drain", drain_d / 2.0, floor + 4.0, 24).translate(
        drain_x,
        channel_y,
        -(body_height + rim_height) / 2.0 + floor / 2.0,
    );

    // ── Mounting holes ──
    let mut mount_holes = Part::empty("mount_holes");

    let mount_positions = [
        (-mount_spacing_x / 2.0, -mount_spacing_y / 2.0),
        (mount_spacing_x / 2.0, -mount_spacing_y / 2.0),
        (-mount_spacing_x / 2.0, mount_spacing_y / 2.0),
        (mount_spacing_x / 2.0, mount_spacing_y / 2.0),
    ];

    for (i, (mx, my)) in mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("mount_{i}"),
            mount_hole_d / 2.0,
            body_height + rim_height + 2.0,
            24,
        )
        .translate(*mx, *my, 0.0);
        mount_holes = mount_holes + hole;
    }

    // ── Assemble ──

    let station = (outer + well_solids + blot_ridge)
        - cavity
        - well_bores
        - blot_pocket
        - drain_channel
        - drain
        - mount_holes;

    // ── Export ──

    station.write_stl("output/wash_station.stl").unwrap();

    println!("Exported: output/wash_station.stl");
    println!();
    println!("── Wash Station Specs ──");
    println!(
        "  Body:           {body_length:.0}mm x {body_width:.0}mm x {:.0}mm (incl. rim)",
        body_height + rim_height
    );
    println!("  Wall:           {wall:.0}mm, Floor: {floor:.0}mm");
    println!("  Well 1 (bleach): {well_id:.0}mm ID x {well_depth:.0}mm deep at X={well1_x:.0}mm");
    println!("  Well 2 (DI):    {well_id:.0}mm ID x {well_depth:.0}mm deep at X={well2_x:.0}mm");
    println!("  Blot pad:       {blot_length:.0}mm x {blot_width:.0}mm x {blot_depth:.0}mm recess at X={blot_x:.0}mm");
    println!("  Fluid inlets:   2x {inlet_d:.1}mm dia (bottom of each well)");
    println!("  Overflow ports:  2x {overflow_d:.0}mm dia (near well rim)");
    println!("  Drain channel:  {channel_width:.0}mm x {channel_depth:.0}mm (along -Y edge)");
    println!("  Drain port:     {drain_d:.0}mm dia at X={drain_x:.0}mm (through bottom)");
    println!("  Mounting:       4x M3 at {mount_spacing_x:.0}mm x {mount_spacing_y:.0}mm");
    println!("  Material:       PETG (HDPE recommended for chemical resistance)");
}
