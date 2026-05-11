use vcad::{centered_cube, centered_cylinder, Part};

// ─── Magnetic Stirrer ───
//
// 3D-printed (PETG) magnetic stirrer housing. Uses an 80mm PC fan
// with magnets on the blades to spin a magnetic stir bar in a beaker
// placed on top. The thin 2mm top plate allows magnetic coupling.
//
// Features:
// - 80mm fan mount recess on bottom (71.5mm diagonal M4 bolt pattern)
// - 2mm thick top plate for magnetic coupling
// - Potentiometer hole (7mm) on front face for speed control
// - Ventilation slots on all 4 sides
// - 4 rubber foot recesses on bottom
//
// Exports: magnetic_stirrer.stl

fn main() {
    // ── Dimensions ──

    let housing_x = 86.0;
    let housing_y = 86.0;
    let housing_z = 30.0;

    let top_plate_thickness = 2.0;

    // Fan mount recess (80mm fan, recessed into bottom)
    let fan_recess_d = 82.0; // slightly larger than fan frame
    let fan_recess_depth = housing_z - top_plate_thickness; // everything below top plate

    // 80mm fan bolt pattern: 71.5mm diagonal = 71.5/sqrt(2) = 50.56mm between holes
    let fan_bolt_spacing = 71.5 / std::f64::consts::SQRT_2; // ~50.56mm
    let fan_bolt_d = 4.2; // M4 clearance

    // Potentiometer hole on front face
    let pot_hole_d = 7.0;
    let pot_z = -(housing_z / 2.0) + housing_z / 2.0; // mid-height

    // Ventilation slots: 4 slots per side, 3mm wide x 20mm tall
    let vent_width = 3.0;
    let vent_height = 20.0;
    let num_vents = 4;
    let vent_spacing = 15.0;

    // Rubber foot recesses on bottom
    let foot_d = 8.0;
    let foot_depth = 2.0;
    let foot_inset = 12.0; // from center to foot center

    // ── Build housing ──

    let body = centered_cube("body", housing_x, housing_y, housing_z);

    // ── Fan recess (from bottom, leave top plate intact) ──

    let fan_recess = centered_cylinder("fan_recess", fan_recess_d / 2.0, fan_recess_depth, 48)
        .translate(0.0, 0.0, -(housing_z / 2.0) + fan_recess_depth / 2.0);

    // Square out the recess for the fan frame corners
    let fan_square_recess = centered_cube("fan_square", 80.0, 80.0, fan_recess_depth).translate(
        0.0,
        0.0,
        -(housing_z / 2.0) + fan_recess_depth / 2.0,
    );

    // ── Fan bolt holes (4x M4, through bottom into housing) ──

    let half_bolt = fan_bolt_spacing / 2.0;
    let bolt_positions: [(f64, f64); 4] = [
        (-half_bolt, -half_bolt),
        (-half_bolt, half_bolt),
        (half_bolt, -half_bolt),
        (half_bolt, half_bolt),
    ];

    let mut fan_bolts = Part::empty("fan_bolts");
    for (i, (bx, by)) in bolt_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("fan_bolt_{i}"),
            fan_bolt_d / 2.0,
            housing_z + 2.0,
            24,
        )
        .translate(*bx, *by, 0.0);
        fan_bolts = fan_bolts + hole;
    }

    // ── Potentiometer hole (front face, Y-axis) ──

    let pot_hole = centered_cylinder("pot_hole", pot_hole_d / 2.0, housing_y + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -(housing_y / 2.0), pot_z);

    // ── Ventilation slots (all 4 sides) ──

    let mut vent_slots = Part::empty("vent_slots");

    // Front and back (Y-axis walls)
    for side_y in [-1.0_f64, 1.0] {
        for i in 0..num_vents {
            let vx = -(num_vents as f64 - 1.0) * vent_spacing / 2.0 + (i as f64) * vent_spacing;
            let slot = centered_cube(
                &format!("vent_fb_{}", i),
                vent_width,
                housing_y + 2.0,
                vent_height,
            )
            .translate(vx, 0.0, -(housing_z / 2.0) + vent_height / 2.0 + 3.0);

            // Only cut where the wall is (mask to wall thickness)
            let wall_mask = centered_cube(
                &format!("wall_mask_fb_{}", i),
                vent_width + 1.0,
                6.0,
                vent_height + 1.0,
            )
            .translate(
                vx,
                side_y * (housing_y / 2.0 - 1.5),
                -(housing_z / 2.0) + vent_height / 2.0 + 3.0,
            );

            vent_slots = vent_slots + (slot & wall_mask);
        }
    }

    // Left and right (X-axis walls)
    for side_x in [-1.0_f64, 1.0] {
        for i in 0..num_vents {
            let vy = -(num_vents as f64 - 1.0) * vent_spacing / 2.0 + (i as f64) * vent_spacing;
            let slot = centered_cube(
                &format!("vent_lr_{}", i),
                housing_x + 2.0,
                vent_width,
                vent_height,
            )
            .translate(0.0, vy, -(housing_z / 2.0) + vent_height / 2.0 + 3.0);

            let wall_mask = centered_cube(
                &format!("wall_mask_lr_{}", i),
                6.0,
                vent_width + 1.0,
                vent_height + 1.0,
            )
            .translate(
                side_x * (housing_x / 2.0 - 1.5),
                vy,
                -(housing_z / 2.0) + vent_height / 2.0 + 3.0,
            );

            vent_slots = vent_slots + (slot & wall_mask);
        }
    }

    // ── Rubber foot recesses (bottom face, 4 corners) ──

    let mut foot_recesses = Part::empty("foot_recesses");
    let foot_positions: [(f64, f64); 4] = [
        (-foot_inset, -foot_inset),
        (-foot_inset, foot_inset),
        (foot_inset, -foot_inset),
        (foot_inset, foot_inset),
    ];

    for (i, (fx, fy)) in foot_positions.iter().enumerate() {
        let recess = centered_cylinder(&format!("foot_{i}"), foot_d / 2.0, foot_depth, 24)
            .translate(*fx, *fy, -(housing_z / 2.0) + foot_depth / 2.0);
        foot_recesses = foot_recesses + recess;
    }

    // ── Assemble ──

    let stirrer =
        body - fan_recess - fan_square_recess - fan_bolts - pot_hole - vent_slots - foot_recesses;

    // ── Export ──

    stirrer.write_stl("output/magnetic_stirrer.stl").unwrap();

    println!("Exported: output/magnetic_stirrer.stl");
    println!();
    println!("── Magnetic Stirrer Specs ──");
    println!("  Housing:         {housing_x:.0}mm x {housing_y:.0}mm x {housing_z:.0}mm");
    println!("  Top plate:       {top_plate_thickness:.0}mm thick (magnetic coupling)");
    println!("  Fan mount:       80mm PC fan, {fan_recess_d:.0}mm recess");
    println!("  Fan bolts:       4x M4 at {fan_bolt_spacing:.1}mm spacing (71.5mm diagonal)");
    println!("  Pot hole:        {pot_hole_d:.0}mm dia (front face)");
    println!("  Vent slots:      {num_vents}x {vent_width:.0}mm x {vent_height:.0}mm per side");
    println!("  Foot recesses:   4x {foot_d:.0}mm dia x {foot_depth:.0}mm deep");
    println!("  Material:        PETG");
}
