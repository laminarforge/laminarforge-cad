use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Still Air Box Accessories ───
//
// 3D-printed (PETG) accessories that mount inside the still air box
// for organizing lab consumables during LAMP testing.
//
// Three accessories:
// 1. Tube rack:        Holds 6 PCR tubes (6.5mm dia) in a compact block
// 2. Reagent holder:   Holds 3 reagent bottles (22mm dia) upright
// 3. Swab receiver:    Angled tray with V-groove for docking a swab
//
// All accessories sit on the SAB floor or mount to rails via friction fit.

fn build_tube_rack() -> Part {
    // ── Tube Rack ──
    // Holds 6 PCR tubes in a 6×1 row
    // Block with blind holes, chamfered tops for easy insertion

    let num_tubes: usize = 6;
    let tube_hole_d = 6.5; // matches PCR tube OD
    let tube_spacing = 10.0;

    let block_length = 80.0;
    let block_width = 30.0;
    let block_height = 25.0;

    let hole_depth = 20.0; // tubes sit ~20mm deep, 5mm of solid base

    let body = centered_cube("rack_body", block_length, block_width, block_height);

    // Drill tube holes in a row along X
    let first_x = -((num_tubes as f64 - 1.0) * tube_spacing) / 2.0;
    let mut holes = Part::empty("tube_holes");

    for i in 0..num_tubes {
        let x = first_x + (i as f64) * tube_spacing;
        // Blind hole from top
        let hole = centered_cylinder(
            &format!("tube_{i}"),
            tube_hole_d / 2.0,
            hole_depth + 0.1,
            24,
        )
        .translate(x, 0.0, (block_height - hole_depth) / 2.0 + 0.05);

        // Chamfer at top for easy insertion (45-degree cone)
        let chamfer = Part::cone(
            &format!("chamfer_{i}"),
            tube_hole_d / 2.0 + 1.0, // bottom radius (wider)
            tube_hole_d / 2.0,       // top radius (matches hole)
            1.5,                     // chamfer height
            24,
        )
        .translate(x, 0.0, block_height / 2.0 - 1.5);

        holes = holes + hole + chamfer;
    }

    let rack = body - holes;

    println!("── Tube Rack Specs ──");
    println!("  Block:        {block_length:.0}mm × {block_width:.0}mm × {block_height:.0}mm");
    println!(
        "  Tubes:        {num_tubes}× {tube_hole_d:.1}mm dia holes, {tube_spacing:.0}mm spacing"
    );
    println!(
        "  Hole depth:   {hole_depth:.0}mm (blind, {:.0}mm solid base)",
        block_height - hole_depth
    );
    println!("  Material:     PETG");

    rack
}

fn build_reagent_holder() -> Part {
    // ── Reagent Holder ──
    // Holds 3 reagent bottles (e.g., LAMP master mix, primers, buffer)
    // upright in large-diameter holes

    let num_bottles: usize = 3;
    let bottle_hole_d = 22.0; // standard small reagent bottle diameter
    let bottle_spacing = 30.0;

    let block_length = 100.0;
    let block_width = 35.0;
    let block_height = 30.0;

    let hole_depth = 25.0; // bottles sit 25mm deep

    let body = centered_cube("holder_body", block_length, block_width, block_height);

    // Drill bottle holes
    let first_x = -((num_bottles as f64 - 1.0) * bottle_spacing) / 2.0;
    let mut holes = Part::empty("bottle_holes");

    for i in 0..num_bottles {
        let x = first_x + (i as f64) * bottle_spacing;
        let hole = centered_cylinder(
            &format!("bottle_{i}"),
            bottle_hole_d / 2.0,
            hole_depth + 0.1,
            32,
        )
        .translate(x, 0.0, (block_height - hole_depth) / 2.0 + 0.05);

        // Chamfer for easy insertion
        let chamfer = Part::cone(
            &format!("bchamfer_{i}"),
            bottle_hole_d / 2.0 + 1.5,
            bottle_hole_d / 2.0,
            2.0,
            32,
        )
        .translate(x, 0.0, block_height / 2.0 - 2.0);

        holes = holes + hole + chamfer;
    }

    let holder = body - holes;

    println!("── Reagent Holder Specs ──");
    println!("  Block:        {block_length:.0}mm × {block_width:.0}mm × {block_height:.0}mm");
    println!("  Bottles:      {num_bottles}× {bottle_hole_d:.0}mm dia holes, {bottle_spacing:.0}mm spacing");
    println!(
        "  Hole depth:   {hole_depth:.0}mm (blind, {:.0}mm solid base)",
        block_height - hole_depth
    );
    println!("  Material:     PETG");

    holder
}

fn build_swab_receiver() -> Part {
    // ── Swab Receiver Tray ──
    // Angled tray with a V-groove for docking a swab collector.
    // The swab rolls/slides into the V-groove and stays put.

    let tray_length = 60.0;
    let tray_width = 30.0;
    let tray_height = 15.0;

    // V-groove dimensions
    let groove_width = 14.0; // wide enough for collector OD (12mm) + clearance
    let groove_depth = 8.0; // deep V to cradle the swab

    let body = centered_cube("tray_body", tray_length, tray_width, tray_height);

    // V-groove: stepped rectangular approximation with rounded bottom
    // Narrow bottom + wider top creates a trapezoidal cradle profile
    let groove_bottom = centered_cube(
        "groove_bottom",
        tray_length + 2.0,
        groove_width / 3.0, // narrow bottom
        groove_depth + 0.1,
    )
    .translate(0.0, 0.0, (tray_height - groove_depth) / 2.0 + 0.05);

    // Wider top portion of groove
    let groove_top = centered_cube(
        "groove_top",
        tray_length + 2.0,
        groove_width,
        groove_depth / 2.0 + 0.1,
    )
    .translate(0.0, 0.0, tray_height / 2.0 - groove_depth / 4.0 + 0.05);

    // Rounded bottom of groove for swab to seat
    let groove_round = centered_cylinder(
        "groove_round",
        COLLECTOR_OD / 2.0 + 0.5, // slightly larger than collector
        tray_length + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        0.0,
        0.0,
        tray_height / 2.0 - groove_depth + COLLECTOR_OD / 2.0,
    );

    let tray = body - groove_bottom - groove_top - groove_round;

    println!("── Swab Receiver Tray Specs ──");
    println!("  Tray:         {tray_length:.0}mm × {tray_width:.0}mm × {tray_height:.0}mm");
    println!("  V-groove:     {groove_width:.0}mm wide × {groove_depth:.0}mm deep");
    println!("  Collector fit: {:.1}mm OD swab collector", COLLECTOR_OD);
    println!("  Material:     PETG");

    tray
}

fn main() {
    println!("── Still Air Box Accessories ──");
    println!();

    // ── Tube Rack ──
    let tube_rack = build_tube_rack();
    tube_rack
        .write_stl("output/still_air_box_tube_rack.stl")
        .unwrap();
    println!("Exported: output/still_air_box_tube_rack.stl");
    println!();

    // ── Reagent Holder ──
    let reagent_holder = build_reagent_holder();
    reagent_holder
        .write_stl("output/still_air_box_reagent_holder.stl")
        .unwrap();
    println!("Exported: output/still_air_box_reagent_holder.stl");
    println!();

    // ── Swab Receiver ──
    let swab_receiver = build_swab_receiver();
    swab_receiver
        .write_stl("output/still_air_box_swab_receiver.stl")
        .unwrap();
    println!("Exported: output/still_air_box_swab_receiver.stl");
    println!();

    println!("── All SAB accessories exported ──");
}
