use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Chip Adapter Plate ───
//
// 3D-printed (PETG) adapter plate for the Rev C 16-chamber microfluidic chip
// on the liquid handler deck. Provides repeatable chip positioning via dowel
// pin registration and M3 clamp holes.
//
// Ticket: T-48ECF88A
// Spec: ANSI/SLAS pocket (127.76×85.48mm), dowel pin corners, M3 clamps
// Must mate with rocker platform.
//
// Features:
// - ANSI/SLAS pocket sized for Rev C chip (0.2mm clearance per side)
// - 3× dowel pin holes at pocket corners (3mm pins, press-fit)
//   - Two at -X/-Y corner (constrains X and Y)
//   - One at +X/-Y corner (constrains rotation)
//   - +X/+Y corner open for easy chip insertion
// - 4× M3 clamp holes along pocket edges (for hold-down clips)
// - 4× M3 deck mounting holes at plate corners
//
// Material: PETG
// Print orientation: flat (Z = plate thickness)

fn main() {
    // ── Plate dimensions ──

    let plate_margin = 12.0; // margin around pocket on each side
    let plate_length = REVC_CHIP_LENGTH + plate_margin * 2.0; // ~151.76mm
    let plate_width = REVC_CHIP_WIDTH + plate_margin * 2.0; // ~109.48mm
    let plate_thickness = 10.0; // mm structural thickness

    // ── Pocket dimensions ──
    // 0.2mm clearance per side for easy chip insertion
    let pocket_clearance = 0.2;
    let pocket_length = REVC_CHIP_LENGTH + pocket_clearance * 2.0; // ~128.16mm
    let pocket_width = REVC_CHIP_WIDTH + pocket_clearance * 2.0; // ~85.88mm
    let pocket_depth = 6.0; // chip protrudes ~8.35mm above plate (14.35 - 6.0)

    // ── Dowel pin registration ──
    // 3mm steel dowel pins press-fit into plate, extend above pocket floor
    // Positioned just outside the chip footprint at 3 corners
    let dowel_pin_d = 3.0; // pin diameter
    let dowel_hole_d = 2.9; // press-fit bore (0.1mm interference)
    let dowel_hole_depth = 8.0; // press-fit depth into plate
    let _dowel_pin_height_above = 10.0; // sticks up above pocket floor

    // Dowel positions relative to pocket center
    // Pin edge contacts chip edge — pin center offset by chip_dim/2 + pin_r

    // ── M3 clamp holes ──
    // For spring or screw-down hold-down clips to secure the chip
    let clamp_hole_d = 3.2; // M3 clearance
    let clamp_cbore_d = 6.5; // counterbore for M3 button head
    let clamp_cbore_depth = 3.0;
    // 4 clamp holes — one per edge, centered on edge
    let clamp_offset_from_pocket = 5.0; // mm from pocket edge toward plate edge

    // ── Deck mounting holes ──
    // 4× M3 near plate corners for bolting to rocker platform / deck
    let mount_hole_d = 3.2; // M3 clearance
    let mount_inset = 6.0; // from plate corner

    // ── Build plate body ──

    let body = centered_cube("body", plate_length, plate_width, plate_thickness);

    // ── Chip pocket ──
    // Cut from the top face, centered on the plate
    let pocket = centered_cube("pocket", pocket_length, pocket_width, pocket_depth + 0.1)
        .translate(0.0, 0.0, (plate_thickness - pocket_depth) / 2.0 + 0.05);

    // ── Dowel pin holes ──
    // Press-fit holes drilled into the pocket floor (not through the plate)
    // 3 pins at 3 corners of the pocket

    let mut dowel_holes = Part::empty("dowels");

    // Pin 1: -X edge, near -Y corner (constrains X)
    // Pin holes IN the pocket floor at the chip edge
    // Better: small post holes at the corners of the chip footprint
    // The dowel pin sits in a hole in the pocket floor, extends upward
    // Chip corner rests against the pin

    // Revised: pins at inner pocket corners, contacting chip edges
    // Pin 1: -X/-Y corner — contacts both -X and -Y edges
    let p1_x_pos = -(REVC_CHIP_LENGTH / 2.0) - dowel_pin_d / 2.0 - 0.05;
    let p1_y_pos = -(REVC_CHIP_WIDTH / 2.0) - dowel_pin_d / 2.0 - 0.05;

    // Pin 2: -X edge, near +Y corner — contacts -X edge
    let p2_x_pos = -(REVC_CHIP_LENGTH / 2.0) - dowel_pin_d / 2.0 - 0.05;
    let p2_y_pos = (REVC_CHIP_WIDTH / 2.0) + dowel_pin_d / 2.0 + 0.05;

    // Pin 3: +X edge, near -Y corner — contacts -Y edge
    let p3_x_pos = (REVC_CHIP_LENGTH / 2.0) + dowel_pin_d / 2.0 + 0.05;
    let p3_y_pos = -(REVC_CHIP_WIDTH / 2.0) - dowel_pin_d / 2.0 - 0.05;

    // Dowel holes through the pocket floor into the plate body
    let pocket_floor_z = (plate_thickness / 2.0) - pocket_depth;
    for (i, (px, py)) in [
        (p1_x_pos, p1_y_pos),
        (p2_x_pos, p2_y_pos),
        (p3_x_pos, p3_y_pos),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("dowel_hole_{i}"),
            dowel_hole_d / 2.0,
            dowel_hole_depth + 1.0,
            32,
        )
        .translate(*px, *py, pocket_floor_z - dowel_hole_depth / 2.0);
        dowel_holes = dowel_holes + hole;
    }

    // ── Chamfered entry at open corner (+X/+Y) ──
    // A small ramp to guide the chip into position
    let chamfer_size = 8.0;
    let chamfer = centered_cube(
        "chamfer",
        chamfer_size * 2.0,
        chamfer_size * 2.0,
        pocket_depth + 0.2,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        pocket_length / 2.0,
        pocket_width / 2.0,
        (plate_thickness - pocket_depth) / 2.0 + 0.05,
    );

    // ── M3 clamp holes ──
    // One per edge, centered on that edge, between pocket and plate edge
    let mut clamp_holes = Part::empty("clamp_holes");

    let clamp_positions = [
        // (-X edge, centered Y)
        (-(pocket_length / 2.0 + clamp_offset_from_pocket), 0.0),
        // (+X edge, centered Y)
        (pocket_length / 2.0 + clamp_offset_from_pocket, 0.0),
        // (centered X, -Y edge)
        (0.0, -(pocket_width / 2.0 + clamp_offset_from_pocket)),
        // (centered X, +Y edge)
        (0.0, pocket_width / 2.0 + clamp_offset_from_pocket),
    ];

    for (i, (cx, cy)) in clamp_positions.iter().enumerate() {
        // Through hole
        let hole = centered_cylinder(
            format!("clamp_{i}"),
            clamp_hole_d / 2.0,
            plate_thickness + 2.0,
            24,
        )
        .translate(*cx, *cy, 0.0);

        // Counterbore from bottom
        let cbore = centered_cylinder(
            format!("clamp_cbore_{i}"),
            clamp_cbore_d / 2.0,
            clamp_cbore_depth + 0.1,
            24,
        )
        .translate(
            *cx,
            *cy,
            -(plate_thickness / 2.0) + clamp_cbore_depth / 2.0 - 0.05,
        );

        clamp_holes = clamp_holes + hole + cbore;
    }

    // ── Deck mounting holes ──
    // 4× M3 near plate corners
    let mut mount_holes = Part::empty("mount_holes");

    let mount_positions = [
        (
            -(plate_length / 2.0 - mount_inset),
            -(plate_width / 2.0 - mount_inset),
        ),
        (
            (plate_length / 2.0 - mount_inset),
            -(plate_width / 2.0 - mount_inset),
        ),
        (
            -(plate_length / 2.0 - mount_inset),
            (plate_width / 2.0 - mount_inset),
        ),
        (
            (plate_length / 2.0 - mount_inset),
            (plate_width / 2.0 - mount_inset),
        ),
    ];

    for (i, (mx, my)) in mount_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("mount_{i}"),
            mount_hole_d / 2.0,
            plate_thickness + 2.0,
            24,
        )
        .translate(*mx, *my, 0.0);
        mount_holes = mount_holes + hole;
    }

    // ── Assemble ──

    let plate = body - pocket - chamfer - dowel_holes - clamp_holes - mount_holes;

    // ── Export ──

    plate.write_stl("output/chip_adapter_plate.stl").unwrap();

    println!("Exported: output/chip_adapter_plate.stl");
    println!();
    println!("── Chip Adapter Plate Specs ──");
    println!("  Plate:          {plate_length:.1}mm x {plate_width:.1}mm x {plate_thickness:.0}mm");
    println!(
        "  Pocket:         {pocket_length:.1}mm x {pocket_width:.1}mm x {pocket_depth:.0}mm deep"
    );
    println!(
        "  Clearance:      {pocket_clearance:.1}mm per side (total {:.1}mm)",
        pocket_clearance * 2.0
    );
    println!(
        "  Chip protrusion: {:.1}mm above plate surface",
        REVC_TOTAL_HEIGHT - pocket_depth
    );
    println!(
        "  Dowel pins:     3x {dowel_pin_d:.0}mm (press-fit bore {dowel_hole_d:.1}mm x {dowel_hole_depth:.0}mm)"
    );
    println!("  Clamp holes:    4x M3 ({clamp_hole_d:.1}mm) with counterbore");
    println!("  Mount holes:    4x M3 ({mount_hole_d:.1}mm) at {mount_inset:.0}mm from corners");
    println!("  Open corner:    +X/+Y (chamfered for chip insertion)");
    println!("  Material:       PETG");
}
