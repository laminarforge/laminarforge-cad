use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Syringe Plunger Pusher ───
//
// 3D-printed (PETG) adapter that connects a T8 lead screw nut
// to the syringe plunger. Rides vertically above the syringe cradle.
//
// The lead screw nut is embedded (press-fit or captured) in the top.
// The bottom has a circular pad that presses on the syringe plunger thumb pad.
// A 4mm guide rod bore prevents rotation (rod runs parallel to lead screw,
// press-fit into motor mount above and cradle below).

fn main() {
    // ── Dimensions ──

    let nut_od = LEADSCREW_NUT_OD; // 22mm
    let nut_h = LEADSCREW_NUT_HEIGHT; // 10mm
    let screw_d = LEADSCREW_DIAMETER; // 8mm

    // Pusher body
    let body_width = nut_od + 6.0; // 28mm
    let body_depth = 16.0;
    let body_height = nut_h + 8.0; // 18mm — nut pocket + push pad

    // Lead screw nut pocket (round with clearance)
    let nut_pocket_d = nut_od + 0.3; // clearance
    let nut_pocket_h = nut_h + 0.5;

    // Lead screw through-hole above nut
    let screw_hole_d = screw_d + 1.0; // 9mm clearance

    // Push pad on bottom: contacts syringe plunger thumb pad
    let pad_d = 10.0; // covers the plunger thumb button
    let pad_h = 3.0;
    let pad_z = -(body_height / 2.0) + pad_h / 2.0;

    // Guide rod bore (sliding fit, 4mm rod + clearance)
    let guide_rod_d = SYRINGE_GUIDE_ROD_DIAMETER; // 4mm
    let guide_bore_d = guide_rod_d + 0.3; // 4.3mm sliding clearance
    let guide_offset = SYRINGE_GUIDE_ROD_OFFSET; // 12mm from lead screw center

    // ── Build body ──

    let body = centered_cube("body", body_width, body_depth, body_height);

    // Push pad (cylinder on bottom face)
    let pad = centered_cylinder("push_pad", pad_d / 2.0, pad_h, 32)
        .translate(0.0, 0.0, pad_z);

    // ── Cuts ──

    // Nut pocket from top
    let nut_pocket = centered_cylinder(
        "nut_pocket",
        nut_pocket_d / 2.0,
        nut_pocket_h,
        48,
    )
    .translate(0.0, 0.0, body_height / 2.0 - nut_pocket_h / 2.0);

    // Lead screw through-hole (above nut, through full height)
    let screw_hole = centered_cylinder(
        "screw_hole",
        screw_hole_d / 2.0,
        body_height + 2.0,
        32,
    );

    // Guide rod bore (sliding fit, through full height)
    let guide_bore = centered_cylinder(
        "guide_bore",
        guide_bore_d / 2.0,
        body_height + 2.0,
        24,
    )
    .translate(guide_offset, 0.0, 0.0);

    // ── Assemble ──

    let pusher = (body + pad)
        - nut_pocket
        - screw_hole
        - guide_bore;

    // ── Export ──

    pusher
        .write_stl("output/syringe_plunger_pusher.stl")
        .unwrap();

    println!("Exported: output/syringe_plunger_pusher.stl");
    println!();
    println!("── Syringe Plunger Pusher Specs ──");
    println!("  Body:            {body_width:.1}mm × {body_depth:.1}mm × {body_height:.1}mm");
    println!("  Nut pocket:      {nut_pocket_d:.1}mm dia × {nut_pocket_h:.1}mm deep (T8 brass nut)");
    println!("  Screw hole:      {screw_hole_d:.1}mm dia (T8 lead screw clearance)");
    println!("  Push pad:        {pad_d:.1}mm dia × {pad_h:.1}mm (contacts plunger)");
    println!("  Guide rod bore:  {guide_bore_d:.1}mm dia at {guide_offset:.0}mm offset (sliding fit on {guide_rod_d:.0}mm rod)");
    println!("  Material:        PETG");
    println!("  Resolution:      {:.4}μL/microstep (11.2nL)", 0.01122);
}
