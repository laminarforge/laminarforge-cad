use vcad::{centered_cube, centered_cylinder, Part};

// ─── Peltier-Controlled Reservoir Block ───
//
// CNC-machined 6061-T6 aluminum block for temperature-controlled
// liquid handling reservoirs. Build two identical blocks:
//   - Warm block: TEC in heating mode → 37C (cell culture media)
//   - Cold block: TEC in cooling mode → 4C (AAV/reagents)
//
// Each block holds a single 50mL conical centrifuge tube (Falcon-style,
// ~29mm OD) with silicone septum cap for needle access + 0.22um vent filter.
//
// TEC1-12706 Peltier module (40x40mm) mounts on the bottom face via a
// shallow registration recess. DS18B20 temperature sensor inserts into
// a pocket drilled from the front face at mid-pocket height.
//
// PID control via ESP32 environmental controller (env_controller.ino).
// Warm block: L298N H-bridge GPIO 12-14, DS18B20 sensor index 2.
// Cold block: L298N H-bridge GPIO 17-21, DS18B20 sensor index 3.
//
// Cross-section (Y-Z plane through tube pocket):
//
//      y=-25        y=0         y=+25
//       |            |            |
// z=+37.5  ════════════════════════  top face
//       |                          |
//       |   ┌──tube pocket──┐     |
//       |   │  30mm dia     │     |
//       |   │  55mm deep    │     |
//       |   │               │     |  ← thermistor at z=+10
//       |   │               │     |
//       |   └───────────────┘     |
// z=-17.5  ─────pocket floor──────
//       |    20mm thermal mass     |
//       |    (heat distribution)   |
// z=-37.0  ─ ─ TEC recess (0.5mm) ─
// z=-37.5  ════════════════════════  bottom face (TEC contact)
//
// Material: 6061-T6 Aluminum
// Dimensions: 50mm x 50mm x 75mm
// Export: output/peltier_reservoir_block.stl

fn main() {
    // ── Block dimensions ──
    let block_x = 50.0; // mm (length)
    let block_y = 50.0; // mm (width)
    let block_z = 75.0; // mm (height)

    // ── Tube pocket ──
    // 50mL conical centrifuge tube: ~29mm OD body
    // 30mm pocket gives ~0.5mm clearance per side for easy insertion
    let pocket_diameter = 30.0; // mm
    let pocket_depth = 55.0; // mm (from top face, immerses most of tube body)

    // ── TEC1-12706 registration recess ──
    // Shallow recess on bottom face to precisely locate the 40x40mm TEC module.
    // Thermal paste fills the interface. Recess prevents lateral sliding.
    let tec_recess_x = 40.2; // mm (0.2mm clearance for 40mm TEC)
    let tec_recess_y = 40.2; // mm
    let tec_recess_depth = 0.5; // mm (just enough to register position)

    // ── Thermistor pocket ──
    // DS18B20 waterproof probe (3mm dia stainless) inserts from front face.
    // Positioned at mid-pocket height for accurate bulk temperature reading.
    // Offset slightly from center toward pocket wall for faster response.
    let therm_diameter = 3.5; // mm (0.5mm clearance for 3mm probe)
    let therm_depth = 20.0; // mm (from front face, reaches near pocket wall)
    let therm_z = (block_z / 2.0) - (pocket_depth / 2.0); // mid-pocket height = +10mm

    // ── M3 mounting holes ──
    // Through-holes at corners for bolting block to deck adapter plate.
    // Also used to clamp heatsink bracket from below.
    let mount_hole_d = 3.2; // mm (M3 clearance)
    let mount_hole_inset = 6.0; // mm from edge
    let mount_x = block_x / 2.0 - mount_hole_inset; // ±19mm
    let mount_y = block_y / 2.0 - mount_hole_inset; // ±19mm

    // ── Heatsink bolt holes ──
    // Two additional M3 holes on the bottom face edges for securing the
    // heatsink/fan assembly. Tapped holes (2.5mm pilot for M3 thread),
    // 10mm deep from bottom, outside the TEC recess area.
    let hs_bolt_d = 2.5; // mm (M3 tap drill)
    let hs_bolt_depth = 10.0; // mm
    let hs_bolt_x = 0.0; // centered on X
    let hs_bolt_y_offset = 22.0; // mm from center (outside TEC area)

    // ── Build body ──

    let body = centered_cube("body", block_x, block_y, block_z);

    // ── Tube pocket (subtractive, from top face) ──
    // Position cylinder so its bottom aligns with pocket floor,
    // and top extends past the block top face.

    let pocket_z_offset = (block_z - pocket_depth) / 2.0; // +10mm
    let pocket = centered_cylinder(
        "tube_pocket",
        pocket_diameter / 2.0,
        pocket_depth + 1.0, // +1mm to cleanly cut through top face
        48,                 // high facet count for smooth bore
    )
    .translate(0.0, 0.0, pocket_z_offset);

    // ── TEC registration recess (subtractive, from bottom face) ──
    // Shallow rectangular pocket centered on bottom face.

    let tec_z_offset = -(block_z / 2.0) + tec_recess_depth / 2.0 - 0.5;
    let tec_recess = centered_cube(
        "tec_recess",
        tec_recess_x,
        tec_recess_y,
        tec_recess_depth + 1.0, // +1mm to cut through bottom face
    )
    .translate(0.0, 0.0, tec_z_offset);

    // ── Thermistor pocket (subtractive, from front face -Y) ──
    // Horizontal bore from front face toward block center.

    let therm_center_y = -(block_y / 2.0) + therm_depth / 2.0;
    let therm_pocket = centered_cylinder(
        "therm_pocket",
        therm_diameter / 2.0,
        therm_depth + 1.0, // +1mm to cut through front face
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, therm_center_y, therm_z);

    // ── M3 corner mounting holes (through-holes, full height) ──

    let mount_positions: [(f64, f64); 4] = [
        (-mount_x, -mount_y),
        (mount_x, -mount_y),
        (-mount_x, mount_y),
        (mount_x, mount_y),
    ];

    let mut mounts = Part::empty("mounts");
    for (i, &(mx, my)) in mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("mount_{i}"),
            mount_hole_d / 2.0,
            block_z + 2.0, // through entire block
            24,
        )
        .translate(mx, my, 0.0);
        mounts = mounts + hole;
    }

    // ── Heatsink bolt holes (blind, from bottom face) ──
    // Two tapped holes on the Y edges for heatsink attachment.

    let hs_z_offset = -(block_z / 2.0) + hs_bolt_depth / 2.0 - 0.5;
    let hs_bolt_1 = centered_cylinder("hs_bolt_1", hs_bolt_d / 2.0, hs_bolt_depth + 1.0, 24)
        .translate(hs_bolt_x, -hs_bolt_y_offset, hs_z_offset);

    let hs_bolt_2 = centered_cylinder("hs_bolt_2", hs_bolt_d / 2.0, hs_bolt_depth + 1.0, 24)
        .translate(hs_bolt_x, hs_bolt_y_offset, hs_z_offset);

    // ── Assemble (subtract all features from body) ──

    let part = body - pocket - tec_recess - therm_pocket - mounts - hs_bolt_1 - hs_bolt_2;

    // ── Export ──

    part.write_stl("output/peltier_reservoir_block.stl")
        .unwrap();

    // ── Print specs ──

    let thermal_mass = block_z - pocket_depth; // 20mm
    let pocket_wall_min = (block_x - pocket_diameter) / 2.0; // 10mm

    println!("Exported: output/peltier_reservoir_block.stl");
    println!();
    println!("── Peltier Reservoir Block Specs ──");
    println!("  Body:              {block_x:.0}mm x {block_y:.0}mm x {block_z:.0}mm");
    println!("  Material:          6061-T6 Aluminum");
    println!(
        "  Tube pocket:       {pocket_diameter:.0}mm dia x {pocket_depth:.0}mm deep (50mL conical tube)"
    );
    println!("  Thermal mass:      {thermal_mass:.0}mm floor below pocket (heat distribution)");
    println!("  Min wall thickness: {pocket_wall_min:.0}mm (pocket to outer face)");
    println!(
        "  TEC recess:        {tec_recess_x:.1}mm x {tec_recess_y:.1}mm x {tec_recess_depth:.1}mm (bottom face)"
    );
    println!(
        "  Thermistor pocket: {therm_diameter:.1}mm dia x {therm_depth:.0}mm deep (from front face)"
    );
    println!("  Thermistor Z:      {therm_z:+.0}mm (mid-pocket height)");
    println!(
        "  Corner mounts:     4x M3 ({mount_hole_d:.1}mm clearance), {mount_hole_inset:.0}mm inset"
    );
    println!(
        "  Heatsink bolts:    2x M3 tapped ({hs_bolt_d:.1}mm pilot), {hs_bolt_depth:.0}mm deep"
    );
    println!();
    println!("── Usage (both blocks are mechanically identical) ──");
    println!("  Warm block: TEC heating mode, setpoint 37C (media)");
    println!("  Cold block: TEC cooling mode, setpoint 4C (AAV)");
    println!("  Cold block: WRAP in 10mm closed-cell foam insulation (Armaflex)");
    println!();
    println!("── Thermal Budget ──");
    println!("  Warm block (37C): dT ~12C above ambient → trivial for TEC1-12706");
    println!("    Hot side dissipation: ~30W → small 40mm heatsink + passive OK");
    println!("  Cold block (4C): dT ~21C below ambient → demanding");
    println!("    Hot side dissipation: ~80W → large CPU cooler or water block REQUIRED");
    println!("    Without insulation: TEC fights ambient heat constantly (failure)");
    println!();
    println!("── Assembly Steps ──");
    println!("  1. Apply thermal paste (Arctic MX-6) to TEC1-12706 cold face");
    println!("  2. Seat TEC in bottom registration recess");
    println!("  3. Apply thermal paste to TEC hot face");
    println!("  4. Attach heatsink to TEC hot side (M3 bolts through hs_bolt holes)");
    println!("     Warm: 40x40x20mm fin heatsink, passive or small fan");
    println!("     Cold: tower cooler with 80mm+ fan, or 40x80mm water block");
    println!("  5. Insert DS18B20 waterproof probe into front thermistor pocket");
    println!("  6. Seal thermistor pocket with dab of silicone or kapton tape");
    println!("  7. Connect TEC to L298N H-bridge (warm: GPIO 12-14, cold: GPIO 17-21)");
    println!("  8. Connect DS18B20 to 1-Wire bus (GPIO 4, shared with other sensors)");
    println!("  9. Bolt block to deck adapter plate via 4x M3 corner holes");
    println!(" 10. Insert 50mL tube with silicone septum cap + 0.22um vent filter");
    println!(" 11. For cold block: wrap exposed surfaces in Armaflex insulation");
    println!();
    println!("── Fabrication Notes ──");
    println!("  Machine from 50mm x 50mm 6061-T6 bar stock.");
    println!("  All features: vertical bore (tube pocket), shallow pocket (TEC recess),");
    println!("  horizontal bore (thermistor), through-holes (mounting), blind holes (HS bolts).");
    println!("  Can be done on a 3-axis CNC mill or manual mill + drill press.");
    println!("  TEC recess bottom face flatness: < 0.05mm for good thermal contact.");
}
