use vcad::{centered_cube, centered_cylinder, Part};

// ─── Reservoir Tray ───
//
// 3D-printed (PETG) tray that holds 3 reagent bottles for the
// dispensing system. Bottles sit in cylindrical pockets with
// a flat base for stability. A label slot on the front face
// allows identification of reagent contents.
//
// Features:
// - 3 cylindrical pockets: 22mm OD, 25mm deep, 30mm spacing
// - Flat base for stable surface mounting
// - Label slot on front face (60x10mm recess, 1mm deep)
// - 2x M3 mounting holes on bottom for frame attachment
//
// Dimensions: ~100mm x 35mm x 30mm
// Material: PETG

fn main() {
    // ── Dimensions ──

    let tray_length = 100.0; // X
    let tray_width = 35.0; // Y
    let tray_height = 30.0; // Z

    let pocket_diameter = 22.0; // mm
    let pocket_depth = 25.0; // mm
    let pocket_spacing = 30.0; // center-to-center
    let num_pockets = 3;

    // Label slot on front face (-Y)
    let label_width = 60.0; // mm (X)
    let label_height = 10.0; // mm (Z)
    let label_depth = 1.0; // mm (recess into Y face)

    // Mounting holes
    let mount_hole_d = 3.2; // M3 clearance
    let mount_hole_spacing = 70.0; // center-to-center along X
    let mount_hole_depth = tray_height; // through bottom into body

    // ── Build tray body ──

    let body = centered_cube("body", tray_length, tray_width, tray_height);

    // ── Bottle pockets ──
    // 3 pockets evenly spaced along X, cut from top

    let first_pocket_x = -((num_pockets as f64 - 1.0) * pocket_spacing) / 2.0;
    let mut pockets = Part::empty("pockets");

    for i in 0..num_pockets {
        let x = first_pocket_x + (i as f64) * pocket_spacing;
        let pocket = centered_cylinder(
            &format!("pocket_{i}"),
            pocket_diameter / 2.0,
            pocket_depth + 1.0,
            32,
        )
        .translate(x, 0.0, (tray_height - pocket_depth) / 2.0 + 0.5);
        pockets = pockets + pocket;
    }

    // ── Label slot ──
    // Shallow recess on the front face (-Y) for a label/sticker

    let label_slot = centered_cube(
        "label_slot",
        label_width,
        label_depth + 0.1,
        label_height,
    )
    .translate(0.0, -(tray_width / 2.0) + label_depth / 2.0, -(tray_height / 2.0) + label_height / 2.0 + 5.0);

    // ── Mounting holes ──
    // Through the bottom face for M3 bolts

    let mount_hole_1 = centered_cylinder(
        "mount_hole_1",
        mount_hole_d / 2.0,
        mount_hole_depth + 2.0,
        24,
    )
    .translate(mount_hole_spacing / 2.0, 0.0, 0.0);

    let mount_hole_2 = centered_cylinder(
        "mount_hole_2",
        mount_hole_d / 2.0,
        mount_hole_depth + 2.0,
        24,
    )
    .translate(-(mount_hole_spacing / 2.0), 0.0, 0.0);

    // ── Assemble ──

    let tray = body - pockets - label_slot - mount_hole_1 - mount_hole_2;

    // ── Export ──

    tray.write_stl("output/reservoir_tray.stl").unwrap();

    println!("Exported: output/reservoir_tray.stl");
    println!();
    println!("── Reservoir Tray Specs ──");
    println!("  Body:           {tray_length:.0}mm x {tray_width:.0}mm x {tray_height:.0}mm");
    println!("  Pockets:        {num_pockets}x {pocket_diameter:.0}mm dia x {pocket_depth:.0}mm deep");
    println!("  Pocket spacing: {pocket_spacing:.0}mm center-to-center");
    println!("  Label slot:     {label_width:.0}mm x {label_height:.0}mm x {label_depth:.0}mm recess (front face)");
    println!("  Mounting holes: 2x M3 ({mount_hole_d:.1}mm) at {mount_hole_spacing:.0}mm spacing");
    println!("  Material:       PETG");
}
