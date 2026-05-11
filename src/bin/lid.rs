use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Snap-On Lid ───
//
// 3D-printed (PETG) lid that snaps onto the enclosure top. Features
// a lip that drops inside the enclosure walls for a secure fit, with
// 8 tube access holes aligned with the tube holder slots so tubes
// can be inserted/removed without removing the lid.
//
// Features:
// - Flat plate sized to match enclosure outer dimensions
// - Inset lip that drops inside enclosure walls (5mm deep, 0.3mm gap)
// - 8 tube access holes (8mm dia) aligned with block tube slots
// - Finger grip cutouts on front and back edges for easy removal
//
// Outer dimensions: 100mm x 76mm x 3mm (plate) + 5mm lip
// Material: PETG

fn main() {
    // ── Lid plate (flat top) ──

    let plate = centered_cube("plate", OUTER_X, OUTER_Y, LID_THICKNESS);

    // ── Lip (drops inside enclosure walls) ──
    // The lip is a rectangular ring that fits inside the enclosure walls.
    // Gap = LID_LIP_CLEARANCE on each side.

    let lip_outer_x = INNER_X - LID_LIP_CLEARANCE * 2.0;
    let lip_outer_y = INNER_Y - LID_LIP_CLEARANCE * 2.0;
    let lip_wall = ENCLOSURE_WALL; // lip wall thickness matches enclosure wall
    let lip_inner_x = lip_outer_x - lip_wall * 2.0;
    let lip_inner_y = lip_outer_y - lip_wall * 2.0;

    let lip_z = -(LID_THICKNESS / 2.0) - LID_LIP_DEPTH / 2.0;

    let lip_outer = centered_cube("lip_outer", lip_outer_x, lip_outer_y, LID_LIP_DEPTH)
        .translate(0.0, 0.0, lip_z);

    let lip_inner = centered_cube("lip_inner", lip_inner_x, lip_inner_y, LID_LIP_DEPTH + 2.0)
        .translate(0.0, 0.0, lip_z);

    let lip = lip_outer - lip_inner;

    // ── Tube access holes (8x, aligned with tube holder slots) ──

    let first_x = first_slot_x();
    let tube_y = shelf_center_y(); // tubes are in the shelf area

    let mut tube_holes = centered_cube("tubeh_init", 0.01, 0.01, 0.01);

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            &format!("tube_access_{i}"),
            LID_TUBE_HOLE_DIAMETER / 2.0,
            LID_THICKNESS + LID_LIP_DEPTH + 2.0, // through plate and lip
            32,
        )
        .translate(x, tube_y, -(LID_LIP_DEPTH / 2.0));
        tube_holes = tube_holes + hole;
    }

    // ── Finger grip cutouts (front and back edges) ──
    // Semicircular cutouts on the front (-Y) and back (+Y) edges
    // for easy grip when removing the lid.

    let grip_width = 25.0;
    let grip_depth = 6.0; // how far the cutout extends into the lid edge
    let grip_height = LID_THICKNESS + 2.0;

    // Front grip (-Y edge)
    let front_grip = centered_cube("front_grip", grip_width, grip_depth, grip_height).translate(
        0.0,
        -(OUTER_Y / 2.0) + grip_depth / 2.0 - 1.0,
        0.0,
    );

    // Rounded front grip (cylinder for ergonomic shape)
    let front_grip_round = centered_cylinder("front_grip_r", grip_depth / 2.0, grip_width, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, -(OUTER_Y / 2.0), 0.0);

    // Back grip (+Y edge)
    let back_grip = centered_cube("back_grip", grip_width, grip_depth, grip_height).translate(
        0.0,
        OUTER_Y / 2.0 - grip_depth / 2.0 + 1.0,
        0.0,
    );

    let back_grip_round = centered_cylinder("back_grip_r", grip_depth / 2.0, grip_width, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, OUTER_Y / 2.0, 0.0);

    // ── Assemble ──

    let lid =
        (plate + lip) - tube_holes - front_grip - front_grip_round - back_grip - back_grip_round;

    // ── Export ──

    lid.write_stl("output/lid.stl").unwrap();

    println!("Exported: output/lid.stl");
    println!();
    println!("── Lid Specs ──");
    println!("  Plate:          {OUTER_X:.1}mm x {OUTER_Y:.1}mm x {LID_THICKNESS:.1}mm");
    println!("  Lip:            {lip_outer_x:.1}mm x {lip_outer_y:.1}mm x {LID_LIP_DEPTH:.1}mm (wall: {lip_wall:.1}mm)");
    println!("  Lip clearance:  {LID_LIP_CLEARANCE:.1}mm per side");
    println!("  Tube holes:     {NUM_SLOTS}x {LID_TUBE_HOLE_DIAMETER:.1}mm dia");
    println!("  Finger grips:   {grip_width:.0}mm x {grip_depth:.0}mm (front + back)");
    println!(
        "  Total height:   {:.1}mm (plate + lip)",
        LID_THICKNESS + LID_LIP_DEPTH
    );
    println!("  Material:       PETG");
}
