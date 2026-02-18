use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── X-Axis Dispensing Carriage ───
//
// 3D-printed (PETG) carriage that rides on the 8mm linear rod
// and is driven by the T8 lead screw. Carries:
// - 2-3 PTFE dispensing tips (one per fluid channel)
// - MG996R capping servo + capping arm mount
//
// The carriage traverses the 8-tube row for dispensing and capping.

fn main() {
    // ── Dimensions ──

    // Carriage body
    let body_length = 35.0; // along X (travel direction)
    let body_width = 25.0; // along Y (front-to-back)
    let body_height = 20.0; // along Z

    // Linear rod bearing bore (LM8UU: 15mm OD × 24mm long)
    let bearing_bore_d = LM8UU_OD + 0.2; // 15.2mm clearance
    let bearing_bore_len = LM8UU_LENGTH + 0.5; // 24.5mm

    // Lead screw nut pocket
    let nut_d = LEADSCREW_NUT_OD + 0.3; // 22.3mm
    let nut_h = LEADSCREW_NUT_HEIGHT; // 10mm
    let screw_d = LEADSCREW_DIAMETER + 1.0; // 9mm clearance

    // Dispensing tip holders: PTFE tubes pass through vertical holes
    let tip_hole_d = PTFE_OD + 0.5; // 2.0mm
    let tip_spacing = 6.0; // spacing between tip holes
    let num_tips = NUM_FLUID_CHANNELS; // 2

    // Servo mount: MG996R dimensions
    // MG996R body: 40.7 × 19.7 × 42.9mm, tab-to-tab: 54.5mm
    let servo_body_w = 20.0; // Y width
    let servo_body_l = 41.0; // X length
    let servo_tab_spacing = 49.0; // mounting tab centers
    let servo_hole_d = 4.2; // M4 clearance for servo tabs
    let servo_recess_depth = 10.0; // how deep servo sits in carriage

    // ── Build carriage body ──

    // Main body
    let body = centered_cube("body", body_length, body_width, body_height);

    // Extended platform for servo mount (wider section)
    let servo_platform = centered_cube(
        "servo_platform",
        servo_body_l + 12.0,
        servo_body_w + 6.0,
        body_height,
    )
    .translate(0.0, body_width / 2.0 + (servo_body_w + 6.0) / 2.0 - 3.0, 0.0);

    // ── Linear bearing bore (horizontal, along X) ──

    let bearing_bore = centered_cylinder(
        "bearing_bore",
        bearing_bore_d / 2.0,
        bearing_bore_len,
        32,
    )
    .rotate(0.0, 90.0, 0.0) // along X
    .translate(0.0, -(body_width / 2.0 - bearing_bore_d / 2.0 - 2.0), 0.0);

    // ── Lead screw nut pocket (on opposite side from bearing) ──

    let nut_pocket = centered_cylinder(
        "nut_pocket",
        nut_d / 2.0,
        nut_h,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, body_width / 2.0 - nut_d / 2.0, 0.0);

    // Lead screw clearance through-hole
    let screw_hole = centered_cylinder(
        "screw_hole",
        screw_d / 2.0,
        body_length + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, body_width / 2.0 - nut_d / 2.0, 0.0);

    // ── Dispensing tip holes (vertical, through bottom) ──

    let first_tip_x = -((num_tips as f64 - 1.0) * tip_spacing) / 2.0;
    let mut tip_holes = centered_cube("tip_init", 0.01, 0.01, 0.01);

    for i in 0..num_tips {
        let x = first_tip_x + (i as f64) * tip_spacing;
        let hole = centered_cylinder(
            &format!("tip_hole_{i}"),
            tip_hole_d / 2.0,
            body_height + 2.0,
            24,
        )
        .translate(x, 0.0, 0.0);
        tip_holes = tip_holes + hole;
    }

    // PTFE tube routing channel from top to tip holes
    let tube_route = centered_cube(
        "tube_route",
        (num_tips as f64) * tip_spacing + tip_hole_d + 2.0,
        tip_hole_d + 1.0,
        body_height / 2.0,
    )
    .translate(0.0, 0.0, body_height / 4.0 + 1.0);

    // ── Servo recess (on the extended platform) ──

    let servo_recess = centered_cube(
        "servo_recess",
        servo_body_l,
        servo_body_w,
        servo_recess_depth,
    )
    .translate(0.0, body_width / 2.0 + (servo_body_w + 6.0) / 2.0 - 3.0, body_height / 2.0 - servo_recess_depth / 2.0);

    // Servo mounting holes
    let servo_hole_1 = centered_cylinder(
        "servo_hole_0",
        servo_hole_d / 2.0,
        body_height + 2.0,
        24,
    )
    .translate(-(servo_tab_spacing / 2.0), body_width / 2.0 + (servo_body_w + 6.0) / 2.0 - 3.0, 0.0);

    let servo_hole_2 = centered_cylinder(
        "servo_hole_1",
        servo_hole_d / 2.0,
        body_height + 2.0,
        24,
    )
    .translate(servo_tab_spacing / 2.0, body_width / 2.0 + (servo_body_w + 6.0) / 2.0 - 3.0, 0.0);

    // ── Assemble ──

    let carriage = (body + servo_platform)
        - bearing_bore
        - nut_pocket
        - screw_hole
        - tip_holes
        - tube_route
        - servo_recess
        - servo_hole_1
        - servo_hole_2;

    // ── Export ──

    carriage
        .write_stl("output/dispensing_carriage.stl")
        .unwrap();

    println!("Exported: output/dispensing_carriage.stl");
    println!();
    println!("── Dispensing Carriage Specs ──");
    println!("  Body:           {body_length:.1}mm × {body_width:.1}mm × {body_height:.1}mm");
    println!("  Bearing bore:   {bearing_bore_d:.1}mm dia × {bearing_bore_len:.1}mm (LM8UU)");
    println!("  Nut pocket:     {nut_d:.1}mm dia × {nut_h:.1}mm (T8 nut)");
    println!("  Tip holes:      {num_tips} × {tip_hole_d:.1}mm at {tip_spacing:.0}mm spacing");
    println!("  Servo mount:    MG996R recess + 2× M4 holes");
    println!("  Material:       PETG");
}
