use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── X-Axis Linear Rail End Mounts ───
//
// Two 3D-printed (PETG) end plates for the dispensing system's X-axis
// linear rail. These mount to the dispensing frame side walls and hold
// the 8mm linear rod + T8 lead screw.
//
// Left mount: Includes NEMA17 motor mounting plate (motor drives lead screw)
// Right mount: Includes microswitch pocket for endstop homing
//
// Both mounts share the same base geometry:
// - 30×8×40mm body (W × D × H)
// - 8.1mm rod bore at Z=-5 from center (8mm rod + 0.1mm clearance)
// - 10mm screw bore at Z=+10 from center (8mm screw + 2mm clearance)
// - 2× M3 frame mounting holes at ±12mm X
//
// Exports:
// - output/rail_mount_left.stl (motor mount)
// - output/rail_mount_right.stl (endstop mount)

fn main() {
    // ── Shared dimensions ──

    let body_w = 30.0;
    let body_d = 8.0;
    let body_h = 40.0;

    let rod_bore_d = LINEAR_ROD_DIAMETER + 0.1; // 8.1mm clearance
    let rod_z_offset = -5.0; // below center

    let screw_bore_d = LEADSCREW_DIAMETER + 2.0; // 10mm clearance
    let screw_z_offset = 10.0; // above center

    let frame_hole_d = 3.2; // M3 clearance
    let frame_hole_x_spacing = 12.0; // ±12mm from center

    // ── Left mount (motor side) ──

    let left_body = centered_cube("left_body", body_w, body_d, body_h);

    // Rod bore (horizontal, through Y)
    let left_rod = centered_cylinder("left_rod", rod_bore_d / 2.0, body_d + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, rod_z_offset);

    // Screw bore (horizontal, through Y)
    let left_screw = centered_cylinder("left_screw", screw_bore_d / 2.0, body_d + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, screw_z_offset);

    // Frame mounting holes (vertical, through Z)
    let left_frame_h1 = centered_cylinder("left_fh1", frame_hole_d / 2.0, body_h + 2.0, 24)
        .translate(-frame_hole_x_spacing, 0.0, 0.0);
    let left_frame_h2 = centered_cylinder("left_fh2", frame_hole_d / 2.0, body_h + 2.0, 24)
        .translate(frame_hole_x_spacing, 0.0, 0.0);

    // NEMA17 motor mounting plate: extends from +Y face of the body
    let motor_plate_size = 50.0;
    let motor_plate_thickness = 5.0;

    let motor_plate = centered_cube(
        "motor_plate",
        motor_plate_size,
        motor_plate_thickness,
        motor_plate_size,
    )
    .translate(0.0, body_d / 2.0 + motor_plate_thickness / 2.0, screw_z_offset);

    // NEMA17 bolt holes (4× M3 at 31mm square pattern)
    let half_bolt = NEMA17_HOLE_SPACING / 2.0; // 15.5mm
    let motor_bolt_positions: [(f64, f64); 4] = [
        (-half_bolt, -half_bolt),
        (-half_bolt, half_bolt),
        (half_bolt, -half_bolt),
        (half_bolt, half_bolt),
    ];

    let mut motor_bolts = centered_cube("mb_init", 0.01, 0.01, 0.01);
    for (i, (bx, bz)) in motor_bolt_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("motor_bolt_{i}"),
            NEMA17_HOLE_DIAMETER / 2.0,
            motor_plate_thickness + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0) // align along Y
        .translate(
            *bx,
            body_d / 2.0 + motor_plate_thickness / 2.0,
            screw_z_offset + *bz,
        );
        motor_bolts = motor_bolts + hole;
    }

    // Boss recess on inner face of motor plate (NEMA17 locating boss)
    let boss_recess_d = NEMA17_BOSS_DIAMETER + 0.5; // 22.5mm
    let boss_recess_depth = 2.5;

    let boss_recess = centered_cylinder("boss_recess", boss_recess_d / 2.0, boss_recess_depth, 48)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            0.0,
            body_d / 2.0 + boss_recess_depth / 2.0 - 0.1,
            screw_z_offset,
        );

    // Shaft hole through motor plate center (7mm for 5mm shaft + coupler clearance)
    let shaft_hole_d = 7.0;
    let shaft_hole = centered_cylinder(
        "shaft_hole",
        shaft_hole_d / 2.0,
        motor_plate_thickness + 2.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, body_d / 2.0 + motor_plate_thickness / 2.0, screw_z_offset);

    let left_mount = (left_body + motor_plate)
        - left_rod
        - left_screw
        - left_frame_h1
        - left_frame_h2
        - motor_bolts
        - boss_recess
        - shaft_hole;

    left_mount
        .write_stl("output/rail_mount_left.stl")
        .unwrap();

    // ── Right mount (endstop side) ──

    let right_body = centered_cube("right_body", body_w, body_d, body_h);

    // Rod bore
    let right_rod = centered_cylinder("right_rod", rod_bore_d / 2.0, body_d + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, rod_z_offset);

    // Screw bore
    let right_screw = centered_cylinder("right_screw", screw_bore_d / 2.0, body_d + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, screw_z_offset);

    // Frame mounting holes
    let right_frame_h1 = centered_cylinder("right_fh1", frame_hole_d / 2.0, body_h + 2.0, 24)
        .translate(-frame_hole_x_spacing, 0.0, 0.0);
    let right_frame_h2 = centered_cylinder("right_fh2", frame_hole_d / 2.0, body_h + 2.0, 24)
        .translate(frame_hole_x_spacing, 0.0, 0.0);

    // Microswitch pocket on outside face (+Y face)
    // Standard microswitch: ~20×6.5×10.5mm body
    let switch_pocket_w = 21.0;
    let switch_pocket_d = 7.0; // depth into the body from +Y face
    let switch_pocket_h = 11.0;

    let switch_pocket = centered_cube(
        "switch_pocket",
        switch_pocket_w,
        switch_pocket_d,
        switch_pocket_h,
    )
    .translate(0.0, body_d / 2.0 - switch_pocket_d / 2.0 + 0.1, screw_z_offset);

    // Microswitch M2 mounting holes (2× at 10mm spacing, horizontal through Y)
    let switch_hole_d = 2.2; // M2 clearance
    let switch_hole_spacing = 10.0;

    let switch_h1 = centered_cylinder("sw_h1", switch_hole_d / 2.0, body_d + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-switch_hole_spacing / 2.0, 0.0, screw_z_offset);

    let switch_h2 = centered_cylinder("sw_h2", switch_hole_d / 2.0, body_d + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(switch_hole_spacing / 2.0, 0.0, screw_z_offset);

    let right_mount = right_body
        - right_rod
        - right_screw
        - right_frame_h1
        - right_frame_h2
        - switch_pocket
        - switch_h1
        - switch_h2;

    right_mount
        .write_stl("output/rail_mount_right.stl")
        .unwrap();

    // ── Print specs ──

    println!("Exported: output/rail_mount_left.stl");
    println!("Exported: output/rail_mount_right.stl");
    println!();
    println!("── Rail Mount Specs (shared) ──");
    println!("  Body:            {body_w:.0}mm x {body_d:.0}mm x {body_h:.0}mm");
    println!("  Rod bore:        {rod_bore_d:.1}mm dia at Z={rod_z_offset:.0}mm (for {LINEAR_ROD_DIAMETER:.0}mm rod)");
    println!("  Screw bore:      {screw_bore_d:.0}mm dia at Z={screw_z_offset:.0}mm (for {LEADSCREW_DIAMETER:.0}mm T8)");
    println!("  Frame holes:     2x M3 at +-{frame_hole_x_spacing:.0}mm X");
    println!();
    println!("── Left Mount (motor side) ──");
    println!("  Motor plate:     {motor_plate_size:.0}mm x {motor_plate_size:.0}mm x {motor_plate_thickness:.0}mm");
    println!("  NEMA17 pattern:  4x M3 at {NEMA17_HOLE_SPACING:.0}mm square");
    println!("  Boss recess:     {boss_recess_d:.1}mm dia x {boss_recess_depth:.1}mm");
    println!("  Shaft hole:      {shaft_hole_d:.0}mm dia");
    println!();
    println!("── Right Mount (endstop side) ──");
    println!("  Switch pocket:   {switch_pocket_w:.0}mm x {switch_pocket_d:.0}mm x {switch_pocket_h:.0}mm");
    println!("  Switch holes:    2x M2 at {switch_hole_spacing:.0}mm spacing");
    println!("  Material:        PETG");
}
