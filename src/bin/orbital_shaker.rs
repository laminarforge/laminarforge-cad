use vcad::{centered_cube, centered_cylinder, Part};

// ─── Orbital Shaker ───
//
// 3D-printed (PETG) orbital shaker for mixing cultures, staining, etc.
// A DC motor with an eccentric offset creates circular orbital motion.
// The platform rides on a 608ZZ bearing for smooth rotation.
//
// Exports:
//   - orbital_shaker_base.stl     (base housing with motor mount)
//   - orbital_shaker_platform.stl (shaking platform with flask pegs)

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 1. BASE HOUSING
    // ══════════════════════════════════════════════════════════════

    let base_x = 120.0;
    let base_y = 120.0;
    let base_z = 40.0;
    let wall = 3.0;

    let base_outer = centered_cube("base_outer", base_x, base_y, base_z);
    let base_inner = centered_cube(
        "base_inner",
        base_x - wall * 2.0,
        base_y - wall * 2.0,
        base_z - wall, // floor at bottom
    )
    .translate(0.0, 0.0, wall / 2.0);

    // ── Motor mount pocket (30mm dia, offset 5mm from center for eccentric motion) ──
    let motor_pocket_d = 30.0;
    let motor_pocket_depth = 25.0;
    let eccentric_offset = 5.0;

    let motor_pocket = centered_cylinder(
        "motor_pocket",
        motor_pocket_d / 2.0,
        motor_pocket_depth,
        32,
    )
    .translate(eccentric_offset, 0.0, -(base_z / 2.0) + wall + motor_pocket_depth / 2.0);

    // Motor shaft through-hole (extends up through top)
    let motor_shaft_hole = centered_cylinder("motor_shaft", 5.0 / 2.0, base_z + 2.0, 24)
        .translate(eccentric_offset, 0.0, 0.0);

    // ── Bearing support (608ZZ: 22mm OD, 8mm ID, 7mm thick) ──
    let bearing_od = 22.0;
    let bearing_recess_d = bearing_od + 0.3; // press-fit clearance
    let bearing_recess_depth = 7.5; // bearing thickness + clearance

    let bearing_recess = centered_cylinder(
        "bearing_recess",
        bearing_recess_d / 2.0,
        bearing_recess_depth,
        32,
    )
    .translate(0.0, 0.0, base_z / 2.0 - bearing_recess_depth / 2.0);

    // Bearing center through-hole (8mm bore)
    let bearing_bore = centered_cylinder("bearing_bore", 8.0 / 2.0, base_z + 2.0, 24);

    // ── Power jack hole (8mm, back face) ──
    let power_jack = centered_cylinder("power_jack", 8.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(20.0, base_y / 2.0, -(base_z / 2.0) + wall + 15.0);

    // ── Speed potentiometer hole (7mm, front face) ──
    let speed_pot = centered_cylinder("speed_pot", 7.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -(base_y / 2.0), -(base_z / 2.0) + wall + 15.0);

    // ── Rubber feet (4 corners) ──
    let foot_d = 8.0;
    let foot_depth = 2.0;
    let foot_inset = 15.0;

    let mut feet = Part::empty("feet");
    for (fx, fy) in [
        (-base_x / 2.0 + foot_inset, -base_y / 2.0 + foot_inset),
        (-base_x / 2.0 + foot_inset, base_y / 2.0 - foot_inset),
        (base_x / 2.0 - foot_inset, -base_y / 2.0 + foot_inset),
        (base_x / 2.0 - foot_inset, base_y / 2.0 - foot_inset),
    ] {
        let foot = centered_cylinder("foot", foot_d / 2.0, foot_depth, 24)
            .translate(fx, fy, -(base_z / 2.0) + foot_depth / 2.0);
        feet = feet + foot;
    }

    // Assemble base
    let base = (base_outer - base_inner)
        - motor_pocket
        - motor_shaft_hole
        - bearing_recess
        - bearing_bore
        - power_jack
        - speed_pot
        - feet;

    base.write_stl("output/orbital_shaker_base.stl").unwrap();

    println!("Exported: output/orbital_shaker_base.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. SHAKING PLATFORM
    // ══════════════════════════════════════════════════════════════

    let platform_x = 100.0;
    let platform_y = 100.0;
    let platform_z = 5.0;

    let platform_body = centered_cube("platform", platform_x, platform_y, platform_z);

    // Rubber pad recess (90x90x1mm on top surface)
    let pad_recess = centered_cube("pad_recess", 90.0, 90.0, 1.0)
        .translate(0.0, 0.0, platform_z / 2.0 - 1.0 / 2.0);

    // Center shaft bore (8mm, through platform for bearing shaft)
    let shaft_bore = centered_cylinder("shaft_bore", 8.0 / 2.0, platform_z + 2.0, 24);

    // 4x flask holder pegs at corners (6mm dia, 15mm tall)
    let peg_d = 6.0;
    let peg_height = 15.0;
    let peg_inset = 35.0; // from center to peg center

    let mut pegs = Part::empty("pegs");
    for (px, py) in [
        (-peg_inset, -peg_inset),
        (-peg_inset, peg_inset),
        (peg_inset, -peg_inset),
        (peg_inset, peg_inset),
    ] {
        let peg = centered_cylinder("peg", peg_d / 2.0, peg_height, 24)
            .translate(px, py, platform_z / 2.0 + peg_height / 2.0);
        pegs = pegs + peg;
    }

    let platform = (platform_body + pegs) - pad_recess - shaft_bore;

    platform
        .write_stl("output/orbital_shaker_platform.stl")
        .unwrap();

    println!("Exported: output/orbital_shaker_platform.stl");

    // ── Print specs ──

    println!();
    println!("── Orbital Shaker Specs ──");
    println!("  Base:            {base_x:.0}mm x {base_y:.0}mm x {base_z:.0}mm");
    println!("  Wall thickness:  {wall:.0}mm");
    println!("  Motor pocket:    {motor_pocket_d:.0}mm dia, offset {eccentric_offset:.0}mm (orbital motion)");
    println!("  Bearing:         608ZZ ({bearing_od:.0}mm OD, 8mm ID)");
    println!("  Power jack:      8mm dia (back face)");
    println!("  Speed pot:       7mm dia (front face)");
    println!("  Platform:        {platform_x:.0}mm x {platform_y:.0}mm x {platform_z:.0}mm");
    println!("  Rubber pad:      90mm x 90mm x 1mm recess");
    println!("  Shaft bore:      8mm dia (center)");
    println!("  Flask pegs:      4x {peg_d:.0}mm dia x {peg_height:.0}mm tall at corners");
    println!("  Material:        PETG");
}
