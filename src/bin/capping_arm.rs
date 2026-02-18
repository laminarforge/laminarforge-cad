use vcad::{centered_cube, centered_cylinder};

// ─── Capping Arm ───
//
// 3D-printed (PETG) servo-driven arm that folds PCR tube caps closed.
// Mounts on the MG996R servo horn on the dispensing carriage.
//
// The arm swings through ~180° to:
// 1. Hook under an open hinged cap
// 2. Fold it over onto the tube opening
// 3. Press it down to snap-seal
//
// Geometry: L-shaped lever with a hook tip and flat press pad.

fn main() {
    // ── Dimensions ──

    let arm_length = 30.0; // total lever length from pivot
    let arm_width = 8.0;
    let arm_thickness = 4.0;

    // Servo horn attachment: standard MG996R 25T spline
    let horn_bore_d = 6.0; // servo horn hub diameter
    let horn_bore_depth = 3.0;
    let horn_screw_d = 3.2; // M3 horn screw

    // Hub (thicker section around servo attachment)
    let hub_d = 14.0;
    let hub_h = arm_thickness + 2.0; // 6mm

    // Hook tip: catches under the open cap hinge
    let hook_length = 5.0;
    let hook_drop = 6.0; // how far the hook extends down
    let hook_thickness = 2.0;

    // Press pad: flat surface that pushes cap down to seal
    let pad_width = 8.0;
    let pad_length = 6.0;
    let pad_thickness = 2.0;

    // ── Build arm ──

    // Main arm bar
    let arm_bar = centered_cube("arm_bar", arm_length, arm_width, arm_thickness)
        .translate(arm_length / 2.0, 0.0, 0.0);

    // Hub at pivot end
    let hub = centered_cylinder("hub", hub_d / 2.0, hub_h, 32);

    // Hook at tip end (L-shaped: horizontal + vertical drop)
    let hook_horiz = centered_cube(
        "hook_horiz",
        hook_length,
        arm_width,
        hook_thickness,
    )
    .translate(arm_length + hook_length / 2.0, 0.0, -(arm_thickness / 2.0) + hook_thickness / 2.0);

    let hook_vert = centered_cube(
        "hook_vert",
        hook_thickness,
        arm_width,
        hook_drop,
    )
    .translate(
        arm_length + hook_length - hook_thickness / 2.0,
        0.0,
        -(arm_thickness / 2.0) - hook_drop / 2.0 + hook_thickness,
    );

    // Press pad on underside of arm near the tip
    let press_pad = centered_cube(
        "press_pad",
        pad_length,
        pad_width,
        pad_thickness,
    )
    .translate(arm_length - pad_length, 0.0, -(arm_thickness / 2.0) - pad_thickness / 2.0);

    // ── Cuts ──

    // Servo horn bore (spline pocket, from bottom of hub)
    let horn_bore = centered_cylinder(
        "horn_bore",
        horn_bore_d / 2.0,
        horn_bore_depth + 0.1,
        32,
    )
    .translate(0.0, 0.0, -(hub_h / 2.0) + horn_bore_depth / 2.0);

    // Horn screw hole (through center)
    let horn_screw = centered_cylinder(
        "horn_screw",
        horn_screw_d / 2.0,
        hub_h + 2.0,
        24,
    );

    // ── Assemble ──

    let capping_arm = (hub + arm_bar + hook_horiz + hook_vert + press_pad)
        - horn_bore
        - horn_screw;

    // ── Export ──

    capping_arm
        .write_stl("output/capping_arm.stl")
        .unwrap();

    println!("Exported: output/capping_arm.stl");
    println!();
    println!("── Capping Arm Specs ──");
    println!("  Arm length:    {arm_length:.1}mm (pivot to tip)");
    println!("  Arm section:   {arm_width:.1}mm × {arm_thickness:.1}mm");
    println!("  Hub:           {hub_d:.1}mm dia × {hub_h:.1}mm");
    println!("  Horn bore:     {horn_bore_d:.1}mm dia × {horn_bore_depth:.1}mm (MG996R 25T spline)");
    println!("  Hook:          {hook_length:.1}mm reach × {hook_drop:.1}mm drop");
    println!("  Press pad:     {pad_length:.1}mm × {pad_width:.1}mm × {pad_thickness:.1}mm");
    println!("  Horn screw:    M3 through-hole");
    println!("  Material:      PETG");
}
