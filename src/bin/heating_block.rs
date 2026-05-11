use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Aluminum Heating Block ───
//
// CNC-machined 6061-T6 aluminum block that replaces both the copper
// spreader plate and PETG tube holder. Provides thermal mass, uniform
// heating via cartridge heater, and direct tube retention.
//
// Sits directly on the PCB. Heater connects via J4, thermistor via J3.
//
// Features:
//   - 8 blind tube wells (6.2mm dia, 15mm deep from top)
//   - Cartridge heater bore (6.1mm dia, 78mm deep from left end)
//   - Thermistor pocket (3mm dia, 7mm deep from front face)
//   - 4× M3 mounting holes at corners
//
// Cross-section (Y-Z plane through a tube well):
//
//      y=-11        y=0         y=+11
//       |            |            |
// z=+12.5  ════════════════════════  top
//       |   [tube well 6.2mm]     |
// z=-2.5   ─────[well floor]──────
//       |                          |
// z=-4.45  ···[heater bore top]···
//       |                          |
// z=-7.5   ···[heater center]···    6.1mm bore
//       |                          |
// z=-10.55 ···[heater bore bot]···
//       |                          |
// z=-12.5  ════════════════════════  bottom
//
// Material: 6061-T6 Aluminum
// Dimensions: 84mm × 22mm × 25mm
// Export: output/heating_block.stl

fn main() {
    // ── Body ──

    let body = centered_cube("body", BLOCK_LENGTH, BLOCK_WIDTH, BLOCK_HEIGHT);

    // ── 8 Tube Wells (blind holes from top) ──
    //
    // Wells are 15mm deep from the top face. We create cylinders taller
    // than the well depth and position them so they only cut from the top,
    // leaving a solid floor at the bottom.

    let first_x = first_slot_x();
    let well_z = (BLOCK_HEIGHT - BLOCK_WELL_DEPTH) / 2.0; // shift up so bottom of cylinder = well floor

    let mut wells = Part::empty("wells");
    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let well = centered_cylinder(
            &format!("well_{i}"),
            BLOCK_TUBE_DIAMETER / 2.0,
            BLOCK_WELL_DEPTH + 1.0, // +1mm to cleanly cut through top face
            32,
        )
        .translate(x, 0.0, well_z);
        wells = wells + well;
    }

    // ── Cartridge Heater Bore (horizontal, from left end) ──
    //
    // 6.1mm diameter bore centered at z = 5mm from bottom face.
    // Drilled from the left end (x = -BLOCK_LENGTH/2), 78mm deep.

    let heater_z = -(BLOCK_HEIGHT / 2.0) + HEATER_BORE_Z_OFFSET; // -7.5mm
                                                                 // Cylinder along X axis: rotate 90° around Y
                                                                 // Position so the bore starts at the left face and extends rightward
    let heater_center_x = -(BLOCK_LENGTH / 2.0) + HEATER_BORE_DEPTH / 2.0; // bore center in X
    let heater_bore = centered_cylinder(
        "heater_bore",
        HEATER_BORE_DIAMETER / 2.0,
        HEATER_BORE_DEPTH + 1.0, // +1mm to cut cleanly through left face
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(heater_center_x, 0.0, heater_z);

    // ── Thermistor Pocket (horizontal, from front face) ──
    //
    // 3mm diameter pocket centered at same Z as heater bore.
    // Drilled from front face (y = -BLOCK_WIDTH/2), 7mm deep.
    // At x=0 (block center), between wells 3 and 4.

    let therm_center_y = -(BLOCK_WIDTH / 2.0) + THERMISTOR_BORE_DEPTH / 2.0;
    let therm_pocket = centered_cylinder(
        "therm_pocket",
        THERMISTOR_BORE_DIAMETER / 2.0,
        THERMISTOR_BORE_DEPTH + 1.0, // +1mm to cut through front face
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, therm_center_y, heater_z);

    // ── 4× M3 Mounting Holes (through-holes, top to bottom) ──
    //
    // Positioned at block corners, outside both tube wells and heater bore.

    let mount_positions: [(f64, f64); 4] = [
        (-BLOCK_MOUNT_HOLE_X, -BLOCK_MOUNT_HOLE_Y),
        (BLOCK_MOUNT_HOLE_X, -BLOCK_MOUNT_HOLE_Y),
        (-BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_Y),
        (BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_Y),
    ];

    let mut mounts = Part::empty("mounts");
    for (i, &(mx, my)) in mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("mount_{i}"),
            BLOCK_MOUNT_HOLE_DIAMETER / 2.0,
            BLOCK_HEIGHT + 2.0,
            24,
        )
        .translate(mx, my, 0.0);
        mounts = mounts + hole;
    }

    // ── Assemble (subtract all features from body) ──

    let part = body - wells - heater_bore - therm_pocket - mounts;

    // ── Export ──

    part.write_stl("output/heating_block.stl").unwrap();

    // ── Print Specs ──

    let well_floor_z = heater_z + HEATER_BORE_DIAMETER / 2.0; // top of heater bore
    let well_bottom_z = -(BLOCK_HEIGHT / 2.0) + (BLOCK_HEIGHT - BLOCK_WELL_DEPTH); // well floor
    let gap = well_bottom_z - well_floor_z;

    println!("Exported: output/heating_block.stl");
    println!();
    println!("── Aluminum Heating Block Specs ──");
    println!("  Body:              {BLOCK_LENGTH:.0}mm × {BLOCK_WIDTH:.0}mm × {BLOCK_HEIGHT:.0}mm");
    println!("  Material:          6061-T6 Aluminum");
    println!("  Tube wells:        {NUM_SLOTS}× {BLOCK_TUBE_DIAMETER:.1}mm dia, {BLOCK_WELL_DEPTH:.0}mm deep (blind)");
    println!("  Slot spacing:      {SLOT_SPACING:.0}mm center-to-center");
    println!("  Heater bore:       {HEATER_BORE_DIAMETER:.1}mm dia, {HEATER_BORE_DEPTH:.0}mm deep (from left end)");
    println!("  Heater Z offset:   {HEATER_BORE_Z_OFFSET:.0}mm from bottom");
    println!("  Thermistor pocket: {THERMISTOR_BORE_DIAMETER:.0}mm dia, {THERMISTOR_BORE_DEPTH:.0}mm deep (from front)");
    println!("  Mounting holes:    4× M3 ({BLOCK_MOUNT_HOLE_DIAMETER:.1}mm) at corners");
    println!("  Well-to-heater:    {gap:.2}mm aluminum between well floor and heater bore");
    println!();
    println!("── Fabrication Notes ──");
    println!("  All features are simple drilled holes/bores.");
    println!("  Can be CNC machined from bar stock or manually drilled with a drill press.");
    println!("  Cartridge heater: 6mm × 12V 40W, inserted from left end.");
    println!("  Thermistor: 10K NTC glass bead, inserted from front face.");
}
