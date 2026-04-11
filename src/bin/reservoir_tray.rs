use vcad::{centered_cube, centered_cylinder, Part};

// ─── Reservoir Tray ───
//
// Bench development fixture: 3D-printed (PLA) tray that holds 3 reagent
// bottles for the v1 LAMP device syringe pump dispensing system.
//
// Bottles (master mix, primers, lysis buffer) sit in cylindrical pockets.
// Silicone tubing (~3mm OD) exits through U-shaped routing notches on the
// back wall of each pocket and runs to the syringe pump via pinch valves.
//
// This is a temporary prototyping fixture for bench testing the v1 fluid
// path (reservoir → tubing → pinch valve → syringe → dispensing tip).
// The production v2 device uses lyophilized reagents in pre-loaded tubes,
// eliminating external bottles entirely. The liquid handler robot (separate
// epic) has its own Peltier-controlled deck-mounted reservoir blocks.
//
// Features:
// - 3 cylindrical pockets: 22mm OD, 25mm deep, 30mm c-to-c spacing
// - Tube routing notches: U-shaped cutouts on back wall per pocket (4mm wide)
// - Per-pocket label recesses on front face (20x8mm, 0.8mm deep)
// - 2x M3 mounting holes on bottom for bench/frame attachment
//
// Dimensions: ~100mm x 35mm x 30mm
// Material: PLA
// Ticket: T-DD17E865

fn main() {
    // ── Dimensions ──

    let tray_length = 100.0; // X
    let tray_width = 35.0; // Y
    let tray_height = 30.0; // Z

    let pocket_diameter = 22.0; // mm (matches 22mm OD reagent bottles)
    let pocket_depth = 25.0; // mm
    let pocket_spacing = 30.0; // center-to-center
    let num_pockets: usize = 3;

    // Tube routing notches — U-shaped cutouts on back wall (+Y side)
    // for silicone tubing (~3mm OD) to exit each pocket
    let notch_width = 4.0; // mm (clearance for 3mm tubing + tolerance)
    let notch_depth = 5.0; // mm (cuts through back wall)

    // Per-pocket label recesses on front face (-Y)
    let label_width = 20.0; // mm (X) — per pocket
    let label_height = 8.0; // mm (Z)
    let label_depth = 0.8; // mm (recess into Y face)

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

    // ── Tube routing notches ──
    // U-shaped cutouts on the back wall (+Y side) of each pocket.
    // Silicone tubing exits here and runs to the syringe pump.
    // Each notch is a rectangular slot from the pocket rim down to
    // just above the pocket floor, cutting through the back wall.

    let notch_height = pocket_depth - 5.0; // leave 5mm floor below notch
    let mut notches = Part::empty("notches");

    for i in 0..num_pockets {
        let x = first_pocket_x + (i as f64) * pocket_spacing;
        let notch = centered_cube(
            &format!("notch_{i}"),
            notch_width,
            notch_depth + 0.1,
            notch_height,
        )
        .translate(
            x,
            tray_width / 2.0 - notch_depth / 2.0 + 0.05,
            (tray_height - notch_height) / 2.0 + 0.5,
        );
        notches = notches + notch;
    }

    // ── Per-pocket label recesses ──
    // Shallow recesses on the front face (-Y), one per pocket,
    // for adhesive labels identifying reagent contents.

    let mut labels = Part::empty("labels");

    for i in 0..num_pockets {
        let x = first_pocket_x + (i as f64) * pocket_spacing;
        let label = centered_cube(
            &format!("label_{i}"),
            label_width,
            label_depth + 0.1,
            label_height,
        )
        .translate(
            x,
            -(tray_width / 2.0) + label_depth / 2.0,
            -(tray_height / 2.0) + label_height / 2.0 + 5.0,
        );
        labels = labels + label;
    }

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

    let tray = body - pockets - notches - labels - mount_hole_1 - mount_hole_2;

    // ── Export ──

    tray.write_stl("output/reservoir_tray.stl").unwrap();

    println!("Exported: output/reservoir_tray.stl");
    println!();
    println!("── Reservoir Tray Specs ──");
    println!("  Body:           {tray_length:.0}mm x {tray_width:.0}mm x {tray_height:.0}mm");
    println!("  Pockets:        {num_pockets}x {pocket_diameter:.0}mm dia x {pocket_depth:.0}mm deep");
    println!("  Pocket spacing: {pocket_spacing:.0}mm center-to-center");
    println!("  Tube notches:   {num_pockets}x {notch_width:.0}mm wide on back wall (for 3mm silicone tubing)");
    println!("  Label recesses: {num_pockets}x {label_width:.0}mm x {label_height:.0}mm x {label_depth:.1}mm (front face, per pocket)");
    println!("  Mounting holes: 2x M3 ({mount_hole_d:.1}mm) at {mount_hole_spacing:.0}mm spacing");
    println!("  Material:       PLA");
    println!();
    println!("── Context ──");
    println!("  Bench fixture for v1 LAMP device fluid path development.");
    println!("  Bottles: master mix, primers, lysis buffer (22mm OD).");
    println!("  Tubing routes through back-wall notches to syringe pump.");
    println!("  NOT a product component — v2 uses lyophilized reagents.");
}
